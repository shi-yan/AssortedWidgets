//! Mouse capture system for drag operations
//!
//! The MouseCapture system allows a widget to "capture" mouse input so that
//! it continues to receive mouse events even when the cursor moves outside
//! its bounds or even outside the window.

use crate::types::WidgetId;

/// Manages mouse capture for drag operations
///
/// # Capture Model
/// - Only one widget can capture the mouse at a time
/// - When captured, ALL mouse events go to the capturing widget
/// - Capture is typically started on mouse down
/// - Capture is typically released on mouse up
/// - Capture should be released if the widget is removed
///
/// # Use Cases
/// - Dragging scrollbar thumbs
/// - Dragging sliders
/// - Drag-and-drop operations
/// - Resizing panels
/// - Detachable tabs/windows
///
/// # Platform Behavior
/// On macOS, mouse events outside the window are still delivered to the
/// capturing widget during a drag operation. This matches native behavior
/// (e.g., dragging a scrollbar outside the window still scrolls).
pub struct MouseCapture {
    /// Widget that has captured mouse input
    captured_id: Option<WidgetId>,
}

impl MouseCapture {
    /// Create a new mouse capture manager with no captured widget
    pub fn new() -> Self {
        MouseCapture { captured_id: None }
    }

    /// Get the widget that has captured mouse input
    pub fn captured_id(&self) -> Option<WidgetId> {
        self.captured_id
    }

    /// Capture mouse input to a specific widget
    ///
    /// All subsequent mouse events will be routed to this widget until
    /// release() is called, even if the mouse moves outside the widget
    /// or window bounds.
    ///
    /// # Arguments
    /// * `widget_id` - Widget to receive all mouse events
    ///
    /// # Example
    /// ```ignore
    /// impl MouseHandler for Scrollbar {
    ///     fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
    ///         // Start dragging the thumb
    ///         mouse_capture.capture(self.id);
    ///         EventResponse::Handled
    ///     }
    /// }
    /// ```
    pub fn capture(&mut self, widget_id: WidgetId) {
        if self.captured_id.is_some() && self.captured_id != Some(widget_id) {
            println!(
                "[MouseCapture] Warning: {:?} is capturing while {:?} already has capture",
                widget_id, self.captured_id
            );
        }
        self.captured_id = Some(widget_id);
    }

    /// Release mouse capture
    ///
    /// Normal hit testing will resume for mouse events.
    ///
    /// # Example
    /// ```ignore
    /// impl MouseHandler for Scrollbar {
    ///     fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
    ///         // Stop dragging
    ///         mouse_capture.release();
    ///         EventResponse::Handled
    ///     }
    /// }
    /// ```
    pub fn release(&mut self) {
        self.captured_id = None;
    }

    /// Check if a widget has captured mouse input
    pub fn is_captured_by(&self, widget_id: WidgetId) -> bool {
        self.captured_id == Some(widget_id)
    }

    /// Check if any widget has captured mouse input
    pub fn is_captured(&self) -> bool {
        self.captured_id.is_some()
    }
}

impl Default for MouseCapture {
    fn default() -> Self {
        Self::new()
    }
}
