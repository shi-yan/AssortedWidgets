//! Switch widget - visual toggle with track and thumb
//!
//! Features:
//! - On/off state toggle
//! - Visual track (background) and thumb (circle)
//! - Per-state styling (off, on, disabled, focused)
//! - Click and keyboard (Space/Enter) activation
//! - State change callbacks
//! - Signal/slot integration via deferred commands
//! - No animation (instant toggle)
//!
//! # Example
//! ```rust,ignore
//! let switch = Switch::new()
//!     .on()
//!     .on_changed(|is_on| println!("Switch: {}", is_on));
//! ```

use std::any::Any;

use crate::event::handlers::{KeyboardHandler, MouseHandler};
use crate::event::input::{EventResponse, InputEventEnum, KeyEvent, MouseEvent, NamedKey};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::primitives::Color;
use crate::paint::types::{Border, Brush, CornerRadius, ShapeStyle};
use crate::paint::PaintContext;
use crate::types::{CursorType, DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;

/// Visual state of the switch
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SwitchState {
    Off,
    OffHovered,
    On,
    OnHovered,
    Disabled,
    Focused,
}

/// Switch widget
pub struct Switch {
    // Widget basics
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // State
    is_on: bool,
    is_hovered: bool,
    is_pressed: bool,
    is_disabled: bool,
    is_focused: bool,
    current_state: SwitchState,

    // Styling
    track_width: f32,
    track_height: f32,
    thumb_size: f32,

    // Colors for different states
    off_track_color: Color,
    on_track_color: Color,
    off_thumb_color: Color,
    on_thumb_color: Color,
    disabled_color: Color,

    // Callback
    on_changed: Option<Box<dyn FnMut(bool) + 'static>>,

    // Deferred commands for signal/slot
    pending_commands: Vec<DeferredCommand>,
}

impl Switch {
    /// Create a new switch (off by default)
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            is_on: false,
            is_hovered: false,
            is_pressed: false,
            is_disabled: false,
            is_focused: false,
            current_state: SwitchState::Off,
            track_width: 44.0,
            track_height: 24.0,
            thumb_size: 20.0,
            off_track_color: Color::rgb(0.3, 0.3, 0.3),
            on_track_color: Color::rgb(0.2, 0.4, 0.8),
            off_thumb_color: Color::rgb(0.8, 0.8, 0.8),
            on_thumb_color: Color::rgb(1.0, 1.0, 1.0),
            disabled_color: Color::rgb(0.5, 0.5, 0.5),
            on_changed: None,
            pending_commands: Vec::new(),
        }
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set the switch as on initially
    pub fn on(mut self) -> Self {
        self.is_on = true;
        self.update_state();
        self
    }

    /// Set the switch as off initially (default)
    pub fn off(mut self) -> Self {
        self.is_on = false;
        self.update_state();
        self
    }

    /// Set the switch as disabled
    pub fn disabled(mut self) -> Self {
        self.is_disabled = true;
        self.update_state();
        self
    }

    /// Set track width
    pub fn track_width(mut self, width: f32) -> Self {
        self.track_width = width;
        self
    }

    /// Set track height
    pub fn track_height(mut self, height: f32) -> Self {
        self.track_height = height;
        self
    }

    /// Set thumb size
    pub fn thumb_size(mut self, size: f32) -> Self {
        self.thumb_size = size;
        self
    }

    /// Set off track color
    pub fn off_track_color(mut self, color: Color) -> Self {
        self.off_track_color = color;
        self
    }

    /// Set on track color
    pub fn on_track_color(mut self, color: Color) -> Self {
        self.on_track_color = color;
        self
    }

    /// Set off thumb color
    pub fn off_thumb_color(mut self, color: Color) -> Self {
        self.off_thumb_color = color;
        self
    }

    /// Set on thumb color
    pub fn on_thumb_color(mut self, color: Color) -> Self {
        self.on_thumb_color = color;
        self
    }

    /// Set disabled color
    pub fn disabled_color(mut self, color: Color) -> Self {
        self.disabled_color = color;
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

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Set the on state
    pub fn set_on(&mut self, on: bool) {
        if self.is_on != on {
            self.is_on = on;
            self.update_state();
            self.dirty = true;
        }
    }

    /// Get the on state
    pub fn is_on(&self) -> bool {
        self.is_on
    }

    /// Set the disabled state
    pub fn set_disabled(&mut self, disabled: bool) {
        if self.is_disabled != disabled {
            self.is_disabled = disabled;
            self.update_state();
            self.dirty = true;
        }
    }

    /// Toggle the on state
    pub fn toggle(&mut self) {
        self.set_on(!self.is_on);
    }

    // ========================================================================
    // State Management
    // ========================================================================

    fn update_state(&mut self) {
        self.current_state = if self.is_disabled {
            SwitchState::Disabled
        } else if self.is_on && self.is_hovered {
            SwitchState::OnHovered
        } else if self.is_on {
            SwitchState::On
        } else if self.is_hovered {
            SwitchState::OffHovered
        } else if self.is_focused {
            SwitchState::Focused
        } else {
            SwitchState::Off
        };
    }

    fn get_track_color(&self) -> Color {
        if self.is_disabled {
            self.disabled_color
        } else if self.is_on {
            self.on_track_color
        } else {
            self.off_track_color
        }
    }

    fn get_thumb_color(&self) -> Color {
        if self.is_disabled {
            self.disabled_color
        } else if self.is_on {
            self.on_thumb_color
        } else {
            self.off_thumb_color
        }
    }

    /// Get the track rectangle
    fn get_track_rect(&self) -> Rect {
        // Center the track within the bounds
        let track_x = self.bounds.origin.x + (self.bounds.size.width - self.track_width as f64) / 2.0;
        let track_y = self.bounds.origin.y + (self.bounds.size.height - self.track_height as f64) / 2.0;

        Rect::new(
            Point::new(track_x, track_y),
            Size::new(self.track_width as f64, self.track_height as f64),
        )
    }

    /// Get the thumb rectangle
    fn get_thumb_rect(&self) -> Rect {
        let track_rect = self.get_track_rect();
        let gap = (self.track_height - self.thumb_size) / 2.0;

        // Calculate thumb position based on state
        let thumb_x = if self.is_on {
            // Right position (ON)
            track_rect.origin.x + (self.track_width - self.thumb_size - gap) as f64
        } else {
            // Left position (OFF)
            track_rect.origin.x + gap as f64
        };

        let thumb_y = track_rect.origin.y + gap as f64;

        Rect::new(
            Point::new(thumb_x, thumb_y),
            Size::new(self.thumb_size as f64, self.thumb_size as f64),
        )
    }

    /// Notify listeners that the state changed
    fn notify_changed(&mut self) {
        // Call local callback
        if let Some(ref mut callback) = self.on_changed {
            callback(self.is_on);
        }

        // Queue deferred command for signal/slot system
        self.pending_commands.push(DeferredCommand {
            target: self.id,
            message: GuiMessage::Custom {
                source: self.id,
                signal_type: "toggled".to_string(),
                data: Box::new(self.is_on),
            },
        });
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

// ========================================================================
// Event Handlers
// ========================================================================

impl MouseHandler for Switch {
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
            self.is_on = !self.is_on;
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

impl KeyboardHandler for Switch {
    fn on_key_down(&mut self, event: &mut KeyEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        match event.key {
            crate::event::input::Key::Named(NamedKey::Space)
            | crate::event::input::Key::Named(NamedKey::Enter) => {
                self.is_on = !self.is_on;
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

impl Widget for Switch {
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
        let track_rect = self.get_track_rect();
        let thumb_rect = self.get_thumb_rect();
        let track_color = self.get_track_color();
        let thumb_color = self.get_thumb_color();

        // Draw track (rounded rectangle)
        ctx.draw_styled_rect(
            track_rect,
            ShapeStyle {
                fill: Brush::Solid(track_color),
                corner_radius: CornerRadius::uniform(self.track_height / 2.0),
                border: None,
                shadow: None,
            },
        );

        // Draw thumb (circle)
        ctx.draw_styled_rect(
            thumb_rect,
            ShapeStyle {
                fill: Brush::Solid(thumb_color),
                corner_radius: CornerRadius::uniform(self.thumb_size / 2.0),
                border: None,
                shadow: None,
            },
        );

        // Draw focus ring if focused
        if self.is_focused && !self.is_disabled {
            let focus_rect = Rect::new(
                Point::new(track_rect.origin.x - 2.0, track_rect.origin.y - 2.0),
                Size::new(track_rect.size.width + 4.0, track_rect.size.height + 4.0),
            );

            ctx.draw_styled_rect(
                focus_rect,
                ShapeStyle {
                    fill: Brush::Solid(Color::TRANSPARENT),
                    corner_radius: CornerRadius::uniform(self.track_height / 2.0 + 2.0),
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
        // Return intrinsic size based on track dimensions
        Some(Size::new(self.track_width as f64, self.track_height as f64))
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
