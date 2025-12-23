//! Custom event system for hardware extensibility
//!
//! This module provides infrastructure for plugins to add custom hardware support
//! without modifying the framework code. Examples: MIDI controllers, game pads,
//! 3D mice, foot pedals, etc.

use std::any::Any;
use std::sync::Arc;
use std::time::Instant;

// ============================================================================
// Custom Event
// ============================================================================

/// Generic container for custom hardware events
///
/// Plugins can post these events to the event bus, and widgets can opt-in to
/// handling specific event types via `CustomInputHandler`.
///
/// # Example
///
/// ```rust
/// // MIDI plugin posts events
/// let midi_event = CustomEvent {
///     event_type: "midi".to_string(),
///     data: Arc::new(MidiEvent {
///         channel: 1,
///         note: 60,
///         velocity: 100,
///     }),
///     timestamp: Instant::now(),
///     propagate: true,
///     default_prevented: false,
/// };
/// event_bus.post(midi_event);
///
/// // Widget handles MIDI events
/// impl CustomInputHandler for SynthSlider {
///     fn handled_event_types(&self) -> &[&str] {
///         &["midi"]
///     }
///
///     fn on_custom_event(&mut self, event: &mut CustomEvent) -> EventResponse {
///         if let Some(midi) = event.data.downcast_ref::<MidiEvent>() {
///             // Map MIDI CC to slider value
///             self.value = midi.velocity as f32 / 127.0;
///             return EventResponse::Handled;
///         }
///         EventResponse::Ignored
///     }
/// }
/// ```
#[derive(Clone)]
pub struct CustomEvent {
    /// Type identifier (e.g., "midi", "gamepad", "footpedal")
    ///
    /// Widgets use this to filter events they care about.
    pub event_type: String,

    /// Event data (plugin-defined)
    ///
    /// Plugins define their own event structs and wrap them in Arc<dyn Any>.
    /// Widgets downcast to the concrete type to extract data.
    pub data: Arc<dyn Any + Send + Sync>,

    /// Event timestamp
    pub timestamp: Instant,

    /// Should this event continue propagating up the widget tree?
    propagate: bool,

    /// Has the default behavior been prevented?
    default_prevented: bool,
}

impl CustomEvent {
    /// Create a new custom event
    pub fn new(event_type: impl Into<String>, data: Arc<dyn Any + Send + Sync>) -> Self {
        Self {
            event_type: event_type.into(),
            data,
            timestamp: Instant::now(),
            propagate: true,
            default_prevented: false,
        }
    }

    /// Create a custom event with a specific timestamp
    pub fn with_timestamp(
        event_type: impl Into<String>,
        data: Arc<dyn Any + Send + Sync>,
        timestamp: Instant,
    ) -> Self {
        Self {
            event_type: event_type.into(),
            data,
            timestamp,
            propagate: true,
            default_prevented: false,
        }
    }

    /// Stop event from bubbling up the widget tree
    pub fn stop_propagation(&mut self) {
        self.propagate = false;
    }

    /// Check if event should continue propagating
    pub fn should_propagate(&self) -> bool {
        self.propagate
    }

    /// Prevent the default behavior for this event
    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    /// Check if default behavior has been prevented
    pub fn is_default_prevented(&self) -> bool {
        self.default_prevented
    }

    /// Mark event as fully handled (stop propagation + prevent default)
    pub fn consume(&mut self) {
        self.stop_propagation();
        self.prevent_default();
    }

    /// Downcast data to concrete type
    ///
    /// # Example
    ///
    /// ```rust
    /// if let Some(midi) = event.downcast_ref::<MidiEvent>() {
    ///     println!("MIDI note: {}", midi.note);
    /// }
    /// ```
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
}

impl std::fmt::Debug for CustomEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomEvent")
            .field("event_type", &self.event_type)
            .field("timestamp", &self.timestamp)
            .field("propagate", &self.propagate)
            .field("default_prevented", &self.default_prevented)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// Example Event Types (for reference/testing)
// ============================================================================

/// Example: MIDI event data
///
/// A MIDI plugin would define this struct and wrap it in CustomEvent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MidiEvent {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
    pub message_type: MidiMessageType,
}

/// MIDI message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiMessageType {
    NoteOn,
    NoteOff,
    ControlChange,
    PitchBend,
    Aftertouch,
    ProgramChange,
}

/// Example: Gamepad event data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamepadEvent {
    pub device_id: u32,
    pub button: Option<GamepadButton>,
    pub axis: Option<(GamepadAxis, f32)>, // axis + value (-1.0 to 1.0)
}

/// Gamepad buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadButton {
    A,
    B,
    X,
    Y,
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,
    Start,
    Select,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

/// Gamepad axes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}
