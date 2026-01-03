//! Multi-line text input widget implementation

use std::any::Any;
use std::cell::RefCell;

use taffy::Style;

use crate::event::input::{EventResponse, InputEventEnum, KeyEvent, MouseEvent};
use crate::event::{ImeEvent, ImeEventType, Key, NamedKey, MouseHandler, WheelEvent};
use crate::paint::primitives::Color;
use crate::paint::types::ShapeStyle;
use crate::paint::PaintContext;
use crate::text::{TextLayout, TextStyle, Truncate};
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId, CursorType, FrameInfo};
use crate::widget::Widget;
use crate::widgets::{Padding, ScrollBar, Cursor};

use super::style::{TextAreaState, TextAreaStyle};

// ============================================================================
// Undo/Redo State
// ============================================================================

/// Snapshot of text area state for undo/redo
#[derive(Clone, Debug)]
struct UndoState {
    text: String,
    cursor_pos: usize,
    selection_start: Option<usize>,
}

// ============================================================================
// TextArea Widget
// ============================================================================

/// A multi-line text input widget
pub struct TextArea {
    // === Essentials ===
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // === Content ===
    text: String,
    preedit_text: String,
    _preedit_cursor: Option<usize>,

    // === Cursor & Selection ===
    cursor: Cursor,                  // Unified cursor with caching (includes line info and preferred_x)
    selection_start: Option<usize>,  // None = no selection (also char index)
    drag_start_pos: Option<Point>,   // For drag selection

    // === Undo/Redo ===
    undo_stack: Vec<UndoState>,
    redo_stack: Vec<UndoState>,
    max_undo_states: usize,

    // === State ===
    is_focused: bool,
    is_hovered: bool,
    is_disabled: bool,
    current_state: TextAreaState,

    // === Styling ===
    normal_style: TextAreaStyle,
    focused_style: TextAreaStyle,
    hovered_style: TextAreaStyle,
    disabled_style: TextAreaStyle,
    error_style: TextAreaStyle,
    font_size: f32,
    padding: Padding,

    // === Wrapping Mode ===
    wrap_enabled: bool, // true = word wrap, false = horizontal scroll

    // === Scrolling State ===
    visible_start_line: u32,
    h_scroll_offset: f64,
    total_lines: u32,
    max_line_width: f64,
    viewport_width: f64,
    viewport_height: f64,
    needs_scroll_to_cursor: bool, // Set after text changes, cleared after scrolling

    // === Embedded Scrollbars ===
    vscrollbar: Option<ScrollBar>,
    hscrollbar: Option<ScrollBar>,
    scrollbar_width: f32,

    // === Optional Features ===
    placeholder: Option<String>,
    validator: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
    validation_error: Option<String>,

    // === Text Rendering ===
    cached_layout: RefCell<Option<TextLayout>>,
    cached_layout_width: RefCell<Option<f32>>,

    // === Callbacks ===
    on_change: Option<Box<dyn FnMut(&str)>>,

    // === Deferred Commands ===
    pending_commands: Vec<DeferredCommand>,
}

impl TextArea {
    /// Create a new multi-line text area
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),

            text: String::new(),
            preedit_text: String::new(),
            _preedit_cursor: None,

            cursor: Cursor::new(),
            selection_start: None,
            drag_start_pos: None,

            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_undo_states: 100,

            is_focused: false,
            is_hovered: false,
            is_disabled: false,
            current_state: TextAreaState::Normal,

            normal_style: TextAreaStyle::normal(),
            focused_style: TextAreaStyle::focused(),
            hovered_style: TextAreaStyle::hovered(),
            disabled_style: TextAreaStyle::disabled(),
            error_style: TextAreaStyle::error(),
            font_size: 14.0,
            padding: Padding::uniform(12.0),

            wrap_enabled: true,

            visible_start_line: 0,
            h_scroll_offset: 0.0,
            total_lines: 0,
            max_line_width: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            needs_scroll_to_cursor: false,

            vscrollbar: None,
            hscrollbar: None,
            scrollbar_width: 12.0,

            placeholder: None,
            validator: None,
            validation_error: None,

            cached_layout: RefCell::new(None),
            cached_layout_width: RefCell::new(None),

            on_change: None,

            pending_commands: Vec::new(),
        }
    }

    // ========================================================================
    // Builder Methods
    // ========================================================================

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Enable or disable text wrapping
    pub fn wrapping(mut self, enabled: bool) -> Self {
        self.wrap_enabled = enabled;
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set validator function
    pub fn validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        self.validator = Some(Box::new(validator));
        self
    }

    /// Set on_change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Set layout style
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        if disabled {
            self.current_state = TextAreaState::Disabled;
        }
        self
    }

    /// Set initial text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self.cursor.set_char_pos(self.text.chars().count());
        self.cursor.update_caches(&self.text);
        *self.cached_layout.borrow_mut() = None;
        self
    }

    // ========================================================================
    // Helper Methods: Byte ↔ Char Conversion
    // ========================================================================

    /// Convert character index to byte position (for cosmic-text API)
    /// Uses cursor caching for the current cursor position (performance optimization)
    fn char_to_byte(&mut self, char_idx: usize) -> usize {
        // Fast path: if asking for current cursor position, use cache
        if char_idx == self.cursor.char_pos() {
            return self.cursor.byte_pos();
        }

        // Slow path: create temporary cursor for other positions
        let temp = Cursor::at(char_idx);
        temp.byte_pos_uncached(&self.text)
    }

    /// Convert byte position to character index (from cosmic-text API)
    fn byte_to_char(&self, byte_pos: usize) -> usize {
        self.text[..byte_pos.min(self.text.len())]
            .chars()
            .count()
    }

    /// Invalidate cached cursor data (call on text change or layout reflow)
    #[inline]
    fn invalidate_cursor_line_cache(&mut self) {
        self.cursor.invalidate_caches();
    }

    // ========================================================================
    // State Management
    // ========================================================================

    fn update_state(&mut self) {
        self.current_state = if self.is_disabled {
            TextAreaState::Disabled
        } else if self.validation_error.is_some() {
            TextAreaState::Error
        } else if self.is_focused {
            TextAreaState::Focused
        } else if self.is_hovered {
            TextAreaState::Hovered
        } else {
            TextAreaState::Normal
        };
    }

    fn get_current_style(&self) -> &TextAreaStyle {
        match self.current_state {
            TextAreaState::Normal => &self.normal_style,
            TextAreaState::Hovered => &self.hovered_style,
            TextAreaState::Focused => &self.focused_style,
            TextAreaState::Disabled => &self.disabled_style,
            TextAreaState::Error => &self.error_style,
        }
    }

    // ========================================================================
    // Text Operations
    // ========================================================================

    /// Insert text at cursor position
    fn insert_text(&mut self, text: &str) {
        // Save undo state
        self.save_undo_state();

        // Delete selection FIRST if any
        if self.selection_start.is_some() {
            self.delete_selection();
        }

        // Get byte position using cached cursor
        let byte_pos = self.cursor.byte_pos();

        // Insert new text
        self.text.insert_str(byte_pos, text);

        // Move cursor forward
        let char_count = text.chars().count();
        self.cursor.move_by(char_count as isize, self.text.chars().count());
        self.cursor.update_caches(&self.text);
        self.selection_start = None;
        self.cursor.clear_preferred_x(); // Reset preferred X when typing

        // Invalidate caches
        *self.cached_layout.borrow_mut() = None;

        // Validate
        self.validate();

        // Emit text changed signal
        self.emit_text_changed();

        // Call on_change callback
        if let Some(ref mut callback) = self.on_change {
            callback(&self.text);
        }

        self.dirty = true;
    }

    /// Delete character before cursor (backspace)
    fn delete_before_cursor(&mut self) {
        if self.cursor.is_at_start() && self.selection_start.is_none() {
            return;
        }

        self.save_undo_state();

        if self.selection_start.is_some() {
            self.delete_selection();
        } else if !self.cursor.is_at_start() {
            let byte_start = self.char_to_byte(self.cursor.char_pos() - 1);
            let byte_end = self.char_to_byte(self.cursor.char_pos());

            self.text.replace_range(byte_start..byte_end, "");
            self.cursor.move_left();
            self.cursor.update_caches(&self.text);
        }

        self.cursor.clear_preferred_x();
        *self.cached_layout.borrow_mut() = None;
        self.validate();
        self.emit_text_changed();

        if let Some(ref mut callback) = self.on_change {
            callback(&self.text);
        }

        self.dirty = true;
    }

    /// Delete character after cursor (delete key)
    fn delete_after_cursor(&mut self) {
        let max_chars = self.text.chars().count();
        if self.cursor.is_at_end(max_chars) && self.selection_start.is_none() {
            return;
        }

        self.save_undo_state();

        if self.selection_start.is_some() {
            self.delete_selection();
        } else {
            let byte_start = self.char_to_byte(self.cursor.char_pos());
            let byte_end = self.char_to_byte(self.cursor.char_pos() + 1);

            self.text.replace_range(byte_start..byte_end, "");
            self.cursor.update_caches(&self.text);
        }

        self.cursor.clear_preferred_x();
        *self.cached_layout.borrow_mut() = None;
        self.validate();
        self.emit_text_changed();

        if let Some(ref mut callback) = self.on_change {
            callback(&self.text);
        }

        self.dirty = true;
    }

    /// Delete selected text
    fn delete_selection(&mut self) {
        if let Some(sel_start) = self.selection_start {
            let cursor_pos = self.cursor.char_pos();
            let (start, end) = if sel_start < cursor_pos {
                (sel_start, cursor_pos)
            } else {
                (cursor_pos, sel_start)
            };

            let byte_start = self.char_to_byte(start);
            let byte_end = self.char_to_byte(end);

            self.text.replace_range(byte_start..byte_end, "");
            self.cursor.set_char_pos(start);
            self.cursor.update_caches(&self.text);
            self.selection_start = None;
        }
    }

    /// Get selected text (for copy/cut)
    fn get_selected_text(&mut self) -> Option<String> {
        if let Some(sel_start) = self.selection_start {
            let cursor_pos = self.cursor.char_pos();
            let (start, end) = if sel_start < cursor_pos {
                (sel_start, cursor_pos)
            } else {
                (cursor_pos, sel_start)
            };

            let byte_start = self.char_to_byte(start);
            let byte_end = self.char_to_byte(end);
            Some(self.text[byte_start..byte_end].to_string())
        } else {
            None
        }
    }

    // ========================================================================
    // Cursor Movement
    // ========================================================================

    /// Move cursor left by one character
    fn move_cursor_left(&mut self, extend_selection: bool) {
        if !self.cursor.is_at_start() {
            if extend_selection {
                if self.selection_start.is_none() {
                    self.selection_start = Some(self.cursor.char_pos());
                }
                self.cursor.move_left();
            } else {
                if self.selection_start.is_some() {
                    // Collapse selection to left
                    let sel_start = self.selection_start.unwrap();
                    self.cursor.set_char_pos(sel_start.min(self.cursor.char_pos()));
                    self.selection_start = None;
                } else {
                    self.cursor.move_left();
                }
            }
            self.cursor.update_caches(&self.text);
            self.cursor.clear_preferred_x(); // Reset preferred X
            self.cursor.reset_blink();
            self.dirty = true;
        }
    }

    /// Move cursor right by one character
    fn move_cursor_right(&mut self, extend_selection: bool) {
        let max_pos = self.text.chars().count();
        if !self.cursor.is_at_end(max_pos) {
            if extend_selection {
                if self.selection_start.is_none() {
                    self.selection_start = Some(self.cursor.char_pos());
                }
                self.cursor.move_right(max_pos);
            } else {
                if self.selection_start.is_some() {
                    // Collapse selection to right
                    let sel_start = self.selection_start.unwrap();
                    self.cursor.set_char_pos(sel_start.max(self.cursor.char_pos()));
                    self.selection_start = None;
                } else {
                    self.cursor.move_right(max_pos);
                }
            }
            self.cursor.update_caches(&self.text);
            self.cursor.clear_preferred_x(); // Reset preferred X
            self.cursor.reset_blink();
            self.dirty = true;
        }
    }

    /// Move cursor up one line
    fn move_cursor_up(&mut self, extend_selection: bool) {

        // Get current cursor position in layout coordinates
        if let Some(layout) = self.cached_layout.borrow().as_ref() {
            let buffer = layout.buffer();

            // Convert global byte position to (line_index, line_relative_byte)
            let global_byte_pos = self.cursor.byte_pos();

            // Find which logical line the cursor is on
            let mut line_byte_start = 0;
            let mut cursor_line_idx = 0;
            for (i, line) in buffer.lines.iter().enumerate() {
                let line_len = line.text().len();
                let line_byte_end = line_byte_start + line_len;

                if global_byte_pos >= line_byte_start && global_byte_pos <= line_byte_end {
                    cursor_line_idx = i;
                    break;
                }

                line_byte_start = line_byte_end + 1; // +1 for newline
            }

            let line_relative_byte = global_byte_pos - line_byte_start;

            // Find the layout run for this line and get its Y position
            let layout_runs: Vec<_> = buffer.layout_runs().collect();
            let current_line_idx = cursor_line_idx;

            if current_line_idx >= layout_runs.len() {
                return;
            }

            let current_run = &layout_runs[current_line_idx];
            let current_line_y = current_run.line_y;

            // If already on first line, move to start of line
            if current_line_idx == 0 {
                if extend_selection {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor.char_pos());
                    }
                } else {
                    self.selection_start = None;
                }
                self.cursor.move_to_start();
                self.cursor.update_caches(&self.text);
                self.cursor.reset_blink();
                self.dirty = true;
                return;
            }

            // Get or calculate preferred X position
            eprintln!("[MOVE_UP] Current line: {}, cursor at line_relative_byte: {}", current_line_idx, line_relative_byte);
            let target_x = if let Some(x) = self.cursor.preferred_x {
                eprintln!("[MOVE_UP] Using PREFERRED cursor X: {:.1}", x);
                x
            } else {
                // Calculate current X position from the current run's glyphs
                eprintln!("[MOVE_UP] No preferred X, calculating from current position...");
                eprintln!("[MOVE_UP] Current run has {} glyphs", current_run.glyphs.len());
                let mut x = 0.0;
                for (i, glyph) in current_run.glyphs.iter().enumerate() {
                    eprintln!("[MOVE_UP]   Glyph {}: start={}, end={}, x={:.1}, w={:.1}",
                             i, glyph.start, glyph.end, glyph.x, glyph.w);
                    if glyph.start == line_relative_byte {
                        x = glyph.x;
                        eprintln!("[MOVE_UP] Found exact match at glyph {}, x={:.1}", i, x);
                        break;
                    }
                    if line_relative_byte < glyph.end {
                        x = glyph.x;
                        eprintln!("[MOVE_UP] Found cursor within glyph {}, x={:.1}", i, x);
                        break;
                    }
                }
                // Check if cursor is at end of line (after last glyph)
                if let Some(last_glyph) = current_run.glyphs.last() {
                    if line_relative_byte >= last_glyph.end {
                        x = last_glyph.x + last_glyph.w;
                        eprintln!("[MOVE_UP] Cursor at end of line, x={:.1}", x);
                    }
                }
                eprintln!("[MOVE_UP] Calculated X: {:.1}, saving as preferred", x);
                self.cursor.preferred_x = Some(x);
                x
            };

            // Move to previous line - use actual line height
            // For moving up, we need the height of the line we're moving TO
            // Estimate using current line's height (should be consistent in most cases)
            let line_height = buffer.layout_runs()
                .nth(current_line_idx)
                .map(|run| run.line_height)
                .unwrap_or(self.font_size * 1.2);
            let target_y = current_line_y - line_height;

            eprintln!("[MOVE_UP] Moving from line {} to target_y={:.1} (line_height={:.1})", current_line_idx, target_y, line_height);

            // Hit test at (target_x, target_y)
            if let Some(mut cursor) = buffer.hit(target_x, target_y) {
                eprintln!("[MOVE_UP] Hit test at ({:.1}, {:.1}) returned: line={}, index={}",
                         target_x, target_y, cursor.line, cursor.index);

                // CRITICAL: Clamp to line end if target_x is beyond the line content
                // Calculate the actual width of the target line
                if cursor.line < layout_runs.len() {
                    let target_run = &layout_runs[cursor.line];
                    let target_line_text = buffer.lines[cursor.line].text();
                    let target_line_len = target_line_text.len();

                    // Calculate the X position at the end of the line
                    let line_end_x = if let Some(last_glyph) = target_run.glyphs.last() {
                        last_glyph.x + last_glyph.w
                    } else {
                        0.0 // Empty line
                    };

                    eprintln!("[MOVE_UP] Target line {}: text='{}', len={}, line_end_x={:.1}",
                             cursor.line, target_line_text, target_line_len, line_end_x);
                    eprintln!("[MOVE_UP] target_x={:.1}, line_end_x={:.1}, needs_clamp={}",
                             target_x, line_end_x, target_x > line_end_x);

                    // If target_x is beyond the line's end, clamp cursor to end of line
                    if target_x > line_end_x && target_line_len > 0 {
                        eprintln!("[MOVE_UP] CLAMPING: cursor.index {} -> {}", cursor.index, target_line_len);
                        cursor.index = target_line_len;
                    } else {
                        eprintln!("[MOVE_UP] NO CLAMP: keeping cursor.index={}", cursor.index);
                    }
                }

                // cosmic-text returns LINE-RELATIVE byte offset
                // Calculate global byte offset
                let line_byte_start: usize = buffer.lines
                    .iter()
                    .take(cursor.line)
                    .map(|line| line.text().len() + 1)  // +1 for newline
                    .sum();

                let global_byte_index = line_byte_start + cursor.index;
                let byte_index = global_byte_index.min(self.text.len());

                eprintln!("[MOVE_UP] line_byte_start={}, cursor.index={}, global_byte_index={}",
                         line_byte_start, cursor.index, global_byte_index);

                // Convert to char index using helper
                let new_pos = self.byte_to_char(byte_index);
                eprintln!("[MOVE_UP] Final cursor position: {} (was {})", new_pos, self.cursor.char_pos());

                if extend_selection {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor.char_pos());
                    }
                } else {
                    self.selection_start = None;
                }

                self.cursor.set_char_pos(new_pos);
                self.cursor.update_caches(&self.text);
                self.cursor.reset_blink();
                self.dirty = true;
            }
        }
    }

    /// Move cursor down one line
    fn move_cursor_down(&mut self, extend_selection: bool) {
        // Get current cursor position in layout coordinates
        if let Some(layout) = self.cached_layout.borrow().as_ref() {
            let buffer = layout.buffer();

            // Convert global byte position to (line_index, line_relative_byte)
            let global_byte_pos = self.cursor.byte_pos();

            // Find which logical line the cursor is on
            let mut line_byte_start = 0;
            let mut cursor_line_idx = 0;
            for (i, line) in buffer.lines.iter().enumerate() {
                let line_len = line.text().len();
                let line_byte_end = line_byte_start + line_len;

                if global_byte_pos >= line_byte_start && global_byte_pos <= line_byte_end {
                    cursor_line_idx = i;
                    break;
                }

                line_byte_start = line_byte_end + 1; // +1 for newline
            }

            let line_relative_byte = global_byte_pos - line_byte_start;

            // Find the layout run for this line and get its Y position
            let layout_runs: Vec<_> = buffer.layout_runs().collect();
            let total_lines = layout_runs.len();
            let current_line_idx = cursor_line_idx;

            if current_line_idx >= layout_runs.len() {
                return;
            }

            let current_run = &layout_runs[current_line_idx];
            let current_line_y = current_run.line_y;

            // If already on last line, move to end of text
            if current_line_idx >= total_lines - 1 {
                if extend_selection {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor.char_pos());
                    }
                } else {
                    self.selection_start = None;
                }
                let end_pos = self.text.chars().count();
                self.cursor.move_to_end(end_pos);
                self.cursor.update_caches(&self.text);
                self.cursor.reset_blink();
                self.dirty = true;
                return;
            }

            // Get or calculate preferred X position
            let target_x = if let Some(x) = self.cursor.preferred_x {
                x
            } else {
                // Calculate current X position from the current run's glyphs
                let mut x = 0.0;
                for glyph in current_run.glyphs.iter() {
                    if glyph.start == line_relative_byte {
                        x = glyph.x;
                        break;
                    }
                    if line_relative_byte < glyph.end {
                        x = glyph.x;
                        break;
                    }
                }
                self.cursor.preferred_x = Some(x);
                x
            };

            // Move to next line - use actual line height from the current run
            let current_run = &layout_runs[current_line_idx];
            let target_y = current_line_y + current_run.line_height;

            // Hit test at (target_x, target_y)
            if let Some(mut cursor) = buffer.hit(target_x, target_y) {
                eprintln!("[MOVE_DOWN] Hit test at ({:.1}, {:.1}) returned: line={}, index={}",
                         target_x, target_y, cursor.line, cursor.index);

                // CRITICAL: Clamp to line end if target_x is beyond the line content
                // Calculate the actual width of the target line
                if cursor.line < layout_runs.len() {
                    let target_run = &layout_runs[cursor.line];
                    let target_line_text = buffer.lines[cursor.line].text();
                    let target_line_len = target_line_text.len();

                    // Calculate the X position at the end of the line
                    let line_end_x = if let Some(last_glyph) = target_run.glyphs.last() {
                        last_glyph.x + last_glyph.w
                    } else {
                        0.0 // Empty line
                    };

                    eprintln!("[MOVE_DOWN] Target line {}: text='{}', len={}, line_end_x={:.1}",
                             cursor.line, target_line_text, target_line_len, line_end_x);
                    eprintln!("[MOVE_DOWN] target_x={:.1}, line_end_x={:.1}, needs_clamp={}",
                             target_x, line_end_x, target_x > line_end_x);

                    // If target_x is beyond the line's end, clamp cursor to end of line
                    if target_x > line_end_x && target_line_len > 0 {
                        eprintln!("[MOVE_DOWN] CLAMPING: cursor.index {} -> {}", cursor.index, target_line_len);
                        cursor.index = target_line_len;
                    } else {
                        eprintln!("[MOVE_DOWN] NO CLAMP: keeping cursor.index={}", cursor.index);
                    }
                }

                // cosmic-text returns LINE-RELATIVE byte offset
                // Calculate global byte offset
                let line_byte_start: usize = buffer.lines
                    .iter()
                    .take(cursor.line)
                    .map(|line| line.text().len() + 1)  // +1 for newline
                    .sum();

                let global_byte_index = line_byte_start + cursor.index;
                let byte_index = global_byte_index.min(self.text.len());

                eprintln!("[MOVE_DOWN] line_byte_start={}, cursor.index={}, global_byte_index={}",
                         line_byte_start, cursor.index, global_byte_index);

                // Convert to char index using helper
                let new_pos = self.byte_to_char(byte_index);
                eprintln!("[MOVE_DOWN] Final cursor position: {} (was {})", new_pos, self.cursor.char_pos());

                if extend_selection {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor.char_pos());
                    }
                } else {
                    self.selection_start = None;
                }

                self.cursor.set_char_pos(new_pos);
                self.cursor.update_caches(&self.text);
                self.cursor.reset_blink();
                self.dirty = true;
            }
        }
    }

    /// Move cursor to start of line
    fn move_cursor_home(&mut self, extend_selection: bool) {
        // Find start of current line using helper
        let byte_pos = self.char_to_byte(self.cursor.char_pos());

        // Search backwards for newline
        let line_start_byte = self.text[..byte_pos]
            .rfind('\n')
            .map(|pos| pos + 1) // Start after the newline
            .unwrap_or(0); // Or start of text

        let line_start_char = self.byte_to_char(line_start_byte);

        if extend_selection {
            if self.selection_start.is_none() {
                self.selection_start = Some(self.cursor.char_pos());
            }
        } else {
            self.selection_start = None;
        }

        self.cursor.set_char_pos(line_start_char);
        self.cursor.update_caches(&self.text);
        self.cursor.clear_preferred_x();
        self.cursor.reset_blink();
        self.dirty = true;
    }

    /// Move cursor to end of line
    fn move_cursor_end(&mut self, extend_selection: bool) {
        // Find end of current line using helper
        let byte_pos = self.char_to_byte(self.cursor.char_pos());

        // Search forwards for newline
        let line_end_byte = self.text[byte_pos..]
            .find('\n')
            .map(|pos| byte_pos + pos)
            .unwrap_or(self.text.len());

        let line_end_char = self.byte_to_char(line_end_byte);

        if extend_selection {
            if self.selection_start.is_none() {
                self.selection_start = Some(self.cursor.char_pos());
            }
        } else {
            self.selection_start = None;
        }

        self.cursor.set_char_pos(line_end_char);
        self.cursor.update_caches(&self.text);
        self.cursor.clear_preferred_x();
        self.cursor.reset_blink();
        self.dirty = true;
    }

    /// Select all text
    fn select_all(&mut self) {
        self.selection_start = Some(0);
        let end_pos = self.text.chars().count();
        self.cursor.move_to_end(end_pos);
        self.cursor.update_caches(&self.text);
        self.cursor.clear_preferred_x();
        self.dirty = true;
    }

    // ========================================================================
    // Undo/Redo
    // ========================================================================

    fn save_undo_state(&mut self) {
        let state = UndoState {
            text: self.text.clone(),
            cursor_pos: self.cursor.snapshot(),
            selection_start: self.selection_start,
        };

        self.undo_stack.push(state);

        // Limit stack size
        if self.undo_stack.len() > self.max_undo_states {
            self.undo_stack.remove(0);
        }

        // Clear redo stack on new edit
        self.redo_stack.clear();
    }

    fn undo(&mut self) {
        if let Some(state) = self.undo_stack.pop() {
            // Save current state to redo stack
            let current_state = UndoState {
                text: self.text.clone(),
                cursor_pos: self.cursor.snapshot(),
                selection_start: self.selection_start,
            };
            self.redo_stack.push(current_state);

            // Restore state
            self.text = state.text;
            self.cursor.restore(state.cursor_pos);
            self.cursor.update_caches(&self.text);
            self.selection_start = state.selection_start;

            *self.cached_layout.borrow_mut() = None;
            self.validate();
            self.emit_text_changed();

            if let Some(ref mut callback) = self.on_change {
                callback(&self.text);
            }

            self.dirty = true;
        }
    }

    fn redo(&mut self) {
        if let Some(state) = self.redo_stack.pop() {
            // Save current state to undo stack
            let current_state = UndoState {
                text: self.text.clone(),
                cursor_pos: self.cursor.snapshot(),
                selection_start: self.selection_start,
            };
            self.undo_stack.push(current_state);

            // Restore state
            self.text = state.text;
            self.cursor.restore(state.cursor_pos);
            self.cursor.update_caches(&self.text);
            self.selection_start = state.selection_start;

            *self.cached_layout.borrow_mut() = None;
            self.validate();
            self.emit_text_changed();

            if let Some(ref mut callback) = self.on_change {
                callback(&self.text);
            }

            self.dirty = true;
        }
    }

    // ========================================================================
    // Validation
    // ========================================================================

    fn validate(&mut self) {
        if let Some(ref validator) = self.validator {
            match validator(&self.text) {
                Ok(()) => {
                    self.validation_error = None;
                }
                Err(err) => {
                    self.validation_error = Some(err);
                }
            }
        }
        self.update_state();
    }

    // ========================================================================
    // Signal Emissions
    // ========================================================================

    fn emit_text_changed(&mut self) {
        self.pending_commands.push(DeferredCommand {
            target: self.id,
            message: GuiMessage::TextChanged(self.id, self.text.clone()),
        });
    }

    fn emit_copy_requested(&mut self) {
        if let Some(text) = self.get_selected_text() {
            self.pending_commands.push(DeferredCommand {
                target: self.id,
                message: GuiMessage::Custom {
                    source: self.id,
                    signal_type: "copy_requested".to_string(),
                    data: Box::new(text),
                },
            });
        }
    }

    fn emit_cut_requested(&mut self) {
        if let Some(text) = self.get_selected_text() {
            self.pending_commands.push(DeferredCommand {
                target: self.id,
                message: GuiMessage::Custom {
                    source: self.id,
                    signal_type: "cut_requested".to_string(),
                    data: Box::new(text),
                },
            });

            // Delete selection after emitting signal
            self.save_undo_state();
            self.delete_selection();
            *self.cached_layout.borrow_mut() = None;
            self.validate();
            self.emit_text_changed();

            if let Some(ref mut callback) = self.on_change {
                callback(&self.text);
            }

            self.dirty = true;
        }
    }

    fn emit_paste_requested(&mut self) {
        self.pending_commands.push(DeferredCommand {
            target: self.id,
            message: GuiMessage::Custom {
                source: self.id,
                signal_type: "paste_requested".to_string(),
                data: Box::new(()),
            },
        });
    }

    // ========================================================================
    // Scrolling
    // ========================================================================

    fn num_visible_lines(&self) -> u32 {
        if self.viewport_height <= 0.0 {
            return 0;
        }

        let line_height = self.font_size as f64 * 1.2;
        if line_height <= 0.0 {
            return 0;
        }

        let full_lines = (self.viewport_height / line_height).floor() as u32;
        full_lines + 1 // +1 for partial line at bottom
    }

    fn update_scrollbars(&mut self) {
        let visible_lines = self.num_visible_lines();
        let needs_vscroll = self.total_lines > visible_lines;
        let needs_hscroll = !self.wrap_enabled && self.max_line_width > self.viewport_width;

        let had_vscroll = self.vscrollbar.is_some();
        let had_hscroll = self.hscrollbar.is_some();

        // Create or destroy vertical scrollbar
        if needs_vscroll {
            if self.vscrollbar.is_none() {
                let total_lines = self.total_lines as i32;
                let visible_lines = self.num_visible_lines() as i32;
                let max_scroll = (total_lines - visible_lines).max(0);
                let vscroll = ScrollBar::vertical(0, max_scroll, visible_lines);
                self.vscrollbar = Some(vscroll);
            } else {
                // Update range
                let total_lines = self.total_lines as i32;
                let visible_lines = self.num_visible_lines() as i32;
                let max_scroll = (total_lines - visible_lines).max(0);
                if let Some(ref mut vscroll) = self.vscrollbar {
                    vscroll.set_range(0, max_scroll);
                    vscroll.set_page_size(visible_lines);
                }
            }
        } else {
            self.vscrollbar = None;
        }

        // Create or destroy horizontal scrollbar
        if needs_hscroll {
            if self.hscrollbar.is_none() {
                let max_scroll = (self.max_line_width - self.viewport_width).max(0.0) as i32;
                let hscroll = ScrollBar::horizontal(0, max_scroll.max(100), 20);
                self.hscrollbar = Some(hscroll);
            } else {
                // Update range
                let max_scroll = (self.max_line_width - self.viewport_width).max(0.0) as i32;
                if let Some(ref mut hscroll) = self.hscrollbar {
                    hscroll.set_range(0, max_scroll.max(100));
                }
            }
        } else {
            self.hscrollbar = None;
            self.h_scroll_offset = 0.0;
        }

        // If scrollbar visibility changed, invalidate layout (viewport width changed!)
        let has_vscroll = self.vscrollbar.is_some();
        let has_hscroll = self.hscrollbar.is_some();
        if had_vscroll != has_vscroll || had_hscroll != has_hscroll {
            *self.cached_layout.borrow_mut() = None;
            self.dirty = true;
        }

        self.position_scrollbars();
    }

    fn position_scrollbars(&mut self) {
        let hscroll_height = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        // Position vertical scrollbar on right edge
        if let Some(ref mut vscroll) = self.vscrollbar {
            vscroll.set_bounds(Rect::new(
                Point::new(
                    self.bounds.origin.x + self.bounds.size.width - self.scrollbar_width as f64,
                    self.bounds.origin.y,
                ),
                Size::new(
                    self.scrollbar_width as f64,
                    self.bounds.size.height - hscroll_height,
                ),
            ));
        }

        // Position horizontal scrollbar on bottom edge
        let vscroll_width = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        if let Some(ref mut hscroll) = self.hscrollbar {
            hscroll.set_bounds(Rect::new(
                Point::new(
                    self.bounds.origin.x,
                    self.bounds.origin.y + self.bounds.size.height - self.scrollbar_width as f64,
                ),
                Size::new(
                    self.bounds.size.width - vscroll_width,
                    self.scrollbar_width as f64,
                ),
            ));
        }
    }

    /// Ensure cursor is visible by adjusting scroll offsets
    fn ensure_cursor_visible(&mut self) {
        eprintln!("[ENSURE_CURSOR] Called (cursor at char {})", self.cursor.char_pos());

        // CRITICAL FIX: If layout is invalidated (None), we can't calculate cursor position yet.
        // This happens after text insertion/deletion. Set a flag and do the scrolling later
        // in update() or paint() after the layout is recreated.
        if self.cached_layout.borrow().is_none() {
            eprintln!("[ENSURE_CURSOR] Layout is None, deferring scroll (setting needs_scroll_to_cursor flag)");
            self.needs_scroll_to_cursor = true;
            self.dirty = true;
            return;
        }

        eprintln!("[ENSURE_CURSOR] Layout is available, calling do_scroll_to_cursor()");
        self.do_scroll_to_cursor();
    }

    /// Perform the actual scrolling to make cursor visible
    /// Called from ensure_cursor_visible() when layout is available,
    /// or from update() after layout is recreated.
    fn do_scroll_to_cursor(&mut self) {
        if let Some(layout) = self.cached_layout.borrow().as_ref() {
            let buffer = layout.buffer();

            // Find cursor position (GLOBAL byte offset in full text)
            let global_byte_pos = self.cursor.byte_pos();

            eprintln!("[CURSOR_SCROLL] Finding cursor line for GLOBAL byte pos: {}", global_byte_pos);

            // Find which visual line the cursor is on and its Y position
            // CRITICAL: cosmic-text uses LINE-RELATIVE byte positions, so we need to track cumulative
            let mut cursor_line = 0_u32;
            let mut cursor_x = 0.0_f32;
            let mut cursor_y = 0.0_f64;
            let mut cursor_height = self.font_size as f64 * 1.2; // fallback

            let mut line_byte_start = 0_usize;
            for (line_idx, line) in buffer.lines.iter().enumerate() {
                let line_len = line.text().len();
                let line_byte_end = line_byte_start + line_len;

                eprintln!("[CURSOR_SCROLL]   Line {}: global byte range [{}, {})", line_idx, line_byte_start, line_byte_end);

                // Check if cursor is in this line (using GLOBAL byte positions)
                if global_byte_pos >= line_byte_start && global_byte_pos <= line_byte_end {
                    cursor_line = line_idx as u32;

                    // Convert to LINE-RELATIVE byte position for glyph lookup
                    let line_relative_byte = global_byte_pos - line_byte_start;

                    eprintln!("[CURSOR_SCROLL]   Cursor found in line {}! Line-relative byte: {}", line_idx, line_relative_byte);

                    // Get the layout run for this line to find Y position and cursor X
                    if let Some(run) = buffer.layout_runs().nth(line_idx) {
                        cursor_y = run.line_top as f64;
                        cursor_height = run.line_height as f64;

                        // Find X position using LINE-RELATIVE byte offset
                        for glyph in run.glyphs.iter() {
                            if glyph.start == line_relative_byte {
                                cursor_x = glyph.x;
                                eprintln!("[CURSOR_SCROLL]   Cursor X at glyph start: {:.1}", cursor_x);
                                break;
                            }
                            if line_relative_byte < glyph.end {
                                cursor_x = glyph.x;
                                eprintln!("[CURSOR_SCROLL]   Cursor X within glyph: {:.1}", cursor_x);
                                break;
                            }
                        }

                        // If cursor is at end of line (after last glyph)
                        if let Some(last_glyph) = run.glyphs.last() {
                            if line_relative_byte >= last_glyph.end {
                                cursor_x = last_glyph.x + last_glyph.w;
                                eprintln!("[CURSOR_SCROLL]   Cursor X at line end: {:.1}", cursor_x);
                            }
                        }
                    }
                    break;
                }

                line_byte_start = line_byte_end + 1; // +1 for newline character
            }

            // Calculate vertical offset for current scroll position (sum of actual line heights)
            let current_offset = buffer.layout_runs()
                .take(self.visible_start_line as usize)
                .map(|run| run.line_height as f64)
                .sum::<f64>();

            // Calculate how many lines are visible
            let total_lines = buffer.layout_runs().count();
            let mut visible_height = 0.0_f64;
            let mut last_visible_line = self.visible_start_line;
            for (i, run) in buffer.layout_runs().enumerate().skip(self.visible_start_line as usize) {
                visible_height += run.line_height as f64;
                if visible_height > self.viewport_height {
                    break;
                }
                last_visible_line = i as u32;
            }

            // Calculate cursor's position relative to viewport
            let cursor_top_in_viewport = cursor_y - current_offset;
            let cursor_bottom_in_viewport = cursor_top_in_viewport + cursor_height;

            eprintln!("[CURSOR_SCROLL] ========================================");
            eprintln!("[CURSOR_SCROLL] Total lines in layout: {}", total_lines);
            eprintln!("[CURSOR_SCROLL] Cursor at line: {} (char pos: {})", cursor_line, self.cursor.char_pos());
            eprintln!("[CURSOR_SCROLL] Visible range: {} to {}", self.visible_start_line, last_visible_line);
            eprintln!("[CURSOR_SCROLL] Viewport height: {:.1}", self.viewport_height);
            eprintln!("[CURSOR_SCROLL] Cursor Y: {:.1}, Height: {:.1}", cursor_y, cursor_height);
            eprintln!("[CURSOR_SCROLL] Cursor top in viewport: {:.1}, bottom: {:.1}", cursor_top_in_viewport, cursor_bottom_in_viewport);

            // Check if cursor is fully visible in viewport
            let needs_scroll = cursor_top_in_viewport < 0.0 || cursor_bottom_in_viewport > self.viewport_height;
            eprintln!("[CURSOR_SCROLL] Cursor is {} visible (needs_scroll: {})",
                     if needs_scroll { "NOT" } else { "FULLY" }, needs_scroll);

            if needs_scroll {
                let old_start = self.visible_start_line;
                if cursor_top_in_viewport < 0.0 {
                    // Cursor is above viewport - scroll up to show it at the top
                    eprintln!("[CURSOR_SCROLL] Cursor above viewport, scrolling UP");
                    self.visible_start_line = cursor_line;
                } else {
                    // Cursor is below viewport - scroll down to show it at the bottom
                    eprintln!("[CURSOR_SCROLL] Cursor below viewport, scrolling DOWN");
                    // Calculate how many lines fit in the viewport starting from the cursor line going up
                    let runs: Vec<_> = buffer.layout_runs().collect();
                    let mut height_sum = 0.0_f64;
                    let mut start_line = cursor_line as usize;

                    for i in (0..=cursor_line as usize).rev() {
                        if let Some(run) = runs.get(i) {
                            height_sum += run.line_height as f64;
                            eprintln!("[CURSOR_SCROLL]   Line {}: height {:.1}, cumulative {:.1}", i, run.line_height, height_sum);
                            if height_sum > self.viewport_height {
                                start_line = i + 1; // This line doesn't fit, start from next one
                                eprintln!("[CURSOR_SCROLL]   Exceeded viewport, starting from line {}", start_line);
                                break;
                            }
                            start_line = i;
                        }
                    }

                    self.visible_start_line = start_line as u32;
                }

                eprintln!("[CURSOR_SCROLL] Scrolled from line {} to line {}", old_start, self.visible_start_line);

                // Update scrollbar
                if let Some(ref mut vscroll) = self.vscrollbar {
                    vscroll.set_value(self.visible_start_line as i32);
                    eprintln!("[CURSOR_SCROLL] Updated scrollbar value to {}", self.visible_start_line);
                }
                self.dirty = true;
            } else {
                eprintln!("[CURSOR_SCROLL] No scrolling needed - cursor is already visible");
            }

            // Horizontal scrolling (if no wrap)
            if !self.wrap_enabled {
                let cursor_x_global = cursor_x as f64 + self.h_scroll_offset;

                if cursor_x_global < 0.0 {
                    self.h_scroll_offset = -(cursor_x as f64);
                    self.dirty = true;
                } else if cursor_x_global > self.viewport_width {
                    self.h_scroll_offset = self.viewport_width - cursor_x as f64;
                    self.dirty = true;
                }

                // Clamp
                let max_scroll = (self.max_line_width - self.viewport_width).max(0.0);
                self.h_scroll_offset = self.h_scroll_offset.clamp(-max_scroll, 0.0);
            }

            self.needs_scroll_to_cursor = false;
        }
    }

    // ========================================================================
    // Hit Testing
    // ========================================================================

    /// Convert click position to character index
    fn hit_test_position(&self, position: Point) -> Option<usize> {
        let layout = self.cached_layout.borrow();
        let layout = layout.as_ref()?;

        let content_origin = Point::new(
            self.bounds.origin.x + self.padding.left as f64,
            self.bounds.origin.y + self.padding.top as f64,
        );

        // Calculate vertical offset by summing actual line heights from layout
        let buffer = layout.buffer();
        let vertical_offset = buffer.layout_runs()
            .take(self.visible_start_line as usize)
            .map(|run| run.line_height as f64)
            .sum::<f64>();

        let text_origin = Point::new(
            content_origin.x + self.h_scroll_offset,
            content_origin.y - vertical_offset,
        );

        let rel_x = (position.x - text_origin.x) as f32;
        let rel_y = (position.y - text_origin.y) as f32;

        eprintln!("[HIT_TEST] position: {:?}, rel_x: {:.1}, rel_y: {:.1}", position, rel_x, rel_y);

        // Cosmic-text hit testing (returns line-relative byte offset)
        if let Some(cursor) = buffer.hit(rel_x, rel_y) {
            eprintln!("[HIT_TEST] cosmic hit: line={}, index={} (LINE-RELATIVE)", cursor.line, cursor.index);

            // cursor.index is LINE-RELATIVE byte offset
            // cursor.line is the logical line index

            // Calculate global byte offset
            let line_byte_start: usize = buffer.lines
                .iter()
                .take(cursor.line)
                .map(|line| line.text().len() + 1)  // +1 for newline
                .sum();

            eprintln!("[HIT_TEST] line_byte_start: {}", line_byte_start);

            let global_byte_index = line_byte_start + cursor.index;
            let byte_index = global_byte_index.min(self.text.len());

            eprintln!("[HIT_TEST] global_byte_index: {}, clamped: {}", global_byte_index, byte_index);

            // Convert to char index using helper
            let char_index = self.byte_to_char(byte_index);

            eprintln!("[HIT_TEST] final char_index: {}", char_index);

            // CRITICAL FIX: Clamp to line end
            // When clicking past the end of a short line, clamp to that line's end
            let line_text = buffer.lines[cursor.line].text();
            let line_char_count = line_text.chars().count();

            // Calculate where this line starts in global char indices
            let mut line_global_start = 0;
            for i in 0..cursor.line {
                line_global_start += buffer.lines[i].text().chars().count() + 1; // +1 for newline
            }

            let line_global_end = line_global_start + line_char_count;

            // Clamp to this line's bounds
            let clamped_char_index = char_index.clamp(line_global_start, line_global_end);

            return Some(clamped_char_index);
        }

        // No hit - return end of text
        Some(self.text.chars().count())
    }

    // ========================================================================
    // Rendering Helpers
    // ========================================================================

    fn get_text_area_rect(&self) -> Rect {
        let vscroll_w = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };
        let hscroll_h = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        Rect::new(
            Point::new(
                self.bounds.origin.x + self.padding.left as f64,
                self.bounds.origin.y + self.padding.top as f64,
            ),
            Size::new(
                (self.bounds.size.width - self.padding.horizontal() as f64 - vscroll_w).max(0.0),
                (self.bounds.size.height - self.padding.vertical() as f64 - hscroll_h).max(0.0),
            ),
        )
    }

    /// Get cursor position for rendering
    fn get_cursor_rect(&self) -> Option<Rect> {
        let text_area = self.get_text_area_rect();

        // If text is empty, cursor is at the start
        if self.text.is_empty() {
            let line_height = self.font_size as f64 * 1.2;
            let cursor_rect = Rect::new(
                Point::new(text_area.origin.x + self.h_scroll_offset, text_area.origin.y),
                Size::new(2.0, line_height),
            );
            return Some(cursor_rect);
        }

        let layout = self.cached_layout.borrow();
        let layout = layout.as_ref()?;
        let buffer = layout.buffer();

        // Calculate vertical offset by summing actual line heights from layout
        let vertical_offset = buffer.layout_runs()
            .take(self.visible_start_line as usize)
            .map(|run| run.line_height as f64)
            .sum::<f64>();

        let text_origin = Point::new(
            text_area.origin.x + self.h_scroll_offset,
            text_area.origin.y - vertical_offset,
        );

        let byte_pos = self.cursor.byte_pos();

        // Find cursor position in layout
        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                if glyph.start == byte_pos {
                    return Some(Rect::new(
                        Point::new(
                            text_origin.x + glyph.x as f64,
                            text_origin.y + run.line_y as f64,
                        ),
                        Size::new(2.0, run.line_height as f64),
                    ));
                }
                if byte_pos < glyph.end {
                    return Some(Rect::new(
                        Point::new(
                            text_origin.x + glyph.x as f64,
                            text_origin.y + run.line_y as f64 - run.line_height as f64,
                        ),
                        Size::new(2.0, run.line_height as f64),
                    ));
                }
            }

            // If cursor is at end of line
            if let Some(last_glyph) = run.glyphs.last() {
                if byte_pos == last_glyph.end {
                    return Some(Rect::new(
                        Point::new(
                            text_origin.x + last_glyph.x as f64 + last_glyph.w as f64,
                            text_origin.y + run.line_y as f64,
                        ),
                        Size::new(2.0, run.line_height as f64),
                    ));
                }
            }
        }

        // Cursor at very end of text (use approximation for fallback height)
        let fallback_height = self.font_size as f64 * 1.2;
        Some(Rect::new(
            Point::new(text_origin.x, text_origin.y),
            Size::new(2.0, fallback_height),
        ))
    }

    // ========================================================================
    // Keyboard Event Handlers
    // ========================================================================

    fn on_key_down(&mut self, event: &mut KeyEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        let cmd = if cfg!(target_os = "macos") {
            event.modifiers.command
        } else {
            event.modifiers.control
        };

        // Handle shortcuts
        if cmd {
            match &event.key {
                Key::Character(c) if *c == 'c' => {
                    self.emit_copy_requested();
                    return EventResponse::Handled;
                }
                Key::Character(c) if *c == 'v' => {
                    self.emit_paste_requested();
                    return EventResponse::Handled;
                }
                Key::Character(c) if *c == 'x' => {
                    self.emit_cut_requested();
                    return EventResponse::Handled;
                }
                Key::Character(c) if *c == 'a' => {
                    self.select_all();
                    return EventResponse::Handled;
                }
                Key::Character(c) if *c == 'z' => {
                    if event.modifiers.shift {
                        self.redo();
                    } else {
                        self.undo();
                    }
                    return EventResponse::Handled;
                }
                Key::Character(c) if *c == 'y' => {
                    self.redo();
                    return EventResponse::Handled;
                }
                _ => {}
            }
        }

        // Handle navigation keys
        match &event.key {
            Key::Named(NamedKey::ArrowLeft) => {
                self.move_cursor_left(event.modifiers.shift);
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::ArrowRight) => {
                self.move_cursor_right(event.modifiers.shift);
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::ArrowUp) => {
                self.move_cursor_up(event.modifiers.shift);
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::ArrowDown) => {
                self.move_cursor_down(event.modifiers.shift);
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::Home) => {
                self.move_cursor_home(event.modifiers.shift);
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::End) => {
                self.move_cursor_end(event.modifiers.shift);
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::Backspace) => {
                self.delete_before_cursor();
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::Delete) => {
                self.delete_after_cursor();
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::Enter) => {
                // Insert newline
                self.insert_text("\n");
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Named(NamedKey::Tab) => {
                // Insert tab (or spaces)
                self.insert_text("    "); // 4 spaces
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            Key::Character(c) => {
                // Insert text
                self.insert_text(&c.to_string());
                self.ensure_cursor_visible();
                return EventResponse::Handled;
            }
            _ => {}
        }

        EventResponse::Ignored
    }

    fn on_key_up(&mut self, _event: &mut KeyEvent) -> EventResponse {
        EventResponse::Ignored
    }

    // ========================================================================
    // IME Event Handlers
    // ========================================================================

    fn on_ime(&mut self, event: &mut ImeEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        match &event.event_type {
            ImeEventType::Preedit(text) => {
                self.preedit_text = text.clone();
                self.dirty = true;
                EventResponse::Handled
            }
            ImeEventType::Commit(text) => {
                self.preedit_text.clear();
                self.insert_text(text);
                self.ensure_cursor_visible();
                EventResponse::Handled
            }
            ImeEventType::Cancel => {
                self.preedit_text.clear();
                self.dirty = true;
                EventResponse::Handled
            }
        }
    }
}

// ============================================================================
// Widget Trait Implementation
// ============================================================================

impl Widget for TextArea {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        if self.bounds != bounds {
            self.bounds = bounds;
            *self.cached_layout.borrow_mut() = None;

            // Update viewport dimensions
            let vscroll_w = if self.vscrollbar.is_some() {
                self.scrollbar_width as f64
            } else {
                0.0
            };
            let hscroll_h = if self.hscrollbar.is_some() {
                self.scrollbar_width as f64
            } else {
                0.0
            };

            self.viewport_width = (bounds.size.width - self.padding.horizontal() as f64 - vscroll_w).max(0.0);
            self.viewport_height = (bounds.size.height - self.padding.vertical() as f64 - hscroll_h).max(0.0);

            self.dirty = true;
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn needs_measure(&self) -> bool {
        true
    }

    fn is_interactive(&self) -> bool {
        !self.is_disabled
    }

    fn is_focusable(&self) -> bool {
        !self.is_disabled
    }

    fn preferred_cursor(&self) -> Option<CursorType> {
        if self.is_disabled {
            None
        } else {
            Some(CursorType::Text)
        }
    }

    fn ime_cursor_rect(&self) -> Option<Rect> {
        if !self.is_focused {
            return None;
        }
        self.get_cursor_rect()
    }

    fn needs_continuous_updates(&self) -> bool {
        self.is_focused // For cursor blinking
    }

    fn update(&mut self, frame_info: &FrameInfo) {
        // Update viewport dimensions
        let vscroll_w = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };
        let hscroll_h = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        self.viewport_width = (self.bounds.size.width - self.padding.horizontal() as f64 - vscroll_w).max(0.0);
        self.viewport_height = (self.bounds.size.height - self.padding.vertical() as f64 - hscroll_h).max(0.0);

        // CRITICAL FIX: Try to scroll to cursor before updating scrollbars
        // This ensures scrollbar ranges are correct for the new scroll position
        if self.needs_scroll_to_cursor {
            if self.cached_layout.borrow().is_some() {
                eprintln!("[UPDATE] needs_scroll_to_cursor is true and layout is available, calling do_scroll_to_cursor()");
                self.do_scroll_to_cursor();
            } else {
                eprintln!("[UPDATE] needs_scroll_to_cursor is true but layout is still None, will retry next frame");
            }
        }

        // Update scrollbars
        self.update_scrollbars();

        // Update scrollbar widgets
        if let Some(ref mut vscroll) = self.vscrollbar {
            vscroll.update(frame_info);
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            hscroll.update(frame_info);
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let style = self.get_current_style();

        // Draw background
        ctx.draw_styled_rect(
            self.bounds,
            ShapeStyle {
                fill: crate::paint::types::Brush::Solid(style.background),
                corner_radius: style.corner_radius.clone(),
                border: style.border.clone(),
                shadow: style.shadow.clone(),
            },
        );

        // Get text to render
        let text_to_render = if self.text.is_empty() {
            self.placeholder.as_deref().unwrap_or("")
        } else {
            &self.text
        };

        // Create layout if needed
        if self.cached_layout.borrow().is_none() && !text_to_render.is_empty() {
            let text_style = TextStyle::new()
                .size(self.font_size)
                .color(if self.text.is_empty() {
                    style.placeholder_color
                } else {
                    style.text_color
                });

            let max_width = if self.wrap_enabled {
                Some(self.viewport_width as f32)
            } else {
                None
            };

            let wrap = if self.wrap_enabled {
                cosmic_text::Wrap::Word
            } else {
                cosmic_text::Wrap::None
            };

            ctx.with_text_engine(|engine| {
                // SAFETY: We're in a rendering context with exclusive access
                // This mutation is needed to cache the layout for immediate use
                unsafe {
                    let this = self as *const Self as *mut Self;

                    let layout = engine.create_layout_with_wrap(
                        text_to_render,
                        &text_style,
                        max_width,
                        Truncate::None,
                        wrap,
                    );

                    // Update metadata
                    let line_height = self.font_size as f64 * 1.2;
                    (*this).total_lines = if line_height > 0.0 {
                        (layout.size().height / line_height).ceil() as u32
                    } else {
                        0
                    };

                    (*this).max_line_width = layout
                        .buffer()
                        .layout_runs()
                        .map(|run| run.line_w as f64)
                        .fold(0.0_f64, |max, w| max.max(w));

                    *self.cached_layout.borrow_mut() = Some(layout);
                    *self.cached_layout_width.borrow_mut() = max_width;

                    // CRITICAL FIX: If we need to scroll to cursor, do it now that layout is ready
                    if (*this).needs_scroll_to_cursor {
                        eprintln!("[PAINT] Layout just created and needs_scroll_to_cursor is true, calling do_scroll_to_cursor()");
                        (*this).do_scroll_to_cursor();
                    }
                }
            });
        }

        let text_area = self.get_text_area_rect();

        // Calculate vertical offset by summing actual line heights from layout
        let vertical_offset = if let Some(ref layout) = *self.cached_layout.borrow() {
            layout.buffer().layout_runs()
                .take(self.visible_start_line as usize)
                .map(|run| run.line_height as f64)
                .sum::<f64>()
        } else {
            0.0
        };

        // Extend clip rect slightly to show partial lines (use approximation for clip extension)
        let line_height = self.font_size as f64 * 1.2;
        let extended_clip = Rect::new(
            text_area.origin,
            Size::new(text_area.size.width, text_area.size.height + line_height),
        );
        ctx.push_clip(extended_clip);

        let text_origin = Point::new(
            text_area.origin.x + self.h_scroll_offset,
            text_area.origin.y - vertical_offset,
        );

        // Track cursor position during text rendering (for accurate positioning)
        let mut cursor_rect_from_rendering: Option<Rect> = None;

        // ✅ PERFORMANCE FIX: Use pre-computed cached values (O(1) reads, no loops!)
        // Cache is updated eagerly when cursor moves via update_caches(), so paint() just reads
        let cursor_global_byte_pos = self.cursor.byte_pos();
        let (cursor_line_idx, cursor_line_relative_byte_pos) =
            self.cursor.line_info().unwrap_or((0, 0));


        // Draw selection if any
        if self.is_focused {
            if let Some(sel_start) = self.selection_start {
                if let Some(layout) = self.cached_layout.borrow().as_ref() {
                    let cursor_pos = self.cursor.char_pos();
                    let (start, end) = if sel_start < cursor_pos {
                        (sel_start, cursor_pos)
                    } else {
                        (cursor_pos, sel_start)
                    };

                    eprintln!("[SELECTION_RENDER] selection char range: [{}, {})", start, end);

                    // Convert char indices to GLOBAL byte offsets
                    let start_byte = Cursor::at(start).byte_pos_uncached(&self.text);
                    let end_byte = Cursor::at(end).byte_pos_uncached(&self.text);

                    eprintln!("[SELECTION_RENDER] selection GLOBAL byte range: [{}, {})", start_byte, end_byte);

                    let buffer = layout.buffer();

                    // Track global byte position as we iterate through lines
                    let mut line_global_byte_start = 0;

                    for (line_idx, run) in buffer.layout_runs().enumerate() {
                        // Calculate this line's GLOBAL byte range
                        let line_text = buffer.lines[run.line_i].text();
                        let line_byte_len = line_text.len();
                        let line_global_byte_end = line_global_byte_start + line_byte_len;

                        eprintln!("[SELECTION_RENDER] line {}: GLOBAL byte range [{}, {})",
                                 line_idx, line_global_byte_start, line_global_byte_end);

                        // Check if selection overlaps this line (using GLOBAL coordinates)
                        if end_byte <= line_global_byte_start || start_byte >= line_global_byte_end {
                            eprintln!("[SELECTION_RENDER] line {}: no overlap, skipping", line_idx);
                            line_global_byte_start = line_global_byte_end + 1; // +1 for newline
                            continue;
                        }

                        // Calculate selection bounds within this line (GLOBAL coordinates)
                        let sel_global_start = start_byte.max(line_global_byte_start);
                        let sel_global_end = end_byte.min(line_global_byte_end);

                        // Convert to LINE-RELATIVE byte offsets for glyph matching
                        let sel_line_start = sel_global_start - line_global_byte_start;
                        let sel_line_end = sel_global_end - line_global_byte_start;

                        eprintln!("[SELECTION_RENDER] line {}: selection LINE-RELATIVE byte range [{}, {})",
                                 line_idx, sel_line_start, sel_line_end);

                        let mut x_start = None;
                        let mut x_end = None;

                        // Find X positions using LINE-RELATIVE byte offsets
                        for glyph in run.glyphs.iter() {
                            if x_start.is_none() && sel_line_start <= glyph.start {
                                x_start = Some(glyph.x);
                                eprintln!("[SELECTION_RENDER] line {}: x_start at glyph.x={:.1}", line_idx, glyph.x);
                            }
                            if sel_line_end <= glyph.end {
                                x_end = Some(glyph.x + glyph.w);
                                eprintln!("[SELECTION_RENDER] line {}: x_end at glyph.x+w={:.1}", line_idx, glyph.x + glyph.w);
                                break;
                            }
                        }

                        // Handle end of line case
                        if x_end.is_none() && sel_line_end >= run.glyphs.last().map(|g| g.end).unwrap_or(0) {
                            if let Some(last_glyph) = run.glyphs.last() {
                                x_end = Some(last_glyph.x + last_glyph.w);
                                eprintln!("[SELECTION_RENDER] line {}: x_end at line end={:.1}", line_idx, last_glyph.x + last_glyph.w);
                            }
                        }

                        if let (Some(x1), Some(x2)) = (x_start, x_end) {
                            let selection_rect = Rect::new(
                                Point::new(
                                    text_origin.x + x1 as f64,
                                    text_origin.y + run.line_top as f64,
                                ),
                                Size::new((x2 - x1) as f64, run.line_height as f64),
                            );
                            eprintln!("[SELECTION_RENDER] line {}: drawing rect at x={:.1}, y={:.1}, width={:.1}",
                                     line_idx, text_origin.x + x1 as f64, text_origin.y + run.line_top as f64, (x2 - x1) as f64);
                            ctx.draw_rect(selection_rect, style.selection_color);
                        } else {
                            eprintln!("[SELECTION_RENDER] line {}: no valid x_start/x_end, skipping render", line_idx);
                        }

                        line_global_byte_start = line_global_byte_end + 1; // +1 for newline
                    }
                }
            }
        }

        // Draw text and calculate cursor position simultaneously
        if !text_to_render.is_empty() {
            if let Some(ref layout) = *self.cached_layout.borrow() {
                let text_color = if self.text.is_empty() {
                    style.placeholder_color
                } else {
                    style.text_color
                };

                // Render text while tracking cursor position
                let buffer = layout.buffer();
                for run in buffer.layout_runs() {
                    let line_height = run.line_height;

                    // Check if this run is the line containing the cursor
                    if cursor_rect_from_rendering.is_none() && run.line_i == cursor_line_idx {
                        for glyph in run.glyphs.iter() {
                            // Check if cursor is at this glyph's position (LINE-RELATIVE!)
                            if cursor_line_relative_byte_pos == glyph.start {
                                let cursor_x = text_origin.x + glyph.x as f64;
                                let cursor_y = text_origin.y + run.line_top as f64;
                                cursor_rect_from_rendering = Some(Rect::new(
                                    Point::new(cursor_x, cursor_y),
                                    Size::new(2.0, line_height as f64),
                                ));
                                break; // Found cursor, stop searching
                            }
                        }
                    }

                    // Check if cursor is at end of this line (after last glyph)
                    if cursor_rect_from_rendering.is_none() && run.line_i == cursor_line_idx {
                        if let Some(last_glyph) = run.glyphs.last() {
                            if cursor_line_relative_byte_pos == last_glyph.end {
                                let cursor_x = text_origin.x + (last_glyph.x + last_glyph.w) as f64;
                                let cursor_y = text_origin.y + run.line_top as f64;
                                cursor_rect_from_rendering = Some(Rect::new(
                                    Point::new(cursor_x, cursor_y),
                                    Size::new(2.0, line_height as f64),
                                ));
                            }
                        }
                    }

                    // CRITICAL: Handle empty lines (no glyphs)
                    // If this is the cursor's line and it has no glyphs, place cursor at line start
                    if cursor_rect_from_rendering.is_none() && run.line_i == cursor_line_idx {
                        if run.glyphs.is_empty() && cursor_line_relative_byte_pos == 0 {
                            let cursor_x = text_origin.x;
                            let cursor_y = text_origin.y + run.line_top as f64;
                            cursor_rect_from_rendering = Some(Rect::new(
                                Point::new(cursor_x, cursor_y),
                                Size::new(2.0, line_height as f64),
                            ));
                        }
                    }
                }

                // Now draw the text using the standard method
                ctx.draw_layout(layout, text_origin, text_color);
            }
        }

        // Draw preedit text if IME is active
        if !self.preedit_text.is_empty() {
            if let Some(cursor_rect) = self.get_cursor_rect() {
                let preedit_style = TextStyle::new()
                    .size(self.font_size)
                    .color(Color::rgba(1.0, 1.0, 0.0, 1.0));

                ctx.draw_text(
                    &self.preedit_text,
                    &preedit_style,
                    cursor_rect.origin,
                    None,
                );
            }
        }

        // Draw cursor if focused (with blinking)
        if self.is_focused && self.selection_start.is_none() {
            const BLINK_INTERVAL_MS: u128 = 530;
            let elapsed = self.cursor.blink_timer.elapsed().as_millis();
            let blink_phase = (elapsed / BLINK_INTERVAL_MS) % 2;

            if blink_phase == 0 {
                // Use cursor position calculated during text rendering
                let cursor_rect = if let Some(rect) = cursor_rect_from_rendering {
                    rect
                } else if self.text.is_empty() {
                    // Empty text - cursor at origin
                    let line_height = self.font_size as f64 * 1.2;
                    Rect::new(
                        Point::new(text_origin.x, text_origin.y),
                        Size::new(2.0, line_height),
                    )
                } else {
                    // This should never happen now that we handle empty lines
                    return;
                };

                // Use draw_styled_rect to ensure z-order works across pipelines
                ctx.draw_styled_rect(
                    cursor_rect,
                    ShapeStyle {
                        fill: crate::paint::types::Brush::Solid(style.cursor_color),
                        corner_radius: crate::paint::types::CornerRadius::zero(),
                        border: None,
                        shadow: None,
                    },
                );
            }
        }

        ctx.pop_clip();

        // Paint scrollbars
        if let Some(ref vscroll) = self.vscrollbar {
            vscroll.paint(ctx);
        }
        if let Some(ref hscroll) = self.hscrollbar {
            hscroll.paint(ctx);
        }

        // Register hitbox
        ctx.register_hitbox(self.id, self.bounds);
    }

    fn on_message(&mut self, message: &GuiMessage) -> Vec<DeferredCommand> {
        if let GuiMessage::Custom {
            signal_type,
            data,
            source,
        } = message
        {
            if signal_type == "value_changed" {
                // Vertical scrollbar changed
                if let Some(ref vscroll) = self.vscrollbar {
                    if *source == vscroll.id() {
                        if let Some(value) = data.downcast_ref::<i32>() {
                            self.visible_start_line = (*value).max(0) as u32;
                            self.dirty = true;
                        }
                    }
                }

                // Horizontal scrollbar changed
                if let Some(ref hscroll) = self.hscrollbar {
                    if *source == hscroll.id() {
                        if let Some(value) = data.downcast_ref::<i32>() {
                            let max_scroll = (self.max_line_width - self.viewport_width).max(0.0);
                            if max_scroll > 0.0 {
                                let max_val = hscroll.max() as f64;
                                if max_val > 0.0 {
                                    let normalized = (*value as f64) / max_val;
                                    self.h_scroll_offset = -(normalized * max_scroll);
                                    self.dirty = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        vec![]
    }

    fn dispatch_mouse_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::MouseMove(e) => MouseHandler::on_mouse_move(self, e),
            InputEventEnum::MouseDown(e) => MouseHandler::on_mouse_down(self, e),
            InputEventEnum::MouseUp(e) => MouseHandler::on_mouse_up(self, e),
            _ => EventResponse::Ignored,
        }
    }

    fn dispatch_key_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::KeyDown(e) => self.on_key_down(e),
            InputEventEnum::KeyUp(e) => self.on_key_up(e),
            _ => EventResponse::Ignored,
        }
    }

    fn on_wheel(&mut self, event: &mut WheelEvent) -> EventResponse {
        // Convert wheel delta to line count for vertical scrolling
        let line_height = self.font_size as f64 * 1.2;
        if line_height <= 0.0 {
            return EventResponse::Ignored;
        }

        let lines_delta = (event.delta.y / line_height).round() as i32;
        if lines_delta != 0 {
            let visible_lines = self.num_visible_lines();
            let max_line = self.total_lines.saturating_sub(visible_lines).max(0);
            let new_line = (self.visible_start_line as i32 + lines_delta).clamp(0, max_line as i32) as u32;

            if new_line != self.visible_start_line {
                self.visible_start_line = new_line;

                // Update vertical scrollbar value
                if let Some(ref mut vscroll) = self.vscrollbar {
                    vscroll.set_value(new_line as i32);
                }

                self.dirty = true;
            }
        }

        // Handle horizontal scrolling if wrapping disabled
        if !self.wrap_enabled && event.delta.x.abs() > 0.001 {
            let new_offset = self.h_scroll_offset - event.delta.x;
            let max_scroll = self.max_line_width - self.viewport_width;
            self.h_scroll_offset = new_offset.clamp(-max_scroll.max(0.0), 0.0);

            // Update horizontal scrollbar
            if let Some(ref mut hscroll) = self.hscrollbar {
                let normalized = if max_scroll > 0.0 {
                    (-self.h_scroll_offset / max_scroll).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let max = hscroll.max();
                if max > 0 {
                    hscroll.set_value((normalized * max as f64) as i32);
                }
            }

            self.dirty = true;
        }

        EventResponse::Handled
    }

    fn dispatch_ime_event(&mut self, event: &mut ImeEvent) -> EventResponse {
        self.on_ime(event)
    }

    fn drain_deferred_commands(&mut self) -> Vec<DeferredCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn on_focus_gained(&mut self) {
        self.is_focused = true;
        self.update_state();
        self.dirty = true;

        // CRITICAL FIX: Ensure cursor is visible when gaining focus
        // This prevents the scrollbar from jumping when pressing arrow keys
        // after manually scrolling to a different position
        self.ensure_cursor_visible();
    }

    fn on_focus_lost(&mut self) {
        self.is_focused = false;
        self.selection_start = None;
        self.update_state();
        self.dirty = true;
    }
}

// ============================================================================
// MouseHandler Implementation
// ============================================================================

impl MouseHandler for TextArea {
    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check if mouse is over scrollbars first
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseMove(event.clone());
                return vscroll.dispatch_mouse_event(&mut input_event);
            }
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseMove(event.clone());
                return hscroll.dispatch_mouse_event(&mut input_event);
            }
        }

        // Handle drag selection
        if let Some(drag_start) = self.drag_start_pos {
            eprintln!("[DRAG_SELECT] drag_start: {:?}, mouse: {:?}", drag_start, event.position);
            eprintln!("[DRAG_SELECT] current cursor_pos: {}, selection_start: {:?}",
                     self.cursor.char_pos(), self.selection_start);

            if let Some(char_index) = self.hit_test_position(event.position) {
                eprintln!("[DRAG_SELECT] hit_test returned char_index: {}", char_index);

                if self.selection_start.is_none() && char_index != self.cursor.char_pos() {
                    self.selection_start = Some(self.cursor.char_pos());
                    eprintln!("[DRAG_SELECT] started selection at: {}", self.cursor.char_pos());
                }
                self.cursor.set_char_pos(char_index);
                self.cursor.update_caches(&self.text);
                self.cursor.clear_preferred_x();
                self.ensure_cursor_visible();
                self.dirty = true;

                eprintln!("[DRAG_SELECT] updated cursor_pos to: {}, selection: {:?}",
                         self.cursor.char_pos(), self.selection_start);
            } else {
                eprintln!("[DRAG_SELECT] hit_test returned None!");
            }
        }

        EventResponse::PassThrough
    }

    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        // Check scrollbars first
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseDown(event.clone());
                return vscroll.dispatch_mouse_event(&mut input_event);
            }
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseDown(event.clone());
                return hscroll.dispatch_mouse_event(&mut input_event);
            }
        }

        // Position cursor
        if let Some(char_index) = self.hit_test_position(event.position) {
            self.cursor.set_char_pos(char_index);
            self.cursor.update_caches(&self.text);
            self.selection_start = None;
            self.drag_start_pos = Some(event.position);
            self.cursor.clear_preferred_x();
            self.cursor.reset_blink();
            self.dirty = true;

            // Double-click selects all
            if event.click_count == 2 {
                self.select_all();
            }
        }

        EventResponse::Handled
    }

    fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Forward to scrollbars if they exist
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseUp(event.clone());
                return vscroll.dispatch_mouse_event(&mut input_event);
            }
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseUp(event.clone());
                return hscroll.dispatch_mouse_event(&mut input_event);
            }
        }

        self.drag_start_pos = None;
        EventResponse::Handled
    }

    fn on_mouse_enter(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if !self.is_disabled {
            self.is_hovered = true;
            self.update_state();
            self.dirty = true;
        }
        EventResponse::Handled
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        self.is_hovered = false;
        self.update_state();
        self.dirty = true;
        EventResponse::Handled
    }
}

impl Default for TextArea {
    fn default() -> Self {
        Self::new()
    }
}
