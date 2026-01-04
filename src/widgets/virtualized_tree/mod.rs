//! Virtualized tree widget for efficient hierarchical table display
//!
//! Supports tree structures with expand/collapse, optional checkboxes, and multi-column display.

mod data_source;
mod widget;

pub use data_source::{TreeDataSource, TreeNodeId};
pub use widget::VirtualizedTree;
