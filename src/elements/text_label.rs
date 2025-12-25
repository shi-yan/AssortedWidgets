//! TextLabel widget with full Taffy layout integration
//!
//! This widget demonstrates:
//! - Measure function integration for intrinsic sizing
//! - Layout invalidation on text/width changes
//! - Bidirectional layout flows (window resize + content changes)
//! - Cached layout management

use std::any::Any;
use std::cell::RefCell;

use crate::widget::Widget;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::text::{TextAlign, TextEngine, TextLayout, TextStyle, Truncate};
use crate::types::{DeferredCommand, GuiMessage, Rect, Size, WidgetId};

/// A text label widget with automatic layout sizing
///
/// This widget uses the measure() API to integrate with Taffy layout:
/// - If parent sets width: Text wraps to that width, height is measured
/// - If width is auto: Text uses intrinsic width (no wrapping)
/// - Layout is cached and only re-shaped when text or width changes
pub struct TextLabel {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    style: Style,

    // Text content and styling
    text: String,
    text_style: TextStyle,
    truncate: Truncate,

    // Cached layout (invalidated on text/width change)
    // Uses RefCell for interior mutability during measurement
    cached_layout: RefCell<Option<TextLayout>>,
    cached_max_width: RefCell<Option<f32>>,
}

impl TextLabel {
    /// Create a new text label
    pub fn new(id: WidgetId, text: impl Into<String>) -> Self {
        Self {
            id,
            bounds: Rect::default(),
            dirty: true,
            style: Style::default(),
            text: text.into(),
            text_style: TextStyle::new(),
            truncate: Truncate::None,
            cached_layout: RefCell::new(None),
            cached_max_width: RefCell::new(None),
        }
    }

    /// Set text content (builder pattern)
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        *self.cached_layout.borrow_mut() = None; // Invalidate cache
        self
    }

    /// Set text style (builder pattern)
    pub fn with_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = text_style;
        *self.cached_layout.borrow_mut() = None; // Invalidate cache
        self
    }

    /// Set layout style (builder pattern)
    pub fn with_layout(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set truncation mode (builder pattern)
    pub fn with_truncate(mut self, truncate: Truncate) -> Self {
        self.truncate = truncate;
        *self.cached_layout.borrow_mut() = None; // Invalidate cache
        self
    }

    /// Set font size (builder pattern)
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.text_style.font_size = size;
        *self.cached_layout.borrow_mut() = None; // Invalidate cache
        self
    }

    /// Set text color (builder pattern)
    pub fn with_color(mut self, color: Color) -> Self {
        self.text_style.text_color = color;
        self
    }

    /// Set text alignment (builder pattern)
    pub fn with_alignment(mut self, alignment: TextAlign) -> Self {
        self.text_style.alignment = alignment;
        *self.cached_layout.borrow_mut() = None; // Invalidate cache
        self
    }

    /// Update text content (runtime method)
    pub fn set_text(&mut self, text: impl Into<String>) {
        let new_text = text.into();
        if self.text != new_text {
            self.text = new_text;
            *self.cached_layout.borrow_mut() = None; // Invalidate cache
            self.dirty = true; // Trigger layout recalculation
        }
    }

    /// Ensure layout is cached for the given max_width
    ///
    /// This is called from both paint() and measure() to ensure
    /// we have a valid layout before using it.
    fn ensure_layout(&self, engine: &mut TextEngine, max_width: Option<f32>) {
        // Only re-shape if text or width changed
        let needs_reshape = self.cached_layout.borrow().is_none()
            || *self.cached_max_width.borrow() != max_width;

        if needs_reshape {
            let layout = engine.create_layout(
                &self.text,
                &self.text_style,
                max_width,
                self.truncate,
            );
            *self.cached_layout.borrow_mut() = Some(layout);
            *self.cached_max_width.borrow_mut() = max_width;
        }
    }

    /// Measure text for a given max_width (called by Window layout system)
    ///
    /// This is exposed so external code (like the window) can call measure
    /// and pass the TextEngine.
    pub fn measure_with_engine(
        &self,
        engine: &mut TextEngine,
        known_dimensions: taffy::Size<Option<f32>>,
    ) -> Size {
        // Case 1: Width is known → wrap to that width
        if let Some(width) = known_dimensions.width {
            self.ensure_layout(engine, Some(width));
            let size = self.cached_layout.borrow().as_ref().unwrap().size();
            return Size::new(width as f64, size.height);
        }

        // Case 2: Width is auto → return intrinsic size (no wrapping)
        self.ensure_layout(engine, None);
        self.cached_layout.borrow().as_ref().unwrap().size()
    }
}

impl Widget for TextLabel {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn on_message(&mut self, message: &GuiMessage) -> Vec<DeferredCommand> {
        // Handle FPS update signals
        if let GuiMessage::Custom { source: _, signal_type, data } = message {
            if signal_type == "fps_update" {
                // Extract FPS value from the signal data
                if let Some(fps) = data.downcast_ref::<f64>() {
                    // Use set_text() to properly invalidate cached layout
                    self.set_text(format!("FPS: {:.1}", fps));
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
        self.bounds = bounds;
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout(&self) -> Style {
        self.style.clone()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Use high-level API for simplicity
        // Note: The layout was already computed during measure phase
        let cached_layout = self.cached_layout.borrow();
        if let Some(layout) = cached_layout.as_ref() {
            ctx.draw_layout(layout, self.bounds.origin, self.text_style.text_color);
        } else {
            // Fallback: Use high-level API if no cached layout
            ctx.draw_text(
                &self.text,
                &self.text_style,
                self.bounds.origin,
                *self.cached_max_width.borrow(),
            );
        }
    }

    fn needs_measure(&self) -> bool {
        true // TextLabel always participates in measurement
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        // NOTE: This method signature doesn't allow us to access TextEngine!
        // The actual measurement happens in measure_with_engine() which is
        // called by the window's layout system via downcast detection.
        //
        // This method just signals that we need measurement (via needs_measure()).
        // The real work is done in Window::render_frame() during layout computation.
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
