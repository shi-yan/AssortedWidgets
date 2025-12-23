use super::{EventResponse, ImeEvent, KeyEvent, MouseEvent, WheelEvent};
use super::custom::CustomEvent;

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

/// Optional trait for widgets that handle IME (Input Method Editor) input
///
/// Widgets implement this trait to respond to IME events for complex text input
/// such as Chinese, Japanese, Korean, etc.
///
/// # IME Event Flow
/// 1. User starts typing with IME
/// 2. on_ime() is called with Preedit events (composition text)
/// 3. Widget displays preedit text with visual distinction (e.g., underline)
/// 4. on_ime() is called with Commit event (final text)
/// 5. Widget inserts committed text and clears preedit
///
/// # Example
/// ```ignore
/// impl ImeHandler for TextInput {
///     fn on_ime(&mut self, event: &mut ImeEvent) -> EventResponse {
///         match &event.event_type {
///             ImeEventType::Preedit(text) => {
///                 self.preedit_text = text.clone();
///                 EventResponse::Handled
///             }
///             ImeEventType::Commit(text) => {
///                 self.insert_text(text);
///                 self.preedit_text.clear();
///                 EventResponse::Handled
///             }
///             ImeEventType::Cancel => {
///                 self.preedit_text.clear();
///                 EventResponse::Handled
///             }
///         }
///     }
/// }
/// ```
pub trait ImeHandler {
    /// Handle IME event
    fn on_ime(&mut self, event: &mut ImeEvent) -> EventResponse {
        let _ = event;
        EventResponse::Ignored
    }
}

/// Optional trait for widgets that handle custom hardware input
///
/// Widgets implement this trait to respond to custom events from hardware plugins
/// (e.g., MIDI controllers, gamepads, 3D mice, foot pedals, etc.).
///
/// # How It Works
/// 1. Plugin posts CustomEvent to event bus with event_type (e.g., "midi")
/// 2. Event dispatcher checks each widget's handled_event_types()
/// 3. If widget handles this event type, on_custom_event() is called
/// 4. Widget downcasts data to concrete type and processes it
///
/// # Example: MIDI Controller
/// ```ignore
/// impl CustomInputHandler for SynthSlider {
///     fn handled_event_types(&self) -> &[&str] {
///         &["midi"]
///     }
///
///     fn on_custom_event(&mut self, event: &mut CustomEvent) -> EventResponse {
///         if event.event_type == "midi" {
///             if let Some(midi) = event.downcast_ref::<MidiEvent>() {
///                 // Map MIDI CC to slider value
///                 if midi.message_type == MidiMessageType::ControlChange {
///                     self.value = midi.velocity as f32 / 127.0;
///                     return EventResponse::Handled;
///                 }
///             }
///         }
///         EventResponse::Ignored
///     }
/// }
/// ```
///
/// # Example: Gamepad
/// ```ignore
/// impl CustomInputHandler for Player {
///     fn handled_event_types(&self) -> &[&str] {
///         &["gamepad"]
///     }
///
///     fn on_custom_event(&mut self, event: &mut CustomEvent) -> EventResponse {
///         if let Some(gamepad) = event.downcast_ref::<GamepadEvent>() {
///             if let Some(GamepadButton::A) = gamepad.button {
///                 self.jump();
///                 return EventResponse::Handled;
///             }
///         }
///         EventResponse::Ignored
///     }
/// }
/// ```
pub trait CustomInputHandler {
    /// Return event types this handler cares about (e.g., "midi", "gamepad")
    ///
    /// The event dispatcher uses this to filter events before calling on_custom_event().
    /// This avoids unnecessary downcasting for events the widget doesn't handle.
    fn handled_event_types(&self) -> &[&str];

    /// Handle a custom event
    ///
    /// Called when a custom event matching one of the handler's event types is received.
    /// The widget should:
    /// 1. Check event.event_type if it handles multiple types
    /// 2. Downcast event.data to the expected concrete type
    /// 3. Process the event data
    /// 4. Return Handled/PassThrough/Ignored as appropriate
    fn on_custom_event(&mut self, event: &mut CustomEvent) -> EventResponse {
        let _ = event;
        EventResponse::Ignored
    }
}
