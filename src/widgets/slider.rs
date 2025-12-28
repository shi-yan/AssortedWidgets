//! Slider widget - draggable slider for selecting numeric values
//!
//! Features:
//! - Horizontal and vertical orientation
//! - Configurable min, max, and step values
//! - Optional label above the slider
//! - Optional value display
//! - Hover and dragging states
//! - Pointer cursor on hover
//! - Value change callback
//! - Customizable appearance
//!
//! # Example
//! ```rust,ignore
//! // Basic slider
//! let slider = Slider::horizontal(0.0, 100.0)
//!     .step(1.0)
//!     .value(50.0)
//!     .on_value_changed(|value| {
//!         println!("Value: {}", value);
//!     });
//!
//! // Slider with label and value display
//! let volume = Slider::horizontal(0.0, 100.0)
//!     .label("Volume")
//!     .show_value(true)
//!     .value_formatter(|v| format!("{}%", v as i32))
//!     .track_height(6.0);
//! ```

use std::any::Any;

use crate::event::handlers::MouseHandler;
use crate::event::input::{EventResponse, InputEventEnum, MouseEvent};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::primitives::Color;
use crate::paint::types::{Brush, CornerRadius, ShapeStyle};
use crate::paint::PaintContext;
use crate::text::TextStyle;
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;

/// Slider orientation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Slider state (determines visual appearance)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SliderState {
    Normal,
    Hovered,
    Dragging,
}

/// Slider widget - draggable slider with continuous values
pub struct Slider {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Orientation
    orientation: Orientation,

    // Value range
    value: f64,
    min: f64,
    max: f64,
    step: f64, // 0.0 = continuous

    // Visual state
    state: SliderState,
    is_dragging: bool,
    is_slider_hovered: bool,
    drag_start_value: f64,
    drag_start_pos: f64,

    // Label and value display
    label: Option<String>,
    show_value: bool,
    value_formatter: Option<Box<dyn Fn(f64) -> String>>,

    // Styling
    track_height: f32,           // Height of the track
    slider_size: f32,            // Size of the draggable slider knob
    track_color: Color,          // Track background color
    fill_color: Color,           // Filled portion color
    slider_color: Color,         // Normal slider color
    slider_hover_color: Color,   // Hovered slider color
    slider_drag_color: Color,    // Dragging slider color
    label_color: Color,          // Label text color
    value_color: Color,          // Value text color

    // Callback
    on_value_changed: Option<Box<dyn FnMut(f64)>>,

    // Pending deferred commands
    pending_commands: Vec<DeferredCommand>,
}

impl Slider {
    /// Create a new slider with the given orientation and value range
    pub fn new(orientation: Orientation, min: f64, max: f64) -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            orientation,
            value: min,
            min,
            max,
            step: 0.0, // Continuous by default
            state: SliderState::Normal,
            is_dragging: false,
            is_slider_hovered: false,
            drag_start_value: 0.0,
            drag_start_pos: 0.0,
            label: None,
            show_value: false,
            value_formatter: None,
            track_height: 6.0,
            slider_size: 20.0,
            track_color: Color::rgba(0.3, 0.3, 0.3, 0.5),
            fill_color: Color::rgba(0.4, 0.6, 0.8, 0.8),
            slider_color: Color::rgba(0.9, 0.9, 0.9, 1.0),
            slider_hover_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            slider_drag_color: Color::rgba(0.8, 0.8, 0.8, 1.0),
            label_color: Color::rgba(0.9, 0.9, 0.9, 1.0),
            value_color: Color::rgba(0.9, 0.9, 0.9, 1.0),
            on_value_changed: None,
            pending_commands: Vec::new(),
        }
    }

    /// Create a horizontal slider
    pub fn horizontal(min: f64, max: f64) -> Self {
        Self::new(Orientation::Horizontal, min, max)
    }

    /// Create a vertical slider
    pub fn vertical(min: f64, max: f64) -> Self {
        Self::new(Orientation::Vertical, min, max)
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set the initial value
    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    /// Set the step size (0.0 for continuous)
    pub fn step(mut self, step: f64) -> Self {
        self.step = step.max(0.0);
        self
    }

    /// Set the label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show/hide the current value
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set a custom value formatter
    pub fn value_formatter<F>(mut self, formatter: F) -> Self
    where
        F: Fn(f64) -> String + 'static,
    {
        self.value_formatter = Some(Box::new(formatter));
        self
    }

    /// Set the track height
    pub fn track_height(mut self, height: f32) -> Self {
        self.track_height = height.max(2.0);
        self
    }

    /// Set the slider knob size
    pub fn slider_size(mut self, size: f32) -> Self {
        self.slider_size = size.max(8.0);
        self
    }

    /// Set the track color
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    /// Set the fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Set the slider knob color
    pub fn slider_color(mut self, color: Color) -> Self {
        self.slider_color = color;
        self
    }

    /// Set the slider hover color
    pub fn slider_hover_color(mut self, color: Color) -> Self {
        self.slider_hover_color = color;
        self
    }

    /// Set the slider drag color
    pub fn slider_drag_color(mut self, color: Color) -> Self {
        self.slider_drag_color = color;
        self
    }

    /// Set value change callback
    pub fn on_value_changed<F>(mut self, callback: F) -> Self
    where
        F: FnMut(f64) + 'static,
    {
        self.on_value_changed = Some(Box::new(callback));
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

    /// Set the current value
    pub fn set_value(&mut self, value: f64) {
        let clamped = self.clamp_to_step(value.clamp(self.min, self.max));
        if (self.value - clamped).abs() > f64::EPSILON {
            self.value = clamped;
            self.dirty = true;
        }
    }

    /// Get the current value
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Set the value range
    pub fn set_range(&mut self, min: f64, max: f64) {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max);
        self.dirty = true;
    }

    // ========================================================================
    // Internal Helpers
    // ========================================================================

    /// Clamp value to step if step is non-zero
    fn clamp_to_step(&self, value: f64) -> f64 {
        if self.step > 0.0 {
            let steps = ((value - self.min) / self.step).round();
            (self.min + steps * self.step).clamp(self.min, self.max)
        } else {
            value
        }
    }

    /// Get the track rectangle (excludes label/value areas)
    fn get_track_bounds(&self) -> Rect {
        let mut bounds = self.bounds;

        // Reserve space for label at the top
        if self.label.is_some() {
            bounds.origin.y += 24.0;
            bounds.size.height -= 24.0;
        }

        // Reserve space for value display
        if self.show_value {
            match self.orientation {
                Orientation::Horizontal => {
                    bounds.size.width -= 60.0; // Space for value on the right
                }
                Orientation::Vertical => {
                    bounds.size.height -= 24.0; // Space for value at bottom
                }
            }
        }

        bounds
    }

    /// Calculate the slider knob position
    fn calculate_slider_position(&self) -> Point {
        if self.max <= self.min {
            return self.bounds.origin;
        }

        let track_bounds = self.get_track_bounds();
        let ratio = (self.value - self.min) / (self.max - self.min);

        match self.orientation {
            Orientation::Horizontal => {
                let available_width = track_bounds.size.width - self.slider_size as f64;
                let x = track_bounds.origin.x + available_width * ratio;
                let y = track_bounds.origin.y + (track_bounds.size.height - self.slider_size as f64) / 2.0;
                Point::new(x, y)
            }
            Orientation::Vertical => {
                let available_height = track_bounds.size.height - self.slider_size as f64;
                let x = track_bounds.origin.x + (track_bounds.size.width - self.slider_size as f64) / 2.0;
                let y = track_bounds.origin.y + available_height * ratio;
                Point::new(x, y)
            }
        }
    }

    /// Get the slider knob rectangle
    fn get_slider_rect(&self) -> Rect {
        let pos = self.calculate_slider_position();
        Rect::new(pos, Size::new(self.slider_size as f64, self.slider_size as f64))
    }

    /// Convert mouse position to value
    fn position_to_value(&self, mouse_pos: Point) -> f64 {
        if self.max <= self.min {
            return self.min;
        }

        let track_bounds = self.get_track_bounds();

        let ratio = match self.orientation {
            Orientation::Horizontal => {
                let available_width = track_bounds.size.width - self.slider_size as f64;
                if available_width <= 0.0 {
                    return self.min;
                }
                let mouse_x = mouse_pos.x - track_bounds.origin.x - (self.slider_size as f64 / 2.0);
                (mouse_x / available_width).clamp(0.0, 1.0)
            }
            Orientation::Vertical => {
                let available_height = track_bounds.size.height - self.slider_size as f64;
                if available_height <= 0.0 {
                    return self.min;
                }
                let mouse_y = mouse_pos.y - track_bounds.origin.y - (self.slider_size as f64 / 2.0);
                (mouse_y / available_height).clamp(0.0, 1.0)
            }
        };

        let value = self.min + ratio * (self.max - self.min);
        self.clamp_to_step(value)
    }

    /// Update state based on flags
    fn update_state(&mut self) {
        self.state = if self.is_dragging {
            SliderState::Dragging
        } else if self.is_slider_hovered {
            SliderState::Hovered
        } else {
            SliderState::Normal
        };
    }

    /// Get the current slider color based on state
    fn get_slider_color(&self) -> Color {
        match self.state {
            SliderState::Normal => self.slider_color,
            SliderState::Hovered => self.slider_hover_color,
            SliderState::Dragging => self.slider_drag_color,
        }
    }

    /// Trigger value changed callback
    fn notify_value_changed(&mut self, new_value: f64) {
        let clamped = self.clamp_to_step(new_value.clamp(self.min, self.max));
        if (self.value - clamped).abs() > f64::EPSILON {
            self.value = clamped;
            self.dirty = true;

            if let Some(ref mut callback) = self.on_value_changed {
                callback(clamped);
            }

            self.pending_commands.push(DeferredCommand {
                target: self.id,
                message: GuiMessage::Custom {
                    source: self.id,
                    signal_type: "value_changed".to_string(),
                    data: Box::new(clamped),
                },
            });
        }
    }

    /// Format the current value for display
    fn format_value(&self) -> String {
        if let Some(ref formatter) = self.value_formatter {
            formatter(self.value)
        } else {
            format!("{:.1}", self.value)
        }
    }
}

// ========================================================================
// Event Handlers
// ========================================================================

impl MouseHandler for Slider {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        let mouse_pos = event.position;
        let slider_rect = self.get_slider_rect();

        if slider_rect.contains(mouse_pos) {
            // Start dragging the slider
            self.is_dragging = true;
            self.drag_start_value = self.value;
            self.drag_start_pos = match self.orientation {
                Orientation::Horizontal => mouse_pos.x,
                Orientation::Vertical => mouse_pos.y,
            };
            self.update_state();
            self.dirty = true;
            EventResponse::Handled
        } else if self.get_track_bounds().contains(mouse_pos) {
            // Clicked on track - jump to that position
            let new_value = self.position_to_value(mouse_pos);
            self.notify_value_changed(new_value);
            EventResponse::Handled
        } else {
            EventResponse::Ignored
        }
    }

    fn on_mouse_up(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if self.is_dragging {
            self.is_dragging = false;
            self.update_state();
            self.dirty = true;
            EventResponse::Handled
        } else {
            EventResponse::Ignored
        }
    }

    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        let mouse_pos = event.position;

        if self.is_dragging {
            let new_value = self.position_to_value(mouse_pos);
            self.notify_value_changed(new_value);
            EventResponse::Handled
        } else {
            // Update hover state
            let slider_rect = self.get_slider_rect();
            let is_hovered = slider_rect.contains(mouse_pos);

            if is_hovered != self.is_slider_hovered {
                self.is_slider_hovered = is_hovered;
                self.update_state();
                self.dirty = true;
            }

            EventResponse::PassThrough
        }
    }

    fn on_mouse_enter(&mut self, event: &mut MouseEvent) -> EventResponse {
        let slider_rect = self.get_slider_rect();
        if slider_rect.contains(event.position) {
            self.is_slider_hovered = true;
            self.update_state();
            self.dirty = true;
        }
        EventResponse::PassThrough
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if !self.is_dragging {
            self.is_slider_hovered = false;
            self.update_state();
            self.dirty = true;
        }
        EventResponse::Handled
    }
}

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for Slider {
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

    fn drain_deferred_commands(&mut self) -> Vec<DeferredCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
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
        let track_bounds = self.get_track_bounds();

        // Draw label if present
        if let Some(ref label) = self.label {
            let label_style = TextStyle::new()
                .size(14.0)
                .color(self.label_color);

            let label_pos = Point::new(self.bounds.origin.x, self.bounds.origin.y + 2.0);
            ctx.draw_text(label, &label_style, label_pos, None);
        }

        // Calculate track rectangle (centered in track_bounds)
        let track_rect = match self.orientation {
            Orientation::Horizontal => Rect::new(
                Point::new(
                    track_bounds.origin.x,
                    track_bounds.origin.y + (track_bounds.size.height - self.track_height as f64) / 2.0,
                ),
                Size::new(track_bounds.size.width, self.track_height as f64),
            ),
            Orientation::Vertical => Rect::new(
                Point::new(
                    track_bounds.origin.x + (track_bounds.size.width - self.track_height as f64) / 2.0,
                    track_bounds.origin.y,
                ),
                Size::new(self.track_height as f64, track_bounds.size.height),
            ),
        };

        // Draw track background
        ctx.draw_styled_rect(
            track_rect,
            ShapeStyle {
                fill: Brush::Solid(self.track_color),
                corner_radius: CornerRadius::uniform(self.track_height / 2.0),
                border: None,
                shadow: None,
            },
        );

        // Draw filled portion
        let ratio = if self.max > self.min {
            (self.value - self.min) / (self.max - self.min)
        } else {
            0.0
        };

        let fill_rect = match self.orientation {
            Orientation::Horizontal => Rect::new(
                track_rect.origin,
                Size::new(track_rect.size.width * ratio, track_rect.size.height),
            ),
            Orientation::Vertical => Rect::new(
                track_rect.origin,
                Size::new(track_rect.size.width, track_rect.size.height * ratio),
            ),
        };

        ctx.draw_styled_rect(
            fill_rect,
            ShapeStyle {
                fill: Brush::Solid(self.fill_color),
                corner_radius: CornerRadius::uniform(self.track_height / 2.0),
                border: None,
                shadow: None,
            },
        );

        // Draw slider knob
        let slider_rect = self.get_slider_rect();
        let slider_color = self.get_slider_color();

        ctx.draw_styled_rect(
            slider_rect,
            ShapeStyle {
                fill: Brush::Solid(slider_color),
                corner_radius: CornerRadius::uniform(self.slider_size / 2.0),
                border: None,
                shadow: None,
            },
        );

        // Draw value if enabled
        if self.show_value {
            let value_text = self.format_value();
            let value_style = TextStyle::new()
                .size(14.0)
                .color(self.value_color);

            let value_pos = match self.orientation {
                Orientation::Horizontal => Point::new(
                    track_bounds.origin.x + track_bounds.size.width + 10.0,
                    track_bounds.origin.y + (track_bounds.size.height - 16.0) / 2.0,
                ),
                Orientation::Vertical => Point::new(
                    track_bounds.origin.x,
                    track_bounds.origin.y + track_bounds.size.height + 8.0,
                ),
            };

            ctx.draw_text(&value_text, &value_style, value_pos, None);
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
        false
    }

    fn preferred_cursor(&self) -> Option<crate::types::CursorType> {
        if self.is_slider_hovered || self.is_dragging {
            Some(crate::types::CursorType::Pointer)
        } else {
            None
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
