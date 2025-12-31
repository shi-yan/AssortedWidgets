//! Multi-line text input widget implementation

use std::any::Any;
use std::cell::RefCell;
use std::time::Instant;

use taffy::Style;

use crate::event::input::{EventResponse, InputEventEnum, KeyEvent, MouseEvent};
use crate::event::{ImeEvent, ImeEventType, Key, NamedKey, MouseHandler, WheelEvent};
use crate::paint::primitives::Color;
use crate::paint::types::ShapeStyle;
use crate::paint::PaintContext;
use crate::text::{TextLayout, TextStyle, Truncate};
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId, CursorType, FrameInfo};
use crate::widget::Widget;
use crate::widgets::{Padding, ScrollBar};

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
    cursor_pos: usize,               // Character index in full text
    selection_start: Option<usize>,  // None = no selection
    cursor_blink_timer: Instant,
    drag_start_pos: Option<Point>,   // For drag selection

    // === Preferred Column (for up/down navigation) ===
    /// When moving up/down, try to maintain this X position
    preferred_cursor_x: Option<f32>,

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

            cursor_pos: 0,
            selection_start: None,
            cursor_blink_timer: Instant::now(),
            drag_start_pos: None,
            preferred_cursor_x: None,

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
        self.cursor_pos = self.text.chars().count();
        *self.cached_layout.borrow_mut() = None;
        self
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

        // Calculate byte position
        let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
        let byte_pos = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

        // Insert new text
        self.text.insert_str(byte_pos, text);
        self.cursor_pos += text.chars().count();
        self.selection_start = None;
        self.preferred_cursor_x = None; // Reset preferred X when typing

        // Invalidate layout
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
        if self.cursor_pos == 0 && self.selection_start.is_none() {
            return;
        }

        self.save_undo_state();

        if self.selection_start.is_some() {
            self.delete_selection();
        } else if self.cursor_pos > 0 {
            let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
            let byte_start = char_indices[self.cursor_pos - 1];
            let byte_end = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

            self.text.replace_range(byte_start..byte_end, "");
            self.cursor_pos -= 1;
        }

        self.preferred_cursor_x = None;
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
        if self.cursor_pos >= self.text.chars().count() && self.selection_start.is_none() {
            return;
        }

        self.save_undo_state();

        if self.selection_start.is_some() {
            self.delete_selection();
        } else {
            let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
            let byte_start = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());
            let byte_end = char_indices.get(self.cursor_pos + 1).copied().unwrap_or(self.text.len());

            self.text.replace_range(byte_start..byte_end, "");
        }

        self.preferred_cursor_x = None;
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
            let (start, end) = if sel_start < self.cursor_pos {
                (sel_start, self.cursor_pos)
            } else {
                (self.cursor_pos, sel_start)
            };

            let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
            let byte_start = char_indices[start];
            let byte_end = char_indices.get(end).copied().unwrap_or(self.text.len());

            self.text.replace_range(byte_start..byte_end, "");
            self.cursor_pos = start;
            self.selection_start = None;
        }
    }

    /// Get selected text (for copy/cut)
    fn get_selected_text(&self) -> Option<String> {
        if let Some(sel_start) = self.selection_start {
            let (start, end) = if sel_start < self.cursor_pos {
                (sel_start, self.cursor_pos)
            } else {
                (self.cursor_pos, sel_start)
            };

            let text_chars: Vec<char> = self.text.chars().collect();
            Some(text_chars[start..end].iter().collect())
        } else {
            None
        }
    }

    // ========================================================================
    // Cursor Movement
    // ========================================================================

    /// Move cursor left by one character
    fn move_cursor_left(&mut self, extend_selection: bool) {
        if self.cursor_pos > 0 {
            if extend_selection {
                if self.selection_start.is_none() {
                    self.selection_start = Some(self.cursor_pos);
                }
                self.cursor_pos -= 1;
            } else {
                if self.selection_start.is_some() {
                    // Collapse selection to left
                    let sel_start = self.selection_start.unwrap();
                    self.cursor_pos = sel_start.min(self.cursor_pos);
                    self.selection_start = None;
                } else {
                    self.cursor_pos -= 1;
                }
            }
            self.preferred_cursor_x = None; // Reset preferred X
            self.cursor_blink_timer = Instant::now();
            self.dirty = true;
        }
    }

    /// Move cursor right by one character
    fn move_cursor_right(&mut self, extend_selection: bool) {
        let max_pos = self.text.chars().count();
        if self.cursor_pos < max_pos {
            if extend_selection {
                if self.selection_start.is_none() {
                    self.selection_start = Some(self.cursor_pos);
                }
                self.cursor_pos += 1;
            } else {
                if self.selection_start.is_some() {
                    // Collapse selection to right
                    let sel_start = self.selection_start.unwrap();
                    self.cursor_pos = sel_start.max(self.cursor_pos);
                    self.selection_start = None;
                } else {
                    self.cursor_pos += 1;
                }
            }
            self.preferred_cursor_x = None; // Reset preferred X
            self.cursor_blink_timer = Instant::now();
            self.dirty = true;
        }
    }

    /// Move cursor up one line
    fn move_cursor_up(&mut self, extend_selection: bool) {
        // Get current cursor position in layout coordinates
        if let Some(layout) = self.cached_layout.borrow().as_ref() {
            let buffer = layout.buffer();

            // Find current cursor position in layout
            let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
            let byte_pos = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

            // Find which line the cursor is on
            let mut current_line_y = 0.0_f32;
            let mut current_line_idx = 0;
            let mut found = false;

            for (line_idx, run) in buffer.layout_runs().enumerate() {
                // Check if cursor byte is in this line's range
                if let Some(last_glyph) = run.glyphs.last() {
                    if byte_pos <= last_glyph.end {
                        current_line_y = run.line_y;
                        current_line_idx = line_idx;
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                return; // Already at top or invalid state
            }

            // If already on first line, move to start of line
            if current_line_idx == 0 {
                if extend_selection {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_start = None;
                }
                self.cursor_pos = 0;
                self.cursor_blink_timer = Instant::now();
                self.dirty = true;
                return;
            }

            // Get or calculate preferred X position
            let target_x = if let Some(x) = self.preferred_cursor_x {
                x
            } else {
                // Calculate current X position
                let mut x = 0.0;
                for run in buffer.layout_runs() {
                    for glyph in run.glyphs.iter() {
                        if glyph.start == byte_pos {
                            x = glyph.x;
                            break;
                        }
                        if byte_pos < glyph.end {
                            x = glyph.x;
                            break;
                        }
                    }
                }
                self.preferred_cursor_x = Some(x);
                x
            };

            // Move to previous line
            let line_height = self.font_size * 1.2; // Approximate line height
            let target_y = current_line_y - line_height;

            // Hit test at (target_x, target_y)
            if let Some(cursor) = buffer.hit(target_x, target_y) {
                let text_before = &self.text[..cursor.index.min(self.text.len())];
                let new_pos = text_before.chars().count();

                if extend_selection {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_start = None;
                }

                self.cursor_pos = new_pos;
                self.cursor_blink_timer = Instant::now();
                self.dirty = true;
            }
        }
    }

    /// Move cursor down one line
    fn move_cursor_down(&mut self, extend_selection: bool) {
        // Get current cursor position in layout coordinates
        if let Some(layout) = self.cached_layout.borrow().as_ref() {
            let buffer = layout.buffer();

            // Find current cursor position in layout
            let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
            let byte_pos = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

            // Find which line the cursor is on
            let mut current_line_y = 0.0_f32;
            let layout_runs: Vec<_> = buffer.layout_runs().collect();
            let total_lines = layout_runs.len();
            let mut current_line_idx = 0;
            let mut found = false;

            for (line_idx, run) in layout_runs.iter().enumerate() {
                // Check if cursor byte is in this line's range
                if let Some(last_glyph) = run.glyphs.last() {
                    if byte_pos <= last_glyph.end {
                        current_line_y = run.line_y;
                        current_line_idx = line_idx;
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                // Cursor is past all lines, already at end
                return;
            }

            // If already on last line, move to end of text
            if current_line_idx >= total_lines - 1 {
                if extend_selection {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_start = None;
                }
                self.cursor_pos = self.text.chars().count();
                self.cursor_blink_timer = Instant::now();
                self.dirty = true;
                return;
            }

            // Get or calculate preferred X position
            let target_x = if let Some(x) = self.preferred_cursor_x {
                x
            } else {
                // Calculate current X position
                let mut x = 0.0;
                for run in buffer.layout_runs() {
                    for glyph in run.glyphs.iter() {
                        if glyph.start == byte_pos {
                            x = glyph.x;
                            break;
                        }
                        if byte_pos < glyph.end {
                            x = glyph.x;
                            break;
                        }
                    }
                }
                self.preferred_cursor_x = Some(x);
                x
            };

            // Move to next line
            let line_height = self.font_size * 1.2; // Approximate line height
            let target_y = current_line_y + line_height;

            // Hit test at (target_x, target_y)
            if let Some(cursor) = buffer.hit(target_x, target_y) {
                let text_before = &self.text[..cursor.index.min(self.text.len())];
                let new_pos = text_before.chars().count();

                if extend_selection {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_start = None;
                }

                self.cursor_pos = new_pos;
                self.cursor_blink_timer = Instant::now();
                self.dirty = true;
            }
        }
    }

    /// Move cursor to start of line
    fn move_cursor_home(&mut self, extend_selection: bool) {
        // Find start of current line
        let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
        let byte_pos = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

        // Search backwards for newline
        let line_start_byte = self.text[..byte_pos]
            .rfind('\n')
            .map(|pos| pos + 1) // Start after the newline
            .unwrap_or(0); // Or start of text

        let line_start_char = self.text[..line_start_byte].chars().count();

        if extend_selection {
            if self.selection_start.is_none() {
                self.selection_start = Some(self.cursor_pos);
            }
        } else {
            self.selection_start = None;
        }

        self.cursor_pos = line_start_char;
        self.preferred_cursor_x = None;
        self.cursor_blink_timer = Instant::now();
        self.dirty = true;
    }

    /// Move cursor to end of line
    fn move_cursor_end(&mut self, extend_selection: bool) {
        // Find end of current line
        let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
        let byte_pos = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

        // Search forwards for newline
        let line_end_byte = self.text[byte_pos..]
            .find('\n')
            .map(|pos| byte_pos + pos)
            .unwrap_or(self.text.len());

        let line_end_char = self.text[..line_end_byte].chars().count();

        if extend_selection {
            if self.selection_start.is_none() {
                self.selection_start = Some(self.cursor_pos);
            }
        } else {
            self.selection_start = None;
        }

        self.cursor_pos = line_end_char;
        self.preferred_cursor_x = None;
        self.cursor_blink_timer = Instant::now();
        self.dirty = true;
    }

    /// Select all text
    fn select_all(&mut self) {
        self.selection_start = Some(0);
        self.cursor_pos = self.text.chars().count();
        self.preferred_cursor_x = None;
        self.dirty = true;
    }

    // ========================================================================
    // Undo/Redo
    // ========================================================================

    fn save_undo_state(&mut self) {
        let state = UndoState {
            text: self.text.clone(),
            cursor_pos: self.cursor_pos,
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
                cursor_pos: self.cursor_pos,
                selection_start: self.selection_start,
            };
            self.redo_stack.push(current_state);

            // Restore state
            self.text = state.text;
            self.cursor_pos = state.cursor_pos;
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
                cursor_pos: self.cursor_pos,
                selection_start: self.selection_start,
            };
            self.undo_stack.push(current_state);

            // Restore state
            self.text = state.text;
            self.cursor_pos = state.cursor_pos;
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
        if let Some(layout) = self.cached_layout.borrow().as_ref() {
            let buffer = layout.buffer();

            // Find cursor position
            let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
            let byte_pos = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

            // Find which visual line the cursor is on
            let mut cursor_line = 0_u32;
            let mut cursor_x = 0.0_f32;

            for (line_idx, run) in buffer.layout_runs().enumerate() {
                if let Some(last_glyph) = run.glyphs.last() {
                    if byte_pos <= last_glyph.end {
                        cursor_line = line_idx as u32;
                        // Find X position
                        for glyph in run.glyphs.iter() {
                            if glyph.start == byte_pos {
                                cursor_x = glyph.x;
                                break;
                            }
                            if byte_pos < glyph.end {
                                cursor_x = glyph.x;
                                break;
                            }
                        }
                        break;
                    }
                }
            }

            // Vertical scrolling
            let visible_lines = self.num_visible_lines();
            if cursor_line < self.visible_start_line {
                self.visible_start_line = cursor_line;
                if let Some(ref mut vscroll) = self.vscrollbar {
                    vscroll.set_value(cursor_line as i32);
                }
            } else if cursor_line >= self.visible_start_line + visible_lines {
                self.visible_start_line = cursor_line.saturating_sub(visible_lines - 1);
                if let Some(ref mut vscroll) = self.vscrollbar {
                    vscroll.set_value(self.visible_start_line as i32);
                }
            }

            // Horizontal scrolling (if no wrap)
            if !self.wrap_enabled {
                let cursor_x_global = cursor_x as f64 + self.h_scroll_offset;

                if cursor_x_global < 0.0 {
                    self.h_scroll_offset = -(cursor_x as f64);
                } else if cursor_x_global > self.viewport_width {
                    self.h_scroll_offset = self.viewport_width - cursor_x as f64;
                }

                // Clamp
                let max_scroll = (self.max_line_width - self.viewport_width).max(0.0);
                self.h_scroll_offset = self.h_scroll_offset.clamp(-max_scroll, 0.0);
            }
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

        let line_height = self.font_size as f64 * 1.2;
        let text_origin = Point::new(
            content_origin.x + self.h_scroll_offset,
            content_origin.y - (self.visible_start_line as f64 * line_height),
        );

        let rel_x = (position.x - text_origin.x) as f32;
        let rel_y = (position.y - text_origin.y) as f32;

        let buffer = layout.buffer();
        if let Some(cursor) = buffer.hit(rel_x, rel_y) {
            let text_before = &self.text[..cursor.index.min(self.text.len())];
            return Some(text_before.chars().count());
        }

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
        let line_height = self.font_size as f64 * 1.2;
        let text_area = self.get_text_area_rect();
        let text_origin = Point::new(
            text_area.origin.x + self.h_scroll_offset,
            text_area.origin.y - (self.visible_start_line as f64 * line_height),
        );

        // If text is empty, cursor is at the start
        if self.text.is_empty() {
            return Some(Rect::new(
                Point::new(text_origin.x, text_origin.y),
                Size::new(2.0, line_height),
            ));
        }

        let layout = self.cached_layout.borrow();
        let layout = layout.as_ref()?;
        let buffer = layout.buffer();

        let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
        let byte_pos = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

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
                            text_origin.y + run.line_y as f64,
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

        // Cursor at very end of text
        Some(Rect::new(
            Point::new(text_origin.x, text_origin.y),
            Size::new(2.0, line_height),
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
                }
            });
        }

        let text_area = self.get_text_area_rect();
        let line_height = self.font_size as f64 * 1.2;

        // Extend clip rect slightly to show partial lines
        let extended_clip = Rect::new(
            text_area.origin,
            Size::new(text_area.size.width, text_area.size.height + line_height),
        );
        ctx.push_clip(extended_clip);

        let text_origin = Point::new(
            text_area.origin.x + self.h_scroll_offset,
            text_area.origin.y - (self.visible_start_line as f64 * line_height),
        );

        // Draw selection if any
        if self.is_focused {
            if let Some(sel_start) = self.selection_start {
                if let Some(layout) = self.cached_layout.borrow().as_ref() {
                    let (start, end) = if sel_start < self.cursor_pos {
                        (sel_start, self.cursor_pos)
                    } else {
                        (self.cursor_pos, sel_start)
                    };

                    // Draw selection rectangles (can span multiple lines)
                    let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
                    let start_byte = char_indices.get(start).copied().unwrap_or(self.text.len());
                    let end_byte = char_indices.get(end).copied().unwrap_or(self.text.len());

                    let buffer = layout.buffer();
                    for run in buffer.layout_runs() {
                        // Find selection range within this line
                        let line_start = run.glyphs.first().map(|g| g.start).unwrap_or(0);
                        let line_end = run.glyphs.last().map(|g| g.end).unwrap_or(0);

                        if end_byte <= line_start || start_byte >= line_end {
                            continue; // Selection doesn't overlap this line
                        }

                        // Calculate selection bounds within this line
                        let sel_line_start = start_byte.max(line_start);
                        let sel_line_end = end_byte.min(line_end);

                        let mut x_start = None;
                        let mut x_end = None;

                        for glyph in run.glyphs.iter() {
                            if x_start.is_none() && sel_line_start <= glyph.start {
                                x_start = Some(glyph.x);
                            }
                            if sel_line_end <= glyph.end {
                                x_end = Some(glyph.x + glyph.w);
                                break;
                            }
                        }

                        if let (Some(x1), Some(x2)) = (x_start, x_end) {
                            let selection_rect = Rect::new(
                                Point::new(
                                    text_origin.x + x1 as f64,
                                    text_origin.y + run.line_y as f64,
                                ),
                                Size::new((x2 - x1) as f64, run.line_height as f64),
                            );
                            ctx.draw_rect(selection_rect, style.selection_color);
                        }
                    }
                }
            }
        }

        // Draw text
        if !text_to_render.is_empty() {
            if let Some(ref layout) = *self.cached_layout.borrow() {
                let text_color = if self.text.is_empty() {
                    style.placeholder_color
                } else {
                    style.text_color
                };
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
            let elapsed = self.cursor_blink_timer.elapsed().as_millis();
            let blink_phase = (elapsed / BLINK_INTERVAL_MS) % 2;

            if blink_phase == 0 {
                if let Some(cursor_rect) = self.get_cursor_rect() {
                    ctx.draw_rect(cursor_rect, style.cursor_color);
                }
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

    fn dispatch_wheel_event(&mut self, event: &mut WheelEvent) -> EventResponse {
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
        if let Some(_drag_start) = self.drag_start_pos {
            if let Some(char_index) = self.hit_test_position(event.position) {
                if self.selection_start.is_none() && char_index != self.cursor_pos {
                    self.selection_start = Some(self.cursor_pos);
                }
                self.cursor_pos = char_index;
                self.preferred_cursor_x = None;
                self.ensure_cursor_visible();
                self.dirty = true;
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
            self.cursor_pos = char_index;
            self.selection_start = None;
            self.drag_start_pos = Some(event.position);
            self.preferred_cursor_x = None;
            self.cursor_blink_timer = Instant::now();
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
