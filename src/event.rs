use crate::types::WidgetId;

// ============================================================================
// OS Events
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
