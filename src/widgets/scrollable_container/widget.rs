//! ScrollableContainer - A container widget with automatic scrolling support
//!
//! Features:
//! - Vertical, horizontal, or bidirectional scrolling
//! - Automatic content size measurement from children
//! - On-demand scrollbars (only shown when content exceeds viewport)
//! - Smooth wheel scrolling with offset clamping
//! - Hierarchical hit testing with coordinate transformation
//!
//! # Example
//! ```rust,ignore
//! let gui_handle = window.gui_handle();
//! let (scroll_container, scroll_children) = ScrollableContainer::new(
//!     ScrollMode::Vertical,
//!     &gui_handle
//! );
//!
//! let scroll_id = window.add_composite(
//!     Box::new(scroll_container),
//!     scroll_style,
//!     None,
//!     scroll_children
//! )?;
//!
//! let content_id = window.scrollable_container_content_id(scroll_id)?;
//! window.add_child(Box::new(label), label_style, content_id)?;
//! ```

use std::any::Any;
use std::cell::Cell;
use std::rc::Rc;
use crate::event::input::{EventResponse, MouseEvent, WheelEvent};
use crate::event::handlers::MouseHandler;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;
use crate::types::{DeferredCommand, DirtyLevel, GuiMessage, Point, Rect, Size, Vector, WidgetId};
use crate::widget::Widget;
use crate::widgets::scrollable_container::viewport::ScrollViewport;
use crate::widgets::scrollable_container::viewport::ScrollMode;
use crate::widgets::ScrollBar;
use crate::WidgetState;


/// ScrollableContainer - A container that supports scrolling with automatic content sizing
///
/// This is a composite widget that manages:
/// - A content container (holds actual child widgets)
/// - Optional vertical/horizontal scrollbars
/// - Scroll offset tracking
/// - Content size measurement
///
/// # Architecture
///
/// The ScrollableContainer uses hierarchical hit testing and rendering:
/// - `transform_point_for_children()`: Transforms mouse coordinates by scroll offset
/// - `paint()`: Pushes clip rect and offset for children
/// - Content size is automatically measured from children bounds after layout
///
/// # Widget Hierarchy
///
/// ```text
/// ScrollableContainer
/// ├── content_container (holds user's widgets)
/// ├── vertical_scrollbar (optional)
/// └── horizontal_scrollbar (optional)
/// ```
pub struct ScrollableContainer {
    // ========================================
    // Widget Essentials
    // ========================================
    state: WidgetState,
    layout_style: Style,

    // ========================================
    // Scroll Configuration
    // ========================================
    /// Scrolling mode (Vertical, Horizontal, Both)
    scroll_mode: ScrollMode,

    // ========================================
    // Scroll State
    // ========================================
    /// Current scroll position (offset from content origin)
    /// Positive values mean content is scrolled down/right
    /// Wrapped in Rc<Cell<>> to share with ScrollViewport without borrowing
    scroll_offset: Rc<Cell<Vector>>,

    /// Total size of content (measured from children bounds)
    /// Updated after layout pass
    content_size: Size,

    /// Viewport size (visible area = bounds minus scrollbars)
    viewport_size: Size,

    // ========================================
    // Child Widget IDs
    // ========================================
    /// Viewport widget ID (applies scroll offset and clipping)
    viewport_id: Option<WidgetId>,

    /// Content container ID (holds actual child widgets)
    content_container_id: Option<WidgetId>,

    // ========================================
    // Embedded Scrollbars (owned directly, like TextArea)
    // ========================================
    /// Vertical scrollbar (created on-demand when content overflows)
    vertical_scrollbar: Option<ScrollBar>,

    /// Horizontal scrollbar (created on-demand when content overflows)
    horizontal_scrollbar: Option<ScrollBar>,

    // ========================================
    // Constants
    // ========================================
    /// Width of vertical scrollbar (in pixels)
    scrollbar_width: f32,

    /// Height of horizontal scrollbar (in pixels)
    scrollbar_height: f32,

    // ========================================
    // GUI Handle
    // ========================================
    /// Handle for emitting signals
    gui_handle: crate::handle::GuiHandle,
}

impl ScrollableContainer {
    /// Scrollbar width constant (vertical scrollbar thickness)
    const SCROLLBAR_WIDTH: f32 = 16.0;

    /// Scrollbar height constant (horizontal scrollbar thickness)
    const SCROLLBAR_HEIGHT: f32 = 16.0;

    /// Create a new ScrollableContainer
    ///
    /// This returns the container widget and a vector of child widgets that need to be
    /// added to the widget tree via `Window::add_composite()`.
    ///
    /// # Arguments
    ///
    /// * `scroll_mode` - Vertical, Horizontal, or Both
    /// * `gui_handle` - Handle to generate widget IDs
    ///
    /// # Returns
    ///
    /// A tuple of:
    /// - `Self`: The ScrollableContainer widget
    /// - `Vec<(Box<dyn Widget>, Style, Option<usize>)>`: Child widgets to add
    ///   Each tuple is (widget, layout_style, parent_index)
    ///   parent_index is None for children of the ScrollableContainer itself,
    ///   or Some(n) to make this widget a child of the nth child
    pub fn new(
        scroll_mode: ScrollMode,
        gui_handle: &crate::handle::GuiHandle,
    ) -> (Self, Vec<(Box<dyn Widget>, Style, Option<usize>)>) {
        use crate::elements::Container;

        // Pre-allocate widget IDs (only for viewport and content container)
        let viewport_id = gui_handle.next_widget_id();
        let content_container_id = gui_handle.next_widget_id();

        // Create shared scroll offset (shared between ScrollableContainer and ScrollViewport)
        let scroll_offset = Rc::new(Cell::new(Vector::new(0.0, 0.0)));

        // Create the ScrollableContainer itself
        // Scrollbars will be created on-demand in update_scrollbar_visibility()
        let container = ScrollableContainer {
            state: WidgetState::new(),
            layout_style: Style::default(),
            scroll_mode,
            scroll_offset: scroll_offset.clone(),
            content_size: Size::new(0.0, 0.0),
            viewport_size: Size::new(0.0, 0.0),
            viewport_id: Some(viewport_id),
            content_container_id: Some(content_container_id),
            vertical_scrollbar: None,  // Created on-demand when content overflows
            horizontal_scrollbar: None,
            scrollbar_width: Self::SCROLLBAR_WIDTH,
            scrollbar_height: Self::SCROLLBAR_HEIGHT,
            gui_handle: gui_handle.clone(),
        };

        // Create child widgets (only viewport and content container)
        let mut children = Vec::new();

        // Viewport widget - applies scroll offset to content only (not scrollbars)
        let viewport_style = Style {
            display: taffy::Display::Flex,
            size: taffy::Size {
                width: taffy::Dimension::percent(1.0),
                height: taffy::Dimension::percent(1.0),
            },
            ..Default::default()
        };
        let mut viewport = Box::new(ScrollViewport::new(scroll_offset.clone(), viewport_style.clone()));
        viewport.set_id(viewport_id);
        children.push((viewport as Box<dyn Widget>, viewport_style, None));

        // Content container - uses flexbox column layout for stacking content
        // This will be a child of the viewport (index 0), not the ScrollableContainer
        // Using index-based parent reference: Some(0) = viewport
        // Width is 100% to constrain children for text wrapping
        let content_style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            size: taffy::Size {
                width: taffy::Dimension::percent(1.0),  // 100% width for text wrapping
                height: taffy::Dimension::auto(),
            },
            ..Default::default()
        };
        let mut content_container = Box::new(Container::new(content_style.clone()));
        content_container.set_id(content_container_id);
        // parent_index = Some(0) means "child of viewport" (viewport is at index 0)
        children.push((content_container as Box<dyn Widget>, content_style, Some(0)));

        // Note: Scrollbars are NOT created here as child widgets
        // They will be created on-demand as owned Option<ScrollBar> fields
        // when content overflows (see update_scrollbar_visibility)

        (container, children)
    }

    /// Get the viewport widget ID (internal, used by Window)
    pub(crate) fn viewport_id(&self) -> Option<WidgetId> {
        self.viewport_id
    }

    /// Get the content container ID
    ///
    /// This is the widget ID where user content should be added.
    /// Note: This returns the stored ID which is set during initialization.
    /// The actual ID is assigned by Window::add_composite.
    pub fn content_container_id(&self) -> Option<WidgetId> {
        self.content_container_id
    }

    /// Get the content size (internal, used by Window)
    pub(crate) fn content_size(&self) -> Size {
        self.content_size
    }

    /// Get the viewport size (internal, used by Window)
    pub(crate) fn viewport_size(&self) -> Size {
        self.viewport_size
    }

    /// Get the scroll offset (internal, used by Window)
    pub(crate) fn scroll_offset(&self) -> Vector {
        self.scroll_offset.get()
    }

    /// Set the viewport widget ID (called after add_composite)
    pub(crate) fn set_viewport_id(&mut self, id: WidgetId) {
        self.viewport_id = Some(id);
    }

    /// Set the content container ID (called after add_composite)
    pub(crate) fn set_content_container_id(&mut self, id: WidgetId) {
        self.content_container_id = Some(id);
    }

    /// Update content size from content container bounds (called after layout)
    pub(crate) fn update_content_size(&mut self, content_bounds: Size) {
        println!("[SCROLLABLE] update_content_size called: content_bounds = {:?}, current content_size = {:?}",
                 content_bounds, self.content_size);
        if self.content_size != content_bounds {
            println!("[SCROLLABLE]   Content size changed, updating scrollbars...");
            self.content_size = content_bounds;
            self.update_scrollbar_visibility(); // Creates/destroys scrollbars and updates ranges
            self.clamp_scroll_offset();
            self.state.dirty = DirtyLevel::Visual; // Content size affects scrollbar visibility
        }
    }

    /// Update scrollbar visibility based on content size vs viewport size
    /// Update scrollbar visibility and ranges based on content overflow
    /// Creates/destroys scrollbars on-demand (overflow:auto behavior)
    fn update_scrollbar_visibility(&mut self) {
        println!("[SCROLLABLE] update_scrollbar_visibility called:");
        println!("[SCROLLABLE]   content_size = {:?}, viewport_size = {:?}",
                 self.content_size, self.viewport_size);

        // Track initial scrollbar state to detect changes
        let had_vscroll = self.vertical_scrollbar.is_some();
        let had_hscroll = self.horizontal_scrollbar.is_some();

        // Determine if scrollbars are needed based on scroll mode and content overflow
        let needs_vscroll = matches!(self.scroll_mode, ScrollMode::Vertical | ScrollMode::Both)
            && self.content_size.height > self.viewport_size.height;
        let needs_hscroll = matches!(self.scroll_mode, ScrollMode::Horizontal | ScrollMode::Both)
            && self.content_size.width > self.viewport_size.width;

        println!("[SCROLLABLE]   needs_vscroll = {}, needs_hscroll = {}", needs_vscroll, needs_hscroll);

        // Create or destroy vertical scrollbar
        if needs_vscroll {
            let max_y = (self.content_size.height - self.viewport_size.height).max(0.0) as i32;
            let page_y = self.viewport_size.height as i32;

            if self.vertical_scrollbar.is_none() {
                // Create new scrollbar
                println!("[SCROLLABLE]   Creating vertical scrollbar: range 0-{}, page {}", max_y, page_y);
                let mut vscroll = ScrollBar::vertical(0, max_y, page_y)
                    .width(self.scrollbar_width);
                vscroll.set_id(self.state.id); // Use container's ID temporarily
                self.vertical_scrollbar = Some(vscroll);
            } else {
                // Update existing scrollbar range
                println!("[SCROLLABLE]   Updating vertical scrollbar range");
                if let Some(ref mut vscroll) = self.vertical_scrollbar {
                    vscroll.set_range(0, max_y);
                    vscroll.set_page_size(page_y);
                }
            }
        } else {
            if self.vertical_scrollbar.is_some() {
                println!("[SCROLLABLE]   Destroying vertical scrollbar");
            }
            self.vertical_scrollbar = None;
        }

        // Create or destroy horizontal scrollbar
        if needs_hscroll {
            let max_x = (self.content_size.width - self.viewport_size.width).max(0.0) as i32;
            let page_x = self.viewport_size.width as i32;

            if self.horizontal_scrollbar.is_none() {
                // Create new scrollbar
                let mut hscroll = ScrollBar::horizontal(0, max_x, page_x)
                    .width(self.scrollbar_height);
                hscroll.set_id(self.state.id); // Use container's ID temporarily
                self.horizontal_scrollbar = Some(hscroll);
            } else {
                // Update existing scrollbar range
                if let Some(ref mut hscroll) = self.horizontal_scrollbar {
                    hscroll.set_range(0, max_x);
                    hscroll.set_page_size(page_x);
                }
            }
        } else {
            self.horizontal_scrollbar = None;
        }

        // If scrollbar state changed, recalculate viewport and check again
        // This handles the case where adding one scrollbar causes the need for another
        let scrollbar_changed = (had_vscroll != self.vertical_scrollbar.is_some())
                             || (had_hscroll != self.horizontal_scrollbar.is_some());

        println!("[SCROLLABLE]   scrollbar_changed = {} (vscroll: {} -> {}, hscroll: {} -> {})",
                 scrollbar_changed, had_vscroll, self.vertical_scrollbar.is_some(),
                 had_hscroll, self.horizontal_scrollbar.is_some());

        if scrollbar_changed {
            println!("[SCROLLABLE]   Scrollbar state changed, recalculating viewport...");
            // Recalculate viewport size with new scrollbar configuration
            self.viewport_size = self.calculate_viewport_size();

            // Position scrollbars with updated sizes
            self.position_scrollbars();

            // Check one more time if we need to add/remove scrollbars with new viewport
            let needs_vscroll_again = matches!(self.scroll_mode, ScrollMode::Vertical | ScrollMode::Both)
                && self.content_size.height > self.viewport_size.height;
            let needs_hscroll_again = matches!(self.scroll_mode, ScrollMode::Horizontal | ScrollMode::Both)
                && self.content_size.width > self.viewport_size.width;

            // Update vertical scrollbar if needed
            if needs_vscroll_again && self.vertical_scrollbar.is_none() {
                let max_y = (self.content_size.height - self.viewport_size.height).max(0.0) as i32;
                let page_y = self.viewport_size.height as i32;
                let mut vscroll = ScrollBar::vertical(0, max_y, page_y)
                    .width(self.scrollbar_width);
                vscroll.set_id(self.state.id);
                self.vertical_scrollbar = Some(vscroll);
            } else if !needs_vscroll_again && self.vertical_scrollbar.is_some() {
                self.vertical_scrollbar = None;
            } else if let Some(ref mut vscroll) = self.vertical_scrollbar {
                // Update range with new viewport size
                let max_y = (self.content_size.height - self.viewport_size.height).max(0.0) as i32;
                let page_y = self.viewport_size.height as i32;
                vscroll.set_range(0, max_y);
                vscroll.set_page_size(page_y);
            }

            // Update horizontal scrollbar if needed
            if needs_hscroll_again && self.horizontal_scrollbar.is_none() {
                let max_x = (self.content_size.width - self.viewport_size.width).max(0.0) as i32;
                let page_x = self.viewport_size.width as i32;
                let mut hscroll = ScrollBar::horizontal(0, max_x, page_x)
                    .width(self.scrollbar_height);
                hscroll.set_id(self.state.id);
                self.horizontal_scrollbar = Some(hscroll);
            } else if !needs_hscroll_again && self.horizontal_scrollbar.is_some() {
                self.horizontal_scrollbar = None;
            } else if let Some(ref mut hscroll) = self.horizontal_scrollbar {
                // Update range with new viewport size
                let max_x = (self.content_size.width - self.viewport_size.width).max(0.0) as i32;
                let page_x = self.viewport_size.width as i32;
                hscroll.set_range(0, max_x);
                hscroll.set_page_size(page_x);
            }

            // Final viewport and position update
            self.viewport_size = self.calculate_viewport_size();
            self.position_scrollbars();

            // Emit signal to resize viewport widget to account for scrollbars
            if let Some(_viewport_id) = self.viewport_id {
                let viewport_bounds = Rect::new(
                    Point::new(self.state.bounds.origin.x, self.state.bounds.origin.y),
                    self.viewport_size,
                );
                self.gui_handle.emit(
                    self.state.id,
                    "viewport_resize".to_string(),
                    Box::new(viewport_bounds),
                );
            }
        }
    }

    /// Calculate viewport size (bounds minus scrollbars)
    fn calculate_viewport_size(&self) -> Size {
        let mut size = self.state.bounds.size;

        // Subtract scrollbar dimensions if they exist
        if self.vertical_scrollbar.is_some() {
            size.width -= self.scrollbar_width as f64;
        }

        if self.horizontal_scrollbar.is_some() {
            size.height -= self.scrollbar_height as f64;
        }

        size
    }

    /// Position scrollbars within the container bounds
    fn position_scrollbars(&mut self) {
        let hscroll_height = if self.horizontal_scrollbar.is_some() {
            self.scrollbar_height as f64
        } else {
            0.0
        };

        // Position vertical scrollbar on right edge
        if let Some(ref mut vscroll) = self.vertical_scrollbar {
            vscroll.set_bounds(Rect::new(
                Point::new(
                    self.state.bounds.origin.x + self.state.bounds.size.width - self.scrollbar_width as f64,
                    self.state.bounds.origin.y,
                ),
                Size::new(
                    self.scrollbar_width as f64,
                    self.state.bounds.size.height - hscroll_height,
                ),
            ));
        }

        // Position horizontal scrollbar on bottom edge
        let vscroll_width = if self.vertical_scrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        if let Some(ref mut hscroll) = self.horizontal_scrollbar {
            hscroll.set_bounds(Rect::new(
                Point::new(
                    self.state.bounds.origin.x,
                    self.state.bounds.origin.y + self.state.bounds.size.height - self.scrollbar_height as f64,
                ),
                Size::new(
                    self.state.bounds.size.width - vscroll_width,
                    self.scrollbar_height as f64,
                ),
            ));
        }
    }

    /// Clamp scroll offset to valid range
    fn clamp_scroll_offset(&mut self) {
        let max_x = (self.content_size.width - self.viewport_size.width).max(0.0);
        let max_y = (self.content_size.height - self.viewport_size.height).max(0.0);

        let mut offset = self.scroll_offset.get();
        offset.x = offset.x.clamp(0.0, max_x);
        offset.y = offset.y.clamp(0.0, max_y);
        self.scroll_offset.set(offset);
    }
}

// ============================================================================
// Widget Trait Implementation
// ============================================================================

impl Widget for ScrollableContainer {
    fn id(&self) -> WidgetId {
        self.state.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.state.id = id;
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        // Scrollbars are now owned directly, no message-based communication needed
        // Scrollbar value changes are handled in update() by checking scrollbar values
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.state.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        if self.state.bounds != bounds {
            println!("[SCROLLABLE] set_bounds called: bounds = {:?}", bounds);
            self.state.bounds = bounds;
            self.viewport_size = self.calculate_viewport_size();
            println!("[SCROLLABLE]   calculated viewport_size = {:?}", self.viewport_size);
            self.update_scrollbar_visibility(); // Creates/destroys scrollbars and updates ranges
            self.position_scrollbars(); // Position scrollbars within new bounds
            self.clamp_scroll_offset();
            self.state.dirty = DirtyLevel::Visual; // Bounds changed (set by layout system)
        }
    }

    fn dirty_level(&self) -> DirtyLevel {
        self.state.dirty
    }

    fn set_dirty_level(&mut self, level: DirtyLevel) {
        self.state.dirty = level;
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn paint(&self, _ctx: &mut PaintContext) {
        // Nothing to paint for the container itself
        // Scrollbars are painted in after_paint_children() to render on top
    }

    fn before_paint_children(&self, _ctx: &mut PaintContext) {
        // Clipping is now handled by ScrollViewport to ensure correct coordinate space
        // See ScrollViewport::before_paint_children()
    }

    fn after_paint_children(&self, ctx: &mut PaintContext) {
        // Paint scrollbars on top of content (owned directly, not as child widgets)
        if let Some(ref vscroll) = self.vertical_scrollbar {
            vscroll.paint(ctx);
        }
        if let Some(ref hscroll) = self.horizontal_scrollbar {
            hscroll.paint(ctx);
        }
    }

    /// Transform point from viewport space to content space
    ///
    /// This is critical for hierarchical hit testing. When the user clicks at (50, 50)
    /// in the viewport, but the content is scrolled down by 200px, we need to transform
    /// that to (50, 250) in content space so hit testing finds the correct widget.
    fn transform_point_for_children(&self, point: Point) -> Point {
        let offset = self.scroll_offset.get();
        Point::new(
            point.x + offset.x,
            point.y + offset.y,
        )
    }

    fn drain_deferred_commands(&mut self) -> Vec<DeferredCommand> {
        // No deferred commands - using signal/slot system instead
        Vec::new()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    // Mark as interactive to receive wheel events for scrolling
    fn is_interactive(&self) -> bool {
        true
    }

    fn update(&mut self, frame_info: &crate::types::FrameInfo) {
        // Check if scrollbar values changed (user dragged scrollbar)
        let old_offset = self.scroll_offset.get();
        let mut new_offset = old_offset;
        let mut changed = false;

        if let Some(ref vscroll) = self.vertical_scrollbar {
            let scrollbar_value = vscroll.value() as f64;
            if (scrollbar_value - old_offset.y).abs() > 0.1 {
                new_offset.y = scrollbar_value;
                changed = true;
            }
        }

        if let Some(ref hscroll) = self.horizontal_scrollbar {
            let scrollbar_value = hscroll.value() as f64;
            if (scrollbar_value - old_offset.x).abs() > 0.1 {
                new_offset.x = scrollbar_value;
                changed = true;
            }
        }

        if changed {
            self.scroll_offset.set(new_offset);
            self.clamp_scroll_offset();
            self.state.dirty = DirtyLevel::Visual;
        }

        // Update scrollbar widgets
        if let Some(ref mut vscroll) = self.vertical_scrollbar {
            vscroll.update(frame_info);
        }
        if let Some(ref mut hscroll) = self.horizontal_scrollbar {
            hscroll.update(frame_info);
        }
    }

    // Handle wheel events for scrolling
    fn on_wheel(&mut self, event: &mut WheelEvent) -> EventResponse {
        let old_offset = self.scroll_offset.get();

        println!("[SCROLLABLE] on_wheel called! delta: ({:.1}, {:.1})", event.delta.x, event.delta.y);
        println!("[SCROLLABLE]   current offset: ({:.1}, {:.1})", old_offset.x, old_offset.y);
        println!("[SCROLLABLE]   content_size: ({:.1}, {:.1})", self.content_size.width, self.content_size.height);
        println!("[SCROLLABLE]   viewport_size: ({:.1}, {:.1})", self.viewport_size.width, self.viewport_size.height);

        let mut new_offset = old_offset;

        // Update scroll offset based on wheel delta
        match self.scroll_mode {
            ScrollMode::Vertical => {
                new_offset.y += event.delta.y;
                println!("[SCROLLABLE]   vertical scroll: {:.1} -> {:.1}", old_offset.y, new_offset.y);
            }
            ScrollMode::Horizontal => {
                new_offset.x += event.delta.x;
                println!("[SCROLLABLE]   horizontal scroll: {:.1} -> {:.1}", old_offset.x, new_offset.x);
            }
            ScrollMode::Both => {
                new_offset.x += event.delta.x;
                new_offset.y += event.delta.y;
                println!("[SCROLLABLE]   both scroll: ({:.1}, {:.1}) -> ({:.1}, {:.1})",
                         old_offset.x, old_offset.y, new_offset.x, new_offset.y);
            }
        }

        // Update the cell and clamp
        self.scroll_offset.set(new_offset);
        self.clamp_scroll_offset();
        let final_offset = self.scroll_offset.get();

        println!("[SCROLLABLE]   after clamp: ({:.1}, {:.1})", final_offset.x, final_offset.y);

        // If scroll offset changed, mark dirty and handle the event
        if final_offset != old_offset {
            self.state.dirty = DirtyLevel::Visual; // Scroll position change (visual only)

            // Update scrollbar visual position directly
            if let Some(ref mut vscroll) = self.vertical_scrollbar {
                println!("[SCROLLABLE]   Setting vertical scrollbar value: {}", final_offset.y as i32);
                vscroll.set_value(final_offset.y as i32);
            }

            if let Some(ref mut hscroll) = self.horizontal_scrollbar {
                println!("[SCROLLABLE]   Setting horizontal scrollbar value: {}", final_offset.x as i32);
                hscroll.set_value(final_offset.x as i32);
            }

            println!("[SCROLLABLE] ✓ Scroll offset changed, returning Handled");
            EventResponse::Handled
        } else {
            println!("[SCROLLABLE] ✗ Scroll offset unchanged (clamped), returning Ignored");
            EventResponse::Ignored
        }
    }
}

// Implement MouseHandler trait for scrollbar interaction
impl MouseHandler for ScrollableContainer {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check if mouse is over vertical scrollbar
        if let Some(ref mut vscroll) = self.vertical_scrollbar {
            if vscroll.bounds().contains(event.position) {
                return vscroll.on_mouse_down(event);
            }
        }

        // Check if mouse is over horizontal scrollbar
        if let Some(ref mut hscroll) = self.horizontal_scrollbar {
            if hscroll.bounds().contains(event.position) {
                return hscroll.on_mouse_down(event);
            }
        }

        EventResponse::Ignored
    }

    fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check if vertical scrollbar is dragging
        if let Some(ref mut vscroll) = self.vertical_scrollbar {
            if vscroll.bounds().contains(event.position) || vscroll.is_dragging() {
                return vscroll.on_mouse_up(event);
            }
        }

        // Check if horizontal scrollbar is dragging
        if let Some(ref mut hscroll) = self.horizontal_scrollbar {
            if hscroll.bounds().contains(event.position) || hscroll.is_dragging() {
                return hscroll.on_mouse_up(event);
            }
        }

        EventResponse::Ignored
    }

    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check if vertical scrollbar is dragging or hovered
        if let Some(ref mut vscroll) = self.vertical_scrollbar {
            if vscroll.is_dragging() || vscroll.bounds().contains(event.position) {
                return vscroll.on_mouse_move(event);
            }
        }

        // Check if horizontal scrollbar is dragging or hovered
        if let Some(ref mut hscroll) = self.horizontal_scrollbar {
            if hscroll.is_dragging() || hscroll.bounds().contains(event.position) {
                return hscroll.on_mouse_move(event);
            }
        }

        EventResponse::Ignored
    }
}
