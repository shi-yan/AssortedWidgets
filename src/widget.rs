use std::any::Any;

use crate::types::{DeferredCommand, FrameInfo, GuiMessage, Rect, Size, WidgetId};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;
use taffy::AvailableSpace;

// ============================================================================
// Widget Boilerplate Macro
// ============================================================================

/// Implements the common boilerplate methods for a widget.
///
/// This macro assumes your widget struct has the following fields:
/// - `id: WidgetId`
/// - `bounds: Rect`
///
/// # Usage
/// ```ignore
/// impl Widget for MyWidget {
///     impl_widget_essentials!();
///
///     fn paint(&self, ctx: &mut PaintContext) {
///         // Your custom paint logic...
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_widget_essentials {
    () => {
        fn id(&self) -> $crate::types::WidgetId {
            self.id
        }

        fn bounds(&self) -> $crate::types::Rect {
            self.bounds
        }

        fn set_bounds(&mut self, bounds: $crate::types::Rect) {
            self.bounds = bounds;
        }

        fn set_dirty(&mut self, _dirty: bool) {
            // Default: no dirty tracking
        }

        fn is_dirty(&self) -> bool {
            false // Default: never dirty
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    };
}

// ============================================================================
// Widget Trait
// ============================================================================

/// Base trait for all UI widgets
///
/// Note: Widget does not require Send since the GUI framework is single-threaded.
/// All widgets live on the main thread and are managed by the event loop.
pub trait Widget {
    /// Returns the unique ID of this widget
    fn id(&self) -> WidgetId;

    /// Handle incoming messages (the "slot" function)
    ///
    /// Default: ignores all messages (returns empty command list)
    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        vec![]
    }

    /// Handle OS events (mouse, keyboard, etc.)
    ///
    /// Default: ignores all events (returns empty command list)
    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        vec![]
    }

    /// Get widget bounds for hit testing (set by layout system)
    fn bounds(&self) -> Rect;

    /// Set widget bounds (called by layout system)
    fn set_bounds(&mut self, bounds: Rect);

    /// Mark this widget as needing redraw
    fn set_dirty(&mut self, dirty: bool);

    /// Check if widget needs redraw
    fn is_dirty(&self) -> bool;

    /// Get layout style for Taffy
    ///
    /// This defines how the element should be laid out (flex, grid, size, etc.)
    ///
    /// Default: returns default style (block layout with auto sizing)
    fn layout(&self) -> Style {
        Style::default()
    }

    /// Paint the widget
    ///
    /// This is called during the paint pass after layout has been computed.
    /// Use the PaintContext to draw primitives.
    fn paint(&self, ctx: &mut PaintContext);

    /// Measure the widget's intrinsic size given available space
    ///
    /// This is called by the layout system to determine the natural size of
    /// content-based widgets (like text that wraps based on available width).
    ///
    /// Returns `None` if the widget doesn't need custom measurement (uses
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

    /// Check if this widget needs a measure function
    ///
    /// Return true if this widget's intrinsic size depends on its content
    /// (e.g., text, images with intrinsic dimensions).
    fn needs_measure(&self) -> bool {
        false // Default: static sizing
    }

    /// Mark this widget as needing layout recalculation
    ///
    /// This should be called when the widget's content changes in a way that
    /// affects its intrinsic size (e.g., text content changes, image loaded).
    fn mark_needs_layout(&mut self) {
        self.set_dirty(true);
    }

    /// Update widget state (called once per frame before layout)
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
    /// Default implementation does nothing. Override for animated widgets.
    fn update(&mut self, _frame: &FrameInfo) {
        // Default: no update logic
    }

    /// Check if this widget needs continuous frame updates
    ///
    /// Return `true` for widgets that animate or change over time.
    /// The window will only call `update()` on widgets that return `true`.
    ///
    /// This is an optimization to avoid calling update() on static widgets.
    ///
    /// # Performance
    /// - Returning `true` means update() is called every frame (60+ times/sec)
    /// - Only return `true` if the widget actually needs continuous updates
    /// - For one-shot animations, toggle this flag on/off as needed
    ///
    /// Default: `false` (static widget, no updates needed)
    fn needs_continuous_updates(&self) -> bool {
        false // Default: static widget
    }

    /// Downcast to Any for type-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // ========================================
    // Event Dispatch Methods (Phase 1)
    // ========================================

    /// Check if this widget can receive input events (mouse, keyboard, etc.)
    ///
    /// Return `true` for widgets that implement event handlers (MouseHandler,
    /// KeyboardHandler, etc.) to allow them to be considered during hit testing.
    ///
    /// Default: `false` (non-interactive widget)
    fn is_interactive(&self) -> bool {
        false // Default: does not receive input events
    }

    /// Check if this widget can receive keyboard focus
    ///
    /// Return `true` for widgets that accept keyboard input (text fields,
    /// buttons, etc.) to allow them to be included in focus navigation.
    ///
    /// Default: `false` (not focusable)
    fn is_focusable(&self) -> bool {
        false // Default: does not accept focus
    }

    /// Get IME cursor position for this widget (if focused)
    ///
    /// Return a rectangle representing where the IME composition window should
    /// be positioned. This is typically at the text insertion point.
    ///
    /// Default: `None` (no IME support)
    fn ime_cursor_rect(&self) -> Option<Rect> {
        None // Default: no IME support
    }

    /// Dispatch mouse event to this widget
    ///
    /// Default implementation returns Ignored. Widgets that implement MouseHandler
    /// should override this to call their handler methods.
    fn dispatch_mouse_event(&mut self, event: &mut crate::event::InputEventEnum) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }

    /// Dispatch keyboard event to this widget
    ///
    /// Default implementation returns Ignored. Widgets that implement KeyboardHandler
    /// should override this to call their handler methods.
    fn dispatch_key_event(&mut self, event: &mut crate::event::InputEventEnum) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }

    /// Dispatch wheel event to this widget
    ///
    /// Default implementation returns Ignored. Widgets that implement WheelHandler
    /// should override this to call their handler methods.
    fn dispatch_wheel_event(&mut self, event: &mut crate::event::WheelEvent) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }

    /// Dispatch IME event to this widget
    ///
    /// Default implementation returns Ignored. Widgets that implement ImeHandler
    /// should override this to call their handler methods.
    fn dispatch_ime_event(&mut self, event: &mut crate::event::ImeEvent) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }

    /// Dispatch custom event to this widget
    ///
    /// Default implementation returns Ignored. Widgets that implement CustomInputHandler
    /// should override this to call their handler methods.
    ///
    /// Custom events are used for hardware plugins (MIDI, gamepad, etc.) that post
    /// events via the EventBus.
    fn dispatch_custom_event(&mut self, event: &mut crate::event::CustomEvent) -> crate::event::EventResponse {
        let _ = event;
        crate::event::EventResponse::Ignored
    }
}
