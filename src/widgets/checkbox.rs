//! Checkbox widget - icon-based boolean toggle with optional label
//!
//! Features:
//! - Checkable/uncheckable state
//! - Optional label text
//! - Per-state styling (normal, hovered, checked, disabled, focused)
//! - Click and keyboard (Space/Enter) activation
//! - State change callbacks
//! - Signal/slot integration via deferred commands
//!
//! # Example
//! ```rust,ignore
//! let checkbox = Checkbox::new("Enable notifications")
//!     .checked()
//!     .on_changed(|is_checked| println!("Checked: {}", is_checked));
//! ```

use std::any::Any;

use crate::event::handlers::{KeyboardHandler, MouseHandler};
use crate::event::input::{EventResponse, InputEventEnum, KeyEvent, MouseEvent, NamedKey};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::primitives::Color;
use crate::paint::types::{Border, CornerRadius, Shadow, ShapeStyle};
use crate::paint::PaintContext;
use crate::text::{TextAlign, TextEngine, TextStyle, Truncate};
use crate::types::{CursorType, DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;

pub use crate::widgets::label::Padding;

/// Style configuration for checkbox in a specific state
#[derive(Clone, Debug)]
pub struct CheckboxStyle {
    pub icon_color: Color,
    pub label_color: Color,
    pub border: Option<Border>,
    pub shadow: Option<Shadow>,
}

impl CheckboxStyle {
    pub fn new(icon_color: Color, label_color: Color) -> Self {
        Self {
            icon_color,
            label_color,
            border: None,
            shadow: None,
        }
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    pub fn with_shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = Some(shadow);
        self
    }
}

/// Visual state of the checkbox
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckboxState {
    Normal,
    Hovered,
    CheckedNormal,
    CheckedHovered,
    Disabled,
    Focused,
}

/// Checkbox widget
pub struct Checkbox {
    // Widget basics
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // State
    is_checked: bool,
    is_hovered: bool,
    is_pressed: bool,
    is_disabled: bool,
    is_focused: bool,
    current_state: CheckboxState,

    // Content
    label: Option<String>,

    // Styling
    font_size: f32,
    font_family: Option<String>,
    icon_size: f32,
    gap: f32,

    // Per-state styles
    unchecked_style: CheckboxStyle,
    checked_style: CheckboxStyle,
    disabled_style: CheckboxStyle,
    focused_style: CheckboxStyle,

    // Callback
    on_changed: Option<Box<dyn FnMut(bool) + 'static>>,

    // Deferred commands for signal/slot
    pending_commands: Vec<DeferredCommand>,
}

impl Checkbox {
    /// Create a new checkbox with a label
    pub fn new(label: impl Into<String>) -> Self {
        let (unchecked, checked, disabled, focused) = Self::default_styles();

        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            is_checked: false,
            is_hovered: false,
            is_pressed: false,
            is_disabled: false,
            is_focused: false,
            current_state: CheckboxState::Normal,
            label: Some(label.into()),
            font_size: 16.0,
            font_family: None,
            icon_size: 20.0,
            gap: 8.0,
            unchecked_style: unchecked,
            checked_style: checked,
            disabled_style: disabled,
            focused_style: focused,
            on_changed: None,
            pending_commands: Vec::new(),
        }
    }

    /// Create a checkbox without a label (icon only)
    pub fn icon_only() -> Self {
        let mut checkbox = Self::new("");
        checkbox.label = None;
        checkbox
    }

    // ========================================================================
    // Default Styles
    // ========================================================================

    fn default_styles() -> (CheckboxStyle, CheckboxStyle, CheckboxStyle, CheckboxStyle) {
        let unchecked = CheckboxStyle::new(
            Color::rgb(0.5, 0.5, 0.5),     // Gray icon
            Color::rgb(0.9, 0.9, 0.9),     // Light gray label
        );

        let checked = CheckboxStyle::new(
            Color::rgb(0.2, 0.4, 0.8),     // Blue icon
            Color::rgb(1.0, 1.0, 1.0),     // White label
        );

        let disabled = CheckboxStyle::new(
            Color::rgb(0.3, 0.3, 0.3),     // Dark gray icon
            Color::rgb(0.5, 0.5, 0.5),     // Gray label
        );

        let focused = CheckboxStyle::new(
            Color::rgb(0.3, 0.5, 0.9),     // Lighter blue icon
            Color::rgb(1.0, 1.0, 1.0),     // White label
        );

        (unchecked, checked, disabled, focused)
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set the checkbox as checked initially
    pub fn checked(mut self) -> Self {
        self.is_checked = true;
        self.update_state();
        self
    }

    /// Set the checkbox as disabled
    pub fn disabled(mut self) -> Self {
        self.is_disabled = true;
        self.update_state();
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set font family
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font_family = Some(font.into());
        self
    }

    /// Set icon size
    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    /// Set gap between icon and label
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set the on_changed callback
    pub fn on_changed<F>(mut self, callback: F) -> Self
    where
        F: FnMut(bool) + 'static,
    {
        self.on_changed = Some(Box::new(callback));
        self
    }

    /// Set layout style
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    /// Set unchecked state style
    pub fn unchecked_style(mut self, style: CheckboxStyle) -> Self {
        self.unchecked_style = style;
        self
    }

    /// Set checked state style
    pub fn checked_style(mut self, style: CheckboxStyle) -> Self {
        self.checked_style = style;
        self
    }

    /// Set disabled state style
    pub fn disabled_style(mut self, style: CheckboxStyle) -> Self {
        self.disabled_style = style;
        self
    }

    /// Set focused state style
    pub fn focused_style(mut self, style: CheckboxStyle) -> Self {
        self.focused_style = style;
        self
    }

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Set the checked state
    pub fn set_checked(&mut self, checked: bool) {
        if self.is_checked != checked {
            self.is_checked = checked;
            self.update_state();
            self.dirty = true;
        }
    }

    /// Get the checked state
    pub fn is_checked(&self) -> bool {
        self.is_checked
    }

    /// Set the disabled state
    pub fn set_disabled(&mut self, disabled: bool) {
        if self.is_disabled != disabled {
            self.is_disabled = disabled;
            self.update_state();
            self.dirty = true;
        }
    }

    /// Toggle the checked state
    pub fn toggle(&mut self) {
        self.set_checked(!self.is_checked);
    }

    // ========================================================================
    // State Management
    // ========================================================================

    fn update_state(&mut self) {
        self.current_state = if self.is_disabled {
            CheckboxState::Disabled
        } else if self.is_checked && self.is_hovered {
            CheckboxState::CheckedHovered
        } else if self.is_checked {
            CheckboxState::CheckedNormal
        } else if self.is_hovered {
            CheckboxState::Hovered
        } else if self.is_focused {
            CheckboxState::Focused
        } else {
            CheckboxState::Normal
        };
    }

    fn get_current_style(&self) -> &CheckboxStyle {
        match self.current_state {
            CheckboxState::Normal => &self.unchecked_style,
            CheckboxState::Hovered => &self.unchecked_style,
            CheckboxState::CheckedNormal => &self.checked_style,
            CheckboxState::CheckedHovered => &self.checked_style,
            CheckboxState::Disabled => &self.disabled_style,
            CheckboxState::Focused => &self.focused_style,
        }
    }

    /// Notify listeners that the checked state changed
    fn notify_changed(&mut self) {
        // Call local callback
        if let Some(ref mut callback) = self.on_changed {
            callback(self.is_checked);
        }

        // Queue deferred command for signal/slot system
        self.pending_commands.push(DeferredCommand {
            target: self.id,
            message: GuiMessage::Custom {
                source: self.id,
                signal_type: "checked_changed".to_string(),
                data: Box::new(self.is_checked),
            },
        });
    }

    // ========================================================================
    // Layout Measurement
    // ========================================================================

    /// Measure checkbox for layout system
    pub fn measure_with_engine(
        &self,
        engine: &mut TextEngine,
        _known_dimensions: taffy::Size<Option<f32>>,
    ) -> Size {
        let mut width = self.icon_size as f64;
        let mut height = self.icon_size as f64;

        // Add label width if present
        if let Some(ref label) = self.label {
            let mut text_style = TextStyle::new()
                .size(self.font_size)
                .align(TextAlign::Left);

            if let Some(ref font_family) = self.font_family {
                text_style = text_style.family(font_family.clone());
            }

            let layout = engine.create_layout_with_wrap(
                label,
                &text_style,
                None,
                Truncate::End,
                cosmic_text::Wrap::Word,
            );

            let text_size = layout.size();
            width += self.gap as f64 + text_size.width;
            height = height.max(text_size.height);
        }

        Size::new(width, height)
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::icon_only()
    }
}

// ========================================================================
// Event Handlers
// ========================================================================

impl MouseHandler for Checkbox {
    fn on_mouse_down(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        self.is_pressed = true;
        self.update_state();
        self.dirty = true;
        EventResponse::Handled
    }

    fn on_mouse_up(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        if self.is_pressed {
            self.is_checked = !self.is_checked;
            self.notify_changed();
        }

        self.is_pressed = false;
        self.update_state();
        self.dirty = true;
        EventResponse::Handled
    }

    fn on_mouse_move(&mut self, _event: &mut MouseEvent) -> EventResponse {
        EventResponse::PassThrough
    }

    fn on_mouse_enter(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        self.is_hovered = true;
        self.update_state();
        self.dirty = true;
        EventResponse::Handled
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        self.is_hovered = false;
        self.is_pressed = false;
        self.update_state();
        self.dirty = true;
        EventResponse::Handled
    }
}

impl KeyboardHandler for Checkbox {
    fn on_key_down(&mut self, event: &mut KeyEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        match event.key {
            crate::event::input::Key::Named(NamedKey::Space)
            | crate::event::input::Key::Named(NamedKey::Enter) => {
                self.is_checked = !self.is_checked;
                self.notify_changed();
                self.update_state();
                self.dirty = true;
                EventResponse::Handled
            }
            _ => EventResponse::Ignored,
        }
    }

    fn on_key_up(&mut self, _event: &mut KeyEvent) -> EventResponse {
        EventResponse::Ignored
    }
}

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for Checkbox {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        if self.bounds != bounds {
            self.bounds = bounds;
            self.dirty = true;
        }
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let style = self.get_current_style();

        // Determine icon ID based on checked state
        let icon_id = if self.is_checked {
            "checkbox_checked"
        } else {
            "checkbox_unchecked"
        };

        // Calculate icon position (vertically centered)
        let icon_y = self.bounds.origin.y + (self.bounds.size.height - self.icon_size as f64) / 2.0;
        let icon_pos = Point::new(self.bounds.origin.x, icon_y);

        // Draw checkbox icon
        ctx.draw_icon(icon_id, icon_pos, self.icon_size, style.icon_color);

        // Draw label if present
        if let Some(ref label) = self.label {
            let mut text_style = TextStyle::new()
                .size(self.font_size)
                .color(style.label_color)
                .align(TextAlign::Left);

            if let Some(ref font_family) = self.font_family {
                text_style = text_style.family(font_family.clone());
            }

            // Calculate label position (after icon + gap, vertically centered)
            let label_x = self.bounds.origin.x + self.icon_size as f64 + self.gap as f64;
            let label_y = self.bounds.origin.y + (self.bounds.size.height - self.font_size as f64) / 2.0;
            let label_pos = Point::new(label_x, label_y);

            ctx.draw_text(label, &text_style, label_pos, None);
        }

        // Draw focus ring if focused
        if self.is_focused && !self.is_disabled {
            let focus_rect = Rect::new(
                Point::new(self.bounds.origin.x - 2.0, self.bounds.origin.y - 2.0),
                Size::new(self.bounds.size.width + 4.0, self.bounds.size.height + 4.0),
            );

            ctx.draw_styled_rect(
                focus_rect,
                ShapeStyle {
                    fill: crate::paint::types::Brush::Solid(Color::TRANSPARENT),
                    corner_radius: CornerRadius::uniform(2.0),
                    border: Some(Border::new(Color::rgb(0.3, 0.5, 0.9), 2.0)),
                    shadow: None,
                },
            );
        }
    }

    fn needs_measure(&self) -> bool {
        true
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        None
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn is_focusable(&self) -> bool {
        !self.is_disabled
    }

    fn preferred_cursor(&self) -> Option<CursorType> {
        if self.is_disabled {
            Some(CursorType::NotAllowed)
        } else {
            Some(CursorType::Pointer)
        }
    }

    fn on_mouse_enter(&mut self, event: &mut MouseEvent) -> EventResponse {
        MouseHandler::on_mouse_enter(self, event)
    }

    fn on_mouse_leave(&mut self, event: &mut MouseEvent) -> EventResponse {
        MouseHandler::on_mouse_leave(self, event)
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

    fn drain_deferred_commands(&mut self) -> Vec<DeferredCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
