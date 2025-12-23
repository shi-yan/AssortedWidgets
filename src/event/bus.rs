//! Event bus for thread-safe event posting
//!
//! This module provides infrastructure for posting events from background threads
//! (e.g., hardware plugins) to the main GUI thread.

use super::InputEventEnum;
use crate::types::WindowId;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// ============================================================================
// Event Bus
// ============================================================================

/// Thread-safe event bus for posting events from background threads
///
/// Hardware plugins and other background tasks can use this to safely post
/// events to the main GUI thread.
///
/// # Example
///
/// ```rust
/// // Create event bus (usually done in Application)
/// let event_bus = EventBus::new();
///
/// // Clone for background thread
/// let bus_clone = event_bus.clone();
/// std::thread::spawn(move || {
///     // MIDI plugin running on background thread
///     loop {
///         let midi_data = read_midi_device();
///         let event = InputEventEnum::Custom(CustomEvent::new(
///             "midi",
///             Arc::new(midi_data),
///         ));
///         bus_clone.post(WindowId::new(1), event);
///     }
/// });
///
/// // Main thread drains events each frame
/// for (window_id, event) in event_bus.drain() {
///     dispatch_event(window_id, event);
/// }
/// ```
#[derive(Clone)]
pub struct EventBus {
    /// Event queue (window_id, event)
    queue: Arc<Mutex<VecDeque<(WindowId, InputEventEnum)>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Post an event to the bus (thread-safe)
    ///
    /// This can be called from any thread. Events are queued and will be
    /// processed on the main thread during the next event loop iteration.
    ///
    /// # Arguments
    /// * `window_id` - Target window for the event
    /// * `event` - Input event to post
    pub fn post(&self, window_id: WindowId, event: InputEventEnum) {
        self.queue.lock().unwrap().push_back((window_id, event));
    }

    /// Post multiple events at once (thread-safe)
    pub fn post_batch(&self, events: Vec<(WindowId, InputEventEnum)>) {
        let mut queue = self.queue.lock().unwrap();
        queue.extend(events);
    }

    /// Drain all pending events (main thread only)
    ///
    /// Returns an iterator over all queued events. This should be called
    /// once per frame from the main event loop.
    pub fn drain(&self) -> Vec<(WindowId, InputEventEnum)> {
        let mut queue = self.queue.lock().unwrap();
        queue.drain(..).collect()
    }

    /// Check how many events are pending
    pub fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    /// Check if the event bus is empty
    pub fn is_empty(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }

    /// Clear all pending events
    pub fn clear(&self) {
        self.queue.lock().unwrap().clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("pending_events", &self.len())
            .finish()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{CustomEvent, MouseButton, MouseEvent};
    use crate::types::Point;
    use std::sync::Arc;

    #[test]
    fn test_event_bus_basic() {
        let bus = EventBus::new();
        let window_id = WindowId::new(1);

        // Post event
        let event = InputEventEnum::MouseDown(MouseEvent::new(
            Point::new(100.0, 200.0),
            MouseButton::Left,
            Default::default(),
        ));
        bus.post(window_id, event);

        // Drain events
        let events = bus.drain();
        assert_eq!(events.len(), 1);

        // Queue should be empty now
        assert!(bus.is_empty());
    }

    #[test]
    fn test_event_bus_multithreaded() {
        let bus = EventBus::new();
        let window_id = WindowId::new(1);

        // Spawn multiple threads posting events
        let mut handles = vec![];
        for i in 0..10 {
            let bus_clone = bus.clone();
            let handle = std::thread::spawn(move || {
                for j in 0..100 {
                    let event = InputEventEnum::Custom(CustomEvent::new(
                        "test",
                        Arc::new(i * 100 + j),
                    ));
                    bus_clone.post(window_id, event);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Should have 1000 events
        assert_eq!(bus.len(), 1000);

        // Drain all
        let events = bus.drain();
        assert_eq!(events.len(), 1000);
        assert!(bus.is_empty());
    }

    #[test]
    fn test_event_bus_batch() {
        let bus = EventBus::new();
        let window_id = WindowId::new(1);

        let events = vec![
            (window_id, InputEventEnum::Custom(CustomEvent::new("test1", Arc::new(1)))),
            (window_id, InputEventEnum::Custom(CustomEvent::new("test2", Arc::new(2)))),
            (window_id, InputEventEnum::Custom(CustomEvent::new("test3", Arc::new(3)))),
        ];

        bus.post_batch(events);
        assert_eq!(bus.len(), 3);

        let drained = bus.drain();
        assert_eq!(drained.len(), 3);
    }
}
