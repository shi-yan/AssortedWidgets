//! Virtualized grid widget for efficient 2D table display
//!
//! Supports row and column virtualization for large datasets like CSV files or spreadsheets.

mod data_source;
mod widget;

pub use data_source::GridDataSource;
pub use widget::VirtualizedGrid;
