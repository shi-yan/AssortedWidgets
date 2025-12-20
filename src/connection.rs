use std::collections::HashMap;

use crate::types::WidgetId;

// ============================================================================
// Signal/Slot Connection System
// ============================================================================

/// Represents a connection from a signal to a slot
#[derive(Clone)]
pub struct Connection {
    pub source: WidgetId,
    pub signal_type: String,
    pub target: WidgetId,
}

/// Manages signal/slot connections
pub struct ConnectionTable {
    /// Map from source widget to list of connections
    connections: HashMap<WidgetId, Vec<Connection>>,
}

impl ConnectionTable {
    pub fn new() -> Self {
        ConnectionTable {
            connections: HashMap::new(),
        }
    }

    /// Connect a signal from source to a slot on target
    pub fn connect(&mut self, source: WidgetId, signal_type: String, target: WidgetId) {
        let connection = Connection {
            source,
            signal_type,
            target,
        };
        self.connections
            .entry(source)
            .or_insert_with(Vec::new)
            .push(connection);
    }

    /// Disconnect a specific connection
    pub fn disconnect(&mut self, source: WidgetId, signal_type: &str, target: WidgetId) {
        if let Some(connections) = self.connections.get_mut(&source) {
            connections.retain(|c| !(c.signal_type == signal_type && c.target == target));
        }
    }

    /// Get all targets for a given signal
    pub fn get_targets(&self, source: WidgetId, signal_type: &str) -> Vec<WidgetId> {
        self.connections
            .get(&source)
            .map(|connections| {
                connections
                    .iter()
                    .filter(|c| c.signal_type == signal_type)
                    .map(|c| c.target)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove all connections for a widget (when it's destroyed)
    pub fn remove_widget(&mut self, id: WidgetId) {
        self.connections.remove(&id);
        // Also remove this widget as a target from all connections
        for connections in self.connections.values_mut() {
            connections.retain(|c| c.target != id);
        }
    }
}
