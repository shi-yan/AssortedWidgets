//! Clickable rectangle widget for testing hit testing and event handling

use crate::widget::Widget;
use crate::WidgetState;
use crate::event::{EventResponse, MouseEvent, MouseHandler, OsEvent};
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::types::{DirtyLevel, DeferredCommand, GuiMessage, Rect, WidgetId};

/// A simple clickable colored rectangle for testing hit testing
///
/// This widget demonstrates:
/// - Interactive widget with MouseHandler implementation
/// - Hit testing with z-order (registers hitbox during paint)
/// - Event logging to terminal for debugging
pub struct ClickableRect {
    state: WidgetState,
    layout_style: Style,
    color: Color,
    hover_color: Color,
    label: String,
    is_hovered: bool,
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

        let _ = (id, bounds); // Ignore parameters
        ClickableRect {
            state: WidgetState::new(),
            layout_style: Style::default(),
            color,
            hover_color,
            label: label.into(),
            is_hovered: false,
        }
    }

    /// Set the label text
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }
}

impl Widget for ClickableRect {
    fn id(&self) -> WidgetId {
        self.state.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.state.id = id;
    }

    fn bounds(&self) -> Rect {
        self.state.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.state.bounds = bounds;
    }

    fn dirty_level(&self) -> crate::types::DirtyLevel {
        self.state.dirty
    }

    fn set_dirty_level(&mut self, level: crate::types::DirtyLevel) {
        self.state.dirty = level;
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.set_dirty_level(crate::types::DirtyLevel::from(dirty));
    }

    fn is_dirty(&self) -> bool {
        self.dirty_level().needs_repaint()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }




    fn layout(&self) -> Style {
        Style {
            size: taffy::Size {
                width: taffy::Dimension::length(self.state.bounds.size.width as f32),
                height: taffy::Dimension::length(self.state.bounds.size.height as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw the rectangle with current color (hover or normal)
        let current_color = if self.is_hovered {
            self.hover_color
        } else {
            self.color
        };

        ctx.draw_rect(self.state.bounds, current_color);

        // Draw label in the center (if we had text rendering set up)
        // For now, just draw the rect
    }

    fn is_interactive(&self) -> bool {
        true // This element handles mouse events
    }

    fn is_focusable(&self) -> bool {
        false // Not keyboard-focusable (yet)
    }
}

impl MouseHandler for ClickableRect {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🖱️  MOUSE DOWN on {} (ID: {:?})", self.label, self.state.id);
        println!("   Position: ({:.1}, {:.1})", event.position.x, event.position.y);
        println!("   Bounds: ({:.0}, {:.0}, {:.0}, {:.0})",
                 self.state.bounds.origin.x,
                 self.state.bounds.origin.y,
                 self.state.bounds.size.width,
                 self.state.bounds.size.height);
        println!("   Color: rgba({:.2}, {:.2}, {:.2}, {:.2})",
                 self.color.r, self.color.g, self.color.b, self.color.a);
        println!("   Button: {:?}", event.button);
        println!("   Modifiers: {:?}", event.modifiers);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Consume the event (stop propagation)
        EventResponse::Handled
    }

    fn on_mouse_up(&mut self, _event: &mut MouseEvent) -> EventResponse {
        println!("🖱️  MOUSE UP on {} (ID: {:?})", self.label, self.state.id);
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

    fn on_mouse_enter(&mut self, _event: &mut MouseEvent) -> EventResponse {
        self.is_hovered = true;
        println!("→ ENTER: {}", self.label);
        EventResponse::Ignored
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        self.is_hovered = false;
        println!("← LEAVE: {}", self.label);
        EventResponse::Ignored
    }
}
