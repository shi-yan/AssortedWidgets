use super::{EventResponse, KeyEvent, MouseEvent, WheelEvent};

// ============================================================================
// Event Handler Traits
// ============================================================================

/// Optional trait for widgets that handle mouse input
///
/// Widgets implement this trait to respond to mouse events.
/// The element system will check if an element implements this trait
/// via downcasting before dispatching mouse events.
pub trait MouseHandler {
    /// Handle mouse button press
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        let _ = event;
        EventResponse::Ignored
    }

    /// Handle mouse button release
    fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
        let _ = event;
        EventResponse::Ignored
    }

    /// Handle mouse movement
    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        let _ = event;
        EventResponse::Ignored
    }

    /// Called when mouse enters the element's bounds
    fn on_mouse_enter(&mut self) -> EventResponse {
        EventResponse::Ignored
    }

    /// Called when mouse leaves the element's bounds
    fn on_mouse_leave(&mut self) -> EventResponse {
        EventResponse::Ignored
    }
}

/// Optional trait for widgets that handle keyboard input
///
/// Widgets implement this trait to respond to keyboard events.
/// Only the focused widget receives keyboard events.
pub trait KeyboardHandler {
    /// Handle keyboard key press
    fn on_key_down(&mut self, event: &mut KeyEvent) -> EventResponse {
        let _ = event;
        EventResponse::Ignored
    }

    /// Handle keyboard key release
    fn on_key_up(&mut self, event: &mut KeyEvent) -> EventResponse {
        let _ = event;
        EventResponse::Ignored
    }
}

/// Optional trait for widgets that handle wheel input
///
/// Widgets implement this trait to respond to mouse wheel/trackpad events.
/// Note: This is called "wheel" (not "scroll") because the wheel can be used
/// for purposes other than scrolling (e.g., zooming in 3D applications).
pub trait WheelHandler {
    /// Handle mouse wheel / trackpad event
    fn on_wheel(&mut self, event: &mut WheelEvent) -> EventResponse {
        let _ = event;
        EventResponse::Ignored
    }
}
