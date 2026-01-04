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

use std::rc::Rc;
use std::cell::Cell;

use crate::impl_widget_essentials;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::PaintContext;
use crate::types::{DeferredCommand, DirtyLevel, GuiMessage, Rect, Vector};
use crate::widget::Widget;
use crate::WidgetState;

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
    state: WidgetState,
    layout_style: Style,
    /// Shared reference to the parent ScrollableContainer's scroll offset
    /// Uses Cell for interior mutability without runtime borrow checking
    scroll_offset: Rc<Cell<Vector>>,
}

impl ScrollViewport {
    pub fn new(scroll_offset: Rc<Cell<Vector>>, layout_style: Style) -> Self {
        ScrollViewport {
            state: WidgetState::new(),
            layout_style,
            scroll_offset,
        }
    }
}

impl Widget for ScrollViewport {
    impl_widget_essentials!();

    fn on_message(&mut self, message: &GuiMessage) -> Vec<DeferredCommand> {
        // Handle viewport_resize signal from parent ScrollableContainer
        if let GuiMessage::Custom { signal_type, data, .. } = message {
            if signal_type == "viewport_resize" {
                if let Some(bounds) = data.downcast_ref::<Rect>() {
                    println!("[VIEWPORT {}] Received viewport_resize signal", self.state.id.as_u64());
                    println!("[VIEWPORT {}]   Old bounds: {:?}", self.state.id.as_u64(), self.state.bounds);
                    println!("[VIEWPORT {}]   New bounds: {:?}", self.state.id.as_u64(), bounds);
                    self.state.bounds = *bounds;

                    // CRITICAL: Mark as Layout dirty so Window updates the Taffy node
                    self.state.dirty = DirtyLevel::Layout;

                    // CRITICAL: Update layout_style to use the new absolute dimensions
                    // This ensures Taffy will use the correct size during layout recalculation
                    self.layout_style.size.width = taffy::Dimension::length(bounds.width() as f32);
                    self.layout_style.size.height = taffy::Dimension::length(bounds.height() as f32);
                    println!("[VIEWPORT {}]   Updated layout_style: width={:?}, height={:?}",
                             self.state.id.as_u64(), self.layout_style.size.width, self.layout_style.size.height);

                    // Request layout recalculation so children reflow to new width
                    // This is sent back through the command queue and Window will intercept it
                    println!("[VIEWPORT {}]   Returning request_layout command", self.state.id.as_u64());
                    return vec![DeferredCommand {
                        target: self.state.id,
                        message: GuiMessage::Custom {
                            source: self.state.id,
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

        // Clip to viewport bounds (self.state.bounds = viewport area)
        ctx.push_clip(self.state.bounds);

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
}
