use std::any::Any;

use crate::types::{DeferredCommand, GuiMessage, Rect, Size, WidgetId};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;
use taffy::AvailableSpace;

// ============================================================================
// Element Trait
// ============================================================================

/// Base trait for all UI elements
///
/// Note: Element does not require Send since the GUI framework is single-threaded.
/// All elements live on the main thread and are managed by the event loop.
pub trait Element {
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

    /// Measure the element's intrinsic size given available space
    ///
    /// This is called by the layout system to determine the natural size of
    /// content-based elements (like text that wraps based on available width).
    ///
    /// Returns `None` if the element doesn't need custom measurement (uses
    /// style dimensions only). Returns `Some(size)` to provide intrinsic dimensions.
    ///
    /// # Arguments
    /// * `known_dimensions` - Dimensions that are already known (e.g., parent width)
    /// * `available_space` - Space available in each dimension
    ///
    /// # Example
    /// ```ignore
    /// fn measure(&self, known: Size<Option<f32>>, available: Size<AvailableSpace>) -> Option<Size> {
    ///     // For text that wraps based on width:
    ///     if let Some(width) = known.width {
    ///         let height = self.calculate_wrapped_height(width);
    ///         Some(Size::new(width as f64, height))
    ///     } else {
    ///         None
    ///     }
    /// }
    /// ```
    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<AvailableSpace>,
    ) -> Option<Size> {
        None // Default: no custom measurement
    }

    /// Check if this element needs a measure function
    ///
    /// Return true if this element's intrinsic size depends on its content
    /// (e.g., text, images with intrinsic dimensions).
    fn needs_measure(&self) -> bool {
        false // Default: static sizing
    }

    /// Mark this element as needing layout recalculation
    ///
    /// This should be called when the element's content changes in a way that
    /// affects its intrinsic size (e.g., text content changes, image loaded).
    fn mark_needs_layout(&mut self) {
        self.set_dirty(true);
    }

    /// Downcast to Any for type-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
