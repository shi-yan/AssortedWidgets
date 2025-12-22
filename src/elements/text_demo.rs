//! Phase 3.2 Text Demo Element
//!
//! Demonstrates the clean two-tier text rendering API:
//! - High-level API: ctx.draw_text() with automatic caching
//! - Low-level API: ctx.draw_layout() with manual TextLayout management
//!
//! This is how real integrators should use the framework!

use crate::element::Element;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::text::TextStyle;
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use std::any::Any;

/// Demo element showcasing Phase 3.2 text rendering features
///
/// This demonstrates:
/// - Ligatures and kerning (text shaping)
/// - Bidirectional text (mixed LTR/RTL)
/// - Multi-language support (automatic font fallback)
/// - Color emoji rendering
/// - Text wrapping with width constraints
/// - High-level API usage (like real widgets would use)
pub struct TextDemoElement {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
}

impl TextDemoElement {
    /// Create a new text demo element
    pub fn new(id: WidgetId) -> Self {
        Self {
            id,
            bounds: Rect::default(),
            dirty: true,
        }
    }
}

impl Element for TextDemoElement {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()  // Demo element doesn't handle messages
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()  // Demo element doesn't handle events
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout(&self) -> Style {
        Style::default()  // Demo uses fixed positioning
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let mut y = 50.0;

        // ================================================================
        // Demo 1: Basic text with ligatures (demonstrates shaping)
        // ================================================================
        ctx.draw_text(
            "The office offers efficient service",  // Tests ligatures: ffi, ff
            &TextStyle::new().size(18.0),
            Point::new(40.0, y),
            None,
        );
        y += 50.0;

        // ================================================================
        // Demo 2: Bidirectional multi-language text
        // ================================================================
        // English (LTR) + Hebrew (RTL) + Arabic (RTL) + Chinese (LTR) + Emoji
        ctx.draw_text(
            "Hello שלום مرحبا 你好 👋",
            &TextStyle::new()
                .size(24.0)
                .color(Color { r: 0.5, g: 1.0, b: 0.5, a: 1.0 }),
            Point::new(40.0, y),
            None,
        );
        y += 60.0;

        // ================================================================
        // Demo 3: Color emoji
        // ================================================================
        ctx.draw_text(
            "🚀 ⭐ 💡 🎨 🔥 ✨",
            &TextStyle::new()
                .size(32.0)
                .bold()
                .color(Color { r: 1.0, g: 1.0, b: 0.5, a: 1.0 }),
            Point::new(40.0, y),
            None,
        );
        y += 60.0;

        // ================================================================
        // Demo 4: Text with width constraint (wrapping)
        // ================================================================
        ctx.draw_text(
            "This is a very long text that should wrap at 250 pixels width",
            &TextStyle::new()
                .size(18.0)
                .color(Color { r: 1.0, g: 0.8, b: 0.5, a: 1.0 }),
            Point::new(40.0, y),
            Some(250.0),  // Width constraint triggers wrapping
        );
        y += 50.0;

        // ================================================================
        // Demo 5: Multi-line paragraph with wrapping
        // ================================================================
        ctx.draw_text(
            "This is a longer paragraph that will wrap to multiple lines when it reaches the edge of the container. This demonstrates automatic text wrapping.",
            &TextStyle::new()
                .size(18.0)
                .color(Color { r: 0.8, g: 0.8, b: 1.0, a: 1.0 }),
            Point::new(40.0, y),
            Some(350.0),  // Width constraint
        );
    }

    fn needs_measure(&self) -> bool {
        false  // Demo element doesn't participate in layout
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        None  // Demo element uses fixed positioning
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
