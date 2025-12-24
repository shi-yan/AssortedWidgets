//! TextLabel widget with full Taffy layout integration
//!
//! This widget demonstrates:
//! - Measure function integration for intrinsic sizing
//! - Layout invalidation on text/width changes
//! - Bidirectional layout flows (window resize + content changes)
//! - Cached layout management

use std::any::Any;

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
    cached_layout: Option<TextLayout>,
    cached_max_width: Option<f32>,
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
            cached_layout: None,
            cached_max_width: None,
        }
    }

    /// Set text content (builder pattern)
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self.cached_layout = None; // Invalidate cache
        self
    }

    /// Set text style (builder pattern)
    pub fn with_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = text_style;
        self.cached_layout = None; // Invalidate cache
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
        self.cached_layout = None; // Invalidate cache
        self
    }

    /// Set font size (builder pattern)
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.text_style.font_size = size;
        self.cached_layout = None; // Invalidate cache
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
        self.cached_layout = None; // Invalidate cache
        self
    }

    /// Update text content (runtime method)
    pub fn set_text(&mut self, text: impl Into<String>) {
        let new_text = text.into();
        if self.text != new_text {
            self.text = new_text;
            self.cached_layout = None; // Invalidate cache
            self.dirty = true; // Trigger layout recalculation
        }
    }

    /// Ensure layout is cached for the given max_width
    ///
    /// This is called from both paint() and measure() to ensure
    /// we have a valid layout before using it.
    fn ensure_layout(&mut self, engine: &mut TextEngine, max_width: Option<f32>) {
        // Only re-shape if text or width changed
        let needs_reshape = self.cached_layout.is_none() || self.cached_max_width != max_width;

        if needs_reshape {
            self.cached_layout = Some(engine.create_layout(
                &self.text,
                &self.text_style,
                max_width,
                self.truncate,
            ));
            self.cached_max_width = max_width;
        }
    }

    /// Measure text for a given max_width (called by measure())
    ///
    /// This is exposed so external code (like the window) can call measure
    /// and pass the TextEngine.
    pub fn measure_with_engine(
        &mut self,
        engine: &mut TextEngine,
        known_dimensions: taffy::Size<Option<f32>>,
    ) -> Size {
        // Case 1: Width is known → wrap to that width
        if let Some(width) = known_dimensions.width {
            self.ensure_layout(engine, Some(width));
            let size = self.cached_layout.as_ref().unwrap().size();
            return Size::new(width as f64, size.height);
        }

        // Case 2: Width is auto → return intrinsic size (no wrapping)
        self.ensure_layout(engine, None);
        self.cached_layout.as_ref().unwrap().size()
    }
}

impl Widget for TextLabel {
    fn id(&self) -> WidgetId {
        self.id
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
        if let Some(layout) = &self.cached_layout {
            ctx.draw_layout(layout, self.bounds.origin, self.text_style.text_color);
        } else {
            // Fallback: Use high-level API if no cached layout
            ctx.draw_text(
                &self.text,
                &self.text_style,
                self.bounds.origin,
                self.cached_max_width,
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
        // called by the window's layout system.
        //
        // This method just signals that we need measurement.
        // The real work is done in Window::compute_layout() which calls
        // measure_with_engine() directly.
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
