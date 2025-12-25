//! Phase 3.3 Text Demo Element
//!
//! Demonstrates all Phase 3 text rendering features:
//! - Two-tier API (high-level + low-level)
//! - Text alignment (left, center, right)
//! - Ellipsis truncation
//! - Text wrapping and multi-line
//! - Bidirectional and multi-language text
//! - Performance stats tracking
//!
//! This is how real integrators should use the framework!

use crate::widget::Widget;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::text::{TextStyle, TextAlign};
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use std::any::Any;

/// Demo widget showcasing Phase 3.2 text rendering features
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

impl Widget for TextDemoElement {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()  // Demo widget doesn't handle messages
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()  // Demo widget doesn't handle events
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
        Style::default()  // Demo widget uses fixed positioning
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let mut y = 30.0;
        let left_margin = 40.0;
        let box_width = 400.0_f32;

        // Header
        ctx.draw_text(
            "Phase 3.3 Text Rendering Features",
            &TextStyle::new()
                .size(24.0)
                .bold()
                .color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            Point::new(left_margin, y),
            None,
        );
        y += 50.0;

        // ================================================================
        // Demo 1: Text Alignment (Phase 3.3)
        // ================================================================
        ctx.draw_text(
            "Text Alignment:",
            &TextStyle::new()
                .size(16.0)
                .color(Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 }),
            Point::new(left_margin, y),
            None,
        );
        y += 30.0;

        // Draw a background box to visualize alignment
        ctx.draw_rect(
            Rect::new(Point::new(left_margin, y), Size::new(box_width as f64, 90.0)),
            Color { r: 0.15, g: 0.15, b: 0.2, a: 1.0 },
        );

        println!("draw rect {} {} {} {}", left_margin, y, box_width, 90.0);

        // Left-aligned (default)
        ctx.draw_text(
            "Left aligned",
            &TextStyle::new()
                .size(18.0)
                .align(TextAlign::Left)
                .color(Color { r: 0.5, g: 1.0, b: 0.5, a: 1.0 }),
            Point::new(left_margin, y + 5.0),
            Some(box_width),
        );

        // Center-aligned
        ctx.draw_text(
            "Center aligned",
            &TextStyle::new()
                .size(18.0)
                .align(TextAlign::Center)
                .color(Color { r: 1.0, g: 1.0, b: 0.5, a: 1.0 }),
            Point::new(left_margin, y + 35.0),
            Some(box_width),
        );

        // Right-aligned
        ctx.draw_text(
            "Right aligned",
            &TextStyle::new()
                .size(18.0)
                .align(TextAlign::Right)
                .color(Color { r: 1.0, g: 0.5, b: 0.5, a: 1.0 }),
            Point::new(left_margin, y + 65.0),
            Some(box_width),
        );
        y += 120.0;

        // ================================================================
        // Demo 2: Ligatures and Kerning (Phase 3.2)
        // ================================================================
        ctx.draw_text(
            "Ligatures: office, efficient",
            &TextStyle::new()
                .size(18.0)
                .color(Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }),
            Point::new(left_margin, y),
            None,
        );
        y += 40.0;

        // ================================================================
        // Demo 3: Bidirectional + Multi-language (Phase 3.2)
        // ================================================================
        ctx.draw_text(
            "Multi-language: Hello שלום مرحبا 你好 👋",
            &TextStyle::new()
                .size(20.0)
                .color(Color { r: 0.5, g: 1.0, b: 0.8, a: 1.0 }),
            Point::new(left_margin, y),
            None,
        );
        y += 50.0;

        // ================================================================
        // Demo 4: Emoji (Phase 3.2)
        // ================================================================
        ctx.draw_text(
            "Emoji: 🚀 ⭐ 💡 🎨 🔥 ✨",
            &TextStyle::new()
                .size(28.0)
                .bold()
                .color(Color { r: 1.0, g: 1.0, b: 0.5, a: 1.0 }),
            Point::new(left_margin, y),
            None,
        );
        y += 60.0;

        // ================================================================
        // Demo 5: Text Wrapping (Phase 3.2)
        // ================================================================
        ctx.draw_text(
            "Wrapping:",
            &TextStyle::new()
                .size(16.0)
                .color(Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 }),
            Point::new(left_margin, y),
            None,
        );
        y += 25.0;

        ctx.draw_text(
            "This is a longer paragraph that will wrap to multiple lines when it reaches the edge of the container width.",
            &TextStyle::new()
                .size(18.0)
                .color(Color { r: 0.8, g: 0.9, b: 1.0, a: 1.0 }),
            Point::new(left_margin, y),
            Some(350.0),  // Width constraint triggers wrapping
        );
        y += 90.0;

        // ================================================================
        // Demo 6: Performance Stats (Phase 3.3)
        // ================================================================
        ctx.draw_text(
            "Performance: Check console for cache stats",
            &TextStyle::new()
                .size(14.0)
                .color(Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }),
            Point::new(left_margin, y),
            None,
        );
    }

    fn needs_measure(&self) -> bool {
        false  // Demo widget doesn't participate in layout
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        None  // Demo widget uses fixed positioning
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
