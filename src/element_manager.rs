use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::connection::ConnectionTable;
use crate::element::Element;
use crate::event::OsEvent;
use crate::handle::GuiHandle;
use crate::types::{DeferredCommand, GuiMessage, WidgetId};

// ============================================================================
// Element Manager
// ============================================================================

/// Manages all UI elements in a flat hash table
pub struct ElementManager {
    /// Flat storage of all elements
    elements: HashMap<WidgetId, Box<dyn Element>>,
    /// ID generator
    next_id: u64,
    /// Pending messages to be processed
    pending_messages: Vec<DeferredCommand>,
    /// Signal/slot connections
    connections: ConnectionTable,
    /// Channel for receiving messages from other threads
    message_rx: Receiver<GuiMessage>,
    /// Sender half (cloned for GuiHandle)
    message_tx: Sender<GuiMessage>,
}

impl ElementManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        ElementManager {
            elements: HashMap::new(),
            next_id: 1,
            pending_messages: Vec::new(),
            connections: ConnectionTable::new(),
            message_rx: rx,
            message_tx: tx,
        }
    }

    /// Generate a new unique WidgetId
    pub fn next_id(&mut self) -> WidgetId {
        let id = WidgetId::new(self.next_id);
        self.next_id += 1;
        id
    }

    /// Add an element to the manager
    pub fn add_element(&mut self, element: Box<dyn Element>) -> WidgetId {
        let id = element.id();
        self.elements.insert(id, element);
        id
    }

    /// Remove an element
    pub fn remove_element(&mut self, id: WidgetId) {
        self.elements.remove(&id);
        self.connections.remove_widget(id);
    }

    /// Get immutable reference to an element
    pub fn get_element(&self, id: WidgetId) -> Option<&dyn Element> {
        self.elements.get(&id).map(|e| e.as_ref())
    }

    /// Alter a child element using a closure (the core mutation pattern)
    pub fn alter_element<F>(&mut self, id: WidgetId, f: F) -> Vec<DeferredCommand>
    where
        F: FnOnce(&mut dyn Element) -> Vec<DeferredCommand>,
    {
        if let Some(element) = self.elements.get_mut(&id) {
            f(element.as_mut())
        } else {
            Vec::new()
        }
    }

    /// Emit a signal from a widget
    pub fn emit_signal(&mut self, source: WidgetId, signal_type: String, message: GuiMessage) {
        // Find all connected slots
        let targets = self.connections.get_targets(source, &signal_type);

        // Queue messages for each target
        for target in targets {
            self.pending_messages.push(DeferredCommand {
                target,
                message: message.clone_for_target(target),
            });
        }
    }

    /// Connect a signal to a slot
    pub fn connect(&mut self, source: WidgetId, signal_type: String, target: WidgetId) {
        self.connections.connect(source, signal_type, target);
    }

    /// Process all pending messages
    pub fn process_messages(&mut self) {
        // Drain pending messages
        let messages = std::mem::take(&mut self.pending_messages);

        for cmd in messages {
            // Get the target element and deliver the message
            let new_commands = self.alter_element(cmd.target, |element| {
                element.on_message(&cmd.message)
            });

            // Queue any new commands generated
            self.pending_messages.extend(new_commands);
        }

        // Process messages from other threads
        while let Ok(msg) = self.message_rx.try_recv() {
            self.handle_external_message(msg);
        }
    }

    /// Handle a message received from another thread
    fn handle_external_message(&mut self, message: GuiMessage) {
        match &message {
            GuiMessage::Clicked(id)
            | GuiMessage::ValueChanged(id, _)
            | GuiMessage::TextChanged(id, _) => {
                self.pending_messages.push(DeferredCommand {
                    target: *id,
                    message,
                });
            }
            GuiMessage::Custom { source, .. } => {
                self.pending_messages.push(DeferredCommand {
                    target: *source,
                    message,
                });
            }
            GuiMessage::ParentToChild { child, .. } => {
                self.pending_messages.push(DeferredCommand {
                    target: *child,
                    message,
                });
            }
            GuiMessage::ChildToParent { parent, .. } => {
                self.pending_messages.push(DeferredCommand {
                    target: *parent,
                    message,
                });
            }
        }
    }

    /// Get a handle for cross-thread communication
    pub fn get_handle(&self) -> GuiHandle {
        GuiHandle::new(self.message_tx.clone())
    }

    /// Handle an OS event by finding the target widget and dispatching
    pub fn handle_os_event(&mut self, event: OsEvent) {
        // For mouse events, we need hit testing (would integrate with SceneGraph)
        // For now, simplified version
        if let Some(target) = event.target_widget() {
            let commands = self.alter_element(target, |element| element.on_event(&event));
            self.pending_messages.extend(commands);
        }
    }

    /// Get element by ID (immutable) - simpler API without lifetime issues
    pub fn get(&self, id: WidgetId) -> Option<&dyn Element> {
        self.elements.get(&id).map(|e| &**e)
    }

    /// Get element by ID (mutable)
    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut (dyn Element + '_)> {
        match self.elements.get_mut(&id) {
            Some(e) => Some(&mut **e),
            None => None,
        }
    }

    /// Iterate over all widget IDs (for when you need to visit all elements)
    pub fn widget_ids(&self) -> impl Iterator<Item = WidgetId> + '_ {
        self.elements.keys().copied()
    }

    /// Add an element with a specific ID (for layout system)
    pub fn add_element_with_id(&mut self, id: WidgetId, element: Box<dyn Element>) {
        self.elements.insert(id, element);
    }
}
