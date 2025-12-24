use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::connection::ConnectionTable;
use crate::widget::Widget;
use crate::event::OsEvent;
use crate::handle::GuiHandle;
use crate::types::{DeferredCommand, GuiMessage, WidgetId};

// ============================================================================
// Widget Manager
// ============================================================================

/// Manages all UI widgets in a flat hash table
pub struct WidgetManager {
    /// Flat storage of all widgets
    widgets: HashMap<WidgetId, Box<dyn Widget>>,
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

impl WidgetManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        WidgetManager {
            widgets: HashMap::new(),
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

    /// Add a widget to the manager
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) -> WidgetId {
        let id = widget.id();
        self.widgets.insert(id, widget);
        id
    }

    /// Remove a widget
    pub fn remove_widget(&mut self, id: WidgetId) {
        self.widgets.remove(&id);
        self.connections.remove_widget(id);
    }

    /// Get immutable reference to a widget
    pub fn get_widget(&self, id: WidgetId) -> Option<&dyn Widget> {
        self.widgets.get(&id).map(|w: &Box<dyn Widget>| w.as_ref())
    }

    /// Alter a widget using a closure (the core mutation pattern)
    pub fn alter_widget<F>(&mut self, id: WidgetId, f: F) -> Vec<DeferredCommand>
    where
        F: FnOnce(&mut dyn Widget) -> Vec<DeferredCommand>,
    {
        if let Some(widget) = self.widgets.get_mut(&id) {
            f(widget.as_mut())
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
            // Get the target widget and deliver the message
            let new_commands = self.alter_widget(cmd.target, |widget| {
                widget.on_message(&cmd.message)
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
        // For mouse events, we need hit testing (would integrate with WidgetTree)
        // For now, simplified version
        if let Some(target) = event.target_widget() {
            let commands = self.alter_widget(target, |widget| widget.on_event(&event));
            self.pending_messages.extend(commands);
        }
    }

    /// Get widget by ID (immutable) - simpler API without lifetime issues
    pub fn get(&self, id: WidgetId) -> Option<&dyn Widget> {
        self.widgets.get(&id).map(|w| &**w)
    }

    /// Get widget by ID (mutable)
    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut (dyn Widget + '_)> {
        match self.widgets.get_mut(&id) {
            Some(w) => Some(&mut **w),
            None => None,
        }
    }

    /// Iterate over all widget IDs (for when you need to visit all widgets)
    pub fn widget_ids(&self) -> impl Iterator<Item = WidgetId> + '_ {
        self.widgets.keys().copied()
    }

    /// Add a widget with a specific ID (for layout system)
    pub fn add_widget_with_id(&mut self, id: WidgetId, widget: Box<dyn Widget>) {
        self.widgets.insert(id, widget);
    }
}
