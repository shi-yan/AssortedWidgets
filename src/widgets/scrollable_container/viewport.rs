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
use std::rc::Rc;
use std::cell::Cell;

use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;
use crate::types::{DeferredCommand, DirtyLevel, GuiMessage, Point, Rect, Vector, WidgetId};
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

/// Internal widget that applies scroll offset to its children (content only, not scrollbars)
///
/// This is an implementation detail of ScrollableContainer. It sits between the
/// ScrollableContainer and the content_container, applying the scroll offset transformation
/// so that scrollbars (which are siblings of this viewport) don't get offset.
pub struct ScrollViewport {
    id: WidgetId,
    bounds: Rect,
    dirty: DirtyLevel,
    layout_style: Style,
    /// Shared reference to the parent ScrollableContainer's scroll offset
    /// Uses Cell for interior mutability without runtime borrow checking
    scroll_offset: Rc<Cell<Vector>>,
}

impl ScrollViewport {
    pub fn new(scroll_offset: Rc<Cell<Vector>>, layout_style: Style) -> Self {
        ScrollViewport {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: DirtyLevel::Layout,
            layout_style,
            scroll_offset,
        }
    }
}

impl Widget for ScrollViewport {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn on_message(&mut self, message: &GuiMessage) -> Vec<DeferredCommand> {
        // Handle viewport_resize signal from parent ScrollableContainer
        if let GuiMessage::Custom { signal_type, data, .. } = message {
            if signal_type == "viewport_resize" {
                if let Some(bounds) = data.downcast_ref::<Rect>() {
                    println!("[VIEWPORT {}] Received viewport_resize signal", self.id.as_u64());
                    println!("[VIEWPORT {}]   Old bounds: {:?}", self.id.as_u64(), self.bounds);
                    println!("[VIEWPORT {}]   New bounds: {:?}", self.id.as_u64(), bounds);
                    self.bounds = *bounds;

                    // CRITICAL: Mark as Layout dirty so Window updates the Taffy node
                    self.dirty = DirtyLevel::Layout;

                    // CRITICAL: Update layout_style to use the new absolute dimensions
                    // This ensures Taffy will use the correct size during layout recalculation
                    self.layout_style.size.width = taffy::Dimension::length(bounds.width() as f32);
                    self.layout_style.size.height = taffy::Dimension::length(bounds.height() as f32);
                    println!("[VIEWPORT {}]   Updated layout_style: width={:?}, height={:?}",
                             self.id.as_u64(), self.layout_style.size.width, self.layout_style.size.height);

                    // Request layout recalculation so children reflow to new width
                    // This is sent back through the command queue and Window will intercept it
                    println!("[VIEWPORT {}]   Returning request_layout command", self.id.as_u64());
                    return vec![DeferredCommand {
                        target: self.id,
                        message: GuiMessage::Custom {
                            source: self.id,
                            signal_type: "request_layout".to_string(),
                            data: Box::new(()),
                        },
                    }];
                }
            }
        }
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
            self.bounds = bounds;
            self.dirty = DirtyLevel::Layout;
        }
    }

    fn set_dirty(&mut self, _dirty: bool) {
        // Deprecated - use set_dirty_level instead
        self.dirty = DirtyLevel::Visual;
    }

    fn is_dirty(&self) -> bool {
        self.dirty != DirtyLevel::Clean
    }

    fn dirty_level(&self) -> DirtyLevel {
        self.dirty
    }

    fn set_dirty_level(&mut self, level: DirtyLevel) {
        self.dirty = level;
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn paint(&self, _ctx: &mut PaintContext) {
        // Nothing to paint
    }

    fn before_paint_children(&self, ctx: &mut PaintContext) {
        // CRITICAL: Push clip rect BEFORE applying offset
        // This keeps the clip rect in viewport coordinates (not content coordinates)
        // so it remains fixed as content scrolls

        // Clip to viewport bounds (self.bounds = viewport area)
        ctx.push_clip(self.bounds);

        // Apply scroll offset transformation to children (content only)
        // Read from the shared Cell - no borrow checking needed!
        let offset = self.scroll_offset.get();
        // Negative offset because scrolling down means content moves up
        ctx.push_offset(Vector::new(-offset.x, -offset.y));
    }

    fn after_paint_children(&self, ctx: &mut PaintContext) {
        // Pop the scroll offset
        ctx.pop_offset();

        // Pop the clip rect
        ctx.pop_clip();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
