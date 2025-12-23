use std::any::Any;

use crate::types::{DeferredCommand, FrameInfo, GuiMessage, Rect, Size, WidgetId};
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

    /// Update element state (called once per frame before layout)
    ///
    /// This is called by the window's render loop before layout computation.
    /// Use this for animations, time-based state changes, physics, etc.
    ///
    /// # Arguments
    /// * `frame` - Frame timing information (delta time, timestamp, frame number)
    ///
    /// # Example
    /// ```ignore
    /// fn update(&mut self, frame: &FrameInfo) {
    ///     // Frame-rate independent animation
    ///     self.rotation += self.angular_velocity * frame.dt;
    ///
    ///     // Or use elapsed time for oscillations
    ///     let elapsed = (frame.timestamp - self.start_time).as_secs_f64();
    ///     self.scale = 1.0 + 0.2 * (elapsed * 2.0 * PI).sin();
    ///
    ///     // Mark layout dirty if size/position changed
    ///     self.mark_needs_layout();
    /// }
    /// ```
    ///
    /// Default implementation does nothing. Override for animated elements.
    fn update(&mut self, _frame: &FrameInfo) {
        // Default: no update logic
    }

    /// Check if this element needs continuous frame updates
    ///
    /// Return `true` for elements that animate or change over time.
    /// The window will only call `update()` on elements that return `true`.
    ///
    /// This is an optimization to avoid calling update() on static elements.
    ///
    /// # Performance
    /// - Returning `true` means update() is called every frame (60+ times/sec)
    /// - Only return `true` if the element actually needs continuous updates
    /// - For one-shot animations, toggle this flag on/off as needed
    ///
    /// Default: `false` (static element, no updates needed)
    fn needs_continuous_updates(&self) -> bool {
        false // Default: static element
    }

    /// Downcast to Any for type-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // ========================================
    // Event Dispatch Methods (Phase 1)
    // ========================================

    /// Check if this element can receive input events (mouse, keyboard, etc.)
    ///
    /// Return `true` for elements that implement event handlers (MouseHandler,
    /// KeyboardHandler, etc.) to allow them to be considered during hit testing.
    ///
    /// Default: `false` (non-interactive element)
    fn is_interactive(&self) -> bool {
        false // Default: does not receive input events
    }

    /// Check if this element can receive keyboard focus
    ///
    /// Return `true` for elements that accept keyboard input (text fields,
    /// buttons, etc.) to allow them to be included in focus navigation.
    ///
    /// Default: `false` (not focusable)
    fn is_focusable(&self) -> bool {
        false // Default: does not accept focus
    }

    /// Get IME cursor position for this element (if focused)
    ///
    /// Return a rectangle representing where the IME composition window should
    /// be positioned. This is typically at the text insertion point.
    ///
    /// Default: `None` (no IME support)
    fn ime_cursor_rect(&self) -> Option<Rect> {
        None // Default: no IME support
    }

    /// Dispatch mouse event to this element
    ///
    /// Default implementation returns Ignored. Elements that implement MouseHandler
    /// should override this to call their handler methods.
    fn dispatch_mouse_event(&mut self, event: &mut crate::event::InputEventEnum) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }

    /// Dispatch keyboard event to this element
    ///
    /// Default implementation returns Ignored. Elements that implement KeyboardHandler
    /// should override this to call their handler methods.
    fn dispatch_key_event(&mut self, event: &mut crate::event::InputEventEnum) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }

    /// Dispatch wheel event to this element
    ///
    /// Default implementation returns Ignored. Elements that implement WheelHandler
    /// should override this to call their handler methods.
    fn dispatch_wheel_event(&mut self, event: &mut crate::event::WheelEvent) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }

    /// Dispatch IME event to this element
    ///
    /// Default implementation returns Ignored. Elements that implement ImeHandler
    /// should override this to call their handler methods.
    fn dispatch_ime_event(&mut self, event: &mut crate::event::ImeEvent) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }
}
