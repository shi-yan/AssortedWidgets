use std::any::Any;
use std::cell::RefCell;
use std::time::Instant;

use taffy::Style;

use crate::event::input::{EventResponse, InputEventEnum, KeyEvent, MouseEvent};
use crate::event::{ImeEvent, ImeEventType, Key, NamedKey, OsEvent};
use crate::paint::primitives::Color;
use crate::paint::types::{CornerRadius, ShapeStyle};
use crate::paint::PaintContext;
use crate::text::{TextEngine, TextLayout, TextStyle, Truncate};
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;

use super::style::{InputState, InputStyle};

// ============================================================================
// Undo/Redo State
// ============================================================================

/// Snapshot of text input state for undo/redo
#[derive(Clone, Debug)]
struct UndoState {
    text: String,
    cursor_pos: usize,
    selection_start: Option<usize>,
}

// ============================================================================
// TextInput Widget
// ============================================================================

/// A single-line text input widget with comprehensive features
pub struct TextInput {
    // === Essentials ===
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // === Content ===
    text: String,
    preedit_text: String,
    preedit_cursor: Option<usize>,

    // === Cursor & Selection ===
    cursor_pos: usize,               // Character index
    selection_start: Option<usize>,  // None = no selection
    cursor_blink_timer: Instant,
    drag_start_pos: Option<Point>,   // For drag selection

    // === Undo/Redo ===
    undo_stack: Vec<UndoState>,
    redo_stack: Vec<UndoState>,
    max_undo_states: usize,

    // === State ===
    is_focused: bool,
    is_hovered: bool,
    is_disabled: bool,
    current_state: InputState,

    // === Styling ===
    normal_style: InputStyle,
    focused_style: InputStyle,
    hovered_style: InputStyle,
    disabled_style: InputStyle,
    error_style: InputStyle,
    font_size: f32,
    padding: f32,

    // === Optional Features ===
    icon: Option<String>,
    icon_size: f32,
    validator: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
    validation_error: Option<String>,
    placeholder: Option<String>,

    // === Password Mode ===
    is_password: bool,
    show_password_toggle: bool,
    password_revealed: bool,
    eye_button_bounds: Rect,
    eye_button_hovered: bool,

    // === Text Rendering ===
    text_layout: RefCell<Option<TextLayout>>,
    scroll_offset: f32,

    // === Callbacks ===
    on_change: Option<Box<dyn FnMut(&str)>>,
    on_submit: Option<Box<dyn FnMut(&str)>>,

    // === Deferred Commands ===
    pending_commands: Vec<DeferredCommand>,
}

impl TextInput {
    /// Create a new text input
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),

            text: String::new(),
            preedit_text: String::new(),
            preedit_cursor: None,

            cursor_pos: 0,
            selection_start: None,
            cursor_blink_timer: Instant::now(),
            drag_start_pos: None,

            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_undo_states: 100,

            is_focused: false,
            is_hovered: false,
            is_disabled: false,
            current_state: InputState::Normal,

            normal_style: InputStyle::normal(),
            focused_style: InputStyle::focused(),
            hovered_style: InputStyle::hovered(),
            disabled_style: InputStyle::disabled(),
            error_style: InputStyle::error(),
            font_size: 14.0,
            padding: 8.0,

            icon: None,
            icon_size: 20.0,
            validator: None,
            validation_error: None,
            placeholder: None,

            is_password: false,
            show_password_toggle: false,
            password_revealed: false,
            eye_button_bounds: Rect::default(),
            eye_button_hovered: false,

            text_layout: RefCell::new(None),
            scroll_offset: 0.0,

            on_change: None,
            on_submit: None,

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
        *self.text_layout.borrow_mut() = None;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set left-side icon (Material Icons ID)
    pub fn icon(mut self, icon_id: impl Into<String>) -> Self {
        self.icon = Some(icon_id.into());
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

    /// Enable password mode (render as bullets)
    pub fn password(mut self, enabled: bool) -> Self {
        self.is_password = enabled;
        self
    }

    /// Show password toggle button (eye icon)
    pub fn show_password_toggle(mut self, show: bool) -> Self {
        self.show_password_toggle = show;
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

    /// Set on_submit callback (triggered by Enter key)
    pub fn on_submit<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str) + 'static,
    {
        self.on_submit = Some(Box::new(callback));
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
            self.current_state = InputState::Disabled;
        }
        self
    }

    // ========================================================================
    // State Management
    // ========================================================================

    fn update_state(&mut self) {
        self.current_state = if self.is_disabled {
            InputState::Disabled
        } else if self.validation_error.is_some() {
            InputState::Error
        } else if self.is_focused {
            InputState::Focused
        } else if self.is_hovered {
            InputState::Hovered
        } else {
            InputState::Normal
        };
    }

    fn get_current_style(&self) -> &InputStyle {
        match self.current_state {
            InputState::Normal => &self.normal_style,
            InputState::Hovered => &self.hovered_style,
            InputState::Focused => &self.focused_style,
            InputState::Disabled => &self.disabled_style,
            InputState::Error => &self.error_style,
        }
    }

    // ========================================================================
    // Text Operations
    // ========================================================================

    /// Get the display text (masked if password mode)
    fn get_display_text(&self) -> String {
        if self.is_password && !self.password_revealed {
            // Render as bullets
            self.text.chars().map(|_| '•').collect()
        } else {
            self.text.clone()
        }
    }

    /// Insert text at cursor position
    fn insert_text(&mut self, text: &str) {
        // Save undo state
        self.save_undo_state();

        // Delete selection FIRST if any (modifies text and cursor_pos)
        if let Some(sel_start) = self.selection_start {
            self.delete_selection();
        }

        // NOW calculate byte position (after any deletion)
        let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
        let byte_pos = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

        // Insert new text
        self.text.insert_str(byte_pos, text);
        self.cursor_pos += text.chars().count();
        self.selection_start = None;

        // Invalidate text layout
        *self.text_layout.borrow_mut() = None;

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

        if let Some(_) = self.selection_start {
            self.delete_selection();
        } else if self.cursor_pos > 0 {
            let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
            let byte_start = char_indices[self.cursor_pos - 1];
            let byte_end = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());

            self.text.replace_range(byte_start..byte_end, "");
            self.cursor_pos -= 1;
        }

        *self.text_layout.borrow_mut() = None;
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

        if let Some(_) = self.selection_start {
            self.delete_selection();
        } else {
            let char_indices: Vec<_> = self.text.char_indices().map(|(i, _)| i).collect();
            let byte_start = char_indices.get(self.cursor_pos).copied().unwrap_or(self.text.len());
            let byte_end = char_indices.get(self.cursor_pos + 1).copied().unwrap_or(self.text.len());

            self.text.replace_range(byte_start..byte_end, "");
        }

        *self.text_layout.borrow_mut() = None;
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
            self.cursor_blink_timer = Instant::now();
            self.dirty = true;
        }
    }

    /// Move cursor to start of text
    fn move_cursor_home(&mut self, extend_selection: bool) {
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
    }

    /// Move cursor to end of text
    fn move_cursor_end(&mut self, extend_selection: bool) {
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
    }

    /// Select all text
    fn select_all(&mut self) {
        self.selection_start = Some(0);
        self.cursor_pos = self.text.chars().count();
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

            *self.text_layout.borrow_mut() = None;
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

            *self.text_layout.borrow_mut() = None;
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
            *self.text_layout.borrow_mut() = None;
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

    fn emit_submit(&mut self) {
        if let Some(ref mut callback) = self.on_submit {
            callback(&self.text);
        }
    }

    // ========================================================================
    // Rendering Helpers
    // ========================================================================

    /// Get the text area rectangle (excluding padding and icons)
    fn get_text_area_rect(&self) -> Rect {
        let mut rect = self.bounds;
        rect.origin.x += self.padding as f64;
        rect.origin.y += self.padding as f64;
        rect.size.width -= (self.padding * 2.0) as f64;
        rect.size.height -= (self.padding * 2.0) as f64;

        // Account for left icon
        if self.icon.is_some() {
            let icon_width = (self.icon_size + 8.0) as f64; // Icon + spacing
            rect.origin.x += icon_width;
            rect.size.width -= icon_width;
        }

        // Account for eye button
        if self.show_password_toggle && self.is_password {
            let eye_width = 24.0; // Eye button + spacing
            rect.size.width -= eye_width;
        }

        rect
    }

    /// Get cursor X position for a given character index
    fn get_cursor_x_position(&self, char_index: usize) -> f32 {
        let text_area = self.get_text_area_rect();

        // Create or get text layout
        let display_text = self.get_display_text();
        if display_text.is_empty() {
            return text_area.origin.x as f32 - self.scroll_offset;
        }

        // Get layout from cache
        if self.text_layout.borrow().is_none() {
            return text_area.origin.x as f32 - self.scroll_offset;
        }

        let layout = self.text_layout.borrow();
        let layout = layout.as_ref().unwrap();
        let buffer = layout.buffer();

        // Find glyph position for character index
        let mut current_char = 0;
        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                // Convert byte range to character count
                let glyph_chars = display_text[glyph.start..glyph.end].chars().count();

                if current_char == char_index {
                    return text_area.origin.x as f32 + glyph.x - self.scroll_offset;
                }

                current_char += glyph_chars;
            }
        }

        // If past all glyphs, position at end
        if let Some(run) = buffer.layout_runs().last() {
            if let Some(glyph) = run.glyphs.last() {
                return text_area.origin.x as f32 + glyph.x + glyph.w - self.scroll_offset;
            }
        }

        text_area.origin.x as f32 - self.scroll_offset
    }

    /// Ensure cursor is visible by adjusting scroll offset
    fn ensure_cursor_visible(&mut self) {
        let text_area = self.get_text_area_rect();
        let cursor_x = self.get_cursor_x_position(self.cursor_pos);

        // Get cursor X relative to text area
        let relative_x = cursor_x - text_area.origin.x as f32 + self.scroll_offset;

        // Cursor too far left (scrolled past view)
        if relative_x - self.scroll_offset < 0.0 {
            self.scroll_offset = relative_x;
        }

        // Cursor too far right (beyond visible area)
        if relative_x - self.scroll_offset > text_area.size.width as f32 {
            self.scroll_offset = relative_x - text_area.size.width as f32;
        }

        // Clamp scroll offset
        self.scroll_offset = self.scroll_offset.max(0.0);
    }

    /// Hit test to convert X position to character index
    fn hit_test_position(&self, x: f32) -> Option<usize> {
        let text_area = self.get_text_area_rect();
        let display_text = self.get_display_text();

        if display_text.is_empty() {
            return Some(0);
        }

        // Get layout
        if self.text_layout.borrow().is_none() {
            return Some(0);
        }

        let layout = self.text_layout.borrow();
        let layout = layout.as_ref().unwrap();
        let buffer = layout.buffer();

        // Convert to relative X
        let rel_x = x - text_area.origin.x as f32 + self.scroll_offset;

        // Use cosmic-text hit testing
        if let Some(cursor) = buffer.hit(rel_x, 0.0) {
            // Cursor.index is a byte index, need to convert to char index
            let text_before = &display_text[..cursor.index.min(display_text.len())];
            return Some(text_before.chars().count());
        }

        Some(display_text.chars().count())
    }

    // ========================================================================
    // Mouse Event Handlers
    // ========================================================================

    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        // Note: Focus is now handled by Window's focus manager via on_focus_gained()
        // No need to manually set is_focused here

        // Check if clicking eye button
        if self.show_password_toggle && self.is_password {
            if self.eye_button_bounds.contains(event.position) {
                self.password_revealed = !self.password_revealed;
                *self.text_layout.borrow_mut() = None;
                self.dirty = true;
                return EventResponse::Handled;
            }
        }

        // Position cursor
        if let Some(char_index) = self.hit_test_position(event.position.x as f32) {
            self.cursor_pos = char_index;
            self.selection_start = None;
            self.drag_start_pos = Some(event.position);
            self.cursor_blink_timer = Instant::now();
            self.dirty = true;

            // Double-click selects all
            if event.click_count == 2 {
                self.select_all();
            }
        }

        EventResponse::Handled
    }

    fn on_mouse_up(&mut self, _event: &mut MouseEvent) -> EventResponse {
        self.drag_start_pos = None;
        EventResponse::Handled
    }

    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Update eye button hover state
        if self.show_password_toggle && self.is_password {
            let was_hovered = self.eye_button_hovered;
            self.eye_button_hovered = self.eye_button_bounds.contains(event.position);

            if was_hovered != self.eye_button_hovered {
                self.dirty = true;
            }
        }

        // Handle drag selection
        if let Some(_drag_start) = self.drag_start_pos {
            if let Some(char_index) = self.hit_test_position(event.position.x as f32) {
                if self.selection_start.is_none() && char_index != self.cursor_pos {
                    self.selection_start = Some(self.cursor_pos);
                }
                self.cursor_pos = char_index;
                self.dirty = true;
            }
        }

        EventResponse::PassThrough
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
        self.eye_button_hovered = false;
        self.update_state();
        self.dirty = true;
        EventResponse::Handled
    }

    // ========================================================================
    // Keyboard Event Handlers
    // ========================================================================

    fn on_key_down(&mut self, event: &mut KeyEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        // Note: We should already have focus if receiving keyboard events
        // Focus is managed by Window's focus manager

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
                self.emit_submit();
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

        // Note: We should already have focus if receiving IME events
        // Focus is managed by Window's focus manager

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

impl Widget for TextInput {
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
            *self.text_layout.borrow_mut() = None;
            self.dirty = true;

            // Debug logging
            println!(
                "[TextInput {:?}] Bounds set: x={:.1}, y={:.1}, w={:.1}, h={:.1}",
                self.id,
                bounds.origin.x,
                bounds.origin.y,
                bounds.size.width,
                bounds.size.height
            );

            // Update eye button bounds if password mode is enabled
            if self.show_password_toggle && self.is_password {
                let eye_size = 20.0;
                let eye_x = self.bounds.origin.x + self.bounds.size.width - self.padding as f64 - eye_size;
                let eye_y = self.bounds.origin.y + (self.bounds.size.height - eye_size) / 2.0;
                self.eye_button_bounds = Rect::new(
                    Point::new(eye_x - 2.0, eye_y - 2.0),
                    Size::new(24.0, 24.0),
                );
            }
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

    fn is_interactive(&self) -> bool {
        !self.is_disabled
    }

    fn is_focusable(&self) -> bool {
        !self.is_disabled
    }

    fn preferred_cursor(&self) -> Option<crate::types::CursorType> {
        if self.is_disabled {
            None
        } else if self.eye_button_hovered {
            Some(crate::types::CursorType::Pointer)
        } else {
            Some(crate::types::CursorType::Text)
        }
    }

    fn ime_cursor_rect(&self) -> Option<Rect> {
        if !self.is_focused {
            return None;
        }

        let cursor_x = self.get_cursor_x_position(self.cursor_pos);
        let text_area = self.get_text_area_rect();

        Some(Rect::new(
            Point::new(cursor_x as f64, text_area.origin.y),
            Size::new(2.0, text_area.size.height),
        ))
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let style = self.get_current_style();
        let text_area = self.get_text_area_rect();

        // Debug logging (only log once per 100 frames to avoid spam)
        static mut PAINT_COUNT: usize = 0;
        unsafe {
            if PAINT_COUNT % 100 == 0 {
                println!(
                    "[TextInput {:?}] Paint: state={:?}, bounds=({:.1}, {:.1}, {:.1}x{:.1}), placeholder={:?}",
                    self.id,
                    self.current_state,
                    self.bounds.origin.x,
                    self.bounds.origin.y,
                    self.bounds.size.width,
                    self.bounds.size.height,
                    self.placeholder
                );
            }
            PAINT_COUNT += 1;
        }

        // Draw background
          ctx.draw_styled_rect(
            self.bounds,
            ShapeStyle {
                fill: crate::paint::types::Brush::Solid(style.background),
                corner_radius: style.corner_radius,
                border: style.border.clone(),
                shadow: style.shadow.clone(),
            },
        );

        // Draw left icon if present
        if let Some(ref icon_id) = self.icon {
            let icon_x = self.bounds.origin.x + self.padding as f64;
            let icon_y = self.bounds.origin.y + (self.bounds.size.height - self.icon_size as f64) / 2.0;
            ctx.draw_icon(icon_id, Point::new(icon_x, icon_y), self.icon_size, style.icon_color);
        }

        // Draw eye button if password mode
        if self.show_password_toggle && self.is_password {
            let eye_size = 20.0;
            let eye_x = self.bounds.origin.x + self.bounds.size.width - self.padding as f64 - eye_size;
            let eye_y = self.bounds.origin.y + (self.bounds.size.height - eye_size) / 2.0;

            // Calculate eye button bounds for rendering (stored for hit testing elsewhere)

            let icon_id = if self.password_revealed {
                "visibility"
            } else {
                "visibility_off"
            };

            let icon_color = if self.eye_button_hovered {
                Color::rgba(0.9, 0.9, 0.95, 1.0)
            } else {
                style.icon_color
            };

            ctx.draw_icon(icon_id, Point::new(eye_x, eye_y), eye_size as f32, icon_color);
        }

        // Create text layout if needed
        let display_text = self.get_display_text();
        let text_to_render = if display_text.is_empty() {
            self.placeholder.as_deref().unwrap_or("")
        } else {
            &display_text
        };

     if self.text_layout.borrow().is_none() && !text_to_render.is_empty() {
            let text_style = TextStyle::new()
                .size(self.font_size)
                .color(if display_text.is_empty() {
                    style.placeholder_color
                } else {
                    style.text_color
                });

            ctx.with_text_engine(|engine| {
                let layout = engine.create_layout(
                    text_to_render,
                    &text_style,
                    None,
                    Truncate::None,
                );
                *self.text_layout.borrow_mut() = Some(layout);
            });
        }

        // Clip to text area
       ctx.push_clip(text_area);

        // Draw selection if any
        if self.is_focused {
            if let Some(sel_start) = self.selection_start {
                let (start, end) = if sel_start < self.cursor_pos {
                    (sel_start, self.cursor_pos)
                } else {
                    (self.cursor_pos, sel_start)
                };

                let start_x = self.get_cursor_x_position(start);
                let end_x = self.get_cursor_x_position(end);

                // DEBUG: Use bright yellow with high alpha for visibility testing
                let selection_color = Color::rgba(1.0, 1.0, 0.0, 0.7); // Bright yellow

                println!(
                    "[TextInput {:?}] Drawing selection: start={}, end={}, start_x={:.1}, end_x={:.1}, width={:.1}, color=({:.2},{:.2},{:.2},{:.2})",
                    self.id, start, end, start_x, end_x, end_x - start_x,
                    selection_color.r, selection_color.g, selection_color.b, selection_color.a
                );

                let selection_rect = Rect::new(
                    Point::new(start_x as f64, text_area.origin.y),
                    Size::new((end_x - start_x) as f64, text_area.size.height),
                );

                println!(
                    "[TextInput {:?}] Selection rect: x={:.1}, y={:.1}, w={:.1}, h={:.1}",
                    self.id, selection_rect.origin.x, selection_rect.origin.y,
                    selection_rect.size.width, selection_rect.size.height
                );

                ctx.draw_rect(selection_rect, selection_color);
            }
        }

        // Draw text
        if !text_to_render.is_empty() {
            if let Some(ref layout) = *self.text_layout.borrow() {
                let text_x = text_area.origin.x - self.scroll_offset as f64;
                let text_y = text_area.origin.y + (text_area.size.height - layout.height() as f64) / 2.0;

                let text_color = if display_text.is_empty() {
                    style.placeholder_color
                } else {
                    style.text_color
                };

                ctx.draw_layout(layout, Point::new(text_x, text_y), text_color);
            }
        }

        // Draw preedit text if IME is active
        if !self.preedit_text.is_empty() {
            let preedit_style = TextStyle::new()
                .size(self.font_size)
                .color(Color::rgba(1.0, 1.0, 0.0, 1.0)); // Yellow for preedit

            ctx.draw_text(
                &self.preedit_text,
                &preedit_style,
                Point::new(self.get_cursor_x_position(self.cursor_pos) as f64, text_area.origin.y),
                None,
            );
        }

        // Draw cursor if focused (always visible for now - TODO: add blinking)
        if self.is_focused && self.selection_start.is_none() {
            let cursor_x = self.get_cursor_x_position(self.cursor_pos);

            // DEBUG: Use bright red and wider cursor for visibility testing
            let cursor_width = 10.0; // Was 2.0
            let cursor_color = Color::rgba(1.0, 0.0, 0.0, 1.0); // Bright red instead of blue

            println!(
                "[TextInput {:?}] Drawing cursor: pos={}, cursor_x={:.1}, text_area.y={:.1}, height={:.1}, width={:.1}, color=({:.2},{:.2},{:.2},{:.2})",
                self.id, self.cursor_pos, cursor_x, text_area.origin.y, text_area.size.height, cursor_width,
                cursor_color.r, cursor_color.g, cursor_color.b, cursor_color.a
            );

            let cursor_rect = Rect::new(
                Point::new(cursor_x as f64, text_area.origin.y),
                Size::new(cursor_width, text_area.size.height),
            );

            println!(
                "[TextInput {:?}] Cursor rect: x={:.1}, y={:.1}, w={:.1}, h={:.1}",
                self.id, cursor_rect.origin.x, cursor_rect.origin.y,
                cursor_rect.size.width, cursor_rect.size.height
            );

            ctx.draw_rect(cursor_rect, cursor_color);
        }

        ctx.pop_clip();

        // Draw validation error if present
        if let Some(ref error_msg) = self.validation_error {
            let error_padding = 8.0;
            let error_y = self.bounds.origin.y - 30.0; // Above input

            let error_style = TextStyle::new()
                .size(12.0)
                .color(Color::WHITE);

            // Measure error text
            let error_width = ctx.with_text_engine(|engine| {
                let layout = engine.create_layout(error_msg, &error_style, None, Truncate::None);
                layout.width() as f32 + error_padding * 2.0
            });

            let error_rect = Rect::new(
                Point::new(self.bounds.origin.x, error_y),
                Size::new(error_width as f64, 24.0),
            );

            // Draw error background
            ctx.draw_styled_rect(
                error_rect,
                ShapeStyle {
                    fill: crate::paint::types::Brush::Solid(Color::rgba(1.0, 0.3, 0.3, 0.95)),
                    corner_radius: CornerRadius::uniform(4.0),
                    border: None,
                    shadow: Some(crate::paint::types::Shadow::new(
                        Color::rgba(0.0, 0.0, 0.0, 0.3),
                        (0.0, 2.0),
                        4.0,
                    )),
                },
            );

            // Draw error text
            ctx.draw_text(
                error_msg,
                &error_style,
                Point::new(error_rect.origin.x + error_padding as f64, error_rect.origin.y + 6.0),
                None,
            );
        }

        // Register hitbox
        ctx.register_hitbox(self.id, self.bounds);
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_mouse_enter(&mut self, event: &mut MouseEvent) -> EventResponse {
        self.on_mouse_enter(event)
    }

    fn on_mouse_leave(&mut self, event: &mut MouseEvent) -> EventResponse {
        self.on_mouse_leave(event)
    }

    fn dispatch_mouse_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::MouseDown(e) => self.on_mouse_down(e),
            InputEventEnum::MouseUp(e) => self.on_mouse_up(e),
            InputEventEnum::MouseMove(e) => self.on_mouse_move(e),
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
        println!("[TextInput {:?}] Focus gained", self.id);
        self.is_focused = true;
        self.update_state();
    }

    fn on_focus_lost(&mut self) {
        println!("[TextInput {:?}] Focus lost", self.id);
        self.is_focused = false;
        self.selection_start = None;  // Clear selection when losing focus
        self.update_state();
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}
