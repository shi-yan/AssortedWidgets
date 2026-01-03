//! Virtualized list widget with efficient widget recycling
//!
//! Only creates widgets for visible items, making it suitable for large datasets.

mod data_source;
mod widget;

pub use data_source::ListDataSource;
pub use widget::{ListLayoutMode, VirtualizedList};
