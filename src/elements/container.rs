use std::any::Any;

use crate::widget::Widget;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;
use crate::types::{DeferredCommand, GuiMessage, Rect, WidgetId};

/// A layout container that can hold child widgets
///
/// This widget uses Taffy for layout (Flexbox/Grid) and doesn't render anything itself.
/// It just positions its children according to the layout style.
pub struct Container {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    style: Style,
}

impl Container {
    pub fn new(id: WidgetId, style: Style) -> Self {
        Container {
            id,
            bounds: Rect::default(),
            dirty: true,
            style,
        }
    }
}

impl Widget for Container {
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

    fn paint(&self, _ctx: &mut PaintContext) {
        // Container doesn't render anything - it just positions child widgets
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
