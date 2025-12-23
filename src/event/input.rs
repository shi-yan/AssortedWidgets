use crate::types::Point;
use std::any::Any;
use std::time::Instant;

// ============================================================================
// Input Event Enum
// ============================================================================

/// Wrapper enum for all input event types
///
/// This allows passing different event types through a uniform interface
/// while preserving type information for downcasting.
#[derive(Debug)]
pub enum InputEventEnum {
    /// Mouse button press
    MouseDown(MouseEvent),

    /// Mouse button release
    MouseUp(MouseEvent),

    /// Mouse movement
    MouseMove(MouseEvent),

    /// Keyboard key press
    KeyDown(KeyEvent),

    /// Keyboard key release
    KeyUp(KeyEvent),

    /// Mouse wheel / trackpad scroll
    Scroll(ScrollEvent),
}

impl InputEventEnum {
    /// Get a reference to the underlying InputEvent trait object
    pub fn as_input_event(&self) -> &dyn InputEvent {
        match self {
            InputEventEnum::MouseDown(e) => e,
            InputEventEnum::MouseUp(e) => e,
            InputEventEnum::MouseMove(e) => e,
            InputEventEnum::KeyDown(e) => e,
            InputEventEnum::KeyUp(e) => e,
            InputEventEnum::Scroll(e) => e,
        }
    }

    /// Get a mutable reference to the underlying InputEvent trait object
    pub fn as_input_event_mut(&mut self) -> &mut dyn InputEvent {
        match self {
            InputEventEnum::MouseDown(e) => e,
            InputEventEnum::MouseUp(e) => e,
            InputEventEnum::MouseMove(e) => e,
            InputEventEnum::KeyDown(e) => e,
            InputEventEnum::KeyUp(e) => e,
            InputEventEnum::Scroll(e) => e,
        }
    }
}

// ============================================================================
// Input Event Trait
// ============================================================================

/// Base trait for all input events
///
/// This provides common event state (propagation, default behavior) that all
/// input events share. Use downcasting to get the concrete event type.
pub trait InputEvent: Any + Send {
    /// Check if event propagation should continue
    fn should_propagate(&self) -> bool;

    /// Stop event from bubbling up the element tree
    fn stop_propagation(&mut self);

    /// Check if default behavior should be prevented
    fn is_default_prevented(&self) -> bool;

    /// Prevent the default behavior (e.g., OS handling)
    fn prevent_default(&mut self);

    /// Mark event as handled (stops propagation and prevents default)
    fn consume(&mut self) {
        self.stop_propagation();
        self.prevent_default();
    }

    /// Get event timestamp
    fn timestamp(&self) -> Instant;

    /// Downcast to concrete type
    fn as_any(&self) -> &dyn Any;

    /// Downcast to mutable concrete type
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// ============================================================================
// Event Response
// ============================================================================

/// Response from event handlers
///
/// Returned by event handler methods to indicate how the event was processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResponse {
    /// Event was handled, stop propagation
    Handled,

    /// Event was handled but continue propagation
    PassThrough,

    /// Event was not handled
    Ignored,
}

// ============================================================================
// Mouse Events
// ============================================================================

/// Mouse button identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// Keyboard modifiers state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub command: bool, // Meta/Super/Command key
}

/// Mouse button press/release event
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// Mouse position in window coordinates
    pub position: Point,

    /// Which button was pressed/released
    pub button: MouseButton,

    /// Keyboard modifiers held during the event
    pub modifiers: Modifiers,

    /// Click count (1 = single, 2 = double, 3 = triple)
    pub click_count: u8,

    /// Event timestamp
    pub timestamp: Instant,

    // Event state
    propagate: bool,
    default_prevented: bool,
}

impl MouseEvent {
    /// Create a new mouse event
    pub fn new(position: Point, button: MouseButton, modifiers: Modifiers) -> Self {
        Self {
            position,
            button,
            modifiers,
            click_count: 1,
            timestamp: Instant::now(),
            propagate: true,
            default_prevented: false,
        }
    }

    /// Create with click count (for double/triple click detection)
    pub fn with_click_count(mut self, count: u8) -> Self {
        self.click_count = count;
        self
    }
}

impl InputEvent for MouseEvent {
    fn should_propagate(&self) -> bool {
        self.propagate
    }

    fn stop_propagation(&mut self) {
        self.propagate = false;
    }

    fn is_default_prevented(&self) -> bool {
        self.default_prevented
    }

    fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    fn timestamp(&self) -> Instant {
        self.timestamp
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// ============================================================================
// Keyboard Events
// ============================================================================

/// Keyboard key identifier (simplified for now)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    /// Character key (a, b, 1, 2, etc.)
    Character(char),

    /// Named key (Enter, Tab, Escape, etc.)
    Named(NamedKey),
}

/// Named keyboard keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedKey {
    Enter,
    Tab,
    Escape,
    Backspace,
    Delete,
    Space,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

/// Keyboard key press/release event
#[derive(Debug, Clone)]
pub struct KeyEvent {
    /// Which key was pressed/released
    pub key: Key,

    /// Keyboard modifiers held during the event
    pub modifiers: Modifiers,

    /// True if this is a repeat event (key held down)
    pub is_repeat: bool,

    /// Event timestamp
    pub timestamp: Instant,

    // Event state
    propagate: bool,
    default_prevented: bool,
}

impl KeyEvent {
    /// Create a new keyboard event
    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self {
            key,
            modifiers,
            is_repeat: false,
            timestamp: Instant::now(),
            propagate: true,
            default_prevented: false,
        }
    }

    /// Mark as a repeat event
    pub fn with_repeat(mut self, is_repeat: bool) -> Self {
        self.is_repeat = is_repeat;
        self
    }
}

impl InputEvent for KeyEvent {
    fn should_propagate(&self) -> bool {
        self.propagate
    }

    fn stop_propagation(&mut self) {
        self.propagate = false;
    }

    fn is_default_prevented(&self) -> bool {
        self.default_prevented
    }

    fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    fn timestamp(&self) -> Instant {
        self.timestamp
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// ============================================================================
// Scroll Events
// ============================================================================

/// Scroll phase (for trackpad/touchpad momentum)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollPhase {
    /// User started scrolling
    Begin,

    /// Scrolling continues
    Update,

    /// User stopped (finger lifted)
    End,

    /// Inertial scrolling after release
    Momentum,
}

/// Mouse wheel / trackpad scroll event
#[derive(Debug, Clone)]
pub struct ScrollEvent {
    /// Scroll delta (positive = scroll down/right)
    pub delta: crate::types::Vector,

    /// Scroll phase (for momentum tracking)
    pub phase: ScrollPhase,

    /// Keyboard modifiers held during the event
    pub modifiers: Modifiers,

    /// Event timestamp
    pub timestamp: Instant,

    // Event state
    propagate: bool,
    default_prevented: bool,
}

impl ScrollEvent {
    /// Create a new scroll event
    pub fn new(delta: crate::types::Vector, modifiers: Modifiers) -> Self {
        Self {
            delta,
            phase: ScrollPhase::Update,
            modifiers,
            timestamp: Instant::now(),
            propagate: true,
            default_prevented: false,
        }
    }

    /// Set the scroll phase
    pub fn with_phase(mut self, phase: ScrollPhase) -> Self {
        self.phase = phase;
        self
    }
}

impl InputEvent for ScrollEvent {
    fn should_propagate(&self) -> bool {
        self.propagate
    }

    fn stop_propagation(&mut self) {
        self.propagate = false;
    }

    fn is_default_prevented(&self) -> bool {
        self.default_prevented
    }

    fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    fn timestamp(&self) -> Instant {
        self.timestamp
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
