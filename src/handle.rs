use std::any::Any;
use std::sync::mpsc::Sender;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::types::{GuiMessage, WidgetId};

// ============================================================================
// Thread-Safe Handle
// ============================================================================

/// Thread-safe handle for sending messages to the GUI from other threads
///
/// Also provides app-level widget ID generation to ensure globally unique IDs
/// across all windows in a multi-window application.
#[derive(Clone)]
pub struct GuiHandle {
    tx: Sender<GuiMessage>,
    /// App-level widget ID counter (shared across all windows)
    /// Ensures globally unique widget IDs in multi-window applications
    next_widget_id: Arc<AtomicU64>,
}

impl GuiHandle {
    pub(crate) fn new(tx: Sender<GuiMessage>, next_widget_id: Arc<AtomicU64>) -> Self {
        GuiHandle { tx, next_widget_id }
    }

    /// Generate a new globally unique WidgetId (app-level)
    ///
    /// This method is thread-safe and can be called from any window.
    /// IDs are guaranteed to be unique across all windows in the application.
    pub fn next_widget_id(&self) -> WidgetId {
        let id = self.next_widget_id.fetch_add(1, Ordering::Relaxed);
        WidgetId::new(id)
    }

    /// Send a message to the GUI thread
    /// This will wake up the event loop
    pub fn send_message(&self, message: GuiMessage) {
        let _ = self.tx.send(message);
        // TODO: Integrate with OS-specific wakeup mechanism
        // On macOS: CFRunLoopWakeUp or dispatch_async
        // On Linux: write to eventfd
        self.wakeup_event_loop();
    }

    /// Emit a signal from another thread
    pub fn emit(&self, source: WidgetId, signal_type: String, data: Box<dyn Any + Send>) {
        self.send_message(GuiMessage::Custom {
            source,
            signal_type,
            data,
        });
    }

    /// Platform-specific event loop wakeup
    #[cfg(target_os = "macos")]
    fn wakeup_event_loop(&self) {
        // TODO: Call CFRunLoopWakeUp or use NSApp.postEvent
        // For now, this is a placeholder
    }

    #[cfg(target_os = "linux")]
    fn wakeup_event_loop(&self) {
        // TODO: Write to eventfd registered with epoll
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    fn wakeup_event_loop(&self) {
        // Placeholder for other platforms
    }
}
