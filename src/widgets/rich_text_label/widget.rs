//! RichTextLabel widget implementation

use std::any::Any;
use std::cell::RefCell;

use crate::event::{OsEvent, InputEventEnum, MouseEvent, WheelEvent, EventResponse, MouseHandler};
use crate::layout::Style;
use crate::paint::{Color, PaintContext, Stroke};
use crate::text::{TextEngine, TextLayout, TextStyle, TextAlign};
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId, CursorType, FrameInfo};
use crate::widget::Widget;
use crate::widgets::{Padding, ScrollBar, Orientation};

use super::types::{RichText, Span, LinkSpan};
use super::markdown::parse_markdown;

/// Rich text label widget with limited markdown support
///
/// Features:
/// - Limited markdown: bold, italic, strikethrough, links, bullets
/// - Optional text wrapping
/// - Line-based vertical scrolling (discrete)
/// - Pixel-based horizontal scrolling (smooth, when wrapping disabled)
/// - Clickable links with cursor changes
/// - Embedded scrollbars (overflow:auto behavior)
pub struct RichTextLabel {
    // Standard widget fields
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Content
    content: RichText,

    // Styling
    base_text_style: TextStyle,
    link_color: Color,
    strikethrough_color: Color,
    bullet_symbol: String,

    // Padding
    padding: Padding,
    bg_color: Option<Color>,

    // Wrapping
    wrap_enabled: bool, // true = word wrap, false = horizontal scroll

    // Scrolling state
    visible_start_line: u32,
    h_scroll_offset: f64,
    total_lines: u32,
    max_line_width: f64,
    viewport_width: f64,
    viewport_height: f64,

    // Mouse interaction
    hovered_link: Option<usize>,

    // Embedded scrollbars (overflow:auto)
    vscrollbar: Option<ScrollBar>,
    hscrollbar: Option<ScrollBar>,
    scrollbar_width: f32,
    show_scrollbars: bool,

    // Cached layout
    cached_layout: RefCell<Option<TextLayout>>,
    cached_layout_width: RefCell<Option<f32>>,

    // Callbacks
    on_link_clicked: Option<Box<dyn FnMut(String)>>,
    pending_commands: Vec<DeferredCommand>,
}

impl RichTextLabel {
    /// Create a new RichTextLabel from markdown
    pub fn new(markdown: &str) -> Self {
        let content = parse_markdown(markdown);
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style {
                // Use flex to fill available width
                flex_grow: 1.0,
                flex_shrink: 1.0,
                flex_basis: taffy::Dimension::auto(),
                ..Style::default()
            },
            content,
            base_text_style: TextStyle::new(),
            link_color: Color::rgb(0.3, 0.6, 0.9),
            strikethrough_color: Color::rgba(0.5, 0.5, 0.5, 0.7),
            bullet_symbol: "•".to_string(),
            padding: Padding::uniform(8.0),
            bg_color: None,
            wrap_enabled: true,
            visible_start_line: 0,
            h_scroll_offset: 0.0,
            total_lines: 0,
            max_line_width: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            hovered_link: None,
            vscrollbar: None,
            hscrollbar: None,
            scrollbar_width: 12.0,
            show_scrollbars: true,
            cached_layout: RefCell::new(None),
            cached_layout_width: RefCell::new(None),
            on_link_clicked: None,
            pending_commands: Vec::new(),
        }
    }

    /// Create from plain text (no markdown)
    pub fn from_plain(text: &str) -> Self {
        Self {
            content: RichText::from_plain(text),
            ..Self::new("")
        }
    }

    /// Enable or disable text wrapping (builder pattern)
    pub fn wrapping(mut self, enabled: bool) -> Self {
        self.wrap_enabled = enabled;
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set link color (builder pattern)
    pub fn link_color(mut self, color: Color) -> Self {
        self.link_color = color;
        self
    }

    /// Set strikethrough color (builder pattern)
    pub fn strikethrough_color(mut self, color: Color) -> Self {
        self.strikethrough_color = color;
        self
    }

    /// Set padding (builder pattern)
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Set background color (builder pattern)
    pub fn background(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set font size (builder pattern)
    pub fn font_size(mut self, size: f32) -> Self {
        self.base_text_style.font_size = size;
        *self.cached_layout.borrow_mut() = None;
        self
    }

    /// Set text color (builder pattern)
    pub fn text_color(mut self, color: Color) -> Self {
        self.base_text_style.text_color = color;
        self
    }

    /// Set layout style (builder pattern)
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    /// Set link clicked callback (builder pattern)
    pub fn on_link_clicked<F>(mut self, callback: F) -> Self
    where
        F: FnMut(String) + 'static,
    {
        self.on_link_clicked = Some(Box::new(callback));
        self
    }

    /// Set markdown content (runtime mutation)
    pub fn set_content(&mut self, markdown: &str) {
        self.content = parse_markdown(markdown);
        *self.cached_layout.borrow_mut() = None;
        self.visible_start_line = 0;
        self.h_scroll_offset = 0.0;
        self.dirty = true;
    }

    /// Enable or disable scrollbars (runtime mutation)
    pub fn set_scrollbars_enabled(&mut self, enabled: bool) {
        self.show_scrollbars = enabled;
        if !enabled {
            self.vscrollbar = None;
            self.hscrollbar = None;
        }
        self.dirty = true;
    }

    /// Scroll to a specific line (vertical)
    pub fn scroll_to_line(&mut self, line: u32) {
        let max_line = self.total_lines.saturating_sub(self.num_visible_lines());
        self.visible_start_line = line.min(max_line);

        // Update scrollbar value
        if let Some(ref mut vscroll) = self.vscrollbar {
            vscroll.set_value(self.visible_start_line as i32);
        }

        self.dirty = true;
    }

    /// Get current scroll position (line number)
    pub fn scroll_position(&self) -> u32 {
        self.visible_start_line
    }

    /// Get total number of lines
    pub fn total_lines(&self) -> u32 {
        self.total_lines
    }

    /// Get number of visible lines
    pub fn visible_lines(&self) -> u32 {
        self.num_visible_lines()
    }

    /// Set horizontal scroll offset (pixels)
    pub fn set_h_scroll(&mut self, offset: f64) {
        let max_scroll = self.max_h_scroll();
        self.h_scroll_offset = offset.clamp(-max_scroll, 0.0);

        // Update scrollbar value
        if let Some(ref mut hscroll) = self.hscrollbar {
            let normalized = if max_scroll > 0.0 {
                (-self.h_scroll_offset / max_scroll).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let max = hscroll.max();
            if max > 0 {
                hscroll.set_value((normalized * max as f64) as i32);
            }
        }

        self.dirty = true;
    }

    /// Get maximum horizontal scroll (for scrollbar range)
    pub fn max_h_scroll(&self) -> f64 {
        if self.wrap_enabled {
            0.0
        } else {
            (self.max_line_width - self.viewport_width).max(0.0)
        }
    }

    /// Calculate number of visible lines based on viewport height
    fn num_visible_lines(&self) -> u32 {
        if self.viewport_height <= 0.0 {
            println!("[RichTextLabel] num_visible_lines: viewport_height <= 0, returning 0");
            return 0;
        }

        let line_height = self.base_text_style.line_height_pixels() as f64;
        if line_height <= 0.0 {
            println!("[RichTextLabel] num_visible_lines: line_height <= 0, returning 0");
            return 0;
        }

        let visible = (self.viewport_height / line_height).ceil() as u32;
        println!("[RichTextLabel] num_visible_lines: {} / {} = {}", self.viewport_height, line_height, visible);
        visible
    }

    /// Measure widget with TextEngine (called by Window during layout)
    pub fn measure_with_engine(
        &self,
        engine: &mut TextEngine,
        known_dimensions: taffy::Size<Option<f32>>,
    ) -> Size {
        println!("\n========================================");
        println!("[RichTextLabel] measure_with_engine called");
        println!("  known_dimensions: {:?}", known_dimensions);
        println!("  current cached_layout exists: {}", self.cached_layout.borrow().is_some());
        println!("  current cached_layout_width: {:?}", *self.cached_layout_width.borrow());
        println!("  wrap_enabled: {}", self.wrap_enabled);
        println!("  Stack trace:");
        // Print a simple backtrace marker
        println!("  >>> Check window.rs layout code <<<");
        println!("========================================");

        let available_width = known_dimensions.width;

        // Calculate max width for text (accounting for padding and potential scrollbar)
        let max_width = available_width.map(|w| {
            let vscroll_w = if self.vscrollbar.is_some() { self.scrollbar_width } else { 0.0 };
            (w - self.padding.horizontal() - vscroll_w).max(0.0)
        });

        // Special case: if wrapping is enabled and we get a None width constraint,
        // but we already have a wrapped layout, keep the existing layout.
        // This handles Taffy calling measure multiple times with different constraints.
        let needs_reshape = if self.wrap_enabled && max_width.is_none() && self.cached_layout.borrow().is_some() {
            println!("[RichTextLabel] Wrapping enabled, no width constraint, keeping existing wrapped layout");
            false  // Keep existing wrapped layout
        } else {
            self.cached_layout.borrow().is_none()
                || *self.cached_layout_width.borrow() != max_width
        };

        if needs_reshape {
            println!("[RichTextLabel] Creating layout with max_width: {:?}", max_width);

            let wrap = if self.wrap_enabled {
                cosmic_text::Wrap::Word
            } else {
                cosmic_text::Wrap::None
            };

            let layout = engine.create_rich_layout(
                &self.content,
                &self.base_text_style,
                max_width,
                wrap,
            );

            println!("[RichTextLabel] Layout created, size: {:?}", layout.size());

            // Store the layout in the cached RefCell
            *self.cached_layout.borrow_mut() = Some(layout);
            *self.cached_layout_width.borrow_mut() = max_width;
        }

        // Update scroll metadata after layout is created/updated
        // This is needed for scrollbar visibility calculation in update()
        if let Some(layout) = self.cached_layout.borrow().as_ref() {
            // Count LAYOUT RUNS (wrapped lines), not buffer lines!
            // buffer().lines.len() gives logical lines before wrapping
            // layout_runs() gives actual visual lines after wrapping
            let total_lines = layout.buffer().layout_runs().count() as u32;
            let max_line_width = layout
                .buffer()
                .layout_runs()
                .map(|run| run.line_w as f64)
                .fold(0.0_f64, |max, w| max.max(w));

            // SAFETY: We need to mutate self but only have &self
            // This is safe because:
            // 1. These fields are not accessed during layout computation
            // 2. We're in a single-threaded context
            // 3. The widget system guarantees no concurrent access
            unsafe {
                let this = self as *const Self as *mut Self;
                (*this).total_lines = total_lines;
                (*this).max_line_width = max_line_width;
            }

            println!("[RichTextLabel] Updated scroll metadata: total_lines={}, max_line_width={}",
                     total_lines, max_line_width);
        }

        // Get the size from the cached layout
        let text_size = self
            .cached_layout
            .borrow()
            .as_ref()
            .map(|l| l.size())
            .unwrap_or_default();

        let result = if let Some(width) = known_dimensions.width {
            Size::new(
                width as f64,
                (text_size.height + self.padding.vertical() as f64).max(0.0),
            )
        } else {
            Size::new(
                text_size.width + self.padding.horizontal() as f64,
                text_size.height + self.padding.vertical() as f64,
            )
        };

        println!("[RichTextLabel] measure_with_engine returning: {:?}", result);
        result
    }

    /// Ensure layout is valid and up-to-date
    fn ensure_layout(&mut self, engine: &mut TextEngine, available_width: Option<f32>) {
        println!("[RichTextLabel] ensure_layout called with available_width: {:?}", available_width);

        let max_width = available_width.map(|w| {
            let vscroll_w = if self.vscrollbar.is_some() { self.scrollbar_width } else { 0.0 };
            (w - self.padding.horizontal() - vscroll_w).max(0.0)
        });

        println!("[RichTextLabel] max_width for text: {:?}", max_width);

        let needs_reshape = self.cached_layout.borrow().is_none()
            || *self.cached_layout_width.borrow() != max_width;

        println!("[RichTextLabel] needs_reshape: {}", needs_reshape);

        if needs_reshape {
            let wrap = if self.wrap_enabled {
                cosmic_text::Wrap::Word
            } else {
                cosmic_text::Wrap::None
            };

            let layout = engine.create_rich_layout(
                &self.content,
                &self.base_text_style,
                max_width,
                wrap,
            );

            println!("[RichTextLabel] layout created, size: {:?}", layout.size());

            // Update scroll metadata (count layout runs, not buffer lines)
            self.total_lines = layout.buffer().layout_runs().count() as u32;
            self.max_line_width = layout
                .buffer()
                .layout_runs()
                .map(|run| run.line_w as f64)
                .fold(0.0_f64, |max, w| max.max(w));

            println!("[RichTextLabel] total_lines: {}, max_line_width: {}", self.total_lines, self.max_line_width);

            *self.cached_layout.borrow_mut() = Some(layout);
            *self.cached_layout_width.borrow_mut() = max_width;
        }
    }

    /// Update scrollbars based on content size (overflow:auto)
    fn update_scrollbars(&mut self) {
        if !self.show_scrollbars {
            self.vscrollbar = None;
            self.hscrollbar = None;
            return;
        }

        let visible_lines = self.num_visible_lines();
        let needs_vscroll = self.total_lines > visible_lines;
        let needs_hscroll = !self.wrap_enabled && self.max_line_width > self.viewport_width;

        println!("[RichTextLabel] update_scrollbars:");
        println!("  total_lines: {}", self.total_lines);
        println!("  visible_lines: {}", visible_lines);
        println!("  needs_vscroll: {} ({} > {})", needs_vscroll, self.total_lines, visible_lines);
        println!("  max_line_width: {}", self.max_line_width);
        println!("  viewport_width: {}", self.viewport_width);
        println!("  needs_hscroll: {}", needs_hscroll);

        // Create or destroy vertical scrollbar
        if needs_vscroll {
            if self.vscrollbar.is_none() {
                println!("[RichTextLabel] Creating vertical scrollbar (total={}, visible={})",
                         self.total_lines, visible_lines);
                let vscroll = ScrollBar::vertical(
                    0,
                    self.total_lines as i32,
                    self.num_visible_lines() as i32,
                );
                self.vscrollbar = Some(vscroll);
            } else {
                // Update range
                let total_lines = self.total_lines as i32;
                let visible_lines = self.num_visible_lines() as i32;
                if let Some(ref mut vscroll) = self.vscrollbar {
                    vscroll.set_range(0, total_lines);
                    vscroll.set_page_size(visible_lines);
                }
            }
        } else {
            if self.vscrollbar.is_some() {
                println!("[RichTextLabel] Destroying vertical scrollbar (no longer needed)");
            }
            self.vscrollbar = None;
        }

        // Create or destroy horizontal scrollbar
        if needs_hscroll {
            if self.hscrollbar.is_none() {
                let max_scroll = self.max_h_scroll() as i32;
                let hscroll = ScrollBar::horizontal(0, max_scroll.max(100), 20);
                self.hscrollbar = Some(hscroll);
            } else {
                // Update range
                let max_scroll = self.max_h_scroll() as i32;
                if let Some(ref mut hscroll) = self.hscrollbar {
                    hscroll.set_range(0, max_scroll.max(100));
                }
            }
        } else {
            self.hscrollbar = None;
            self.h_scroll_offset = 0.0;
        }

        self.position_scrollbars();
    }

    /// Position scrollbars on widget edges
    fn position_scrollbars(&mut self) {
        let hscroll_height = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        // Position vertical scrollbar on right edge
        if let Some(ref mut vscroll) = self.vscrollbar {
            vscroll.set_bounds(Rect::new(
                Point::new(
                    self.bounds.origin.x + self.bounds.size.width - self.scrollbar_width as f64,
                    self.bounds.origin.y,
                ),
                Size::new(
                    self.scrollbar_width as f64,
                    self.bounds.size.height - hscroll_height,
                ),
            ));
        }

        // Position horizontal scrollbar on bottom edge
        let vscroll_width = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        if let Some(ref mut hscroll) = self.hscrollbar {
            hscroll.set_bounds(Rect::new(
                Point::new(
                    self.bounds.origin.x,
                    self.bounds.origin.y + self.bounds.size.height - self.scrollbar_width as f64,
                ),
                Size::new(
                    self.bounds.size.width - vscroll_width,
                    self.scrollbar_width as f64,
                ),
            ));
        }
    }

    /// Hit test for links
    fn hit_test_link(&self, position: Point) -> Option<usize> {
        let layout = self.cached_layout.borrow();
        let layout = layout.as_ref()?;

        // Calculate content area (excluding scrollbars and padding)
        let vscroll_w = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };
        let hscroll_h = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        let content_origin = Point::new(
            self.bounds.origin.x + self.padding.left as f64,
            self.bounds.origin.y + self.padding.top as f64,
        );

        let content_rect = Rect::new(
            content_origin,
            Size::new(
                self.bounds.size.width - self.padding.horizontal() as f64 - vscroll_w,
                self.bounds.size.height - self.padding.vertical() as f64 - hscroll_h,
            ),
        );

        // Check if position is within content area
        if !content_rect.contains(position) {
            return None;
        }

        // Convert to layout coordinates
        let line_height = self.base_text_style.line_height_pixels() as f64;
        let text_origin = Point::new(
            content_origin.x + self.h_scroll_offset,
            content_origin.y - (self.visible_start_line as f64 * line_height),
        );

        let rel_x = (position.x - text_origin.x) as f32;
        let rel_y = (position.y - text_origin.y) as f32;

        // Use cosmic-text hit testing
        let buffer = layout.buffer();
        let cursor = buffer.hit(rel_x, rel_y)?;
        let char_index = cursor.index;

        // Check if this character is within any link span
        for (idx, link) in self.content.links.iter().enumerate() {
            if link.char_range.contains(&char_index) {
                return Some(idx);
            }
        }

        None
    }

    /// Draw strikethrough lines for strikethrough spans
    fn draw_strikethrough(&self, ctx: &mut PaintContext, text_origin: Point) {
        let layout = self.cached_layout.borrow();
        let Some(layout) = layout.as_ref() else {
            return;
        };

        let buffer = layout.buffer();
        let mut lines_drawn = 0;

        for span in &self.content.spans {
            if !span.attrs.strikethrough {
                continue;
            }

            println!("[RichTextLabel] Drawing strikethrough for span: {:?}", span.range);

            // Find glyphs in this span's byte range
            for run in buffer.layout_runs() {
                for glyph in run.glyphs.iter() {
                    // Check if glyph is within span range
                    let glyph_start = glyph.start;
                    let glyph_end = glyph.end;

                    if glyph_start >= span.range.start && glyph_end <= span.range.end {
                        // Draw strikethrough line at 60% down from top (middle of lowercase letters)
                        let y = text_origin.y + run.line_y as f64 + run.line_height as f64 * 0.6;
                        let x_start = text_origin.x + glyph.x as f64;
                        let x_end = x_start + glyph.w as f64;

                        println!("  Drawing line from ({}, {}) to ({}, {})", x_start, y, x_end, y);

                        ctx.draw_line(
                            Point::new(x_start, y),
                            Point::new(x_end, y),
                            Stroke::new(Color::rgb(1.0, 0.2, 0.2), 2.0),  // Bright red, thicker
                        );
                        lines_drawn += 1;
                    }
                }
            }
        }

        if lines_drawn > 0 {
            println!("[RichTextLabel] Drew {} strikethrough lines", lines_drawn);
        }
    }

    /// Draw underline for hovered link
    fn draw_link_underline(&self, ctx: &mut PaintContext, text_origin: Point, link_idx: usize) {
        let layout = self.cached_layout.borrow();
        let Some(layout) = layout.as_ref() else {
            return;
        };

        let Some(link) = self.content.links.get(link_idx) else {
            return;
        };

        let buffer = layout.buffer();

        // Convert char range to byte range for hit testing
        let text_chars: Vec<char> = self.content.text.chars().collect();
        let start_char = link.char_range.start;
        let end_char = link.char_range.end;

        if start_char >= text_chars.len() || end_char > text_chars.len() {
            return;
        }

        let start_byte: usize = text_chars.iter().take(start_char).map(|c| c.len_utf8()).sum();
        let end_byte: usize = text_chars.iter().take(end_char).map(|c| c.len_utf8()).sum();

        // Draw underlines for glyphs in link range
        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                if glyph.start >= start_byte && glyph.end <= end_byte {
                    let y = text_origin.y + run.line_y as f64 + run.line_height as f64 * 0.95;
                    let x_start = text_origin.x + glyph.x as f64;
                    let x_end = x_start + glyph.w as f64;

                    ctx.draw_line(
                        Point::new(x_start, y),
                        Point::new(x_end, y),
                        Stroke::new(self.link_color, 1.0),
                    );
                }
            }
        }
    }
}

// Widget trait implementation
impl Widget for RichTextLabel {
    // Essential widget methods (manually implemented to allow custom set_bounds)
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    // Custom set_bounds to update scrollbars when widget is resized
    fn set_bounds(&mut self, bounds: Rect) {
        println!("[RichTextLabel] set_bounds called: {:?}", bounds);

        let bounds_changed = self.bounds != bounds;
        self.bounds = bounds;

        if bounds_changed {
            println!("[RichTextLabel] Bounds changed, will update viewport and scrollbars");

            // Update viewport dimensions
            let vscroll_w = if self.vscrollbar.is_some() {
                self.scrollbar_width as f64
            } else {
                0.0
            };
            let hscroll_h = if self.hscrollbar.is_some() {
                self.scrollbar_width as f64
            } else {
                0.0
            };

            self.viewport_width =
                (self.bounds.size.width - self.padding.horizontal() as f64 - vscroll_w).max(0.0);
            self.viewport_height =
                (self.bounds.size.height - self.padding.vertical() as f64 - hscroll_h).max(0.0);

            println!("[RichTextLabel] viewport updated: {}x{}", self.viewport_width, self.viewport_height);

            // Update scrollbars based on new viewport
            self.update_scrollbars();
        }
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn needs_measure(&self) -> bool {
        true
    }

    fn update(&mut self, frame_info: &FrameInfo) {
        println!("\n========================================");
        println!("[RichTextLabel] update called");
        println!("  bounds: {:?}", self.bounds);
        println!("  total_lines (before update_scrollbars): {}", self.total_lines);
        println!("  max_line_width (before update_scrollbars): {}", self.max_line_width);

        // Update viewport dimensions
        let vscroll_w = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };
        let hscroll_h = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        self.viewport_width =
            (self.bounds.size.width - self.padding.horizontal() as f64 - vscroll_w).max(0.0);
        self.viewport_height =
            (self.bounds.size.height - self.padding.vertical() as f64 - hscroll_h).max(0.0);

        println!("  viewport: {}x{}", self.viewport_width, self.viewport_height);
        println!("  padding: {:?}", self.padding);
        println!("========================================\n");

        // Update scrollbars based on content/viewport
        self.update_scrollbars();

        // Update scrollbar widgets
        if let Some(ref mut vscroll) = self.vscrollbar {
            vscroll.update(frame_info);
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            hscroll.update(frame_info);
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        println!("[RichTextLabel] paint called, bounds: {:?}", self.bounds);
        println!("[RichTextLabel] has cached layout: {}", self.cached_layout.borrow().is_some());

        // Draw background
        if let Some(bg) = self.bg_color {
            ctx.draw_rect(self.bounds, bg);
        }

        // Calculate content viewport (excluding scrollbars and padding)
        let vscroll_w = if self.vscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };
        let hscroll_h = if self.hscrollbar.is_some() {
            self.scrollbar_width as f64
        } else {
            0.0
        };

        let content_rect = Rect::new(
            Point::new(
                self.bounds.origin.x + self.padding.left as f64,
                self.bounds.origin.y + self.padding.top as f64,
            ),
            Size::new(
                (self.bounds.size.width - self.padding.horizontal() as f64 - vscroll_w).max(0.0),
                (self.bounds.size.height - self.padding.vertical() as f64 - hscroll_h).max(0.0),
            ),
        );

        // Push clip rect for content area
        ctx.push_clip(content_rect);

        // Calculate text origin with scroll offsets
        let line_height = self.base_text_style.line_height_pixels() as f64;
        let text_origin = Point::new(
            content_rect.origin.x + self.h_scroll_offset,
            content_rect.origin.y - (self.visible_start_line as f64 * line_height),
        );

        // Draw text layout
        if let Some(layout) = self.cached_layout.borrow().as_ref() {
            ctx.draw_layout(layout, text_origin, self.base_text_style.text_color);

            // Draw strikethrough
            self.draw_strikethrough(ctx, text_origin);

            // Draw link underlines for hovered links
            if let Some(link_idx) = self.hovered_link {
                self.draw_link_underline(ctx, text_origin, link_idx);
            }
        }

        ctx.pop_clip();

        // Paint scrollbars on top
        if let Some(ref vscroll) = self.vscrollbar {
            vscroll.paint(ctx);
        }
        if let Some(ref hscroll) = self.hscrollbar {
            hscroll.paint(ctx);
        }

        // Register hitbox for content area
        ctx.register_hitbox(self.id, content_rect);
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn preferred_cursor(&self) -> Option<CursorType> {
        // Check if hovering over link
        if self.hovered_link.is_some() {
            return Some(CursorType::Pointer);
        }

        None
    }

    fn dispatch_mouse_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::MouseMove(e) => MouseHandler::on_mouse_move(self, e),
            InputEventEnum::MouseDown(e) => MouseHandler::on_mouse_down(self, e),
            InputEventEnum::MouseUp(e) => MouseHandler::on_mouse_up(self, e),
            _ => EventResponse::Ignored,
        }
    }

    fn dispatch_wheel_event(&mut self, event: &mut WheelEvent) -> EventResponse {
        // Convert wheel delta to line count for vertical scrolling
        let line_height = self.base_text_style.line_height_pixels() as f64;
        if line_height <= 0.0 {
            return EventResponse::Ignored;
        }

        let lines_delta = (event.delta.y / line_height).round() as i32;
        if lines_delta != 0 {
            let max_line = self
                .total_lines
                .saturating_sub(self.num_visible_lines())
                .max(0);
            let new_line =
                (self.visible_start_line as i32 + lines_delta).clamp(0, max_line as i32) as u32;

            if new_line != self.visible_start_line {
                self.visible_start_line = new_line;

                // Update vertical scrollbar value
                if let Some(ref mut vscroll) = self.vscrollbar {
                    vscroll.set_value(new_line as i32);
                }

                self.dirty = true;
            }
        }

        // Handle horizontal scrolling if wrapping disabled
        if !self.wrap_enabled && event.delta.x.abs() > 0.001 {
            let new_offset = self.h_scroll_offset - event.delta.x;
            self.set_h_scroll(new_offset);
        }

        EventResponse::Handled
    }

    fn on_message(&mut self, message: &GuiMessage) -> Vec<DeferredCommand> {
        if let GuiMessage::Custom {
            signal_type,
            data,
            source,
        } = message
        {
            if signal_type == "value_changed" {
                // Vertical scrollbar changed
                if let Some(ref vscroll) = self.vscrollbar {
                    if *source == vscroll.id() {
                        if let Some(value) = data.downcast_ref::<i32>() {
                            self.visible_start_line = (*value).max(0) as u32;
                            self.dirty = true;
                        }
                    }
                }

                // Horizontal scrollbar changed
                if let Some(ref hscroll) = self.hscrollbar {
                    if *source == hscroll.id() {
                        if let Some(value) = data.downcast_ref::<i32>() {
                            let max_scroll = self.max_h_scroll();
                            if max_scroll > 0.0 {
                                let max_val = hscroll.max() as f64;
                                if max_val > 0.0 {
                                    let normalized = (*value as f64) / max_val;
                                    self.h_scroll_offset = -(normalized * max_scroll);
                                    self.dirty = true;
                                }
                            }
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

// MouseHandler trait implementation
impl MouseHandler for RichTextLabel {
    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check if mouse is over scrollbars first
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseMove(event.clone());
                return vscroll.dispatch_mouse_event(&mut input_event);
            }
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseMove(event.clone());
                return hscroll.dispatch_mouse_event(&mut input_event);
            }
        }

        // Check content area for links
        let hovered_link = self.hit_test_link(event.position);

        if hovered_link != self.hovered_link {
            self.hovered_link = hovered_link;
            self.dirty = true;
        }

        EventResponse::PassThrough
    }

    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Check scrollbars first
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseDown(event.clone());
                return vscroll.dispatch_mouse_event(&mut input_event);
            }
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseDown(event.clone());
                return hscroll.dispatch_mouse_event(&mut input_event);
            }
        }

        // Check if click on link
        if let Some(link_idx) = self.hit_test_link(event.position) {
            if let Some(link) = self.content.links.get(link_idx) {
                let url = link.url.clone();

                // Call callback
                if let Some(ref mut callback) = self.on_link_clicked {
                    callback(url.clone());
                }

                // Emit signal
                self.pending_commands.push(DeferredCommand {
                    target: self.id,
                    message: GuiMessage::Custom {
                        source: self.id,
                        signal_type: "link_clicked".to_string(),
                        data: Box::new(url),
                    },
                });

                return EventResponse::Handled;
            }
        }

        EventResponse::Ignored
    }

    fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Forward to scrollbars if they exist
        if let Some(ref mut vscroll) = self.vscrollbar {
            if vscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseUp(event.clone());
                return vscroll.dispatch_mouse_event(&mut input_event);
            }
        }
        if let Some(ref mut hscroll) = self.hscrollbar {
            if hscroll.bounds().contains(event.position) {
                let mut input_event = InputEventEnum::MouseUp(event.clone());
                return hscroll.dispatch_mouse_event(&mut input_event);
            }
        }

        EventResponse::Ignored
    }

    fn on_mouse_enter(&mut self, _event: &mut MouseEvent) -> EventResponse {
        EventResponse::PassThrough
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if self.hovered_link.is_some() {
            self.hovered_link = None;
            self.dirty = true;
        }
        EventResponse::PassThrough
    }
}
