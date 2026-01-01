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

use crate::event::input::{EventResponse, WheelEvent};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, Vector, WidgetId};
use crate::widget::Widget;

/// Scrolling mode for ScrollableContainer
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScrollMode {
    /// Vertical scrolling only
    Vertical,
    /// Horizontal scrolling only
    Horizontal,
    /// Both vertical and horizontal scrolling
    Both,
}

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
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
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
    scroll_offset: Vector,

    /// Total size of content (measured from children bounds)
    /// Updated after layout pass
    content_size: Size,

    /// Viewport size (visible area = bounds minus scrollbars)
    viewport_size: Size,

    // ========================================
    // Child Widget IDs
    // ========================================
    /// Content container ID (holds actual child widgets)
    content_container_id: Option<WidgetId>,

    /// Vertical scrollbar widget ID (if vertical scrolling enabled)
    vertical_scrollbar_id: Option<WidgetId>,

    /// Horizontal scrollbar widget ID (if horizontal scrolling enabled)
    horizontal_scrollbar_id: Option<WidgetId>,

    // ========================================
    // Scrollbar Visibility
    // ========================================
    /// True if vertical scrollbar should be shown
    show_vertical_scrollbar: bool,

    /// True if horizontal scrollbar should be shown
    show_horizontal_scrollbar: bool,

    // ========================================
    // Constants
    // ========================================
    /// Width of vertical scrollbar (in pixels)
    scrollbar_width: f32,

    /// Height of horizontal scrollbar (in pixels)
    scrollbar_height: f32,

    // ========================================
    // Deferred Commands
    // ========================================
    /// Queue for signals and other deferred operations
    pending_commands: Vec<DeferredCommand>,
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
    /// - `Vec<(Box<dyn Widget>, Style, Option<WidgetId>)>`: Child widgets to add
    ///   Each tuple is (widget, layout_style, parent_id_override)
    ///   parent_id_override is None for children of the ScrollableContainer itself
    pub fn new(
        scroll_mode: ScrollMode,
        gui_handle: &crate::handle::GuiHandle,
    ) -> (Self, Vec<(Box<dyn Widget>, Style, Option<WidgetId>)>) {
        use crate::elements::Container;

        // Pre-allocate widget IDs
        let content_container_id = gui_handle.next_widget_id();
        let vertical_scrollbar_id = if matches!(scroll_mode, ScrollMode::Vertical | ScrollMode::Both) {
            Some(gui_handle.next_widget_id())
        } else {
            None
        };
        let horizontal_scrollbar_id = if matches!(scroll_mode, ScrollMode::Horizontal | ScrollMode::Both) {
            Some(gui_handle.next_widget_id())
        } else {
            None
        };

        // Create the ScrollableContainer itself
        let container = ScrollableContainer {
            id: WidgetId::new(0), // Will be set by Window
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            scroll_mode,
            scroll_offset: Vector::new(0.0, 0.0),
            content_size: Size::new(0.0, 0.0),
            viewport_size: Size::new(0.0, 0.0),
            content_container_id: Some(content_container_id),
            vertical_scrollbar_id,
            horizontal_scrollbar_id,
            show_vertical_scrollbar: false,
            show_horizontal_scrollbar: false,
            scrollbar_width: Self::SCROLLBAR_WIDTH,
            scrollbar_height: Self::SCROLLBAR_HEIGHT,
            pending_commands: Vec::new(),
        };

        // Create child widgets
        let mut children = Vec::new();

        // Content container - uses flexbox column layout for stacking content
        let content_style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            size: taffy::Size {
                width: taffy::Dimension::auto(),
                height: taffy::Dimension::auto(),
            },
            ..Default::default()
        };
        let mut content_container = Box::new(Container::new(content_style.clone()));
        content_container.set_id(content_container_id);
        children.push((content_container as Box<dyn Widget>, content_style, None));

        // Create scrollbars if needed
        use crate::widgets::ScrollBar;

        if let Some(vscroll_id) = vertical_scrollbar_id {
            // Vertical scrollbar - positioned on the right edge
            let scrollbar_style = Style {
                display: taffy::Display::Flex,
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    top: taffy::LengthPercentageAuto::length(0.0),
                    right: taffy::LengthPercentageAuto::length(0.0),
                    bottom: taffy::LengthPercentageAuto::length(0.0),
                    left: taffy::LengthPercentageAuto::auto(),
                },
                size: taffy::Size {
                    width: taffy::Dimension::length(Self::SCROLLBAR_WIDTH),
                    height: taffy::Dimension::percent(1.0),
                },
                ..Default::default()
            };

            let mut scrollbar = Box::new(
                ScrollBar::vertical(0, 100, 10)
                    .width(Self::SCROLLBAR_WIDTH)
                    .layout_style(scrollbar_style.clone())
            );
            scrollbar.set_id(vscroll_id);
            children.push((scrollbar as Box<dyn Widget>, scrollbar_style, None));
        }

        if let Some(hscroll_id) = horizontal_scrollbar_id {
            // Horizontal scrollbar - positioned on the bottom edge
            let scrollbar_style = Style {
                display: taffy::Display::Flex,
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    top: taffy::LengthPercentageAuto::auto(),
                    right: taffy::LengthPercentageAuto::length(0.0),
                    bottom: taffy::LengthPercentageAuto::length(0.0),
                    left: taffy::LengthPercentageAuto::length(0.0),
                },
                size: taffy::Size {
                    width: taffy::Dimension::percent(1.0),
                    height: taffy::Dimension::length(Self::SCROLLBAR_HEIGHT),
                },
                ..Default::default()
            };

            let mut scrollbar = Box::new(
                ScrollBar::horizontal(0, 100, 10)
                    .width(Self::SCROLLBAR_HEIGHT)
                    .layout_style(scrollbar_style.clone())
            );
            scrollbar.set_id(hscroll_id);
            children.push((scrollbar as Box<dyn Widget>, scrollbar_style, None));
        }

        (container, children)
    }

    /// Get the content container ID
    ///
    /// This is the widget ID where user content should be added.
    /// Note: This returns the stored ID which is set during initialization.
    /// The actual ID is assigned by Window::add_composite.
    pub fn content_container_id(&self) -> Option<WidgetId> {
        self.content_container_id
    }

    /// Get the vertical scrollbar ID (internal, used by Window)
    pub(crate) fn vertical_scrollbar_id(&self) -> Option<WidgetId> {
        self.vertical_scrollbar_id
    }

    /// Get the horizontal scrollbar ID (internal, used by Window)
    pub(crate) fn horizontal_scrollbar_id(&self) -> Option<WidgetId> {
        self.horizontal_scrollbar_id
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
        self.scroll_offset
    }

    /// Set the content container ID (called after add_composite)
    pub(crate) fn set_content_container_id(&mut self, id: WidgetId) {
        self.content_container_id = Some(id);
    }

    /// Update content size from content container bounds (called after layout)
    pub(crate) fn update_content_size(&mut self, content_bounds: Size) {
        if self.content_size != content_bounds {
            self.content_size = content_bounds;
            self.update_scrollbar_visibility();
            self.clamp_scroll_offset();
            self.dirty = true;
        }
    }

    /// Update scrollbar visibility based on content size vs viewport size
    fn update_scrollbar_visibility(&mut self) {
        match self.scroll_mode {
            ScrollMode::Vertical => {
                self.show_vertical_scrollbar = self.content_size.height > self.viewport_size.height;
                self.show_horizontal_scrollbar = false;
            }
            ScrollMode::Horizontal => {
                self.show_horizontal_scrollbar = self.content_size.width > self.viewport_size.width;
                self.show_vertical_scrollbar = false;
            }
            ScrollMode::Both => {
                self.show_vertical_scrollbar = self.content_size.height > self.viewport_size.height;
                self.show_horizontal_scrollbar = self.content_size.width > self.viewport_size.width;
            }
        }
    }

    /// Calculate viewport size (bounds minus scrollbars)
    fn calculate_viewport_size(&self) -> Size {
        let mut size = self.bounds.size;

        if self.show_vertical_scrollbar {
            size.width -= self.scrollbar_width as f64;
        }

        if self.show_horizontal_scrollbar {
            size.height -= self.scrollbar_height as f64;
        }

        size
    }

    /// Clamp scroll offset to valid range
    fn clamp_scroll_offset(&mut self) {
        let max_x = (self.content_size.width - self.viewport_size.width).max(0.0);
        let max_y = (self.content_size.height - self.viewport_size.height).max(0.0);

        self.scroll_offset.x = self.scroll_offset.x.clamp(0.0, max_x);
        self.scroll_offset.y = self.scroll_offset.y.clamp(0.0, max_y);
    }
}

// ============================================================================
// Widget Trait Implementation
// ============================================================================

impl Widget for ScrollableContainer {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        if self.bounds != bounds {
            println!("[SCROLLABLE] set_bounds called: bounds = {:?}", bounds);
            self.bounds = bounds;
            self.viewport_size = self.calculate_viewport_size();
            println!("[SCROLLABLE]   calculated viewport_size = {:?}", self.viewport_size);
            println!("[SCROLLABLE]   show_vertical_scrollbar = {}", self.show_vertical_scrollbar);
            self.update_scrollbar_visibility();
            self.clamp_scroll_offset();
            self.dirty = true;
        }
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn paint(&self, _ctx: &mut PaintContext) {
        // Nothing to paint for the container itself
        // Scrollbars are separate child widgets painted by Window traverse
    }

    fn before_paint_children(&self, ctx: &mut PaintContext) {
        // Push clip rect to viewport bounds (clips content to visible area)
        ctx.push_clip(Rect::new(
            self.bounds.origin,
            self.viewport_size,
        ));

        // Push offset for scrolling (children will be rendered with this offset)
        // Negative offset because we're moving the content up/left when scrolling down/right
        ctx.push_offset(Vector::new(-self.scroll_offset.x, -self.scroll_offset.y));
    }

    fn after_paint_children(&self, ctx: &mut PaintContext) {
        // Pop offset and clip after children are painted
        ctx.pop_offset();
        ctx.pop_clip();
    }

    /// Transform point from viewport space to content space
    ///
    /// This is critical for hierarchical hit testing. When the user clicks at (50, 50)
    /// in the viewport, but the content is scrolled down by 200px, we need to transform
    /// that to (50, 250) in content space so hit testing finds the correct widget.
    fn transform_point_for_children(&self, point: Point) -> Point {
        Point::new(
            point.x + self.scroll_offset.x,
            point.y + self.scroll_offset.y,
        )
    }

    fn drain_deferred_commands(&mut self) -> Vec<DeferredCommand> {
        std::mem::take(&mut self.pending_commands)
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

    // Handle wheel events for scrolling
    fn on_wheel(&mut self, event: &mut WheelEvent) -> EventResponse {
        println!("[SCROLLABLE] on_wheel called! delta: ({:.1}, {:.1})", event.delta.x, event.delta.y);
        println!("[SCROLLABLE]   current offset: ({:.1}, {:.1})", self.scroll_offset.x, self.scroll_offset.y);
        println!("[SCROLLABLE]   content_size: ({:.1}, {:.1})", self.content_size.width, self.content_size.height);
        println!("[SCROLLABLE]   viewport_size: ({:.1}, {:.1})", self.viewport_size.width, self.viewport_size.height);

        let old_offset = self.scroll_offset;

        // Update scroll offset based on wheel delta
        match self.scroll_mode {
            ScrollMode::Vertical => {
                self.scroll_offset.y += event.delta.y;
                println!("[SCROLLABLE]   vertical scroll: {:.1} -> {:.1}", old_offset.y, self.scroll_offset.y);
            }
            ScrollMode::Horizontal => {
                self.scroll_offset.x += event.delta.x;
                println!("[SCROLLABLE]   horizontal scroll: {:.1} -> {:.1}", old_offset.x, self.scroll_offset.x);
            }
            ScrollMode::Both => {
                self.scroll_offset.x += event.delta.x;
                self.scroll_offset.y += event.delta.y;
                println!("[SCROLLABLE]   both scroll: ({:.1}, {:.1}) -> ({:.1}, {:.1})",
                         old_offset.x, old_offset.y, self.scroll_offset.x, self.scroll_offset.y);
            }
        }

        // Clamp to valid range
        self.clamp_scroll_offset();
        println!("[SCROLLABLE]   after clamp: ({:.1}, {:.1})", self.scroll_offset.x, self.scroll_offset.y);

        // If scroll offset changed, mark dirty and handle the event
        if self.scroll_offset != old_offset {
            self.dirty = true;
            println!("[SCROLLABLE] ✓ Scroll offset changed, returning Handled");
            EventResponse::Handled
        } else {
            println!("[SCROLLABLE] ✗ Scroll offset unchanged (clamped), returning Ignored");
            EventResponse::Ignored
        }
    }
}
