use std::any::Any;

use crate::widget::Widget;
use crate::WidgetState;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;
use crate::types::{DirtyLevel, DeferredCommand, GuiMessage, Rect, WidgetId};

/// A layout container that can hold child widgets
///
/// This widget uses Taffy for layout (Flexbox/Grid) and doesn't render anything itself.
/// It just positions its children according to the layout style.
pub struct Container {
    state: WidgetState,
    layout_style: Style,
    style: Style,
}

impl Container {
    pub fn new(style: Style) -> Self {
        Container {
            state: WidgetState::new(),
            layout_style: Style::default(),
            style,
        }
    }
}

impl Widget for Container {
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

    fn paint(&self, _ctx: &mut PaintContext) {
        // Container doesn't render anything - it just positions child widgets
    }
}
