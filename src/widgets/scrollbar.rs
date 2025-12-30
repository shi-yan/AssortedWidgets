//! ScrollBar widget - minimal scrollbar for horizontal and vertical scrolling
//!
//! Features:
//! - Horizontal and vertical orientation
//! - Integer values (for discrete scrolling like line numbers)
//! - Minimal design: capsule slider + transparent background
//! - Hover and dragging states
//! - Pointer cursor on hover
//! - Configurable width (bar thickness)
//! - Value change callback
//!
//! # Example
//! ```rust,ignore
//! // Vertical scrollbar
//! let vscroll = ScrollBar::vertical(0, 100, 10)
//!     .width(12.0)
//!     .on_value_changed(|value| {
//!         println!("Scrolled to: {}", value);
//!     });
//!
//! // Horizontal scrollbar
//! let hscroll = ScrollBar::horizontal(0, 200, 20)
//!     .width(10.0)
//!     .slider_color(Color::rgb(0.4, 0.6, 0.8));
//! ```

use std::any::Any;

use crate::event::handlers::MouseHandler;
use crate::event::input::{EventResponse, InputEventEnum, MouseEvent};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::primitives::Color;
use crate::paint::types::{Brush, CornerRadius, ShapeStyle};
use crate::paint::PaintContext;
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;

/// Scrollbar orientation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Scrollbar state (determines visual appearance)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScrollBarState {
    Normal,
    Hovered,
    Dragging,
}

/// ScrollBar widget - minimal scrollbar with integer values
///
/// The scrollbar uses integer values for discrete scrolling (e.g., line numbers in text).
/// The visual slider size is calculated based on the page_size relative to the total range.
pub struct ScrollBar {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Orientation
    orientation: Orientation,

    // Value range (all integers for discrete scrolling)
    value: i32,
    min: i32,
    max: i32,
    page_size: i32, // How much content is visible (determines slider size)

    // Visual state
    state: ScrollBarState,
    is_dragging: bool,
    is_slider_hovered: bool, // True when mouse is over the slider (not just the track)
    drag_start_value: i32,
    drag_start_pos: f64, // Mouse position when drag started

    // Styling
    bar_width: f32,              // Thickness of the scrollbar (perpendicular to scroll direction)
    track_color: Color,          // Background color (default: transparent)
    slider_color: Color,         // Normal slider color
    slider_hover_color: Color,   // Hovered slider color
    slider_drag_color: Color,    // Dragging slider color
    slider_corner_radius: f32,   // Corner radius for capsule shape

    // Callback
    on_value_changed: Option<Box<dyn FnMut(i32)>>,

    // Cached slider bounds (updated during paint)
    _cached_slider_rect: Option<Rect>,

    // Pending deferred commands (signals to emit)
    pending_commands: Vec<DeferredCommand>,
}

impl ScrollBar {
    /// Create a new scrollbar with the given orientation and value range
    ///
    /// # Arguments
    /// * `orientation` - Horizontal or Vertical
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    /// * `page_size` - How much content is visible (determines slider size)
    pub fn new(orientation: Orientation, min: i32, max: i32, page_size: i32) -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            orientation,
            value: min,
            min,
            max,
            page_size: page_size.max(1), // Ensure page_size is at least 1
            state: ScrollBarState::Normal,
            is_dragging: false,
            is_slider_hovered: false,
            drag_start_value: 0,
            drag_start_pos: 0.0,
            bar_width: 12.0,
            track_color: Color::TRANSPARENT,
            slider_color: Color::rgba(0.6, 0.6, 0.6, 0.7),
            slider_hover_color: Color::rgba(0.7, 0.7, 0.7, 0.85),
            slider_drag_color: Color::rgba(0.5, 0.5, 0.5, 0.9),
            slider_corner_radius: 6.0,
            on_value_changed: None,
            _cached_slider_rect: None,
            pending_commands: Vec::new(),
        }
    }

    /// Create a vertical scrollbar
    pub fn vertical(min: i32, max: i32, page_size: i32) -> Self {
        Self::new(Orientation::Vertical, min, max, page_size)
    }

    /// Create a horizontal scrollbar
    pub fn horizontal(min: i32, max: i32, page_size: i32) -> Self {
        Self::new(Orientation::Horizontal, min, max, page_size)
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set the scrollbar width (thickness perpendicular to scroll direction)
    pub fn width(mut self, width: f32) -> Self {
        self.bar_width = width.max(4.0); // Minimum 4px
        self.slider_corner_radius = width / 2.0; // Capsule shape
        self
    }

    /// Set the track (background) color
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    /// Set the slider color (normal state)
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
        F: FnMut(i32) + 'static,
    {
        self.on_value_changed = Some(Box::new(callback));
        self
    }

    /// Set layout style (for Taffy)
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Set the current value (clamped to [min, max])
    pub fn set_value(&mut self, value: i32) {
        let clamped = value.clamp(self.min, self.max);
        if self.value != clamped {
            self.value = clamped;
            self.dirty = true;
        }
    }

    /// Get the current value
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Get the maximum value
    pub fn max(&self) -> i32 {
        self.max
    }

    /// Set the value range
    pub fn set_range(&mut self, min: i32, max: i32) {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max);
        self.dirty = true;
    }

    /// Set the page size (how much content is visible)
    pub fn set_page_size(&mut self, page_size: i32) {
        self.page_size = page_size.max(1);
        self.dirty = true;
    }

    // ========================================================================
    // Internal Helpers
    // ========================================================================

    /// Calculate the slider rectangle based on current value and bounds
    fn calculate_slider_rect(&self) -> Rect {
        if self.max <= self.min {
            // Invalid range, return empty rect
            return self.bounds;
        }

        let total_range = (self.max - self.min) as f64;
        let value_ratio = (self.value - self.min) as f64 / total_range;

        // Slider size is proportional to page_size / (total_range + page_size)
        // This makes the slider represent the visible portion
        let slider_ratio = (self.page_size as f64) / (total_range + self.page_size as f64);

        match self.orientation {
            Orientation::Vertical => {
                let track_height = self.bounds.size.height;
                let slider_height = (track_height * slider_ratio).max(20.0); // Minimum 20px
                let available_travel = track_height - slider_height;
                let slider_y = self.bounds.origin.y + available_travel * value_ratio;

                Rect::new(
                    Point::new(self.bounds.origin.x, slider_y),
                    Size::new(self.bounds.size.width, slider_height),
                )
            }
            Orientation::Horizontal => {
                let track_width = self.bounds.size.width;
                let slider_width = (track_width * slider_ratio).max(20.0); // Minimum 20px
                let available_travel = track_width - slider_width;
                let slider_x = self.bounds.origin.x + available_travel * value_ratio;

                Rect::new(
                    Point::new(slider_x, self.bounds.origin.y),
                    Size::new(slider_width, self.bounds.size.height),
                )
            }
        }
    }

    /// Convert mouse position to value
    fn position_to_value(&self, mouse_pos: Point) -> i32 {
        if self.max <= self.min {
            return self.min;
        }

        let total_range = (self.max - self.min) as f64;
        let slider_rect = self.calculate_slider_rect();

        let value = match self.orientation {
            Orientation::Vertical => {
                let track_height = self.bounds.size.height;
                let slider_height = slider_rect.size.height;
                let available_travel = track_height - slider_height;

                if available_travel <= 0.0 {
                    return self.min;
                }

                let mouse_y = mouse_pos.y - self.bounds.origin.y;
                let ratio = mouse_y / available_travel;
                let value = self.min as f64 + ratio * total_range;
                value.round() as i32
            }
            Orientation::Horizontal => {
                let track_width = self.bounds.size.width;
                let slider_width = slider_rect.size.width;
                let available_travel = track_width - slider_width;

                if available_travel <= 0.0 {
                    return self.min;
                }

                let mouse_x = mouse_pos.x - self.bounds.origin.x;
                let ratio = mouse_x / available_travel;
                let value = self.min as f64 + ratio * total_range;
                value.round() as i32
            }
        };

        value.clamp(self.min, self.max)
    }

    /// Update state based on hover flag
    fn update_state(&mut self) {
        self.state = if self.is_dragging {
            ScrollBarState::Dragging
        } else {
            ScrollBarState::Normal
        };
    }

    /// Get the current slider color based on state
    fn get_slider_color(&self) -> Color {
        match self.state {
            ScrollBarState::Normal => self.slider_color,
            ScrollBarState::Hovered => self.slider_hover_color,
            ScrollBarState::Dragging => self.slider_drag_color,
        }
    }

    /// Trigger value changed callback if value actually changed
    fn notify_value_changed(&mut self, new_value: i32) {
        if new_value != self.value {
            self.value = new_value;
            self.dirty = true;

            // Call the callback if present
            if let Some(ref mut callback) = self.on_value_changed {
                callback(new_value);
            }

            // Emit a signal that other widgets can listen to
            self.pending_commands.push(DeferredCommand {
                target: self.id,
                message: GuiMessage::Custom {
                    source: self.id,
                    signal_type: "value_changed".to_string(),
                    data: Box::new(new_value),
                },
            });
        }
    }
}

// ========================================================================
// Event Handlers
// ========================================================================

impl MouseHandler for ScrollBar {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        let mouse_pos = event.position;
        let slider_rect = self.calculate_slider_rect();

        if slider_rect.contains(mouse_pos) {
            // Start dragging the slider
            self.is_dragging = true;
            self.drag_start_value = self.value;
            self.drag_start_pos = match self.orientation {
                Orientation::Vertical => mouse_pos.y,
                Orientation::Horizontal => mouse_pos.x,
            };
            self.update_state();
            self.dirty = true;
            EventResponse::Handled
        } else if self.bounds.contains(mouse_pos) {
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
            // Calculate value based on drag distance
            let current_pos = match self.orientation {
                Orientation::Vertical => mouse_pos.y,
                Orientation::Horizontal => mouse_pos.x,
            };

            let delta = current_pos - self.drag_start_pos;
            let slider_rect = self.calculate_slider_rect();

            let available_travel = match self.orientation {
                Orientation::Vertical => self.bounds.size.height - slider_rect.size.height,
                Orientation::Horizontal => self.bounds.size.width - slider_rect.size.width,
            };

            if available_travel > 0.0 {
                let total_range = (self.max - self.min) as f64;
                let delta_value = (delta / available_travel) * total_range;
                let new_value = (self.drag_start_value as f64 + delta_value).round() as i32;
                self.notify_value_changed(new_value.clamp(self.min, self.max));
            }

            EventResponse::Handled
        } else {
            // Update hover state
            let slider_rect = self.calculate_slider_rect();
            let is_hovered = slider_rect.contains(mouse_pos);
            let was_hovered = matches!(self.state, ScrollBarState::Hovered);

            if is_hovered != was_hovered {
                self.is_slider_hovered = is_hovered;
                self.state = if is_hovered {
                    ScrollBarState::Hovered
                } else {
                    ScrollBarState::Normal
                };
                self.dirty = true;
            }

            EventResponse::PassThrough
        }
    }

    fn on_mouse_enter(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check if mouse entered directly over the slider
        let slider_rect = self.calculate_slider_rect();
        if slider_rect.contains(event.position) {
            self.is_slider_hovered = true;
            self.state = ScrollBarState::Hovered;
            self.dirty = true;
        }
        EventResponse::PassThrough
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if !self.is_dragging {
            self.is_slider_hovered = false;
            self.state = ScrollBarState::Normal;
            self.dirty = true;
        }
        EventResponse::Handled
    }
}

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for ScrollBar {
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
        // Draw track (background)
        if self.track_color.a > 0.0 {
            ctx.draw_styled_rect(
                self.bounds,
                ShapeStyle {
                    fill: Brush::Solid(self.track_color),
                    corner_radius: CornerRadius::uniform(self.bar_width / 2.0),
                    border: None,
                    shadow: None,
                },
            );
        }

        // Calculate slider rect
        let slider_rect = self.calculate_slider_rect();

        // Draw slider (capsule shape)
        // IMPORTANT: Corner radius must not exceed half the smaller dimension
        let slider_corner_radius = match self.orientation {
            Orientation::Horizontal => {
                (slider_rect.size.height as f32 / 2.0).min(slider_rect.size.width as f32 / 2.0)
            }
            Orientation::Vertical => {
                (slider_rect.size.width as f32 / 2.0).min(slider_rect.size.height as f32 / 2.0)
            }
        };

        let slider_color = self.get_slider_color();

        ctx.draw_styled_rect(
            slider_rect,
            ShapeStyle {
                fill: Brush::Solid(slider_color),
                corner_radius: CornerRadius::uniform(slider_corner_radius),
                border: None,
                shadow: None,
            },
        );
    }

    fn needs_measure(&self) -> bool {
        true // Match Button/Label pattern - required for layout system
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        None // No intrinsic size - uses layout style dimensions
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn is_focusable(&self) -> bool {
        false // Scrollbars typically don't get keyboard focus
    }

    fn preferred_cursor(&self) -> Option<crate::types::CursorType> {
        // Show pointer cursor when hovering over the slider or dragging
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
