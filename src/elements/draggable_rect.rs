//! Draggable rectangle widget for testing mouse capture and cross-window drag-drop
//!
//! This widget demonstrates:
//! - Mouse capture during drag operations
//! - Creating floating proxy windows
//! - Cross-window drag and drop
//! - Transparent/borderless window support

use crate::element::Element;
use crate::event::{EventResponse, MouseEvent, MouseHandler};
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::text::TextStyle;
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, WidgetId};

/// Draggable rectangle that can be dragged between windows
pub struct DraggableRect {
    id: WidgetId,
    bounds: Rect,
    is_dirty: bool,
    color: Color,
    label: String,

    // Drag state
    is_dragging: bool,
    drag_start_pos: Point,
    drag_offset: Point,
}

impl DraggableRect {
    /// Create a new draggable rectangle
    pub fn new(id: WidgetId, bounds: Rect, color: Color, label: &str) -> Self {
        Self {
            id,
            bounds,
            is_dirty: true,
            color,
            label: label.to_string(),
            is_dragging: false,
            drag_start_pos: Point::new(0.0, 0.0),
            drag_offset: Point::new(0.0, 0.0),
        }
    }

    /// Get the label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the color
    pub fn color(&self) -> Color {
        self.color
    }

    /// Check if currently dragging
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// Get drag offset (mouse position - rect origin)
    pub fn drag_offset(&self) -> Point {
        self.drag_offset
    }
}

impl Element for DraggableRect {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &crate::event::OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        if self.bounds != bounds {
            self.bounds = bounds;
            self.is_dirty = true;
        }
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn layout(&self) -> Style {
        Style {
            size: taffy::Size {
                width: taffy::Dimension::length(self.bounds.size.width as f32),
                height: taffy::Dimension::length(self.bounds.size.height as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw the rectangle with current color
        let alpha = if self.is_dragging { 0.5 } else { 1.0 };
        let color = Color::rgba(
            self.color.r,
            self.color.g,
            self.color.b,
            alpha,
        );
        ctx.draw_rect(self.bounds, color);

        // Draw label in center
        let text_pos = Point::new(
            self.bounds.origin.x + self.bounds.size.width / 2.0 - 40.0,
            self.bounds.origin.y + self.bounds.size.height / 2.0 - 10.0,
        );
        let label_style = TextStyle::new().size(16.0).color(Color::WHITE);
        ctx.draw_text(&self.label, &label_style, text_pos, None);

        // Draw "DRAG ME" hint
        let hint_pos = Point::new(
            self.bounds.origin.x + self.bounds.size.width / 2.0 - 30.0,
            self.bounds.origin.y + self.bounds.size.height / 2.0 + 10.0,
        );
        let hint_style = TextStyle::new()
            .size(12.0)
            .color(Color::rgba(200.0/255.0, 200.0/255.0, 200.0/255.0, 1.0));
        ctx.draw_text("DRAG ME", &hint_style, hint_pos, None);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn dispatch_mouse_event(
        &mut self,
        event: &mut crate::event::InputEventEnum,
    ) -> crate::event::EventResponse {
        use crate::event::InputEventEnum;

        match event {
            InputEventEnum::MouseDown(mouse_event) => {
                self.on_mouse_down(mouse_event)
            }
            InputEventEnum::MouseUp(mouse_event) => {
                self.on_mouse_up(mouse_event)
            }
            InputEventEnum::MouseMove(mouse_event) => {
                self.on_mouse_move(mouse_event)
            }
            _ => EventResponse::Ignored,
        }
    }
}

impl MouseHandler for DraggableRect {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        println!(
            "[DraggableRect '{}'] Mouse down at ({}, {})",
            self.label, event.position.x, event.position.y
        );

        // Start drag
        self.is_dragging = true;
        self.drag_start_pos = event.position;
        self.drag_offset = Point::new(
            event.position.x - self.bounds.origin.x,
            event.position.y - self.bounds.origin.y,
        );

        println!(
            "[DraggableRect '{}'] Starting drag - offset: ({}, {})",
            self.label, self.drag_offset.x, self.drag_offset.y
        );

        // Note: Mouse capture will be handled by the window
        // The window needs to call mouse_capture.capture(self.id)

        EventResponse::Handled
    }

    fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
        if !self.is_dragging {
            return EventResponse::Ignored;
        }

        println!(
            "[DraggableRect '{}'] Mouse up at ({}, {}) - ending drag",
            self.label, event.position.x, event.position.y
        );

        self.is_dragging = false;

        // Note: Mouse capture release will be handled by the window

        EventResponse::Handled
    }

    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        if !self.is_dragging {
            return EventResponse::Ignored;
        }

        // Update position during drag
        let new_x = event.position.x - self.drag_offset.x;
        let new_y = event.position.y - self.drag_offset.y;

        self.bounds.origin.x = new_x;
        self.bounds.origin.y = new_y;
        self.is_dirty = true;

        println!(
            "[DraggableRect '{}'] Dragging to ({}, {})",
            self.label, new_x, new_y
        );

        EventResponse::Handled
    }
}
