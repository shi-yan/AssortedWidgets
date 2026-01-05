//! VirtualizedList widget implementation

use std::any::Any;
use std::ops::Range;

use crate::event::{EventResponse, InputEventEnum, MouseEvent, WheelEvent};
use crate::event::handlers::MouseHandler;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::types::{DirtyLevel, DeferredCommand, FrameInfo, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;
use crate::WidgetState;
use crate::widgets::ScrollBar;

use super::data_source::ListDataSource;

/// Layout mode for the virtualized list
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListLayoutMode {
    /// All items have the same fixed height (no prefix sum needed)
    FixedHeight(f64),
    /// Items have variable heights (uses prefix sum + binary search)
    VariableHeight,
}

/// VirtualizedList widget - efficient scrollable list with widget recycling
///
/// Only creates widgets for visible items, recycling them as the user scrolls.
/// Supports both fixed-height (optimized) and variable-height modes.
///
/// # Example
///
/// ```rust,ignore
/// let list = VirtualizedList::new()
///     .fixed_height(40.0)  // All items are 40px tall
///     .data_source(Box::new(MyDataSource::new()));
/// ```
pub struct VirtualizedList {
    // Standard widget fields
    state: WidgetState,
    layout_style: Style,

    // Layout mode
    layout_mode: ListLayoutMode,

    // Scrolling state
    scroll_offset: f64,
    total_content_height: f64,
    viewport_height: f64,

    // Scrollbar
    scrollbar: Option<ScrollBar>,
    scrollbar_width: f32,
    show_scrollbar: bool,

    // Data source
    data_source: Option<Box<dyn ListDataSource>>,

    // Widget pooling
    visible_cells: Vec<CellInfo>,
    recycled_cells: Vec<Box<dyn Widget>>,

    // Cached measurements (for variable height mode)
    item_heights: Vec<f64>,      // Height of each item
    item_offsets: Vec<f64>,      // Y offset of each item (prefix sum)

    // Background color
    bg_color: Option<Color>,

    // Pending commands
    pending_commands: Vec<DeferredCommand>,
}

/// Information about a visible cell
struct CellInfo {
    data_index: usize,           // Which data item this cell represents
    widget: Box<dyn Widget>,     // The actual widget
}

impl VirtualizedList {
    /// Create a new virtualized list with variable height mode
    pub fn new() -> Self {
        Self {
            state: WidgetState::new(),
            layout_style: Style {
                flex_grow: 1.0,
                flex_shrink: 1.0,
                ..Style::default()
            },
            layout_mode: ListLayoutMode::VariableHeight,
            scroll_offset: 0.0,
            total_content_height: 0.0,
            viewport_height: 0.0,
            scrollbar: None,
            scrollbar_width: 12.0,
            show_scrollbar: true,
            data_source: None,
            visible_cells: Vec::new(),
            recycled_cells: Vec::new(),
            item_heights: Vec::new(),
            item_offsets: Vec::new(),
            bg_color: None,
            pending_commands: Vec::new(),
        }
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set the data source
    pub fn data_source(mut self, source: Box<dyn ListDataSource>) -> Self {
        self.data_source = Some(source);
        self
    }

    /// Set a fixed height for all items (optimized mode, no prefix sum needed)
    pub fn fixed_height(mut self, height: f64) -> Self {
        self.layout_mode = ListLayoutMode::FixedHeight(height);
        self
    }

    /// Use variable heights (query from data source or measure widgets)
    pub fn variable_height(mut self) -> Self {
        self.layout_mode = ListLayoutMode::VariableHeight;
        self
    }

    /// Set the scrollbar width
    pub fn scrollbar_width(mut self, width: f32) -> Self {
        self.scrollbar_width = width;
        self
    }

    /// Enable or disable the scrollbar
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
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
    pub fn set_data_source(&mut self, source: Box<dyn ListDataSource>) {
        self.data_source = Some(source);
        self.invalidate_layout();
    }

    /// Reload all data from the data source
    pub fn reload_data(&mut self) {
        self.invalidate_layout();
    }

    /// Scroll to a specific item index
    pub fn scroll_to_item(&mut self, index: usize) {
        if let Some(offset) = self.offset_of_item(index) {
            self.scroll_offset = offset.clamp(0.0, self.max_scroll_offset());
            self.update_visible_cells();
            self.update_scrollbar();
            self.state.dirty = DirtyLevel::Visual;
        }
    }

    // ========================================================================
    // Internal Helpers - Layout Calculations
    // ========================================================================

    /// Invalidate cached layout and force recalculation
    fn invalidate_layout(&mut self) {
        self.item_heights.clear();
        self.item_offsets.clear();
        self.total_content_height = 0.0;

        // Move all visible cells to recycled pool
        for cell in self.visible_cells.drain(..) {
            self.recycled_cells.push(cell.widget);
        }

        self.state.dirty = DirtyLevel::Visual;
    }

    /// Calculate heights and offsets for all items
    fn calculate_layout(&mut self) {
        let Some(ref mut data_source) = self.data_source else {
            return;
        };

        let item_count = data_source.item_count();

        match self.layout_mode {
            ListLayoutMode::FixedHeight(height) => {
                // No need to store individual heights or offsets
                // We can calculate them on-the-fly: offset = index * height
                self.total_content_height = item_count as f64 * height;
            }
            ListLayoutMode::VariableHeight => {
                // Calculate heights and build prefix sum array
                self.item_heights.clear();
                self.item_offsets.clear();

                let mut offset = 0.0;
                for i in 0..item_count {
                    // Get height from data source or use default
                    let height = data_source
                        .item_height(i)
                        .unwrap_or(40.0);

                    self.item_heights.push(height);
                    self.item_offsets.push(offset);
                    offset += height;
                }

                self.total_content_height = offset;
            }
        }

        eprintln!("[VirtualizedList] Layout calculated: {} items, total_height = {}",
                  item_count, self.total_content_height);
    }

    /// Get the Y offset of an item (O(1) for both modes)
    fn offset_of_item(&self, index: usize) -> Option<f64> {
        match self.layout_mode {
            ListLayoutMode::FixedHeight(height) => {
                Some(index as f64 * height)
            }
            ListLayoutMode::VariableHeight => {
                self.item_offsets.get(index).copied()
            }
        }
    }

    /// Get the height of an item (O(1) for both modes)
    fn height_of_item(&self, index: usize) -> Option<f64> {
        match self.layout_mode {
            ListLayoutMode::FixedHeight(height) => {
                Some(height)
            }
            ListLayoutMode::VariableHeight => {
                self.item_heights.get(index).copied()
            }
        }
    }

    /// Calculate which items are currently visible using binary search
    ///
    /// Returns the range of data indices that should be visible.
    /// Uses binary search on the prefix sum array for O(log n) complexity.
    fn calculate_visible_range(&self) -> Range<usize> {
        let Some(ref data_source) = self.data_source else {
            return 0..0;
        };

        let item_count = data_source.item_count();
        if item_count == 0 {
            return 0..0;
        }

        let viewport_start = self.scroll_offset;
        let viewport_end = viewport_start + self.viewport_height;

        match self.layout_mode {
            ListLayoutMode::FixedHeight(height) => {
                // Simple arithmetic for fixed height
                let start = (viewport_start / height).floor() as usize;
                let end = ((viewport_end / height).ceil() as usize).min(item_count);
                start..end
            }
            ListLayoutMode::VariableHeight => {
                // Binary search on prefix sum array
                if self.item_offsets.is_empty() {
                    return 0..0;
                }

                // Find first visible item (first item whose bottom edge is >= viewport_start)
                let start = self.item_offsets.partition_point(|&offset| {
                    // Check if this item's bottom edge is before viewport start
                    let item_idx = self.item_offsets.iter().position(|&o| o == offset).unwrap();
                    let item_bottom = offset + self.item_heights.get(item_idx).copied().unwrap_or(0.0);
                    item_bottom <= viewport_start
                });

                // Find last visible item (first item whose top edge is >= viewport_end)
                let end = self.item_offsets.partition_point(|&offset| {
                    offset < viewport_end
                }).min(item_count);

                start..end
            }
        }
    }

    /// Maximum scroll offset
    fn max_scroll_offset(&self) -> f64 {
        (self.total_content_height - self.viewport_height).max(0.0)
    }

    // ========================================================================
    // Internal Helpers - Widget Management
    // ========================================================================

    /// Update which cells are visible based on current scroll position
    fn update_visible_cells(&mut self) {
        // Calculate visible range first (before borrowing data_source mutably)
        let visible_range = self.calculate_visible_range();

        let Some(ref mut data_source) = self.data_source else {
            return;
        };

        eprintln!("[VirtualizedList] Visible range: {:?} (scroll_offset = {}, viewport_height = {})",
                  visible_range, self.scroll_offset, self.viewport_height);

        // Collect currently visible data indices
        let current_visible: Vec<usize> = self.visible_cells
            .iter()
            .map(|cell| cell.data_index)
            .collect();

        // Determine which cells to recycle and which to keep
        let mut cells_to_keep = Vec::new();
        for cell in self.visible_cells.drain(..) {
            if visible_range.contains(&cell.data_index) {
                cells_to_keep.push(cell);
            } else {
                // Recycle this widget
                self.recycled_cells.push(cell.widget);
            }
        }

        // Create new cells for items that weren't previously visible
        for data_index in visible_range {
            // Check if we already have this cell
            if !cells_to_keep.iter().any(|c| c.data_index == data_index) {
                // Need to create/reuse a cell
                let reused = self.recycled_cells.pop();
                let widget = data_source.widget_for_item(data_index, reused);

                cells_to_keep.push(CellInfo {
                    data_index,
                    widget,
                });
            }
        }

        // Sort by data_index for consistent ordering
        cells_to_keep.sort_by_key(|c| c.data_index);

        self.visible_cells = cells_to_keep;

        eprintln!("[VirtualizedList] Now showing {} cells, {} recycled",
                  self.visible_cells.len(), self.recycled_cells.len());

        // Layout the visible cells
        self.layout_visible_cells();
    }

    /// Position all visible cells based on their data indices
    fn layout_visible_cells(&mut self) {
        let content_width = self.state.bounds.size.width
            - if self.scrollbar.is_some() {
                self.scrollbar_width as f64
            } else {
                0.0
            };

        // Collect the layout data before mutably iterating
        let layout_data: Vec<(usize, f64, f64)> = self.visible_cells
            .iter()
            .filter_map(|cell| {
                let offset = self.offset_of_item(cell.data_index)?;
                let height = self.height_of_item(cell.data_index).unwrap_or(40.0);
                Some((cell.data_index, offset, height))
            })
            .collect();

        // Now apply the layout
        for (i, cell) in self.visible_cells.iter_mut().enumerate() {
            if let Some(&(_, offset, height)) = layout_data.get(i) {
                let y = self.state.bounds.origin.y + (offset - self.scroll_offset);

                cell.widget.set_bounds(Rect::new(
                    Point::new(self.state.bounds.origin.x, y),
                    Size::new(content_width, height),
                ));
            }
        }
    }

    // ========================================================================
    // Internal Helpers - Scrollbar
    // ========================================================================

    /// Update scrollbar state
    fn update_scrollbar(&mut self) {
        if !self.show_scrollbar {
            self.scrollbar = None;
            return;
        }

        let needs_scrollbar = self.total_content_height > self.viewport_height;

        if needs_scrollbar {
            let max_scroll = self.max_scroll_offset() as i32;
            let page_size = self.viewport_height as i32;

            if let Some(ref mut scrollbar) = self.scrollbar {
                // Update existing scrollbar
                scrollbar.set_range(0, max_scroll.max(1));
                scrollbar.set_page_size(page_size.max(1));
                scrollbar.set_value(self.scroll_offset as i32);
            } else {
                // Create new scrollbar
                let mut scrollbar = ScrollBar::vertical(0, max_scroll.max(1), page_size.max(1));
                scrollbar.set_value(self.scroll_offset as i32);
                self.scrollbar = Some(scrollbar);
            }

            // Position scrollbar
            self.position_scrollbar();
        } else {
            self.scrollbar = None;
        }
    }

    /// Position scrollbar on the right edge
    fn position_scrollbar(&mut self) {
        if let Some(ref mut scrollbar) = self.scrollbar {
            scrollbar.set_bounds(Rect::new(
                Point::new(
                    self.state.bounds.origin.x + self.state.bounds.size.width - self.scrollbar_width as f64,
                    self.state.bounds.origin.y,
                ),
                Size::new(
                    self.scrollbar_width as f64,
                    self.state.bounds.size.height,
                ),
            ));
        }
    }
}

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for VirtualizedList {
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

    fn update(&mut self, frame_info: &FrameInfo) {
        // Update scrollbar
        if let Some(ref mut scrollbar) = self.scrollbar {
            scrollbar.update(frame_info);
        }

        // Update visible cells
        for cell in &mut self.visible_cells {
            cell.widget.update(frame_info);
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw background
        if let Some(bg_color) = self.bg_color {
            ctx.draw_rect(self.state.bounds, bg_color);
        }

        // Calculate content area (excluding scrollbar)
        let content_rect = Rect::new(
            self.state.bounds.origin,
            Size::new(
                self.state.bounds.size.width
                    - if self.scrollbar.is_some() {
                        self.scrollbar_width as f64
                    } else {
                        0.0
                    },
                self.state.bounds.size.height,
            ),
        );

        // Clip to content area
        ctx.push_clip(content_rect);

        // Paint visible cells
        for cell in &self.visible_cells {
            cell.widget.paint(ctx);
        }

        ctx.pop_clip();

        // Paint scrollbar on top
        if let Some(ref scrollbar) = self.scrollbar {
            scrollbar.paint(ctx);
        }

        // Register hitbox
        ctx.register_hitbox(self.state.id, self.state.bounds);
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
        eprintln!("[VirtualizedList] on_wheel: delta.y = {}", event.delta.y);

        let new_offset = (self.scroll_offset + event.delta.y * 3.0)
            .clamp(0.0, self.max_scroll_offset());

        if (new_offset - self.scroll_offset).abs() > 0.1 {
            self.scroll_offset = new_offset;

            // Update scrollbar
            if let Some(ref mut scrollbar) = self.scrollbar {
                scrollbar.set_value(new_offset as i32);
            }

            // Update visible cells
            self.update_visible_cells();

            self.state.dirty = DirtyLevel::Visual;
        }

        EventResponse::Handled
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
                if let Some(ref scrollbar) = self.scrollbar {
                    if *source == scrollbar.id() {
                        if let Some(value) = data.downcast_ref::<i32>() {
                            self.scroll_offset = (*value).max(0) as f64;
                            self.update_visible_cells();
                            self.state.dirty = DirtyLevel::Visual;
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

impl MouseHandler for VirtualizedList {
    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check scrollbar first (higher z-order)
        if let Some(ref mut scrollbar) = self.scrollbar {
            let should_forward = scrollbar.is_dragging() || scrollbar.bounds().contains(event.position);

            if should_forward {
                let old_value = scrollbar.value();
                let mut input_event = InputEventEnum::MouseMove(event.clone());
                let response = scrollbar.dispatch_mouse_event(&mut input_event);

                // Check if scrollbar value changed
                let new_value = scrollbar.value();
                if new_value != old_value {
                    self.scroll_offset = new_value.max(0) as f64;
                    self.update_visible_cells();
                    self.state.dirty = DirtyLevel::Visual;
                }

                return response;
            }
        }

        // Forward to visible cells
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
        // Check scrollbar first
        if let Some(ref mut scrollbar) = self.scrollbar {
            if scrollbar.bounds().contains(event.position) {
                let old_value = scrollbar.value();
                let mut input_event = InputEventEnum::MouseDown(event.clone());
                let response = scrollbar.dispatch_mouse_event(&mut input_event);

                // Check if scrollbar value changed
                let new_value = scrollbar.value();
                if new_value != old_value {
                    self.scroll_offset = new_value.max(0) as f64;
                    self.update_visible_cells();
                    self.state.dirty = DirtyLevel::Visual;
                }

                return response;
            }
        }

        // Forward to visible cells
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
        // Forward to scrollbar if dragging
        if let Some(ref mut scrollbar) = self.scrollbar {
            if scrollbar.is_dragging() || scrollbar.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseUp(event.clone());
                return scrollbar.dispatch_mouse_event(&mut input_event);
            }
        }

        // Forward to visible cells
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

impl Default for VirtualizedList {
    fn default() -> Self {
        Self::new()
    }
}
