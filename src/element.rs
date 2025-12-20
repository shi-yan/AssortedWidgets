use std::any::Any;

use crate::types::{DeferredCommand, GuiMessage, Rect, WidgetId};
use crate::event::OsEvent;

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

    /// Get element bounds for hit testing
    fn bounds(&self) -> Rect;

    /// Mark this element as needing redraw
    fn set_dirty(&mut self, dirty: bool);

    /// Check if element needs redraw
    fn is_dirty(&self) -> bool;

    /// Downcast to Any for type-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
