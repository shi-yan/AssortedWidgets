// Code Editor Widget - A text editor for code with virtualized rendering

use std::any::Any;
use std::sync::Arc;
use std::cell::RefCell;
use taffy::Style;

use crate::paint::primitives::Color;
use crate::paint::types::{Border, ShapeStyle};
use crate::paint::PaintContext;
use crate::text::{TextStyle, TextEngine, Truncate};
use crate::types::{DirtyLevel, Point, Rect, Size, WidgetId};
use crate::widget::Widget;
use crate::WidgetState;
use crate::event::{EventResponse, InputEventEnum, KeyEvent, MouseEvent, WheelEvent};
use crate::event::handlers::{KeyboardHandler, MouseHandler, WheelHandler};

mod config;
mod model;
mod layout_cache;
mod minimap;
mod theme;
mod syntax;
mod gutter;
mod folding;

pub use config::{EditorConfig, WrapMode, MonospaceFontConfig};
pub use model::EditorModel;
pub use minimap::{MinimapManager, CharSheet, MinimapPalette};
pub use theme::EditorTheme;
pub use syntax::SyntaxHighlighter;
pub use folding::{FoldManager, FoldableRegion, FoldKind};
use layout_cache::LayoutCache;
use gutter::Gutter;

/// Information about a wrapped line segment
#[derive(Debug, Clone)]
struct WrappedSegment {
    /// Start character index within the logical line (byte offset)
    start: usize,
    /// End character index within the logical line (byte offset)
    end: usize,
    /// Visual width in pixels
    width: f32,
}

/// Information about how a logical line is wrapped into visual lines
#[derive(Debug, Clone)]
struct WrappedLine {
    /// Logical line index
    line_idx: usize,
    /// Segments (visual lines) for this logical line
    segments: Vec<WrappedSegment>,
}

/// Code editor widget with virtualized rendering for large files
///
/// Phase 1 features:
/// - Rope-based text storage (ropey)
/// - Virtualized line rendering (only visible lines)
/// - LRU cache for text layouts
/// - Basic editing (insert, delete, cursor movement)
/// - Scrolling support
pub struct CodeEditor {
    // === Widget Essentials ===
    state: WidgetState,
    layout_style: Style,

    // === Data Model ===
    model: EditorModel,
    config: EditorConfig,

    // === Layout Cache ===
    layout_cache: LayoutCache,

    // === Minimap (Phase 2) ===
    /// Minimap manager (uses RefCell for interior mutability in paint())
    minimap: Option<RefCell<MinimapManager>>,

    // === Phase 3: UI Elements ===
    /// Color theme
    theme: EditorTheme,

    /// Gutter (line numbers)
    gutter: Gutter,

    /// Syntax highlighter
    highlighter: Box<dyn SyntaxHighlighter>,

    /// Current active line (where cursor is)
    current_line: usize,

    // === Scrolling State ===
    /// First visible line (0-indexed logical line)
    scroll_line: usize,

    /// Vertical scroll offset in pixels (for smooth scrolling within lines)
    scroll_offset_y: f32,

    /// Horizontal scroll offset in pixels (for no-wrap mode)
    scroll_offset_x: f32,

    /// Maximum line width (for horizontal scrollbar in NoWrap mode)
    max_line_width: RefCell<f32>,

    // === Text Wrapping ===
    /// Cached wrapped line information (only used in SoftWrap mode)
    wrapped_lines: RefCell<Vec<WrappedLine>>,

    /// Whether wrapped lines cache is valid
    wrap_cache_valid: RefCell<bool>,

    // === Scrollbar State ===
    /// Whether the user is dragging the horizontal scrollbar
    dragging_h_scrollbar: bool,

    /// Mouse drag start position (for scrollbar dragging)
    drag_start_x: f32,

    /// Scroll offset when drag started
    drag_start_scroll: f32,

    // === Viewport ===
    /// Cached viewport size (updated in layout())
    viewport_size: Size,

    /// Line height in pixels (calculated from font metrics)
    line_height: f32,

    // === Text Measurement ===
    /// Text engine for measuring text width (not used for rendering)
    text_engine: RefCell<TextEngine>,

    /// Cached monospace character width (calculated once)
    char_width: RefCell<Option<f32>>,

    // === State ===
    is_focused: bool,
    is_hovered: bool,
}

impl CodeEditor {
    /// Create a new empty code editor
    pub fn new() -> Self {
        let theme = EditorTheme::dark();

        // Create minimap with theme-based palette
        let palette = theme.to_minimap_palette();
        let minimap = MinimapManager::new(100); // 100 pixels wide
        // TODO: Set minimap palette once the API supports it

        // Initialize config and calculate base_advance
        let mut config = EditorConfig::default();

        // Calculate base advance width for monospace grid
        let mut text_engine = TextEngine::new();
        let style = TextStyle::new()
            .size(config.font_size)
            .family(&config.font_family);
        let layout = text_engine.create_layout("0", &style, None, Truncate::None);
        config.monospace_config.base_advance = Some(layout.width() as f32);

        Self {
            state: WidgetState::new(),
            layout_style: Style::default(),
            model: EditorModel::new(),
            config,
            layout_cache: LayoutCache::new(),
            minimap: Some(RefCell::new(minimap)),
            theme,
            gutter: Gutter::new(),
            highlighter: Box::new(syntax::RegexHighlighter::rust()),
            current_line: 0,
            scroll_line: 0,
            scroll_offset_y: 0.0,
            scroll_offset_x: 0.0,
            max_line_width: RefCell::new(0.0),
            wrapped_lines: RefCell::new(Vec::new()),
            wrap_cache_valid: RefCell::new(false),
            dragging_h_scrollbar: false,
            drag_start_x: 0.0,
            drag_start_scroll: 0.0,
            viewport_size: Size::new(800.0, 600.0),
            line_height: 20.0, // Will be updated from font metrics
            text_engine: RefCell::new(text_engine),
            char_width: RefCell::new(None),
            is_focused: false,
            is_hovered: false,
        }
    }

    /// Create a code editor with initial text
    pub fn with_text(text: &str) -> Self {
        let mut editor = Self::new();
        editor.model.set_text(text);
        editor
    }

    /// Set the editor configuration
    pub fn with_config(mut self, config: EditorConfig) -> Self {
        self.config = config;
        self
    }

    /// Get a reference to the text content
    pub fn text(&self) -> String {
        self.model.text()
    }

    /// Set the text content
    pub fn set_text(&mut self, text: &str) {
        self.model.set_text(text);
        self.layout_cache.clear();
        self.scroll_line = 0;
        self.scroll_offset_y = 0.0;
        *self.wrap_cache_valid.borrow_mut() = false;
        self.state.dirty = DirtyLevel::Visual;
    }

    /// Scroll to a specific line
    pub fn scroll_to_line(&mut self, line: usize) {
        let max_line = self.model.len_lines().saturating_sub(1);
        self.scroll_line = line.min(max_line);
        self.scroll_offset_y = 0.0;
        self.state.dirty = DirtyLevel::Visual;
    }

    // ========================================================================
    // Internal Helpers
    // ========================================================================

    /// Calculate wrapped segments for a single line
    fn calculate_wrapped_segments(&self, line_text: &str, max_width: f32) -> Vec<WrappedSegment> {
        let mut segments = Vec::new();

        if line_text.is_empty() {
            // Empty line has one empty segment
            segments.push(WrappedSegment {
                start: 0,
                end: 0,
                width: 0.0,
            });
            return segments;
        }

        let line_without_newline = line_text.trim_end_matches('\n');
        let char_width = self.get_char_width();

        let mut current_start = 0;
        let mut current_width = 0.0;
        let mut last_space_byte = None;
        let mut last_space_width = 0.0;

        for (byte_idx, ch) in line_without_newline.char_indices() {
            use crate::widgets::code_editor::config::MonospaceFontConfig;
            let ch_width = char_width * MonospaceFontConfig::get_width_multiplier(ch);

            // Check if adding this character would exceed max width
            if current_width + ch_width > max_width && current_start < byte_idx {
                // We need to wrap here
                let wrap_at = if let Some(space_byte) = last_space_byte {
                    // Wrap at last space
                    let end = space_byte;
                    let width = last_space_width;
                    last_space_byte = None;
                    (end, width)
                } else {
                    // No space found, force wrap at current position
                    (byte_idx, current_width)
                };

                segments.push(WrappedSegment {
                    start: current_start,
                    end: wrap_at.0,
                    width: wrap_at.1,
                });

                current_start = wrap_at.0;
                current_width = ch_width;

                // Skip leading spaces on new line
                if ch.is_whitespace() {
                    current_start = byte_idx + ch.len_utf8();
                    current_width = 0.0;
                }
            } else {
                current_width += ch_width;
            }

            // Track last space for word wrapping
            if ch.is_whitespace() {
                last_space_byte = Some(byte_idx);
                last_space_width = current_width;
            }
        }

        // Add final segment
        if current_start < line_without_newline.len() || segments.is_empty() {
            segments.push(WrappedSegment {
                start: current_start,
                end: line_without_newline.len(),
                width: current_width,
            });
        }

        segments
    }

    /// Calculate wrapped lines for all visible lines
    fn update_wrapped_lines(&self) {
        let text_area_width = self.viewport_size.width as f32
            - self.gutter.width
            - 20.0; // Subtract padding

        let mut wrapped = Vec::new();

        for line_idx in 0..self.model.len_lines() {
            if let Some(line_text) = self.model.line(line_idx) {
                let segments = self.calculate_wrapped_segments(&line_text, text_area_width);
                wrapped.push(WrappedLine {
                    line_idx,
                    segments,
                });
            }
        }

        *self.wrapped_lines.borrow_mut() = wrapped;
        *self.wrap_cache_valid.borrow_mut() = true;
    }

    /// Get the number of visual lines (accounting for wrapping)
    fn visual_line_count(&self) -> usize {
        match self.config.wrap_mode {
            WrapMode::NoWrap => self.model.len_lines(),
            WrapMode::SoftWrap => {
                if !*self.wrap_cache_valid.borrow() {
                    self.update_wrapped_lines();
                }

                self.wrapped_lines.borrow()
                    .iter()
                    .map(|w| w.segments.len())
                    .sum()
            }
        }
    }

    /// Calculate the range of visible lines based on viewport and scroll position
    fn visible_line_range(&self) -> std::ops::Range<usize> {
        let total_lines = self.model.len_lines();
        if total_lines == 0 {
            return 0..0;
        }

        // Calculate how many lines fit in the viewport
        let lines_in_viewport = (self.viewport_size.height / self.line_height as f64).ceil() as usize;

        let start = self.scroll_line;
        let end = (start + lines_in_viewport + 1).min(total_lines); // +1 for partial line at bottom

        start..end
    }

    /// Get the monospace character width (cached)
    fn get_char_width(&self) -> f32 {
        // Use the pre-calculated base_advance from config
        if let Some(width) = self.config.monospace_config.base_advance {
            return width;
        }

        // Fallback: measure on-the-fly (shouldn't happen normally)
        let style = TextStyle::new()
            .size(self.config.font_size)
            .family(&self.config.font_family);

        let layout = self.text_engine.borrow_mut().create_layout(
            "0", // Use "0" as reference character for monospace fonts
            &style,
            None,
            Truncate::None,
        );

        layout.width() as f32
    }

    /// Calculate the maximum line width (for horizontal scrollbar)
    fn calculate_max_line_width(&self) -> f32 {
        let mut max_width: f32 = 0.0;

        for line_idx in 0..self.model.len_lines() {
            if let Some(line_text) = self.model.line(line_idx) {
                let line_without_newline = line_text.trim_end_matches('\n');
                let width = self.measure_text_width(line_without_newline);
                max_width = max_width.max(width);
            }
        }

        max_width
    }

    /// Render horizontal scrollbar (for NoWrap mode)
    fn paint_h_scrollbar(&self, ctx: &mut PaintContext, bounds: Rect) {
        const SCROLLBAR_HEIGHT: f64 = 12.0;
        const SCROLLBAR_MARGIN: f64 = 2.0;

        let text_area_width = bounds.size.width - self.gutter.width as f64;

        // Only show scrollbar if content is wider than viewport
        let max_width = *self.max_line_width.borrow();
        if max_width <= text_area_width as f32 {
            return;
        }

        // Scrollbar background
        let scrollbar_rect = Rect::new(
            Point::new(
                bounds.origin.x + self.gutter.width as f64,
                bounds.origin.y + bounds.size.height - SCROLLBAR_HEIGHT - SCROLLBAR_MARGIN,
            ),
            Size::new(text_area_width, SCROLLBAR_HEIGHT),
        );

        ctx.draw_styled_rect(
            scrollbar_rect,
            ShapeStyle::solid(Color::rgba(0.2, 0.2, 0.2, 0.5)),
        );

        // Scrollbar thumb
        let content_width = *self.max_line_width.borrow() as f64;
        let thumb_width = (text_area_width / content_width * text_area_width).max(30.0);
        let max_scroll = (content_width - text_area_width).max(0.0);
        let scroll_ratio = if max_scroll > 0.0 {
            (self.scroll_offset_x as f64 / max_scroll).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let thumb_x = scroll_ratio * (text_area_width - thumb_width);

        let thumb_rect = Rect::new(
            Point::new(
                scrollbar_rect.origin.x + thumb_x,
                scrollbar_rect.origin.y + 2.0,
            ),
            Size::new(thumb_width, SCROLLBAR_HEIGHT - 4.0),
        );

        ctx.draw_styled_rect(
            thumb_rect,
            ShapeStyle::rounded(Color::rgba(0.5, 0.5, 0.5, 0.8), 4.0),
        );
    }

    /// Measure the width of text using monospace grid rules
    fn measure_text_width(&self, text: &str) -> f32 {
        use crate::widgets::code_editor::config::MonospaceFontConfig;

        let base_advance = self.get_char_width();
        let mut total_width = 0.0;

        // Debug: log character widths for mixed content
        let has_special = text.chars().any(|ch| {
            MonospaceFontConfig::is_cjk(ch) || MonospaceFontConfig::is_emoji(ch)
        });

        if has_special && text.len() < 50 {
            eprint!("[measure_text_width] '{}' => ", text.escape_debug());
            for ch in text.chars() {
                let multiplier = MonospaceFontConfig::get_width_multiplier(ch);
                total_width += base_advance * multiplier;
                if MonospaceFontConfig::is_cjk(ch) {
                    eprint!("[{}:CJK:2x]", ch);
                } else if MonospaceFontConfig::is_emoji(ch) {
                    eprint!("[{}:Emoji:2x]", ch);
                } else {
                    eprint!("[{}:1x]", ch);
                }
            }
            eprintln!(" = {:.2}px", total_width);
        } else {
            for ch in text.chars() {
                let multiplier = MonospaceFontConfig::get_width_multiplier(ch);
                total_width += base_advance * multiplier;
            }
        }

        total_width
    }

    /// Handle text insertion at cursor
    fn insert_text(&mut self, text: &str) {
        self.model.insert_at_cursor(text);
        self.model.update_cursor_caches();

        // Invalidate affected layouts
        if let Some((line_idx, _)) = self.model.cursor().line_info() {
            self.layout_cache.invalidate_range(line_idx..line_idx + 1);
        }

        self.state.dirty = DirtyLevel::Visual;
    }

    /// Handle backspace key
    fn handle_backspace(&mut self) {
        self.model.delete_before_cursor();
        self.model.update_cursor_caches();

        // Invalidate affected layouts
        if let Some((line_idx, _)) = self.model.cursor().line_info() {
            self.layout_cache.invalidate_range(line_idx.saturating_sub(1)..line_idx + 1);
        }

        self.state.dirty = DirtyLevel::Visual;
    }

    /// Handle delete key
    fn handle_delete(&mut self) {
        self.model.delete_at_cursor();
        self.model.update_cursor_caches();

        // Invalidate affected layouts
        if let Some((line_idx, _)) = self.model.cursor().line_info() {
            self.layout_cache.invalidate_range(line_idx..line_idx + 1);
        }

        self.state.dirty = DirtyLevel::Visual;
    }

    /// Handle cursor movement
    fn move_cursor_left(&mut self) {
        self.model.cursor_mut().move_left();
        self.model.update_cursor_caches();
        self.state.dirty = DirtyLevel::Visual;
    }

    fn move_cursor_right(&mut self) {
        let max_chars = self.model.len_chars();
        self.model.cursor_mut().move_right(max_chars);
        self.model.update_cursor_caches();
        self.state.dirty = DirtyLevel::Visual;
    }

    fn move_cursor_to_start(&mut self) {
        self.model.cursor_mut().move_to_start();
        self.model.update_cursor_caches();
        self.state.dirty = DirtyLevel::Visual;
    }

    fn move_cursor_to_end(&mut self) {
        let max_chars = self.model.len_chars();
        self.model.cursor_mut().move_to_end(max_chars);
        self.model.update_cursor_caches();
        self.state.dirty = DirtyLevel::Visual;
    }

    /// Paint in NoWrap mode (with horizontal scrolling)
    fn paint_nowrap(&self, ctx: &mut PaintContext, bounds: Rect, text_area_x: f32) {
        // Calculate max line width for horizontal scrollbar (using interior mutability)
        *self.max_line_width.borrow_mut() = self.calculate_max_line_width();

        // Calculate visible line range
        let visible_lines = self.visible_line_range();

        // Draw gutter (line numbers)
        self.gutter.paint(
            ctx,
            bounds,
            &self.theme,
            visible_lines.clone(),
            self.scroll_offset_y,
            self.line_height,
            Some(self.current_line),
        );

        // Draw current line highlight (in text area)
        let current_line_y = bounds.origin.y as f32
            + (self.current_line as f32 * self.line_height)
            - (self.scroll_line as f32 * self.line_height)
            - self.scroll_offset_y;

        if self.current_line >= visible_lines.start && self.current_line < visible_lines.end {
            let highlight_rect = Rect::new(
                Point::new(text_area_x as f64, current_line_y as f64),
                Size::new(
                    (bounds.size.width - self.gutter.width as f64).max(0.0),
                    self.line_height as f64,
                ),
            );
            ctx.draw_styled_rect(
                highlight_rect,
                ShapeStyle::solid(self.theme.current_line_background),
            );
        }

        // Draw visible lines
        let mut y = bounds.origin.y as f32 - self.scroll_offset_y;
        let x = text_area_x - self.scroll_offset_x + 10.0; // 10px padding

        for line_idx in visible_lines.clone() {
            if let Some(line_text) = self.model.line(line_idx) {
                let text_pos = Point::new(x as f64, y as f64 + 4.0);

                let text_style = TextStyle::new()
                    .size(self.config.font_size)
                    .family(&self.config.font_family)
                    .color(self.theme.default_text);

                ctx.draw_text(
                    &line_text.trim_end_matches('\n'),
                    &text_style,
                    text_pos,
                    None,
                );
            }

            y += self.line_height;

            if y > (bounds.origin.y + bounds.size.height) as f32 {
                break;
            }
        }

        // Draw cursor (if focused)
        if self.is_focused {
            if let Some((line_idx, byte_offset)) = self.model.cursor().line_info() {
                if line_idx >= visible_lines.start && line_idx < visible_lines.end {
                    let mut cursor_x = x;

                    if let Some(line_text) = self.model.line(line_idx) {
                        let text_before_cursor = &line_text[..byte_offset.min(line_text.len())];
                        let text_width = self.measure_text_width(text_before_cursor);
                        cursor_x = x + text_width;
                    }

                    let cursor_y = bounds.origin.y as f32 + (line_idx - self.scroll_line) as f32 * self.line_height - self.scroll_offset_y;

                    let cursor_rect = Rect::new(
                        Point::new(cursor_x as f64, cursor_y as f64 + 4.0),
                        Size::new(2.0, self.line_height as f64 - 4.0),
                    );

                    ctx.draw_styled_rect(cursor_rect, ShapeStyle::solid(self.theme.cursor));
                }
            }
        }

        // Draw horizontal scrollbar
        self.paint_h_scrollbar(ctx, bounds);
    }

    /// Paint in SoftWrap mode (with line wrapping)
    fn paint_wrapped(&self, ctx: &mut PaintContext, bounds: Rect, text_area_x: f32) {
        // Update wrapped lines cache if needed
        if !*self.wrap_cache_valid.borrow() {
            self.update_wrapped_lines();
        }

        let visible_lines = self.visible_line_range();

        // Draw gutter (line numbers)
        self.gutter.paint(
            ctx,
            bounds,
            &self.theme,
            visible_lines.clone(),
            self.scroll_offset_y,
            self.line_height,
            Some(self.current_line),
        );

        // Draw current line highlight
        let current_line_y = bounds.origin.y as f32
            + (self.current_line as f32 * self.line_height)
            - (self.scroll_line as f32 * self.line_height)
            - self.scroll_offset_y;

        if self.current_line >= visible_lines.start && self.current_line < visible_lines.end {
            // In wrapped mode, we need to highlight all visual lines for the current logical line
            let wrapped = self.wrapped_lines.borrow();
            if let Some(wrapped_line) = wrapped.get(self.current_line) {
                for seg_idx in 0..wrapped_line.segments.len() {
                    let seg_y = current_line_y + (seg_idx as f32 * self.line_height);
                    let highlight_rect = Rect::new(
                        Point::new(text_area_x as f64, seg_y as f64),
                        Size::new(
                            (bounds.size.width - self.gutter.width as f64).max(0.0),
                            self.line_height as f64,
                        ),
                    );
                    ctx.draw_styled_rect(
                        highlight_rect,
                        ShapeStyle::solid(self.theme.current_line_background),
                    );
                }
            }
        }

        // Draw wrapped line segments
        let mut y = bounds.origin.y as f32 - self.scroll_offset_y;
        let x = text_area_x + 10.0; // 10px padding (no horizontal scroll in wrap mode)

        let wrapped = self.wrapped_lines.borrow();
        for line_idx in visible_lines.clone() {
            if let Some(wrapped_line) = wrapped.get(line_idx) {
                if let Some(line_text) = self.model.line(line_idx) {
                    let line_without_newline = line_text.trim_end_matches('\n');

                    for segment in &wrapped_line.segments {
                        let segment_text = &line_without_newline[segment.start..segment.end];
                        let text_pos = Point::new(x as f64, y as f64 + 4.0);

                        let text_style = TextStyle::new()
                            .size(self.config.font_size)
                            .family(&self.config.font_family)
                            .color(self.theme.default_text);

                        ctx.draw_text(
                            segment_text,
                            &text_style,
                            text_pos,
                            None,
                        );

                        y += self.line_height;

                        if y > (bounds.origin.y + bounds.size.height) as f32 {
                            return;
                        }
                    }
                }
            } else {
                // No wrapped info for this line, skip it
                y += self.line_height;
            }
        }

        // Draw cursor (if focused)
        if self.is_focused {
            if let Some((line_idx, byte_offset)) = self.model.cursor().line_info() {
                if line_idx >= visible_lines.start && line_idx < visible_lines.end {
                    // Find which segment contains the cursor
                    if let Some(wrapped_line) = wrapped.get(line_idx) {
                        if let Some(line_text) = self.model.line(line_idx) {
                            let mut seg_y_offset = 0.0;
                            let mut found = false;

                            for segment in &wrapped_line.segments {
                                if byte_offset >= segment.start && byte_offset <= segment.end {
                                    // Cursor is in this segment
                                    let text_before_cursor = &line_text[segment.start..byte_offset.min(segment.end)];
                                    let text_width = self.measure_text_width(text_before_cursor);
                                    let cursor_x = x + text_width;

                                    let cursor_y = bounds.origin.y as f32
                                        + (line_idx - self.scroll_line) as f32 * self.line_height
                                        - self.scroll_offset_y
                                        + seg_y_offset;

                                    let cursor_rect = Rect::new(
                                        Point::new(cursor_x as f64, cursor_y as f64 + 4.0),
                                        Size::new(2.0, self.line_height as f64 - 4.0),
                                    );

                                    ctx.draw_styled_rect(cursor_rect, ShapeStyle::solid(self.theme.cursor));
                                    found = true;
                                    break;
                                }
                                seg_y_offset += self.line_height;
                            }

                            if !found {
                                // Fallback: cursor at end of last segment
                                if let Some(last_seg) = wrapped_line.segments.last() {
                                    let cursor_x = x + last_seg.width;
                                    let seg_y_offset = (wrapped_line.segments.len() - 1) as f32 * self.line_height;
                                    let cursor_y = bounds.origin.y as f32
                                        + (line_idx - self.scroll_line) as f32 * self.line_height
                                        - self.scroll_offset_y
                                        + seg_y_offset;

                                    let cursor_rect = Rect::new(
                                        Point::new(cursor_x as f64, cursor_y as f64 + 4.0),
                                        Size::new(2.0, self.line_height as f64 - 4.0),
                                    );

                                    ctx.draw_styled_rect(cursor_rect, ShapeStyle::solid(self.theme.cursor));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Paint the minimap on the right side of the editor (helper function)
    fn paint_minimap_impl(ctx: &mut PaintContext, bounds: Rect, minimap: &MinimapManager, theme: &EditorTheme) {
        let minimap_width = 100.0; // pixels
        let minimap_x = bounds.origin.x + bounds.size.width - minimap_width;
        let minimap_y = bounds.origin.y;

        // Draw minimap background
        let minimap_rect = Rect::new(
            Point::new(minimap_x, minimap_y),
            Size::new(minimap_width, bounds.size.height),
        );
        ctx.draw_styled_rect(
            minimap_rect,
            ShapeStyle::solid(Color::rgb(0.08, 0.09, 0.10)), // Slightly darker than editor
        );

        // Get minimap palette
        let palette_data = theme.to_minimap_palette();

        // Render minimap pages
        // For simplicity, we draw each pixel as a small rectangle
        // (This is not the most efficient approach, but works for Phase 2)
        let pixel_size = 1.0; // Each pixel is 1x1 screen pixels

        for page_num in 0..minimap.page_count() {
            if let Some(page) = minimap.get_page(page_num) {
                let page_y_offset = (page.start_line * 2) as f64; // 2 pixels per line

                // Only render pages that are visible
                if page_y_offset > bounds.size.height {
                    break; // Page is below viewport
                }

                // Draw pixels
                for y in 0..page.height_pixels.min((bounds.size.height as usize).saturating_sub(page_y_offset as usize)) {
                    for x in 0..page.width_pixels.min(minimap_width as usize) {
                        if let Some((intensity, color_idx)) = page.get_pixel(x, y) {
                            // Skip fully transparent pixels
                            if intensity == 0 {
                                continue;
                            }

                            // Get color from palette
                            let color = palette_data.get(color_idx)
                                .map(|entry| {
                                    // Blend foreground color with intensity
                                    let alpha = intensity as f32 / 255.0;
                                    Color::rgba(
                                        entry.fg.r * alpha,
                                        entry.fg.g * alpha,
                                        entry.fg.b * alpha,
                                        1.0,
                                    )
                                })
                                .unwrap_or(Color::WHITE);

                            let pixel_rect = Rect::new(
                                Point::new(
                                    minimap_x + x as f64 * pixel_size,
                                    minimap_y + page_y_offset + y as f64 * pixel_size,
                                ),
                                Size::new(pixel_size, pixel_size),
                            );

                            ctx.draw_rect(pixel_rect, color);
                        }
                    }
                }
            }
        }

        // Draw border
        ctx.draw_styled_rect(
            minimap_rect,
            ShapeStyle::solid(Color::TRANSPARENT)
                .with_border(Border::new(Color::rgb(0.3, 0.3, 0.3), 1.0)),
        );
    }
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for CodeEditor {
    fn id(&self) -> WidgetId {
        self.state.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.state.id = id;
    }

    fn bounds(&self) -> Rect {
        self.state.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        let old_size = self.viewport_size;
        self.viewport_size = Size::new(bounds.width(), bounds.height());

        // If size changed, invalidate all layouts and wrap cache
        if (old_size.width - self.viewport_size.width).abs() > 1.0
            || (old_size.height - self.viewport_size.height).abs() > 1.0
        {
            self.layout_cache.clear();
            *self.wrap_cache_valid.borrow_mut() = false;
        }

        self.state.bounds = bounds;
        self.state.dirty = DirtyLevel::Layout;
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bounds = self.state.bounds;

        // Draw background
        ctx.draw_styled_rect(
            bounds,
            ShapeStyle::solid(self.theme.background),
        );

        // Update minimap before rendering (Phase 2)
        // Minimap uses RefCell for interior mutability since it's just a rendering cache
        if let Some(ref minimap) = self.minimap {
            let center_line = self.scroll_line + (self.viewport_size.height / self.line_height as f64 / 2.0) as usize;
            minimap.borrow_mut().update(&self.model, center_line);
        }

        // Text area starts after the gutter
        let text_area_x = bounds.origin.x as f32 + self.gutter.width;

        // Handle different wrap modes
        match self.config.wrap_mode {
            WrapMode::NoWrap => {
                self.paint_nowrap(ctx, bounds, text_area_x);
            }
            WrapMode::SoftWrap => {
                self.paint_wrapped(ctx, bounds, text_area_x);
            }
        }

        // Draw minimap (Phase 2)
        if let Some(ref minimap) = self.minimap {
            Self::paint_minimap_impl(ctx, bounds, &minimap.borrow(), &self.theme);
        }
    }

    // ========================================================================
    // Event Handling (makes editor interactive)
    // ========================================================================

    fn is_interactive(&self) -> bool {
        true // Editor accepts mouse and keyboard input
    }

    fn is_focusable(&self) -> bool {
        true // Editor can be focused
    }

    fn on_focus_gained(&mut self) {
        self.is_focused = true;
        self.state.dirty = DirtyLevel::Visual; // Redraw to show cursor
    }

    fn on_focus_lost(&mut self) {
        self.is_focused = false;
        self.state.dirty = DirtyLevel::Visual; // Redraw to hide cursor
    }

    fn dispatch_mouse_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::MouseDown(e) => self.on_mouse_down(e),
            InputEventEnum::MouseUp(e) => self.on_mouse_up(e),
            InputEventEnum::MouseMove(e) => self.on_mouse_move(e),
            _ => EventResponse::Ignored,
        }
    }

    fn dispatch_key_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::KeyDown(e) => self.on_key_down(e),
            InputEventEnum::KeyUp(e) => self.on_key_up(e),
            _ => EventResponse::Ignored,
        }
    }

    fn on_wheel(&mut self, event: &mut WheelEvent) -> EventResponse {
        WheelHandler::on_wheel(self, event)
    }
}

// ============================================================================
// Mouse Handler Implementation
// ============================================================================

impl MouseHandler for CodeEditor {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        // Focus the editor on click
        if !self.is_focused {
            self.is_focused = true;
        }

        let bounds = self.state.bounds;
        let text_area_x = bounds.origin.x as f32 + self.gutter.width;

        // Check if clicking on horizontal scrollbar (NoWrap mode only)
        if self.config.wrap_mode == WrapMode::NoWrap {
            const SCROLLBAR_HEIGHT: f64 = 12.0;
            const SCROLLBAR_MARGIN: f64 = 2.0;

            let scrollbar_y = bounds.origin.y + bounds.size.height - SCROLLBAR_HEIGHT - SCROLLBAR_MARGIN;
            let scrollbar_x = bounds.origin.x + self.gutter.width as f64;
            let text_area_width = bounds.size.width - self.gutter.width as f64;

            // Check if click is in scrollbar area
            let max_width = *self.max_line_width.borrow();
            if event.position.y >= scrollbar_y && event.position.y <= scrollbar_y + SCROLLBAR_HEIGHT
                && event.position.x >= scrollbar_x && event.position.x <= scrollbar_x + text_area_width
                && max_width > text_area_width as f32
            {
                // Clicked on scrollbar - start dragging
                self.dragging_h_scrollbar = true;
                self.drag_start_x = event.position.x as f32;
                self.drag_start_scroll = self.scroll_offset_x;

                // Calculate thumb position and check if clicked on thumb
                let content_width = max_width as f64;
                let thumb_width = (text_area_width / content_width * text_area_width).max(30.0);
                let max_scroll = (content_width - text_area_width).max(0.0);
                let scroll_ratio = if max_scroll > 0.0 {
                    (self.scroll_offset_x as f64 / max_scroll).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let thumb_x = scrollbar_x + scroll_ratio * (text_area_width - thumb_width);

                // If clicked outside thumb, jump to that position
                if event.position.x < thumb_x || event.position.x > thumb_x + thumb_width {
                    let click_ratio = ((event.position.x - scrollbar_x) / text_area_width).clamp(0.0, 1.0);
                    self.scroll_offset_x = (click_ratio * max_scroll) as f32;
                    self.scroll_offset_x = self.scroll_offset_x.clamp(0.0, max_scroll as f32);
                }

                self.state.dirty = DirtyLevel::Visual;
                return EventResponse::Handled;
            }
        }

        // Calculate which line was clicked

        // Convert click position to line index
        let relative_y = event.position.y as f32 - bounds.origin.y as f32 + self.scroll_offset_y;
        let clicked_line = (relative_y / self.line_height) as usize;
        let line_in_doc = self.scroll_line + clicked_line;

        // Clamp to valid line range
        let max_line = self.model.len_lines().saturating_sub(1);
        let target_line = line_in_doc.min(max_line);

        // Update current line
        self.current_line = target_line;

        // Calculate column position from click X coordinate
        let line_start_char = self.model.rope().line_to_char(target_line);
        let mut target_char_pos = line_start_char;

        if let Some(line_text) = self.model.line(target_line) {
            // Calculate relative X from text area start
            let relative_x = event.position.x as f32 - text_area_x - 10.0 + self.scroll_offset_x; // 10px is the padding

            // Find character position at click X by measuring text width
            let line_without_newline = line_text.trim_end_matches('\n');

            // Use monospace character width for efficient binary search-like approach
            let char_width = self.get_char_width();
            let estimated_col = (relative_x / char_width).max(0.0) as usize;

            // Measure actual width to find exact position
            let mut best_col = 0;
            let mut best_distance = f32::MAX;

            // Check a range around the estimated position for accuracy
            let start_col = estimated_col.saturating_sub(2);
            let end_col = (estimated_col + 3).min(line_without_newline.chars().count());

            for col in start_col..=end_col {
                let text_before = &line_without_newline[..line_without_newline.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line_without_newline.len())];
                let width = self.measure_text_width(text_before);
                let distance = (width - relative_x).abs();

                if distance < best_distance {
                    best_distance = distance;
                    best_col = col;
                }
            }

            target_char_pos = line_start_char + best_col;
        }

        // Always move cursor when clicking (don't check if line changed)
        self.model.cursor_mut().set_char_pos(target_char_pos);
        self.model.update_cursor_caches();

        self.state.dirty = DirtyLevel::Visual;
        EventResponse::Handled
    }

    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        if self.dragging_h_scrollbar {
            let bounds = self.state.bounds;
            let text_area_width = bounds.size.width - self.gutter.width as f64;
            let content_width = *self.max_line_width.borrow() as f64;
            let max_scroll = (content_width - text_area_width).max(0.0);

            if max_scroll > 0.0 {
                // Calculate scroll based on drag delta
                let drag_delta = event.position.x as f32 - self.drag_start_x;
                let scroll_delta = drag_delta * (content_width / text_area_width) as f32;

                self.scroll_offset_x = (self.drag_start_scroll + scroll_delta).clamp(0.0, max_scroll as f32);
                self.state.dirty = DirtyLevel::Visual;
            }

            return EventResponse::Handled;
        }

        EventResponse::PassThrough
    }

    fn on_mouse_up(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if self.dragging_h_scrollbar {
            self.dragging_h_scrollbar = false;
            return EventResponse::Handled;
        }

        EventResponse::PassThrough
    }

    fn on_mouse_enter(&mut self, _event: &mut MouseEvent) -> EventResponse {
        self.is_hovered = true;
        EventResponse::PassThrough
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        self.is_hovered = false;
        self.dragging_h_scrollbar = false; // Stop dragging if mouse leaves
        EventResponse::PassThrough
    }
}

// ============================================================================
// Keyboard Handler Implementation
// ============================================================================

impl KeyboardHandler for CodeEditor {
    fn on_key_down(&mut self, event: &mut KeyEvent) -> EventResponse {
        use crate::event::input::{Key, NamedKey};

        match &event.key {
            // Character input
            Key::Character(ch) => {
                let text = ch.to_string();
                self.insert_text(&text);

                // Update current line based on cursor position
                if let Some((line_idx, _)) = self.model.cursor().line_info() {
                    self.current_line = line_idx;
                }

                EventResponse::Handled
            }

            // Named keys
            Key::Named(named) => match named {
                NamedKey::Backspace => {
                    self.handle_backspace();

                    // Update current line
                    if let Some((line_idx, _)) = self.model.cursor().line_info() {
                        self.current_line = line_idx;
                    }

                    EventResponse::Handled
                }

                NamedKey::Delete => {
                    self.handle_delete();

                    // Update current line
                    if let Some((line_idx, _)) = self.model.cursor().line_info() {
                        self.current_line = line_idx;
                    }

                    EventResponse::Handled
                }

                NamedKey::Enter => {
                    self.insert_text("\n");

                    // Update current line
                    if let Some((line_idx, _)) = self.model.cursor().line_info() {
                        self.current_line = line_idx;
                    }

                    EventResponse::Handled
                }

                NamedKey::ArrowLeft => {
                    self.move_cursor_left();

                    // Update current line
                    if let Some((line_idx, _)) = self.model.cursor().line_info() {
                        self.current_line = line_idx;
                    }

                    EventResponse::Handled
                }

                NamedKey::ArrowRight => {
                    self.move_cursor_right();

                    // Update current line
                    if let Some((line_idx, _)) = self.model.cursor().line_info() {
                        self.current_line = line_idx;
                    }

                    EventResponse::Handled
                }

                NamedKey::Home => {
                    self.move_cursor_to_start();

                    // Update current line
                    if let Some((line_idx, _)) = self.model.cursor().line_info() {
                        self.current_line = line_idx;
                    }

                    EventResponse::Handled
                }

                NamedKey::End => {
                    self.move_cursor_to_end();

                    // Update current line
                    if let Some((line_idx, _)) = self.model.cursor().line_info() {
                        self.current_line = line_idx;
                    }

                    EventResponse::Handled
                }

                _ => EventResponse::Ignored,
            },
        }
    }
}

// ============================================================================
// Wheel Handler Implementation
// ============================================================================

impl WheelHandler for CodeEditor {
    fn on_wheel(&mut self, event: &mut WheelEvent) -> EventResponse {
        // Vertical scrolling
        let delta_y = event.delta.y as f32;

        // Update scroll offset
        self.scroll_offset_y -= delta_y;

        // Adjust scroll_line if we've scrolled past a line boundary
        while self.scroll_offset_y < 0.0 && self.scroll_line > 0 {
            self.scroll_line -= 1;
            self.scroll_offset_y += self.line_height;
        }

        let max_line = self.model.len_lines().saturating_sub(1);
        while self.scroll_offset_y >= self.line_height && self.scroll_line < max_line {
            self.scroll_line += 1;
            self.scroll_offset_y -= self.line_height;
        }

        // Clamp scroll offset
        self.scroll_offset_y = self.scroll_offset_y.max(0.0);

        self.state.dirty = DirtyLevel::Visual;
        EventResponse::Handled
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_create_empty_editor() {
        let editor = CodeEditor::new();
        assert_eq!(editor.text(), "");
        assert_eq!(editor.model.len_lines(), 1); // Rope always has at least 1 line
        assert_eq!(editor.scroll_line, 0);
    }

    #[test]
    fn test_create_with_text() {
        let text = "Line 1\nLine 2\nLine 3";
        let editor = CodeEditor::with_text(text);
        assert_eq!(editor.text(), text);
        assert_eq!(editor.model.len_lines(), 3);
    }

    #[test]
    fn test_set_text() {
        let mut editor = CodeEditor::new();
        let text = "Hello\nWorld";
        editor.set_text(text);

        assert_eq!(editor.text(), text);
        assert_eq!(editor.model.len_lines(), 2);
        assert_eq!(editor.scroll_line, 0);
        assert!(editor.layout_cache.is_empty());
    }

    #[test]
    fn test_scroll_to_line() {
        let mut editor = CodeEditor::new();
        let text = (0..100).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        editor.set_text(&text);

        editor.scroll_to_line(50);
        assert_eq!(editor.scroll_line, 50);

        // Test clamping
        editor.scroll_to_line(500);
        assert_eq!(editor.scroll_line, 99); // 100 lines = 0-99
    }

    #[test]
    fn test_visible_line_range() {
        let mut editor = CodeEditor::new();
        let text = (0..100).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        editor.set_text(&text);

        // Set viewport size to fit ~10 lines
        editor.line_height = 20.0;
        editor.viewport_size = Size::new(800.0, 200.0); // 200px / 20px = 10 lines

        let range = editor.visible_line_range();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 11); // 10 + 1 for partial line

        // Scroll down
        editor.scroll_line = 50;
        let range = editor.visible_line_range();
        assert_eq!(range.start, 50);
        assert_eq!(range.end, 61);
    }

    #[test]
    fn test_large_file_virtualization() {
        let mut editor = CodeEditor::new();

        // Create a 10,000 line file
        let text = (0..10000)
            .map(|i| format!("This is line number {} with some text content", i))
            .collect::<Vec<_>>()
            .join("\n");

        editor.set_text(&text);
        assert_eq!(editor.model.len_lines(), 10000);

        // Scroll to middle of file
        editor.scroll_to_line(5000);
        assert_eq!(editor.scroll_line, 5000);

        // Set realistic viewport
        editor.line_height = 20.0;
        editor.viewport_size = Size::new(800.0, 600.0); // ~30 lines visible

        let visible_range = editor.visible_line_range();

        // Should only render visible lines
        let visible_count = visible_range.end - visible_range.start;
        assert!(visible_count <= 32); // 30 visible + 2 for partial lines
        assert_eq!(visible_range.start, 5000);

        // Verify we can access lines efficiently
        let start = Instant::now();
        for line_idx in visible_range {
            let _ = editor.model.line(line_idx);
        }
        let elapsed = start.elapsed();

        // Should be very fast (< 1ms for ~30 lines)
        assert!(elapsed.as_millis() < 10);
    }

    #[test]
    fn test_basic_text_insertion() {
        let mut editor = CodeEditor::new();
        editor.set_text("Hello");

        // Move cursor to end
        editor.move_cursor_to_end();
        assert_eq!(editor.model.cursor().char_pos(), 5);

        // Insert text
        editor.insert_text(" World");
        assert_eq!(editor.text(), "Hello World");
        assert_eq!(editor.model.cursor().char_pos(), 11);
    }

    #[test]
    fn test_backspace_delete() {
        let mut editor = CodeEditor::new();
        editor.set_text("Hello World");

        // Position cursor at 'W' (position 6)
        editor.model.cursor_mut().set_char_pos(6);
        editor.model.update_cursor_caches();

        // Delete (should delete 'W')
        editor.handle_delete();
        assert_eq!(editor.text(), "Hello orld");

        // Backspace (should delete space)
        editor.handle_backspace();
        assert_eq!(editor.text(), "Helloorld");
    }

    #[test]
    fn test_cursor_movement() {
        let mut editor = CodeEditor::new();
        editor.set_text("Hello World");

        // Start at position 0
        assert_eq!(editor.model.cursor().char_pos(), 0);

        // Move right
        editor.move_cursor_right();
        assert_eq!(editor.model.cursor().char_pos(), 1);

        // Move to end
        editor.move_cursor_to_end();
        assert_eq!(editor.model.cursor().char_pos(), 11);

        // Move left
        editor.move_cursor_left();
        assert_eq!(editor.model.cursor().char_pos(), 10);

        // Move to start
        editor.move_cursor_to_start();
        assert_eq!(editor.model.cursor().char_pos(), 0);
    }

    #[test]
    fn test_multiline_editing() {
        let mut editor = CodeEditor::new();
        editor.set_text("Line 1\nLine 2\nLine 3");

        assert_eq!(editor.model.len_lines(), 3);
        assert_eq!(editor.model.line(0), Some("Line 1\n".to_string()));
        assert_eq!(editor.model.line(1), Some("Line 2\n".to_string()));
        assert_eq!(editor.model.line(2), Some("Line 3".to_string()));
    }

    #[test]
    fn test_layout_cache_invalidation() {
        let mut editor = CodeEditor::new();
        editor.set_text("Line 1\nLine 2\nLine 3");

        // Initially cache should be empty
        assert_eq!(editor.layout_cache.len(), 0);

        // After text change, cache should be cleared
        editor.set_text("New text");
        assert_eq!(editor.layout_cache.len(), 0);

        // After resize, cache should be cleared
        editor.set_bounds(Rect::new(
            Point::new(0.0, 0.0),
            Size::new(1000.0, 800.0),
        ));
        assert_eq!(editor.layout_cache.len(), 0);
    }

    #[test]
    fn test_scrolling_bounds() {
        let mut editor = CodeEditor::new();
        let text = (0..50).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        editor.set_text(&text);

        // Scroll to valid position
        editor.scroll_to_line(25);
        assert_eq!(editor.scroll_line, 25);
        assert_eq!(editor.scroll_offset_y, 0.0);

        // Scroll beyond end (should clamp)
        editor.scroll_to_line(100);
        assert_eq!(editor.scroll_line, 49); // Max line is 49 (0-indexed, 50 lines)

        // Scroll to start
        editor.scroll_to_line(0);
        assert_eq!(editor.scroll_line, 0);
    }

    #[test]
    fn test_empty_file() {
        let mut editor = CodeEditor::new();
        editor.set_text("");

        let range = editor.visible_line_range();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 0); // Empty range for empty file
    }

    #[test]
    fn test_single_line_file() {
        let mut editor = CodeEditor::new();
        editor.set_text("Single line");

        assert_eq!(editor.model.len_lines(), 1);
        let range = editor.visible_line_range();
        assert_eq!(range.start, 0);
        assert!(range.end >= 1);
    }

    #[test]
    fn test_configuration() {
        let config = EditorConfig {
            wrap_mode: WrapMode::SoftWrap,
            tab_size: 2,
            show_line_numbers: false,
            highlight_current_line: false,
            font_size: 12.0,
            font_family: "Courier".to_string(),
            monospace_config: Default::default(),
        };

        let editor = CodeEditor::new().with_config(config.clone());
        assert_eq!(editor.config.tab_size, 2);
        assert_eq!(editor.config.font_size, 12.0);
        assert_eq!(editor.config.wrap_mode, WrapMode::SoftWrap);
    }

    #[test]
    fn test_widget_bounds() {
        let mut editor = CodeEditor::new();
        let bounds = Rect::new(Point::new(10.0, 20.0), Size::new(800.0, 600.0));

        editor.set_bounds(bounds);

        assert_eq!(editor.bounds(), bounds);
        assert_eq!(editor.viewport_size.width, 800.0);
        assert_eq!(editor.viewport_size.height, 600.0);
    }

    #[test]
    fn test_dirty_tracking() {
        let mut editor = CodeEditor::new();

        // Initially clean
        assert_eq!(editor.dirty_level(), DirtyLevel::Clean);

        // Setting text marks dirty
        editor.set_text("Hello");
        assert_eq!(editor.dirty_level(), DirtyLevel::Visual);

        // Clear dirty
        editor.set_dirty_level(DirtyLevel::Clean);
        assert_eq!(editor.dirty_level(), DirtyLevel::Clean);

        // Scrolling marks dirty
        editor.scroll_to_line(10);
        assert_eq!(editor.dirty_level(), DirtyLevel::Visual);
    }
}
