use std::any::Any;

use crate::types::{DeferredCommand, GuiMessage, Rect, WidgetId};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;

// ============================================================================
// Element Trait
// ============================================================================

/// Base trait for all UI elements
pub trait Element: Send {
    /// Returns the unique ID of this element
    fn id(&self) -> WidgetId;

    /// Handle incoming messages (the "slot" function)
    fn on_message(&mut self, message: &GuiMessage) -> Vec<DeferredCommand>;

    /// Handle OS events (mouse, keyboard, etc.)
    fn on_event(&mut self, event: &OsEvent) -> Vec<DeferredCommand>;

    /// Get element bounds for hit testing (set by layout system)
    fn bounds(&self) -> Rect;

    /// Set element bounds (called by layout system)
    fn set_bounds(&mut self, bounds: Rect);

    /// Mark this element as needing redraw
    fn set_dirty(&mut self, dirty: bool);

    /// Check if element needs redraw
    fn is_dirty(&self) -> bool;

    /// Get layout style for Taffy
    ///
    /// This defines how the element should be laid out (flex, grid, size, etc.)
    fn layout(&self) -> Style;

    /// Paint the element
    ///
    /// This is called during the paint pass after layout has been computed.
    /// Use the PaintContext to draw primitives.
    fn paint(&self, ctx: &mut PaintContext);

    /// Downcast to Any for type-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
