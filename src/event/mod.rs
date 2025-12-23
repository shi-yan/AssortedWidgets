use crate::types::{Point, Rect, WidgetId};

#[cfg(target_os = "macos")]
use crate::platform::PlatformInput;

// Sub-modules
pub mod bus;
pub mod capture;
pub mod custom;
pub mod focus;
pub mod handlers;
pub mod hit_test;
pub mod input;

// Re-exports
pub use bus::EventBus;
pub use capture::MouseCapture;
pub use custom::{CustomEvent, GamepadAxis, GamepadButton, GamepadEvent, MidiEvent, MidiMessageType};
pub use focus::FocusManager;
pub use handlers::{CustomInputHandler, ImeHandler, KeyboardHandler, MouseHandler, WheelHandler};
pub use hit_test::{HitTester, HitTestEntry};
pub use input::{
    EventResponse, ImeEvent, ImeEventType, InputEvent, InputEventEnum, Key, KeyEvent, Modifiers,
    MouseButton, MouseEvent, NamedKey, WheelEvent, WheelPhase,
};

// ============================================================================
// GUI Event Queue Events
// ============================================================================

/// Events that flow through the event queue
///
/// These events are posted by platform callbacks and processed by the main event loop.
/// This allows clean separation between platform layer (which posts events)
/// and application layer (which processes them).
#[derive(Debug)]
pub enum GuiEvent {
    /// Window needs to be redrawn
    RedrawRequested,

    /// Window was resized
    Resize(Rect),

    /// Platform input event (mouse, keyboard, etc.) - LEGACY
    /// This will be replaced by InputEvent variant
    #[cfg(target_os = "macos")]
    Input(PlatformInput),

    /// New input event system
    InputEvent(InputEventEnum),

    /// Window close requested
    Close,

    // ========================================
    // Cross-Window Drag-Drop Events
    // ========================================
    /// Request to start a cross-window drag operation
    /// Window sends this when a widget wants to be dragged across windows
    StartCrossWindowDrag {
        widget_id: WidgetId,
        color: crate::paint::Color,
        label: String,
        size: crate::types::Size,
        drag_offset: Point,
        screen_position: Point,
    },

    /// Update drag position during cross-window drag
    UpdateCrossWindowDrag {
        screen_position: Point,
    },

    /// End cross-window drag operation
    EndCrossWindowDrag {
        screen_position: Point,
    },
}

// ============================================================================
// OS Events (Legacy - may be refactored)
// ============================================================================

/// Represents OS-level events (mouse, keyboard, etc.)
#[derive(Debug, Clone)]
pub enum OsEvent {
    MouseDown { x: f64, y: f64 },
    MouseUp { x: f64, y: f64 },
    MouseMove { x: f64, y: f64 },
    KeyDown { key: String },
    KeyUp { key: String },
}

impl OsEvent {
    /// Get the target widget for this event (simplified)
    pub(crate) fn target_widget(&self) -> Option<WidgetId> {
        // This would actually be resolved through hit testing
        // using the SceneGraph
        None
    }
}
