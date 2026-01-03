//! DataSource trait for virtualized grids (2D tables)
//!
//! Inspired by spreadsheet and table view patterns, this trait allows
//! the grid widget to query for tabular data efficiently.

use crate::widget::Widget;

/// Data source trait for virtualized grids (2D tables)
///
/// This trait provides a flexible interface for displaying large tabular data
/// with efficient virtualization of both rows and columns.
///
/// # Example
///
/// ```rust,ignore
/// struct CsvDataSource {
///     rows: Vec<Vec<String>>,
///     headers: Vec<String>,
/// }
///
/// impl GridDataSource for CsvDataSource {
///     fn row_count(&self) -> usize {
///         self.rows.len()
///     }
///
///     fn column_count(&self) -> usize {
///         self.headers.len()
///     }
///
///     fn column_header(&self, col: usize) -> String {
///         self.headers[col].clone()
///     }
///
///     fn column_width(&self, _col: usize) -> f64 {
///         120.0  // Fixed width
///     }
///
///     fn row_height(&self, _row: usize) -> Option<f64> {
///         Some(32.0)  // Fixed height
///     }
///
///     fn cell_widget(
///         &mut self,
///         row: usize,
///         col: usize,
///         reused_widget: Option<Box<dyn Widget>>,
///     ) -> Box<dyn Widget> {
///         let text = &self.rows[row][col];
///
///         // Reuse widget if possible
///         if let Some(mut widget) = reused_widget {
///             if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
///                 label.set_text(text);
///                 return widget;
///             }
///         }
///
///         Box::new(Label::new(text))
///     }
/// }
/// ```
pub trait GridDataSource {
    /// Return the total number of rows (excluding header)
    fn row_count(&self) -> usize;

    /// Return the total number of columns
    fn column_count(&self) -> usize;

    /// Return the header text for a column
    ///
    /// # Arguments
    /// * `col` - Column index (0-based)
    fn column_header(&self, col: usize) -> String;

    /// Return the width for a specific column
    ///
    /// # Arguments
    /// * `col` - Column index (0-based)
    ///
    /// # Returns
    /// Width in pixels for this column
    fn column_width(&self, col: usize) -> f64;

    /// Return the height for a specific row
    ///
    /// If None, the grid will use a default row height.
    ///
    /// # Arguments
    /// * `row` - Row index (0-based, excluding header)
    ///
    /// # Returns
    /// Some(height) if the height is known, None to use default
    fn row_height(&self, row: usize) -> Option<f64>;

    /// Create or configure a widget for a specific cell
    ///
    /// The grid provides a reused widget if available (may be None for first use).
    /// The data source should update the widget's content to match the cell at (row, col).
    ///
    /// **Important:** If `reused_widget` is provided, check its type before reusing it!
    /// Use `widget.as_any().downcast_ref::<YourType>()` to verify compatibility.
    ///
    /// # Arguments
    /// * `row` - Row index (0-based, excluding header)
    /// * `col` - Column index (0-based)
    /// * `reused_widget` - A widget to reuse if available (None for first use)
    ///
    /// # Returns
    /// A configured widget for this cell. May be the reused widget or a new one.
    fn cell_widget(
        &mut self,
        row: usize,
        col: usize,
        reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget>;

    /// Optional: Return header widget for a column
    ///
    /// Default implementation creates a Label with column_header() text.
    /// Override to customize header appearance (e.g., sortable headers with icons).
    ///
    /// # Arguments
    /// * `col` - Column index (0-based)
    /// * `reused_widget` - A widget to reuse if available
    fn header_widget(
        &mut self,
        col: usize,
        _reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget> {
        // Default implementation - create a simple label
        // Users can override this for custom headers
        use crate::widgets::Label;
        Box::new(Label::new(&self.column_header(col)))
    }

    /// Optional: Enable row headers (first column frozen)
    ///
    /// Default is false. If true, the grid will show a fixed column on the left
    /// with row indices or custom content.
    fn has_row_headers(&self) -> bool {
        false
    }

    /// Optional: Return row header widget
    ///
    /// Only called if has_row_headers() returns true.
    ///
    /// # Arguments
    /// * `row` - Row index (0-based)
    fn row_header_widget(
        &mut self,
        row: usize,
        _reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget> {
        use crate::widgets::Label;
        Box::new(Label::new(&format!("{}", row + 1)))
    }

    /// Optional: Width of the row header column
    ///
    /// Only used if has_row_headers() returns true.
    fn row_header_width(&self) -> f64 {
        50.0
    }
}
