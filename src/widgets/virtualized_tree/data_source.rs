//! DataSource trait for virtualized tree views (hierarchical tables)
//!
//! Inspired by AppKit's NSOutlineViewDataSource, this trait allows
//! the tree widget to query hierarchical data efficiently.

use crate::widget::Widget;

/// Unique identifier for tree nodes
///
/// The data source is responsible for creating stable, unique identifiers
/// for each node in the tree. These IDs are used to track expand/collapse
/// state and should remain consistent across queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TreeNodeId(pub u64);

/// Data source trait for virtualized tree views (hierarchical tables)
///
/// This trait provides a flexible interface for displaying large hierarchical data
/// with efficient virtualization. Only visible rows are rendered.
///
/// # Example
///
/// ```rust,ignore
/// struct FileSystemDataSource {
///     root: PathBuf,
///     // ... other fields
/// }
///
/// impl TreeDataSource for FileSystemDataSource {
///     fn root_count(&self) -> usize {
///         // Number of items at root level
///         self.list_directory(&self.root).len()
///     }
///
///     fn root_node(&self, index: usize) -> TreeNodeId {
///         TreeNodeId(index as u64)
///     }
///
///     fn child_count(&self, node: TreeNodeId) -> usize {
///         // Number of children for this node
///         if self.is_directory(node) {
///             self.list_directory_for_node(node).len()
///         } else {
///             0
///         }
///     }
///
///     fn column_count(&self) -> usize {
///         3  // Name, Size, Modified Date
///     }
///
///     fn cell_widget(
///         &mut self,
///         node: TreeNodeId,
///         col: usize,
///         depth: usize,
///         is_expanded: bool,
///         reused_widget: Option<Box<dyn Widget>>,
///     ) -> Box<dyn Widget> {
///         // Return widget for this cell
///         Box::new(Label::new(&self.get_cell_data(node, col)))
///     }
/// }
/// ```
pub trait TreeDataSource {
    /// Return the number of root-level nodes
    fn root_count(&self) -> usize;

    /// Return the node ID for a root-level item
    ///
    /// # Arguments
    /// * `index` - Index in the root level (0-based)
    fn root_node(&self, index: usize) -> TreeNodeId;

    /// Return the number of children for a given node
    ///
    /// Return 0 if the node is a leaf (has no children).
    ///
    /// # Arguments
    /// * `node` - The parent node to query
    fn child_count(&self, node: TreeNodeId) -> usize;

    /// Return the child node ID at a specific index
    ///
    /// # Arguments
    /// * `node` - The parent node
    /// * `index` - Child index (0-based)
    fn child_node(&self, node: TreeNodeId, index: usize) -> TreeNodeId;

    /// Return the total number of columns
    ///
    /// The first column will contain the tree structure with indentation
    /// and expand/collapse chevrons. Additional columns display data.
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
    fn column_width(&self, col: usize) -> f64;

    /// Return the height for a specific row
    ///
    /// If None, the tree will use a default row height.
    ///
    /// # Arguments
    /// * `node` - The node for this row
    fn row_height(&self, node: TreeNodeId) -> Option<f64>;

    /// Create or configure a widget for a specific cell
    ///
    /// For the first column (col=0), the widget should only contain the
    /// text/content. The tree will automatically add indentation, chevron,
    /// and optional checkbox.
    ///
    /// **Important:** If `reused_widget` is provided, check its type before reusing it!
    /// Use `widget.as_any().downcast_ref::<YourType>()` to verify compatibility.
    ///
    /// # Arguments
    /// * `node` - The node for this cell
    /// * `col` - Column index (0-based)
    /// * `depth` - Depth level in the tree (0 = root)
    /// * `is_expanded` - True if this node is currently expanded
    /// * `reused_widget` - A widget to reuse if available
    ///
    /// # Returns
    /// A configured widget for this cell
    fn cell_widget(
        &mut self,
        node: TreeNodeId,
        col: usize,
        depth: usize,
        is_expanded: bool,
        reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget>;

    /// Optional: Return header widget for a column
    ///
    /// Default implementation creates a Label with column_header() text.
    ///
    /// # Arguments
    /// * `col` - Column index (0-based)
    /// * `reused_widget` - A widget to reuse if available
    fn header_widget(
        &mut self,
        col: usize,
        _reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget> {
        use crate::widgets::Label;
        Box::new(Label::new(&self.column_header(col)))
    }

    /// Optional: Enable checkboxes in the tree column
    ///
    /// Default is false. If true, the tree will show checkboxes next to items.
    fn has_checkboxes(&self) -> bool {
        false
    }

    /// Optional: Return checkbox state for a node
    ///
    /// Only called if has_checkboxes() returns true.
    ///
    /// # Arguments
    /// * `node` - The node to query
    ///
    /// # Returns
    /// true if checked, false if unchecked
    fn is_checked(&self, _node: TreeNodeId) -> bool {
        false
    }

    /// Optional: Called when user toggles checkbox
    ///
    /// Only called if has_checkboxes() returns true.
    ///
    /// # Arguments
    /// * `node` - The node being toggled
    /// * `checked` - New checked state
    fn set_checked(&mut self, _node: TreeNodeId, _checked: bool) {
        // Default: no-op
    }

    /// Optional: Get display text for a node (used for accessibility)
    ///
    /// Default implementation returns empty string.
    ///
    /// # Arguments
    /// * `node` - The node to query
    fn node_label(&self, _node: TreeNodeId) -> String {
        String::new()
    }
}
