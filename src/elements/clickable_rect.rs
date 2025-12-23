//! Clickable rectangle widget for testing hit testing and event handling

use crate::element::Element;
use crate::event::{EventResponse, MouseEvent, MouseHandler, OsEvent};
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::types::{DeferredCommand, GuiMessage, Rect, WidgetId};

/// A simple clickable colored rectangle for testing hit testing
///
/// This widget demonstrates:
/// - Interactive element with MouseHandler implementation
/// - Hit testing with z-order (registers hitbox during paint)
/// - Event logging to terminal for debugging
pub struct ClickableRect {
    id: WidgetId,
    bounds: Rect,
    color: Color,
    hover_color: Color,
    label: String,
    is_hovered: bool,
    is_dirty: bool,
}

impl ClickableRect {
    /// Create a new clickable rectangle
    pub fn new(id: WidgetId, bounds: Rect, color: Color, label: impl Into<String>) -> Self {
        // Calculate hover color (slightly brighter)
        let hover_color = Color {
            r: (color.r * 1.2).min(1.0),
            g: (color.g * 1.2).min(1.0),
            b: (color.b * 1.2).min(1.0),
            a: color.a,
        };

        ClickableRect {
            id,
            bounds,
            color,
            hover_color,
            label: label.into(),
            is_hovered: false,
            is_dirty: true,
        }
    }

    /// Set the label text
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }
}

impl Element for ClickableRect {
    fn id(&self) -> WidgetId {
        self.id
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

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
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
        // IMPORTANT: Register hitbox BEFORE drawing
        // This ensures the z-order matches the visual layering
        ctx.register_hitbox(self.id, self.bounds);

        // Draw the rectangle with current color (hover or normal)
        let current_color = if self.is_hovered {
            self.hover_color
        } else {
            self.color
        };

        ctx.draw_rect(self.bounds, current_color);

        // Draw label in the center (if we had text rendering set up)
        // For now, just draw the rect
    }

    fn is_interactive(&self) -> bool {
        true // This element handles mouse events
    }

    fn is_focusable(&self) -> bool {
        false // Not keyboard-focusable (yet)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl MouseHandler for ClickableRect {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🖱️  MOUSE DOWN on {} (ID: {:?})", self.label, self.id);
        println!("   Position: ({:.1}, {:.1})", event.position.x, event.position.y);
        println!("   Bounds: ({:.0}, {:.0}, {:.0}, {:.0})",
                 self.bounds.origin.x,
                 self.bounds.origin.y,
                 self.bounds.size.width,
                 self.bounds.size.height);
        println!("   Color: rgba({:.2}, {:.2}, {:.2}, {:.2})",
                 self.color.r, self.color.g, self.color.b, self.color.a);
        println!("   Button: {:?}", event.button);
        println!("   Modifiers: {:?}", event.modifiers);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Consume the event (stop propagation)
        EventResponse::Handled
    }

    fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
        println!("🖱️  MOUSE UP on {} (ID: {:?})", self.label, self.id);
        EventResponse::Handled
    }

    fn on_mouse_move(&mut self, _event: &mut MouseEvent) -> EventResponse {
        // Don't log every mouse move (too noisy), but we could track hover state
        if !self.is_hovered {
            self.is_hovered = true;
            println!("🎯 HOVER ENTER: {}", self.label);
        }
        EventResponse::PassThrough // Let mouse moves propagate
    }

    fn on_mouse_enter(&mut self) -> EventResponse {
        self.is_hovered = true;
        println!("→ ENTER: {}", self.label);
        EventResponse::Ignored
    }

    fn on_mouse_leave(&mut self) -> EventResponse {
        self.is_hovered = false;
        println!("← LEAVE: {}", self.label);
        EventResponse::Ignored
    }
}
