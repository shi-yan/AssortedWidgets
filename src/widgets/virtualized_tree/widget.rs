//! VirtualizedTree widget - Efficient hierarchical table view
//!
//! Displays large tree structures with:
//! - Expand/collapse functionality
//! - Optional checkboxes
//! - Multi-column support
//! - Widget virtualization (only visible rows created)
//! - Horizontal and vertical scrolling

/*
Current Limitations:
⚠️ No Interactive Expand/Collapse - Expand/collapse state is tracked internally but can't be toggled via UI yet (requires event handling integration)
⚠️ No Scrollbars - Scrolling functionality commented out (requires event system)
⚠️ No Chevron Icons - Expand/collapse indicators not rendered (requires path drawing or icon support)
⚠️ No Checkbox Interaction - Checkboxes are rendered but can't be toggled (requires event handling) The widget follows the same patterns as VirtualizedList and VirtualizedGrid, providing a clean API for hierarchical data display. Interactive features (expand/collapse, checkbox toggling, scrolling) can be added once the framework's event handling system is integrated with these widgets. 

*/

use crate::layout::Style;
use crate::paint::{Border, Color, PaintContext, ShapeStyle, Stroke};
use crate::types::{DirtyLevel, rect, Point, Rect, Size, WidgetId};
use crate::widget::Widget;
use crate::WidgetState;
use crate::widgets::ScrollBar;
use std::any::Any;
use std::collections::HashSet;

use super::data_source::{TreeDataSource, TreeNodeId};

/// Represents a visible row in the flattened tree
#[derive(Debug, Clone)]
struct VisibleRow {
    node: TreeNodeId,
    depth: usize,
    has_children: bool,
    is_expanded: bool,
}

/// Cell widget with position information
struct CellInfo {
    row_index: usize, // Index in visible_rows
    col: usize,
    widget: Box<dyn Widget>,
}

/// Header widget with column information
struct HeaderInfo {
    col: usize,
    widget: Box<dyn Widget>,
}

/// VirtualizedTree - Efficient hierarchical table view
///
/// Only creates widgets for visible rows, recycles widgets when scrolling.
/// Supports expand/collapse, optional checkboxes, and multi-column display.
///
/// # Example
///
/// ```rust,ignore
/// let tree = VirtualizedTree::new()
///     .data_source(Box::new(MyTreeDataSource::new()))
///     .default_row_height(32.0)
///     .header_height(40.0)
///     .indent_width(20.0)
///     .show_checkboxes(true)
///     .background(Color::WHITE);
/// ```
pub struct VirtualizedTree {
    // Widget essentials
    state: WidgetState,
    layout_style: Style,

    // Data source
    data_source: Option<Box<dyn TreeDataSource>>,

    // Expand/collapse state
    expanded_nodes: HashSet<TreeNodeId>,

    // Flattened visible rows (regenerated when expand state changes)
    visible_rows: Vec<VisibleRow>,
    visible_rows_dirty: bool,

    // Widget pools
    cell_widgets: Vec<CellInfo>,
    header_widgets: Vec<HeaderInfo>,
    recycled_widgets: Vec<Box<dyn Widget>>,

    // Scrolling
    vertical_scroll: Option<ScrollBar>,
    horizontal_scroll: Option<ScrollBar>,
    scrollbar_width: f32,
    show_scrollbars: bool,
    scroll_x: f64,
    scroll_y: f64,

    // Layout configuration
    default_row_height: f64,
    header_height: f64,
    indent_width: f64,   // Indentation per depth level
    chevron_width: f64,  // Space for expand/collapse chevron
    checkbox_width: f64, // Space for checkbox (if enabled)

    // Cached layout (prefix sums for O(1) offset lookup)
    row_heights: Vec<f64>,
    row_offsets: Vec<f64>,
    column_widths: Vec<f64>,
    column_offsets: Vec<f64>,
    total_content_height: f64,
    total_content_width: f64,
    layout_dirty: bool,

    // Styling
    background: Color,
    header_background: Color,
    show_grid_lines: bool,
    grid_line_color: Color,
    chevron_color: Color,
}

impl VirtualizedTree {
    /// Create a new virtualized tree
    pub fn new() -> Self {
        Self {
            state: WidgetState::new(),
            layout_style: Style::default(),
            data_source: None,
            expanded_nodes: HashSet::new(),
            visible_rows: Vec::new(),
            visible_rows_dirty: true,
            cell_widgets: Vec::new(),
            header_widgets: Vec::new(),
            recycled_widgets: Vec::new(),
            vertical_scroll: None,
            horizontal_scroll: None,
            scrollbar_width: 12.0,
            show_scrollbars: true,
            scroll_x: 0.0,
            scroll_y: 0.0,
            default_row_height: 32.0,
            header_height: 40.0,
            indent_width: 20.0,
            chevron_width: 20.0,
            checkbox_width: 24.0,
            row_heights: Vec::new(),
            row_offsets: Vec::new(),
            column_widths: Vec::new(),
            column_offsets: Vec::new(),
            total_content_height: 0.0,
            total_content_width: 0.0,
            layout_dirty: true,
            background: Color::WHITE,
            header_background: Color::rgb(0.95, 0.95, 0.95),
            show_grid_lines: true,
            grid_line_color: Color::rgb(0.9, 0.9, 0.9),
            chevron_color: Color::rgb(0.4, 0.4, 0.4),
        }
    }

    /// Set the data source
    pub fn data_source(mut self, data_source: Box<dyn TreeDataSource>) -> Self {
        self.data_source = Some(data_source);
        self.visible_rows_dirty = true;
        self.layout_dirty = true;
        self.state.dirty = DirtyLevel::Visual;
        self
    }

    /// Set default row height
    pub fn default_row_height(mut self, height: f64) -> Self {
        self.default_row_height = height;
        self.layout_dirty = true;
        self.state.dirty = DirtyLevel::Visual;
        self
    }

    /// Set header height
    pub fn header_height(mut self, height: f64) -> Self {
        self.header_height = height;
        self.state.dirty = DirtyLevel::Visual;
        self
    }

    /// Set indentation width per depth level
    pub fn indent_width(mut self, width: f64) -> Self {
        self.indent_width = width;
        self.state.dirty = DirtyLevel::Visual;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self.state.dirty = DirtyLevel::Visual;
        self
    }

    /// Set header background color
    pub fn header_background(mut self, color: Color) -> Self {
        self.header_background = color;
        self.state.dirty = DirtyLevel::Visual;
        self
    }

    /// Enable/disable grid lines
    pub fn show_grid_lines(mut self, show: bool) -> Self {
        self.show_grid_lines = show;
        self.state.dirty = DirtyLevel::Visual;
        self
    }

    /// Set grid line color
    pub fn grid_line_color(mut self, color: Color) -> Self {
        self.grid_line_color = color;
        self.state.dirty = DirtyLevel::Visual;
        self
    }

    /// Set layout style
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    /// Rebuild the flattened visible rows list
    fn rebuild_visible_rows(&mut self) {
        self.visible_rows.clear();

        // Collect root nodes before recursive calls to avoid borrow checker issues
        let root_nodes = {
            let Some(ref data_source) = self.data_source else {
                self.visible_rows_dirty = false;
                return;
            };

            let root_count = data_source.root_count();
            (0..root_count)
                .map(|i| data_source.root_node(i))
                .collect::<Vec<_>>()
        };

        // Recursively build visible rows starting from root
        for node in root_nodes {
            self.add_visible_row(node, 0);
        }

        self.visible_rows_dirty = false;
        self.layout_dirty = true;
    }

    /// Recursively add a node and its expanded children to visible rows
    fn add_visible_row(&mut self, node: TreeNodeId, depth: usize) {
        // Pre-collect all child data before recursive calls to avoid borrow checker issues
        let (child_count, has_children, is_expanded, children) = {
            let Some(ref data_source) = self.data_source else {
                return;
            };

            let child_count = data_source.child_count(node);
            let has_children = child_count > 0;
            let is_expanded = self.expanded_nodes.contains(&node);

            // Collect children if expanded
            let children = if is_expanded && has_children {
                (0..child_count)
                    .map(|i| data_source.child_node(node, i))
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };

            (child_count, has_children, is_expanded, children)
        };

        self.visible_rows.push(VisibleRow {
            node,
            depth,
            has_children,
            is_expanded,
        });

        // Recursively add children
        for child in children {
            self.add_visible_row(child, depth + 1);
        }
    }

    /// Rebuild layout cache (row offsets, column offsets)
    fn rebuild_layout_cache(&mut self) {
        self.row_heights.clear();
        self.row_offsets.clear();
        self.column_widths.clear();
        self.column_offsets.clear();

        let Some(ref data_source) = self.data_source else {
            self.layout_dirty = false;
            return;
        };

        // Build row heights and offsets
        let row_count = self.visible_rows.len();
        self.row_heights.reserve(row_count);
        self.row_offsets.reserve(row_count);

        let mut offset = 0.0;
        for row in &self.visible_rows {
            self.row_offsets.push(offset);
            let height = data_source
                .row_height(row.node)
                .unwrap_or(self.default_row_height);
            self.row_heights.push(height);
            offset += height;
        }
        self.total_content_height = offset;

        // Build column widths and offsets
        let col_count = data_source.column_count();
        self.column_widths.reserve(col_count);
        self.column_offsets.reserve(col_count);

        let mut offset = 0.0;
        for col in 0..col_count {
            self.column_offsets.push(offset);
            let width = data_source.column_width(col);
            self.column_widths.push(width);
            offset += width;
        }
        self.total_content_width = offset;

        self.layout_dirty = false;
    }

    /// Get the Y offset of a row
    fn row_offset(&self, row_index: usize) -> Option<f64> {
        self.row_offsets.get(row_index).copied()
    }

    /// Get the height of a row
    fn row_height(&self, row_index: usize) -> Option<f64> {
        self.row_heights.get(row_index).copied()
    }

    /// Get the X offset of a column
    fn column_offset(&self, col: usize) -> Option<f64> {
        self.column_offsets.get(col).copied()
    }

    /// Get the width of a column
    fn column_width(&self, col: usize) -> Option<f64> {
        self.column_widths.get(col).copied()
    }

    /// Calculate visible row range using binary search
    fn calculate_visible_row_range(&self) -> std::ops::Range<usize> {
        let viewport_y = self.scroll_y;
        let viewport_height = self.state.bounds.height() - self.header_height;
        let viewport_start = viewport_y;
        let viewport_end = viewport_y + viewport_height;

        let row_count = self.visible_rows.len();
        if row_count == 0 {
            return 0..0;
        }

        // Binary search for start row
        let start = self
            .row_offsets
            .partition_point(|&offset| {
                let row_idx = self.row_offsets.iter().position(|&o| o == offset).unwrap();
                let row_bottom = offset + self.row_heights[row_idx];
                row_bottom <= viewport_start
            })
            .min(row_count);

        // Binary search for end row
        let end = self
            .row_offsets
            .partition_point(|&offset| offset < viewport_end)
            .min(row_count);

        start..end
    }

    /// Calculate visible column range
    fn calculate_visible_column_range(&self) -> std::ops::Range<usize> {
        let Some(ref data_source) = self.data_source else {
            return 0..0;
        };

        let viewport_x = self.scroll_x;
        let viewport_width = self.state.bounds.width();
        let viewport_start = viewport_x;
        let viewport_end = viewport_x + viewport_width;

        let col_count = data_source.column_count();
        if col_count == 0 {
            return 0..0;
        }

        // Binary search for start column
        let start = self
            .column_offsets
            .partition_point(|&offset| {
                let col_idx = self.column_offsets.iter().position(|&o| o == offset).unwrap();
                let col_right = offset + self.column_widths[col_idx];
                col_right <= viewport_start
            })
            .min(col_count);

        // Binary search for end column
        let end = self
            .column_offsets
            .partition_point(|&offset| offset < viewport_end)
            .min(col_count);

        start..end
    }

    /// Update visible cells based on scroll position
    fn update_visible_cells(&mut self) {
        if self.visible_rows_dirty {
            self.rebuild_visible_rows();
        }

        if self.layout_dirty {
            self.rebuild_layout_cache();
        }

        let visible_row_range = self.calculate_visible_row_range();
        let visible_col_range = self.calculate_visible_column_range();

        // Recycle cells that are no longer visible
        let mut cells_to_keep = Vec::new();
        for cell in self.cell_widgets.drain(..) {
            if visible_row_range.contains(&cell.row_index) && visible_col_range.contains(&cell.col) {
                cells_to_keep.push(cell);
            } else {
                self.recycled_widgets.push(cell.widget);
            }
        }
        self.cell_widgets = cells_to_keep;

        // Create new cells for visible range
        {
            let Some(ref mut data_source) = self.data_source else {
                return;
            };

            for row_index in visible_row_range.clone() {
                let visible_row = &self.visible_rows[row_index];
                for col in visible_col_range.clone() {
                    // Check if cell already exists
                    if self
                        .cell_widgets
                        .iter()
                        .any(|c| c.row_index == row_index && c.col == col)
                    {
                        continue;
                    }

                    // Try to reuse a widget
                    let reused = self.recycled_widgets.pop();

                    // Create widget
                    let widget = data_source.cell_widget(
                        visible_row.node,
                        col,
                        visible_row.depth,
                        visible_row.is_expanded,
                        reused,
                    );

                    self.cell_widgets.push(CellInfo {
                        row_index,
                        col,
                        widget,
                    });
                }
            }
        }

        // Update header cells
        self.update_header_cells();

        // Layout all cells
        self.layout_visible_cells();
    }

    /// Update header cells
    fn update_header_cells(&mut self) {
        let visible_col_range = self.calculate_visible_column_range();

        // Recycle headers no longer visible
        let mut headers_to_keep = Vec::new();
        for header in self.header_widgets.drain(..) {
            if visible_col_range.contains(&header.col) {
                headers_to_keep.push(header);
            } else {
                self.recycled_widgets.push(header.widget);
            }
        }
        self.header_widgets = headers_to_keep;

        // Create new headers
        let Some(ref mut data_source) = self.data_source else {
            return;
        };

        for col in visible_col_range {
            if self.header_widgets.iter().any(|h| h.col == col) {
                continue;
            }

            let reused = self.recycled_widgets.pop();
            let widget = data_source.header_widget(col, reused);

            self.header_widgets.push(HeaderInfo { col, widget });
        }
    }

    /// Layout visible cells
    fn layout_visible_cells(&mut self) {
        let Some(ref data_source) = self.data_source else {
            return;
        };

        let has_checkboxes = data_source.has_checkboxes();

        // Pre-collect cell layouts
        let cell_layouts: Vec<(usize, usize, Rect)> = self
            .cell_widgets
            .iter()
            .filter_map(|cell| {
                let visible_row = &self.visible_rows[cell.row_index];
                let x_offset = self.column_offset(cell.col)?;
                let y_offset = self.row_offset(cell.row_index)?;
                let width = self.column_width(cell.col)?;
                let height = self.row_height(cell.row_index)?;

                // For first column, add space for indentation, chevron, and checkbox
                let mut x = x_offset - self.scroll_x;
                let mut content_width = width;

                if cell.col == 0 {
                    let indent = visible_row.depth as f64 * self.indent_width;
                    let chevron_space = if visible_row.has_children {
                        self.chevron_width
                    } else {
                        self.chevron_width // Keep space for alignment
                    };
                    let checkbox_space = if has_checkboxes { self.checkbox_width } else { 0.0 };

                    x += indent + chevron_space + checkbox_space;
                    content_width -= indent + chevron_space + checkbox_space;
                }

                let y = y_offset - self.scroll_y + self.header_height;

                Some((
                    cell.row_index,
                    cell.col,
                    rect(x, y, content_width, height),
                ))
            })
            .collect();

        // Apply layouts
        for (row_index, col, rect) in cell_layouts {
            if let Some(cell) = self
                .cell_widgets
                .iter_mut()
                .find(|c| c.row_index == row_index && c.col == col)
            {
                cell.widget.set_bounds(rect);
            }
        }

        // Pre-collect header layouts
        let header_layouts: Vec<(usize, Rect)> = self
            .header_widgets
            .iter()
            .filter_map(|header| {
                let x_offset = self.column_offset(header.col)?;
                let width = self.column_width(header.col)?;
                let x = x_offset - self.scroll_x;
                Some((header.col, rect(x, 0.0, width, self.header_height)))
            })
            .collect();

        // Apply header layouts
        for (col, rect) in header_layouts {
            if let Some(header) = self.header_widgets.iter_mut().find(|h| h.col == col) {
                header.widget.set_bounds(rect);
            }
        }
    }

    // TODO: Implement scrollbar support
    // Currently commented out until proper event handling is added
    /*
    fn update_scrollbars(&mut self) {
        // Scrollbar management will be added in future update
    }
    */

    // TODO: Implement expand/collapse interaction
    // Currently commented out until proper event handling is added
    /*
    fn toggle_expand(&mut self, node: TreeNodeId) {
        if self.expanded_nodes.contains(&node) {
            self.expanded_nodes.remove(&node);
        } else {
            self.expanded_nodes.insert(node);
        }
        self.visible_rows_dirty = true;
        self.state.dirty = DirtyLevel::Visual;
    }

    fn handle_tree_column_click(&mut self, point: Point) -> bool {
        // Event handling will be added in future update
        false
    }
    */
}

impl Widget for VirtualizedTree {
    fn id(&self) -> WidgetId {
        self.state.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.state.id = id;
    }

    fn bounds(&self) -> Rect {
        self.state.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.state.bounds = bounds;
    }

    fn dirty_level(&self) -> crate::types::DirtyLevel {
        self.state.dirty
    }

    fn set_dirty_level(&mut self, level: crate::types::DirtyLevel) {
        self.state.dirty = level;
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.set_dirty_level(crate::types::DirtyLevel::from(dirty));
    }

    fn is_dirty(&self) -> bool {
        self.dirty_level().needs_repaint()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }



    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn needs_measure(&self) -> bool {
        true
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        Some(Size::new(300.0, 400.0)) // Default size
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Background
        ctx.draw_rect(self.state.bounds, self.background);

        // Draw header background
        let header_rect = rect(0.0, 0.0, self.state.bounds.width(), self.header_height);
        ctx.draw_rect(header_rect, self.header_background);

        // Draw header widgets
        for header in &self.header_widgets {
            header.widget.paint(ctx);
        }

        // Draw header bottom border
        if self.show_grid_lines {
            ctx.draw_line(
                Point::new(0.0, self.header_height),
                Point::new(self.state.bounds.width(), self.header_height),
                Stroke::new(self.grid_line_color, 1.0),
            );
        }

        let Some(ref data_source) = self.data_source else {
            return;
        };

        let has_checkboxes = data_source.has_checkboxes();

        // Draw grid lines and tree decorations
        for cell in &self.cell_widgets {
            let visible_row = &self.visible_rows[cell.row_index];
            let cell_bounds = cell.widget.bounds();

            // Draw cell widget
            cell.widget.paint(ctx);

            // Draw tree-specific decorations for first column
            if cell.col == 0 {
                let y_offset = self.row_offset(cell.row_index).unwrap_or(0.0);
                let height = self.row_height(cell.row_index).unwrap_or(32.0);
                let y = y_offset - self.scroll_y + self.header_height;

                let indent = visible_row.depth as f64 * self.indent_width;

                // TODO: Draw chevron if node has children (requires path drawing or icon support)
                // For now, we'll skip the chevron rendering

                // Draw checkbox if enabled
                if has_checkboxes {
                    let checkbox_x = indent + self.chevron_width + 4.0;
                    let checkbox_y = y + height / 2.0 - 8.0;
                    let checkbox_size = 16.0;

                    // Checkbox with border
                    ctx.draw_styled_rect(
                        rect(checkbox_x, checkbox_y, checkbox_size, checkbox_size),
                        ShapeStyle::solid(Color::WHITE).with_border(Border::new(self.grid_line_color, 1.0)),
                    );

                    // Checkmark if checked
                    if data_source.is_checked(visible_row.node) {
                        let check_color = Color::rgb(0.2, 0.6, 1.0);
                        ctx.draw_line(
                            Point::new(checkbox_x + 3.0, checkbox_y + 8.0),
                            Point::new(checkbox_x + 6.0, checkbox_y + 11.0),
                            Stroke::new(check_color, 2.0),
                        );
                        ctx.draw_line(
                            Point::new(checkbox_x + 6.0, checkbox_y + 11.0),
                            Point::new(checkbox_x + 13.0, checkbox_y + 4.0),
                            Stroke::new(check_color, 2.0),
                        );
                    }
                }
            }

            // Draw grid lines
            if self.show_grid_lines {
                // Horizontal line
                ctx.draw_line(
                    Point::new(0.0, cell_bounds.origin.y + cell_bounds.height()),
                    Point::new(self.state.bounds.width(), cell_bounds.origin.y + cell_bounds.height()),
                    Stroke::new(self.grid_line_color, 1.0),
                );

                // Vertical line
                ctx.draw_line(
                    Point::new(cell_bounds.origin.x + cell_bounds.width(), cell_bounds.origin.y),
                    Point::new(
                        cell_bounds.origin.x + cell_bounds.width(),
                        cell_bounds.origin.y + cell_bounds.height(),
                    ),
                    Stroke::new(self.grid_line_color, 1.0),
                );
            }
        }

        // TODO: Draw scrollbars when event handling is implemented
        // Currently no scrollbars are shown
    }
}
