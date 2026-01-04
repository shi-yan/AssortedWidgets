use std::any::Any;

use crate::widget::Widget;
use crate::WidgetState;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::types::{DirtyLevel, DeferredCommand, GuiMessage, Rect, WidgetId};

/// A simple colored rectangle for debugging layouts
///
/// This widget renders a filled rectangle with the specified color.
/// Useful for visualizing layout boundaries and testing the rendering system.
pub struct DebugRect {
    state: WidgetState,
    layout_style: Style,
    color: Color,
    style: Style,
}

impl DebugRect {
    pub fn new(id: WidgetId, color: Color) -> Self {
        let _ = id; // Ignore id parameter for now
        DebugRect {
            state: WidgetState::new(),
            layout_style: Style::default(),
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

impl Widget for DebugRect {
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
        // Draw a filled rectangle with our color
        ctx.draw_rect(self.state.bounds, self.color);
    }
}
