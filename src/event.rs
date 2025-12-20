use crate::types::{Rect, WidgetId};

#[cfg(target_os = "macos")]
use crate::platform::PlatformInput;

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

    /// Platform input event (mouse, keyboard, etc.)
    #[cfg(target_os = "macos")]
    Input(PlatformInput),

    /// Window close requested
    Close,
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
