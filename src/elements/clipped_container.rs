use std::any::Any;

use crate::element::Element;
use crate::event::OsEvent;
use crate::layout::{Style, Dimension};
use crate::paint::{Color, PaintContext};
use crate::types::{DeferredCommand, GuiMessage, Rect, WidgetId, Point, Size};

/// A container that clips its content to its bounds
///
/// This element demonstrates shader-based clipping by:
/// 1. Rendering a background color
/// 2. Pushing its bounds as a clip rect
/// 3. Rendering overflow content (will be clipped)
/// 4. Popping the clip rect
pub struct ClippedContainer {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    style: Style,
    bg_color: Color,
    overflow_color: Color,
}

impl ClippedContainer {
    pub fn new(id: WidgetId, bg_color: Color, overflow_color: Color) -> Self {
        ClippedContainer {
            id,
            bounds: Rect::default(),
            dirty: true,
            style: Style::default(),
            bg_color,
            overflow_color,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Element for ClippedContainer {
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
        // Draw background (within bounds, no clipping needed)
        ctx.draw_rect(self.bounds, self.bg_color);

        // Push clip rect to clip to our bounds
        ctx.push_clip(self.bounds);

        // Draw overflow content - a larger rectangle that extends beyond our bounds
        // This will be clipped by the shader
        let overflow_rect = Rect::new(
            Point::new(
                self.bounds.origin.x - 50.0,
                self.bounds.origin.y - 50.0,
            ),
            Size::new(
                self.bounds.size.width + 100.0,
                self.bounds.size.height + 100.0,
            ),
        );
        ctx.draw_rect(overflow_rect, self.overflow_color);

        // Pop clip rect
        ctx.pop_clip();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
