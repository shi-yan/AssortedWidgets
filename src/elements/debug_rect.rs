use std::any::Any;

use crate::element::Element;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::types::{DeferredCommand, GuiMessage, Rect, WidgetId};

/// A simple colored rectangle for debugging layouts
///
/// This element renders a filled rectangle with the specified color.
/// Useful for visualizing layout boundaries and testing the rendering system.
pub struct DebugRect {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    color: Color,
    style: Style,
}

impl DebugRect {
    pub fn new(id: WidgetId, color: Color) -> Self {
        DebugRect {
            id,
            bounds: Rect::default(),
            dirty: true,
            color,
            style: Style::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_flex_grow(mut self, grow: f32) -> Self {
        self.style.flex_grow = grow;
        self
    }
}

impl Element for DebugRect {
    fn id(&self) -> WidgetId {
        self.id
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
        self.bounds = bounds;
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout(&self) -> Style {
        self.style.clone()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw a filled rectangle with our color
        ctx.draw_rect(self.bounds, self.color);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
