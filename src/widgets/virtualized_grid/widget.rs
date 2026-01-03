//! VirtualizedGrid widget implementation - efficient 2D table virtualization

use std::any::Any;
use std::ops::Range;

use crate::event::{EventResponse, InputEventEnum, MouseEvent, WheelEvent};
use crate::event::handlers::MouseHandler;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::types::{DeferredCommand, FrameInfo, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;
use crate::widgets::ScrollBar;

use super::data_source::GridDataSource;

/// VirtualizedGrid widget - efficient 2D table with row and column virtualization
///
/// Only creates widgets for visible cells, making it suitable for large datasets
/// (e.g., CSV files, spreadsheets, database tables).
///
/// # Features
/// - Row and column virtualization
/// - Fixed header row
/// - Optional row headers (frozen first column)
/// - Horizontal and vertical scrolling
/// - Cell widget recycling
/// - Variable row heights
/// - Custom column widths
///
/// # Example
///
/// ```rust,ignore
/// let grid = VirtualizedGrid::new()
///     .data_source(Box::new(MyCsvDataSource::new()))
///     .default_row_height(32.0)
///     .header_height(40.0);
/// ```
pub struct VirtualizedGrid {
    // Standard widget fields
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Scrolling state
    scroll_offset_x: f64,  // Horizontal scroll (columns)
    scroll_offset_y: f64,  // Vertical scroll (rows)
    total_content_width: f64,
    total_content_height: f64,
    viewport_width: f64,
    viewport_height: f64,

    // Scrollbars
    vscrollbar: Option<ScrollBar>,
    hscrollbar: Option<ScrollBar>,
    scrollbar_width: f32,
    show_scrollbars: bool,

    // Data source
    data_source: Option<Box<dyn GridDataSource>>,

    // Widget pooling
    visible_cells: Vec<CellInfo>,
    header_cells: Vec<HeaderCellInfo>,
    row_header_cells: Vec<RowHeaderCellInfo>,
    recycled_cells: Vec<Box<dyn Widget>>,

    // Cached measurements
    row_heights: Vec<f64>,       // Height of each row
    row_offsets: Vec<f64>,       // Y offset of each row (prefix sum)
    column_widths: Vec<f64>,     // Width of each column
    column_offsets: Vec<f64>,    // X offset of each column (prefix sum)

    // Layout configuration
    default_row_height: f64,
    header_height: f64,

    // Colors
    bg_color: Option<Color>,
    header_bg_color: Color,
    grid_line_color: Color,
    show_grid_lines: bool,

    // Pending commands
    pending_commands: Vec<DeferredCommand>,
}

/// Information about a visible cell
struct CellInfo {
    row: usize,
    col: usize,
    widget: Box<dyn Widget>,
}

/// Information about a header cell
struct HeaderCellInfo {
    col: usize,
    widget: Box<dyn Widget>,
}

/// Information about a row header cell
struct RowHeaderCellInfo {
    row: usize,
    widget: Box<dyn Widget>,
}

impl VirtualizedGrid {
    /// Create a new virtualized grid
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style {
                flex_grow: 1.0,
                flex_shrink: 1.0,
                ..Style::default()
            },
            scroll_offset_x: 0.0,
            scroll_offset_y: 0.0,
            total_content_width: 0.0,
            total_content_height: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            vscrollbar: None,
            hscrollbar: None,
            scrollbar_width: 12.0,
            show_scrollbars: true,
            data_source: None,
            visible_cells: Vec::new(),
            header_cells: Vec::new(),
            row_header_cells: Vec::new(),
            recycled_cells: Vec::new(),
            row_heights: Vec::new(),
            row_offsets: Vec::new(),
            column_widths: Vec::new(),
            column_offsets: Vec::new(),
            default_row_height: 32.0,
            header_height: 40.0,
            bg_color: Some(Color::WHITE),
            header_bg_color: Color::rgb(0.95, 0.95, 0.95),
            grid_line_color: Color::rgb(0.85, 0.85, 0.85),
            show_grid_lines: true,
            pending_commands: Vec::new(),
        }
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set the data source
    pub fn data_source(mut self, source: Box<dyn GridDataSource>) -> Self {
        self.data_source = Some(source);
        self
    }

    /// Set default row height
    pub fn default_row_height(mut self, height: f64) -> Self {
        self.default_row_height = height;
        self
    }

    /// Set header row height
    pub fn header_height(mut self, height: f64) -> Self {
        self.header_height = height;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set header background color
    pub fn header_background(mut self, color: Color) -> Self {
        self.header_bg_color = color;
        self
    }

    /// Enable or disable grid lines
    pub fn show_grid_lines(mut self, show: bool) -> Self {
        self.show_grid_lines = show;
        self
    }

    /// Set grid line color
    pub fn grid_line_color(mut self, color: Color) -> Self {
        self.grid_line_color = color;
        self
    }

    /// Set layout style
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Set the data source at runtime
    pub fn set_data_source(&mut self, source: Box<dyn GridDataSource>) {
        self.data_source = Some(source);
        self.invalidate_layout();
    }

    /// Reload all data from the data source
    pub fn reload_data(&mut self) {
        self.invalidate_layout();
    }

    /// Scroll to a specific cell
    pub fn scroll_to_cell(&mut self, row: usize, col: usize) {
        if let (Some(x_offset), Some(y_offset)) = (self.offset_of_column(col), self.offset_of_row(row)) {
            self.scroll_offset_x = x_offset.clamp(0.0, self.max_scroll_x());
            self.scroll_offset_y = y_offset.clamp(0.0, self.max_scroll_y());
            self.update_visible_cells();
            self.update_scrollbars();
            self.dirty = true;
        }
    }

    // ========================================================================
    // Internal Helpers - Layout Calculations
    // ========================================================================

    /// Invalidate cached layout and force recalculation
    fn invalidate_layout(&mut self) {
        self.row_heights.clear();
        self.row_offsets.clear();
        self.column_widths.clear();
        self.column_offsets.clear();
        self.total_content_width = 0.0;
        self.total_content_height = 0.0;

        // Recycle all widgets
        for cell in self.visible_cells.drain(..) {
            self.recycled_cells.push(cell.widget);
        }
        for header in self.header_cells.drain(..) {
            self.recycled_cells.push(header.widget);
        }
        for row_header in self.row_header_cells.drain(..) {
            self.recycled_cells.push(row_header.widget);
        }

        self.dirty = true;
    }

    /// Calculate column widths and offsets (prefix sum)
    fn calculate_column_layout(&mut self) {
        let Some(ref data_source) = self.data_source else {
            return;
        };

        let col_count = data_source.column_count();
        self.column_widths.clear();
        self.column_offsets.clear();

        let mut offset = 0.0;

        // Add row header column if enabled
        if data_source.has_row_headers() {
            let width = data_source.row_header_width();
            self.column_widths.push(width);
            self.column_offsets.push(offset);
            offset += width;
        }

        // Add data columns
        for col in 0..col_count {
            let width = data_source.column_width(col);
            self.column_widths.push(width);
            self.column_offsets.push(offset);
            offset += width;
        }

        self.total_content_width = offset;
    }

    /// Calculate row heights and offsets (prefix sum)
    fn calculate_row_layout(&mut self) {
        let Some(ref data_source) = self.data_source else {
            return;
        };

        let row_count = data_source.row_count();
        self.row_heights.clear();
        self.row_offsets.clear();

        let mut offset = 0.0;
        for row in 0..row_count {
            let height = data_source.row_height(row).unwrap_or(self.default_row_height);
            self.row_heights.push(height);
            self.row_offsets.push(offset);
            offset += height;
        }

        self.total_content_height = offset;
    }

    /// Get the X offset of a column (O(1))
    fn offset_of_column(&self, col: usize) -> Option<f64> {
        self.column_offsets.get(col).copied()
    }

    /// Get the width of a column (O(1))
    fn width_of_column(&self, col: usize) -> Option<f64> {
        self.column_widths.get(col).copied()
    }

    /// Get the Y offset of a row (O(1))
    fn offset_of_row(&self, row: usize) -> Option<f64> {
        self.row_offsets.get(row).copied()
    }

    /// Get the height of a row (O(1))
    fn height_of_row(&self, row: usize) -> Option<f64> {
        self.row_heights.get(row).copied()
    }

    /// Calculate which rows are currently visible
    fn calculate_visible_row_range(&self) -> Range<usize> {
        if self.row_offsets.is_empty() {
            return 0..0;
        }

        let viewport_start = self.scroll_offset_y;
        let viewport_end = viewport_start + (self.viewport_height - self.header_height);

        // Binary search for first visible row
        let start = self.row_offsets.partition_point(|&offset| {
            let row_idx = self.row_offsets.iter().position(|&o| o == offset).unwrap();
            let row_bottom = offset + self.row_heights.get(row_idx).copied().unwrap_or(0.0);
            row_bottom <= viewport_start
        });

        // Binary search for last visible row
        let end = self.row_offsets.partition_point(|&offset| {
            offset < viewport_end
        }).min(self.row_heights.len());

        start..end
    }

    /// Calculate which columns are currently visible
    fn calculate_visible_column_range(&self) -> Range<usize> {
        let Some(ref data_source) = self.data_source else {
            return 0..0;
        };

        if self.column_offsets.is_empty() {
            return 0..0;
        }

        let viewport_start = self.scroll_offset_x;
        let viewport_end = viewport_start + self.viewport_width;

        // Adjust for row header column
        let col_offset_adjustment = if data_source.has_row_headers() { 1 } else { 0 };

        // Binary search for first visible column
        let start = self.column_offsets[col_offset_adjustment..].partition_point(|&offset| {
            let col_idx = self.column_offsets.iter().position(|&o| o == offset).unwrap();
            let col_right = offset + self.column_widths.get(col_idx).copied().unwrap_or(0.0);
            col_right <= viewport_start
        });

        // Binary search for last visible column
        let end = self.column_offsets[col_offset_adjustment..].partition_point(|&offset| {
            offset < viewport_end
        }).min(data_source.column_count());

        start..end
    }

    /// Maximum horizontal scroll offset
    fn max_scroll_x(&self) -> f64 {
        (self.total_content_width - self.viewport_width).max(0.0)
    }

    /// Maximum vertical scroll offset
    fn max_scroll_y(&self) -> f64 {
        (self.total_content_height - (self.viewport_height - self.header_height)).max(0.0)
    }

    // ========================================================================
    // Internal Helpers - Widget Management
    // ========================================================================

    /// Update which cells are visible based on current scroll position
    fn update_visible_cells(&mut self) {
        let visible_rows = self.calculate_visible_row_range();
        let visible_cols = self.calculate_visible_column_range();

        eprintln!("[VirtualizedGrid] Visible rows: {:?}, cols: {:?}", visible_rows, visible_cols);

        // Recycle cells that are no longer visible
        let mut cells_to_keep = Vec::new();
        for cell in self.visible_cells.drain(..) {
            if visible_rows.contains(&cell.row) && visible_cols.contains(&cell.col) {
                cells_to_keep.push(cell);
            } else {
                // Recycle this cell's widget
                self.recycled_cells.push(cell.widget);
            }
        }
        self.visible_cells = cells_to_keep;

        // Create cells for newly visible positions
        {
            let Some(ref mut data_source) = self.data_source else {
                return;
            };

            for row in visible_rows.clone() {
                for col in visible_cols.clone() {
                    // Check if we already have this cell
                    if !self.visible_cells.iter().any(|c| c.row == row && c.col == col) {
                        let reused = self.recycled_cells.pop();
                        let widget = data_source.cell_widget(row, col, reused);

                        self.visible_cells.push(CellInfo { row, col, widget });
                    }
                }
            }
        } // data_source borrow dropped here

        // Update header cells
        self.update_header_cells();

        // Update row header cells
        let has_row_headers = self.data_source.as_ref().map(|ds| ds.has_row_headers()).unwrap_or(false);
        if has_row_headers {
            self.update_row_header_cells();
        }

        // Layout all widgets
        self.layout_visible_cells();
    }

    /// Update header cells
    fn update_header_cells(&mut self) {
        let visible_cols = self.calculate_visible_column_range();

        let Some(ref mut data_source) = self.data_source else {
            return;
        };

        // Recycle headers that are no longer visible
        let mut headers_to_keep = Vec::new();
        for header in self.header_cells.drain(..) {
            if visible_cols.contains(&header.col) {
                headers_to_keep.push(header);
            } else {
                self.recycled_cells.push(header.widget);
            }
        }
        self.header_cells = headers_to_keep;

        // Create headers for newly visible columns
        for col in visible_cols {
            if !self.header_cells.iter().any(|h| h.col == col) {
                let reused = self.recycled_cells.pop();
                let widget = data_source.header_widget(col, reused);

                self.header_cells.push(HeaderCellInfo { col, widget });
            }
        }
    }

    /// Update row header cells
    fn update_row_header_cells(&mut self) {
        let visible_rows = self.calculate_visible_row_range();

        let Some(ref mut data_source) = self.data_source else {
            return;
        };

        // Recycle row headers that are no longer visible
        let mut row_headers_to_keep = Vec::new();
        for row_header in self.row_header_cells.drain(..) {
            if visible_rows.contains(&row_header.row) {
                row_headers_to_keep.push(row_header);
            } else {
                self.recycled_cells.push(row_header.widget);
            }
        }
        self.row_header_cells = row_headers_to_keep;

        // Create row headers for newly visible rows
        for row in visible_rows {
            if !self.row_header_cells.iter().any(|rh| rh.row == row) {
                let reused = self.recycled_cells.pop();
                let widget = data_source.row_header_widget(row, reused);

                self.row_header_cells.push(RowHeaderCellInfo { row, widget });
            }
        }
    }

    /// Position all visible cells
    fn layout_visible_cells(&mut self) {
        let Some(ref data_source) = self.data_source else {
            return;
        };

        let row_header_width = if data_source.has_row_headers() {
            data_source.row_header_width()
        } else {
            0.0
        };
        let has_row_headers = data_source.has_row_headers();

        // Pre-collect all layout data
        let cell_layouts: Vec<(usize, usize, Rect)> = self.visible_cells
            .iter()
            .filter_map(|cell| {
                let col_idx = cell.col + if has_row_headers { 1 } else { 0 };
                let x_offset = self.offset_of_column(col_idx)?;
                let y_offset = self.offset_of_row(cell.row)?;
                let width = self.width_of_column(col_idx).unwrap_or(100.0);
                let height = self.height_of_row(cell.row).unwrap_or(self.default_row_height);

                let x = self.bounds.origin.x + row_header_width + (x_offset - self.scroll_offset_x - row_header_width);
                let y = self.bounds.origin.y + self.header_height + (y_offset - self.scroll_offset_y);

                Some((cell.row, cell.col, Rect::new(Point::new(x, y), Size::new(width, height))))
            })
            .collect();

        let header_layouts: Vec<(usize, Rect)> = self.header_cells
            .iter()
            .filter_map(|header| {
                let col_idx = header.col + if has_row_headers { 1 } else { 0 };
                let x_offset = self.offset_of_column(col_idx)?;
                let width = self.width_of_column(col_idx).unwrap_or(100.0);

                let x = self.bounds.origin.x + row_header_width + (x_offset - self.scroll_offset_x - row_header_width);
                let y = self.bounds.origin.y;

                Some((header.col, Rect::new(Point::new(x, y), Size::new(width, self.header_height))))
            })
            .collect();

        let row_header_layouts: Vec<(usize, Rect)> = self.row_header_cells
            .iter()
            .filter_map(|row_header| {
                let y_offset = self.offset_of_row(row_header.row)?;
                let height = self.height_of_row(row_header.row).unwrap_or(self.default_row_height);

                let x = self.bounds.origin.x;
                let y = self.bounds.origin.y + self.header_height + (y_offset - self.scroll_offset_y);

                Some((row_header.row, Rect::new(Point::new(x, y), Size::new(row_header_width, height))))
            })
            .collect();

        // Apply layouts
        for (row, col, rect) in cell_layouts {
            if let Some(cell) = self.visible_cells.iter_mut().find(|c| c.row == row && c.col == col) {
                cell.widget.set_bounds(rect);
            }
        }

        for (col, rect) in header_layouts {
            if let Some(header) = self.header_cells.iter_mut().find(|h| h.col == col) {
                header.widget.set_bounds(rect);
            }
        }

        for (row, rect) in row_header_layouts {
            if let Some(row_header) = self.row_header_cells.iter_mut().find(|rh| rh.row == row) {
                row_header.widget.set_bounds(rect);
            }
        }
    }

    // ========================================================================
    // Internal Helpers - Scrollbars
    // ========================================================================

    /// Update scrollbar state
    fn update_scrollbars(&mut self) {
        if !self.show_scrollbars {
            self.vscrollbar = None;
            self.hscrollbar = None;
            return;
        }

        // Vertical scrollbar
        let needs_vscroll = self.total_content_height > (self.viewport_height - self.header_height);
        if needs_vscroll {
            let max_scroll = self.max_scroll_y() as i32;
            let page_size = (self.viewport_height - self.header_height) as i32;

            if let Some(ref mut vscroll) = self.vscrollbar {
                vscroll.set_range(0, max_scroll.max(1));
                vscroll.set_page_size(page_size.max(1));
                vscroll.set_value(self.scroll_offset_y as i32);
            } else {
                let mut vscroll = ScrollBar::vertical(0, max_scroll.max(1), page_size.max(1));
                vscroll.set_value(self.scroll_offset_y as i32);
                self.vscrollbar = Some(vscroll);
            }
        } else {
            self.vscrollbar = None;
        }

        // Horizontal scrollbar
        let needs_hscroll = self.total_content_width > self.viewport_width;
        if needs_hscroll {
            let max_scroll = self.max_scroll_x() as i32;
            let page_size = self.viewport_width as i32;

            if let Some(ref mut hscroll) = self.hscrollbar {
                hscroll.set_range(0, max_scroll.max(1));
                hscroll.set_page_size(page_size.max(1));
                hscroll.set_value(self.scroll_offset_x as i32);
            } else {
                let mut hscroll = ScrollBar::horizontal(0, max_scroll.max(1), page_size.max(1));
                hscroll.set_value(self.scroll_offset_x as i32);
                self.hscrollbar = Some(hscroll);
            }
        } else {
            self.hscrollbar = None;
        }

        self.position_scrollbars();
    }

    /// Position scrollbars on edges
    fn position_scrollbars(&mut self) {
        let hscroll_h = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        // Vertical scrollbar on right edge
        if let Some(ref mut vscroll) = self.vscrollbar {
            vscroll.set_bounds(Rect::new(
                Point::new(
                    self.bounds.origin.x + self.bounds.size.width - self.scrollbar_width as f64,
                    self.bounds.origin.y,
                ),
                Size::new(
                    self.scrollbar_width as f64,
                    self.bounds.size.height - hscroll_h,
                ),
            ));
        }

        // Horizontal scrollbar on bottom edge
        let vscroll_w = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        if let Some(ref mut hscroll) = self.hscrollbar {
            hscroll.set_bounds(Rect::new(
                Point::new(
                    self.bounds.origin.x,
                    self.bounds.origin.y + self.bounds.size.height - self.scrollbar_width as f64,
                ),
                Size::new(
                    self.bounds.size.width - vscroll_w,
                    self.scrollbar_width as f64,
                ),
            ));
        }
    }
}

// Dummy widget for mem::replace placeholder
struct DummyWidget;
impl Widget for DummyWidget {
    fn id(&self) -> WidgetId { WidgetId::new(0) }
    fn set_id(&mut self, _id: WidgetId) {}
    fn bounds(&self) -> Rect { Rect::default() }
    fn set_bounds(&mut self, _bounds: Rect) {}
    fn set_dirty(&mut self, _dirty: bool) {}
    fn is_dirty(&self) -> bool { false }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn layout(&self) -> Style { Style::default() }
    fn paint(&self, _ctx: &mut PaintContext) {}
}

// Widget trait implementation continues in next part...

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for VirtualizedGrid {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        if self.bounds != bounds {
            self.bounds = bounds;
            self.viewport_width = bounds.size.width - if self.vscrollbar.is_some() {
                self.scrollbar_width as f64
            } else {
                0.0
            };
            self.viewport_height = bounds.size.height - if self.hscrollbar.is_some() {
                self.scrollbar_width as f64
            } else {
                0.0
            };

            // Calculate layout if needed
            if self.column_offsets.is_empty() && self.data_source.is_some() {
                self.calculate_column_layout();
                self.calculate_row_layout();
            }

            self.update_scrollbars();
            self.update_visible_cells();
            self.dirty = true;
        }
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn needs_measure(&self) -> bool {
        true
    }

    fn update(&mut self, frame_info: &FrameInfo) {
        // Update scrollbars
        if let Some(ref mut vscroll) = self.vscrollbar {
            vscroll.update(frame_info);
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            hscroll.update(frame_info);
        }

        // Update all visible cells
        for cell in &mut self.visible_cells {
            cell.widget.update(frame_info);
        }
        for header in &mut self.header_cells {
            header.widget.update(frame_info);
        }
        for row_header in &mut self.row_header_cells {
            row_header.widget.update(frame_info);
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw background
        if let Some(bg_color) = self.bg_color {
            ctx.draw_rect(self.bounds, bg_color);
        }

        let Some(ref data_source) = self.data_source else {
            return;
        };

        let row_header_width = if data_source.has_row_headers() {
            data_source.row_header_width()
        } else {
            0.0
        };

        // Calculate content area (excluding scrollbars)
        let vscroll_w = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };
        let hscroll_h = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        // Clip to content area (excluding header and scrollbars)
        let content_rect = Rect::new(
            Point::new(
                self.bounds.origin.x + row_header_width,
                self.bounds.origin.y + self.header_height,
            ),
            Size::new(
                self.bounds.size.width - row_header_width - vscroll_w,
                self.bounds.size.height - self.header_height - hscroll_h,
            ),
        );

        ctx.push_clip(content_rect);

        // Paint data cells
        for cell in &self.visible_cells {
            cell.widget.paint(ctx);

            // Draw grid lines
            if self.show_grid_lines {
                let bounds = cell.widget.bounds();
                // Right border
                ctx.draw_line(
                    Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y),
                    Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
                    crate::paint::Stroke::new(self.grid_line_color, 1.0),
                );
                // Bottom border
                ctx.draw_line(
                    Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height),
                    Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
                    crate::paint::Stroke::new(self.grid_line_color, 1.0),
                );
            }
        }

        ctx.pop_clip();

        // Draw header background
        let header_rect = Rect::new(
            Point::new(self.bounds.origin.x + row_header_width, self.bounds.origin.y),
            Size::new(self.bounds.size.width - row_header_width - vscroll_w, self.header_height),
        );
        ctx.draw_rect(header_rect, self.header_bg_color);

        // Clip to header area
        ctx.push_clip(header_rect);

        // Paint header cells
        for header in &self.header_cells {
            header.widget.paint(ctx);

            // Draw grid lines
            if self.show_grid_lines {
                let bounds = header.widget.bounds();
                ctx.draw_line(
                    Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y),
                    Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
                    crate::paint::Stroke::new(self.grid_line_color, 1.0),
                );
            }
        }

        ctx.pop_clip();

        // Draw row headers if enabled
        if data_source.has_row_headers() {
            let row_header_rect = Rect::new(
                Point::new(self.bounds.origin.x, self.bounds.origin.y + self.header_height),
                Size::new(row_header_width, self.bounds.size.height - self.header_height - hscroll_h),
            );
            ctx.draw_rect(row_header_rect, self.header_bg_color);

            ctx.push_clip(row_header_rect);

            for row_header in &self.row_header_cells {
                row_header.widget.paint(ctx);

                // Draw grid lines
                if self.show_grid_lines {
                    let bounds = row_header.widget.bounds();
                    ctx.draw_line(
                        Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y),
                        Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
                        crate::paint::Stroke::new(self.grid_line_color, 1.0),
                    );
                    ctx.draw_line(
                        Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height),
                        Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
                        crate::paint::Stroke::new(self.grid_line_color, 1.0),
                    );
                }
            }

            ctx.pop_clip();

            // Draw corner cell (top-left, above row headers)
            let corner_rect = Rect::new(
                self.bounds.origin,
                Size::new(row_header_width, self.header_height),
            );
            ctx.draw_rect(corner_rect, self.header_bg_color);
        }

        // Paint scrollbars on top
        if let Some(ref vscroll) = self.vscrollbar {
            vscroll.paint(ctx);
        }
        if let Some(ref hscroll) = self.hscrollbar {
            hscroll.paint(ctx);
        }

        // Register hitbox
        ctx.register_hitbox(self.id, self.bounds);
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn dispatch_mouse_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::MouseMove(e) => MouseHandler::on_mouse_move(self, e),
            InputEventEnum::MouseDown(e) => MouseHandler::on_mouse_down(self, e),
            InputEventEnum::MouseUp(e) => MouseHandler::on_mouse_up(self, e),
            _ => EventResponse::Ignored,
        }
    }

    fn on_wheel(&mut self, event: &mut WheelEvent) -> EventResponse {
        eprintln!("[VirtualizedGrid] on_wheel: delta=({}, {})", event.delta.x, event.delta.y);

        let mut handled = false;

        // Vertical scrolling
        if event.delta.y.abs() > 0.1 {
            let new_offset = (self.scroll_offset_y + event.delta.y * 3.0)
                .clamp(0.0, self.max_scroll_y());

            if (new_offset - self.scroll_offset_y).abs() > 0.1 {
                self.scroll_offset_y = new_offset;

                if let Some(ref mut vscroll) = self.vscrollbar {
                    vscroll.set_value(new_offset as i32);
                }

                handled = true;
            }
        }

        // Horizontal scrolling
        if event.delta.x.abs() > 0.1 {
            let new_offset = (self.scroll_offset_x + event.delta.x * 3.0)
                .clamp(0.0, self.max_scroll_x());

            if (new_offset - self.scroll_offset_x).abs() > 0.1 {
                self.scroll_offset_x = new_offset;

                if let Some(ref mut hscroll) = self.hscrollbar {
                    hscroll.set_value(new_offset as i32);
                }

                handled = true;
            }
        }

        if handled {
            self.update_visible_cells();
            self.dirty = true;
            EventResponse::Handled
        } else {
            EventResponse::Ignored
        }
    }

    fn on_message(&mut self, message: &GuiMessage) -> Vec<DeferredCommand> {
        // Handle scrollbar value changes
        if let GuiMessage::Custom {
            signal_type,
            data,
            source,
        } = message
        {
            if signal_type == "value_changed" {
                // Vertical scrollbar
                if let Some(ref vscroll) = self.vscrollbar {
                    if *source == vscroll.id() {
                        if let Some(value) = data.downcast_ref::<i32>() {
                            self.scroll_offset_y = (*value).max(0) as f64;
                            self.update_visible_cells();
                            self.dirty = true;
                        }
                    }
                }

                // Horizontal scrollbar
                if let Some(ref hscroll) = self.hscrollbar {
                    if *source == hscroll.id() {
                        if let Some(value) = data.downcast_ref::<i32>() {
                            self.scroll_offset_x = (*value).max(0) as f64;
                            self.update_visible_cells();
                            self.dirty = true;
                        }
                    }
                }
            }
        }
        vec![]
    }

    fn drain_deferred_commands(&mut self) -> Vec<DeferredCommand> {
        std::mem::take(&mut self.pending_commands)
    }
}

// ========================================================================
// MouseHandler Trait Implementation
// ========================================================================

impl MouseHandler for VirtualizedGrid {
    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check vertical scrollbar first
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.is_dragging() || vscroll.bounds().contains(event.position) {
                let old_value = vscroll.value();
                let mut input_event = InputEventEnum::MouseMove(event.clone());
                let response = vscroll.dispatch_mouse_event(&mut input_event);

                let new_value = vscroll.value();
                if new_value != old_value {
                    self.scroll_offset_y = new_value.max(0) as f64;
                    self.update_visible_cells();
                    self.dirty = true;
                }

                return response;
            }
        }

        // Check horizontal scrollbar
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.is_dragging() || hscroll.bounds().contains(event.position) {
                let old_value = hscroll.value();
                let mut input_event = InputEventEnum::MouseMove(event.clone());
                let response = hscroll.dispatch_mouse_event(&mut input_event);

                let new_value = hscroll.value();
                if new_value != old_value {
                    self.scroll_offset_x = new_value.max(0) as f64;
                    self.update_visible_cells();
                    self.dirty = true;
                }

                return response;
            }
        }

        // Forward to cells
        for cell in &mut self.visible_cells {
            if cell.widget.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseMove(event.clone());
                let response = cell.widget.dispatch_mouse_event(&mut input_event);
                if response != EventResponse::Ignored {
                    return response;
                }
            }
        }

        EventResponse::PassThrough
    }

    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check vertical scrollbar
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.bounds().contains(event.position) {
                let old_value = vscroll.value();
                let mut input_event = InputEventEnum::MouseDown(event.clone());
                let response = vscroll.dispatch_mouse_event(&mut input_event);

                let new_value = vscroll.value();
                if new_value != old_value {
                    self.scroll_offset_y = new_value.max(0) as f64;
                    self.update_visible_cells();
                    self.dirty = true;
                }

                return response;
            }
        }

        // Check horizontal scrollbar
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.bounds().contains(event.position) {
                let old_value = hscroll.value();
                let mut input_event = InputEventEnum::MouseDown(event.clone());
                let response = hscroll.dispatch_mouse_event(&mut input_event);

                let new_value = hscroll.value();
                if new_value != old_value {
                    self.scroll_offset_x = new_value.max(0) as f64;
                    self.update_visible_cells();
                    self.dirty = true;
                }

                return response;
            }
        }

        // Forward to cells
        for cell in &mut self.visible_cells {
            if cell.widget.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseDown(event.clone());
                let response = cell.widget.dispatch_mouse_event(&mut input_event);
                if response != EventResponse::Ignored {
                    return response;
                }
            }
        }

        EventResponse::Ignored
    }

    fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Forward to scrollbars if dragging
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.is_dragging() || vscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseUp(event.clone());
                return vscroll.dispatch_mouse_event(&mut input_event);
            }
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.is_dragging() || hscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseUp(event.clone());
                return hscroll.dispatch_mouse_event(&mut input_event);
            }
        }

        // Forward to cells
        for cell in &mut self.visible_cells {
            if cell.widget.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseUp(event.clone());
                let response = cell.widget.dispatch_mouse_event(&mut input_event);
                if response != EventResponse::Ignored {
                    return response;
                }
            }
        }

        EventResponse::Ignored
    }

    fn on_mouse_enter(&mut self, _event: &mut MouseEvent) -> EventResponse {
        EventResponse::PassThrough
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        EventResponse::PassThrough
    }
}

impl Default for VirtualizedGrid {
    fn default() -> Self {
        Self::new()
    }
}
