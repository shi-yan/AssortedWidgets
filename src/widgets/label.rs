//! Label widget - the foundational text display widget
//!
//! Features:
//! - Multiple wrapping and truncation modes
//! - Padding with sensible defaults
//! - Optional background color
//! - Text styling (color, font, size)
//! - Text alignment (left, center, right)
//! - Optional icon
//! - Automatic URL detection and link styling
//! - Ergonomic builder pattern API

use std::any::Any;
use std::cell::RefCell;

use crate::widget::Widget;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::text::{TextAlign, TextEngine, TextLayout, TextStyle, Truncate};
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};

/// Text wrapping and truncation mode
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WrapMode {
    /// Single line, no wrapping, truncate or clip without ellipses
    SingleLine,

    /// Single line, no wrapping, truncate with ellipses
    SingleLineEllipsis,

    /// Multi-line with word wrapping (can break mid-word if necessary)
    WrapAnywhere,

    /// Multi-line with word wrapping and hyphenation (adds hyphen when breaking mid-word)
    WrapAnywhereHyphen,

    /// Multi-line with word wrapping (only breaks between words, never mid-word)
    WrapWord,
}

impl Default for WrapMode {
    fn default() -> Self {
        WrapMode::SingleLineEllipsis
    }
}

/// Padding configuration
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    /// Create uniform padding
    pub fn uniform(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create padding with different horizontal and vertical values
    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create padding with individual values
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }

    /// Total horizontal padding (left + right)
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    /// Total vertical padding (top + bottom)
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::uniform(8.0)
    }
}

/// URL segment within text
#[derive(Clone, Debug)]
struct UrlSegment {
    /// Start byte index in the text
    start: usize,
    /// End byte index in the text
    end: usize,
    /// The URL string
    url: String,
}

/// Label widget - the foundational text display widget
///
/// # Example
/// ```rust,ignore
/// // Minimal usage with defaults
/// let label = Label::new("Hello, World!");
///
/// // With customization
/// let label = Label::new("Welcome to AssortedWidgets!")
///     .wrap_mode(WrapMode::WrapWord)
///     .padding(Padding::uniform(12.0))
///     .bg_color(Color::rgb(0.2, 0.2, 0.25))
///     .text_color(Color::rgb(0.9, 0.9, 0.95))
///     .font_size(18.0)
///     .align(TextAlign::Center);
/// ```
pub struct Label {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Content
    text: String,
    // icon: Option<IconId>,  // TODO: Implement icon support when IconId is available

    // Text styling
    text_style: TextStyle,
    custom_text_color: Option<Color>, // Override theme text color
    font_family: Option<String>,

    // Layout
    wrap_mode: WrapMode,
    padding: Padding,
    bg_color: Option<Color>, // None = transparent

    // URL detection and styling
    link_color: Color,
    url_segments: Vec<UrlSegment>,

    // Cached layout (invalidated on text/width change)
    cached_layout: RefCell<Option<TextLayout>>,
    cached_max_width: RefCell<Option<f32>>,
}

impl Label {
    /// Create a new label with the given text
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let url_segments = Self::detect_urls(&text);

        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            text,
            // icon: None,  // TODO: Implement icon support
            text_style: TextStyle::new(),
            custom_text_color: None,
            font_family: None,
            wrap_mode: WrapMode::default(),
            padding: Padding::default(),
            bg_color: None,
            link_color: Color::rgb(0.3, 0.6, 0.9), // Blue links
            url_segments,
            cached_layout: RefCell::new(None),
            cached_max_width: RefCell::new(None),
        }
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set the text content
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self.url_segments = Self::detect_urls(&self.text);
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set the wrapping mode
    pub fn wrap_mode(mut self, mode: WrapMode) -> Self {
        self.wrap_mode = mode;
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set background color (None = transparent)
    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set transparent background
    pub fn transparent(mut self) -> Self {
        self.bg_color = None;
        self
    }

    /// Set text color (overrides theme)
    pub fn text_color(mut self, color: Color) -> Self {
        self.custom_text_color = Some(color);
        self.text_style.text_color = color;
        self
    }

    /// Set font family
    pub fn font(mut self, font: impl Into<String>) -> Self {
        let font_str = font.into();
        self.font_family = Some(font_str.clone());
        self.text_style = self.text_style.family(font_str);
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.text_style = self.text_style.size(size);
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set text alignment
    pub fn align(mut self, alignment: TextAlign) -> Self {
        self.text_style = self.text_style.align(alignment);
        *self.cached_layout.borrow_mut() = None;
        self
    }

    // TODO: Implement icon support when IconId is available
    // /// Set icon
    // pub fn icon(mut self, icon: IconId) -> Self {
    //     self.icon = Some(icon);
    //     self
    // }

    /// Set link color for detected URLs
    pub fn link_color(mut self, color: Color) -> Self {
        self.link_color = color;
        self
    }

    /// Set layout style (for Taffy)
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Update text content at runtime
    pub fn set_text(&mut self, text: impl Into<String>) {
        let new_text = text.into();
        if self.text != new_text {
            self.text = new_text;
            self.url_segments = Self::detect_urls(&self.text);
            *self.cached_layout.borrow_mut() = None;
            self.dirty = true;
        }
    }

    /// Update wrap mode at runtime
    pub fn set_wrap_mode(&mut self, mode: WrapMode) {
        if self.wrap_mode != mode {
            self.wrap_mode = mode;
            *self.cached_layout.borrow_mut() = None;
            self.dirty = true;
        }
    }

    // ========================================================================
    // URL Detection
    // ========================================================================

    /// Detect URLs in the text (simple implementation)
    fn detect_urls(text: &str) -> Vec<UrlSegment> {
        let mut segments = Vec::new();
        let patterns = ["https://", "http://"];

        for pattern in &patterns {
            let mut start = 0;
            while let Some(pos) = text[start..].find(pattern) {
                let url_start = start + pos;

                // Find the end of the URL (whitespace or end of string)
                let url_end = text[url_start..]
                    .find(|c: char| c.is_whitespace())
                    .map(|i| url_start + i)
                    .unwrap_or(text.len());

                segments.push(UrlSegment {
                    start: url_start,
                    end: url_end,
                    url: text[url_start..url_end].to_string(),
                });

                start = url_end;
            }
        }

        // Sort by start position
        segments.sort_by_key(|s| s.start);
        segments
    }

    // ========================================================================
    // Layout Measurement
    // ========================================================================

    /// Ensure layout is cached for the given max_width
    fn ensure_layout(&self, engine: &mut TextEngine, available_width: Option<f32>) {
        // Calculate max_width for text (accounting for padding)
        let max_width = available_width.map(|w| (w - self.padding.horizontal()).max(0.0));

        // Only re-shape if text or width changed
        let needs_reshape = self.cached_layout.borrow().is_none()
            || *self.cached_max_width.borrow() != max_width;

        if needs_reshape {
            // Convert WrapMode to cosmic-text wrapping and truncation
            let (truncate, wrap) = match self.wrap_mode {
                WrapMode::SingleLine => (Truncate::None, cosmic_text::Wrap::None),
                WrapMode::SingleLineEllipsis => (Truncate::End, cosmic_text::Wrap::None),
                WrapMode::WrapAnywhere => (Truncate::None, cosmic_text::Wrap::Glyph),
                WrapMode::WrapAnywhereHyphen => (Truncate::None, cosmic_text::Wrap::Glyph), // TODO: Add hyphen support
                WrapMode::WrapWord => (Truncate::None, cosmic_text::Wrap::Word),
            };

            // Use the extended API to specify custom wrap mode
            let layout = engine.create_layout_with_wrap(
                &self.text,
                &self.text_style,
                max_width,
                truncate,
                wrap,
            );

            *self.cached_layout.borrow_mut() = Some(layout);
            *self.cached_max_width.borrow_mut() = max_width;
        }
    }

    /// Measure text for layout system
    pub fn measure_with_engine(
        &self,
        engine: &mut TextEngine,
        known_dimensions: taffy::Size<Option<f32>>,
    ) -> Size {
        // Case 1: Width is known → wrap to that width
        if let Some(width) = known_dimensions.width {
            self.ensure_layout(engine, Some(width));
            let text_size = self.cached_layout.borrow().as_ref().unwrap().size();

            // Add padding to the measured size
            return Size::new(
                width as f64,
                (text_size.height + self.padding.vertical() as f64).max(0.0),
            );
        }

        // Case 2: Width is auto → return intrinsic size (no wrapping for single-line modes)
        let max_width = match self.wrap_mode {
            WrapMode::SingleLine | WrapMode::SingleLineEllipsis => None,
            _ => known_dimensions.width,
        };

        self.ensure_layout(engine, max_width);
        let text_size = self.cached_layout.borrow().as_ref().unwrap().size();

        Size::new(
            text_size.width + self.padding.horizontal() as f64,
            text_size.height + self.padding.vertical() as f64,
        )
    }
}

impl Widget for Label {
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
        self.bounds = bounds;
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

    fn paint(&self, ctx: &mut PaintContext) {
        // CRITICAL: Ensure text layout is valid for current bounds width
        // The layout system may have called measure() multiple times with different
        // constraints, potentially invalidating our cached layout. We must re-ensure
        // the layout using the final bounds before painting.
        ctx.with_text_engine(|engine| {
            self.ensure_layout(engine, Some(self.bounds.size.width as f32));
        });

        // Draw background if specified
        if let Some(bg_color) = self.bg_color {
            ctx.draw_rect(self.bounds, bg_color);
        }

        // Calculate content bounds (bounds minus padding) for clipping
        let content_rect = Rect::new(
            Point::new(
                self.bounds.origin.x + self.padding.left as f64,
                self.bounds.origin.y + self.padding.top as f64,
            ),
            Size::new(
                (self.bounds.size.width - self.padding.horizontal() as f64).max(0.0),
                (self.bounds.size.height - self.padding.vertical() as f64).max(0.0),
            ),
        );

        // Push clip rect to ensure text doesn't overflow the label bounds
        ctx.push_clip(content_rect);

        // Calculate text rendering position (with padding)
        let text_origin = Point::new(
            self.bounds.origin.x + self.padding.left as f64,
            self.bounds.origin.y + self.padding.top as f64,
        );

        // Draw text using the correctly cached layout
        let cached_layout = self.cached_layout.borrow();
        if let Some(layout) = cached_layout.as_ref() {
            // For now, draw entire text with single color
            // TODO: Implement URL-aware rendering with link color
            ctx.draw_layout(layout, text_origin, self.text_style.text_color);
        } else {
            // This should never happen after ensure_layout, but keep as fallback
            ctx.draw_text(
                &self.text,
                &self.text_style,
                text_origin,
                *self.cached_max_width.borrow(),
            );
        }

        // Pop clip rect
        ctx.pop_clip();

        // TODO: Draw icon if specified
        // if let Some(icon) = &self.icon {
        //     ctx.draw_icon(icon, icon_position, icon_size, icon_color);
        // }
    }

    fn needs_measure(&self) -> bool {
        true
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        // Actual measurement happens in measure_with_engine() called by Window
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
