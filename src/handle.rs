use std::any::Any;
use std::sync::mpsc::Sender;

use crate::types::{GuiMessage, WidgetId};

// ============================================================================
// Thread-Safe Handle
// ============================================================================

/// Thread-safe handle for sending messages to the GUI from other threads
#[derive(Clone)]
pub struct GuiHandle {
    tx: Sender<GuiMessage>,
}

impl GuiHandle {
    pub(crate) fn new(tx: Sender<GuiMessage>) -> Self {
        GuiHandle { tx }
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
