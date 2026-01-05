use std::any::Any;

use crate::widget::Widget;
use crate::WidgetState;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::types::{DirtyLevel, DeferredCommand, GuiMessage, Rect, WidgetId, Point, Size};

/// A container that clips its content to its bounds
///
/// This widget demonstrates shader-based clipping by:
/// 1. Rendering a background color
/// 2. Pushing its bounds as a clip rect
/// 3. Rendering overflow content (will be clipped)
/// 4. Popping the clip rect
pub struct ClippedContainer {
    state: WidgetState,
    layout_style: Style,
    style: Style,
    bg_color: Color,
    overflow_color: Color,
}

impl ClippedContainer {
    pub fn new(id: WidgetId, bg_color: Color, overflow_color: Color) -> Self {
        ClippedContainer {
            state: WidgetState::with_id(id),
            layout_style: Style::default(),
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

impl Widget for ClippedContainer {
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
        self.style.clone()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw blue background (within bounds, no clipping needed)
        ctx.draw_rect(self.state.bounds, self.bg_color);

        // Push clip rect to clip to our bounds
        ctx.push_clip(self.state.bounds);

        // Draw a pattern that extends beyond bounds to demonstrate clipping
        // Draw stripes that extend past the clip boundary
        let stripe_width = 60.0;
        let num_stripes = 8;

        for i in 0..num_stripes {
            let x = self.state.bounds.origin.x - 100.0 + (i as f64 * stripe_width);
            let stripe_rect = Rect::new(
                Point::new(x, self.state.bounds.origin.y - 100.0),
                Size::new(40.0, self.state.bounds.size.height + 200.0),
            );
            ctx.draw_rect(stripe_rect, self.overflow_color);
        }

        // Pop clip rect
        ctx.pop_clip();
    }
}
