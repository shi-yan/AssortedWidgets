// Code Editor Widget - A text editor for code with virtualized rendering

use std::any::Any;
use std::sync::Arc;
use taffy::Style;

use crate::paint::primitives::Color;
use crate::paint::types::ShapeStyle;
use crate::paint::PaintContext;
use crate::text::TextStyle;
use crate::types::{DirtyLevel, Point, Rect, Size, WidgetId};
use crate::widget::Widget;
use crate::WidgetState;

mod config;
mod model;
mod layout_cache;

pub use config::{EditorConfig, WrapMode};
pub use model::EditorModel;
use layout_cache::LayoutCache;

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

    // === Scrolling State ===
    /// First visible line (0-indexed logical line)
    scroll_line: usize,

    /// Vertical scroll offset in pixels (for smooth scrolling within lines)
    scroll_offset_y: f32,

    /// Horizontal scroll offset in pixels (for no-wrap mode)
    scroll_offset_x: f32,

    // === Viewport ===
    /// Cached viewport size (updated in layout())
    viewport_size: Size,

    /// Line height in pixels (calculated from font metrics)
    line_height: f32,

    // === State ===
    is_focused: bool,
    is_hovered: bool,
}

impl CodeEditor {
    /// Create a new empty code editor
    pub fn new() -> Self {
        Self {
            state: WidgetState::new(),
            layout_style: Style::default(),
            model: EditorModel::new(),
            config: EditorConfig::default(),
            layout_cache: LayoutCache::new(),
            scroll_line: 0,
            scroll_offset_y: 0.0,
            scroll_offset_x: 0.0,
            viewport_size: Size::new(800.0, 600.0),
            line_height: 20.0, // Will be updated from font metrics
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

        // If size changed, invalidate all layouts
        if (old_size.width - self.viewport_size.width).abs() > 1.0
            || (old_size.height - self.viewport_size.height).abs() > 1.0
        {
            self.layout_cache.clear();
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
            ShapeStyle::solid(Color::rgb(0.11, 0.12, 0.13)), // Dark background
        );

        // Calculate visible line range
        let visible_lines = self.visible_line_range();

        // Draw visible lines
        let mut y = bounds.origin.y as f32 - self.scroll_offset_y;
        let x = bounds.origin.x as f32 - self.scroll_offset_x;

        for line_idx in visible_lines.clone() {
            if let Some(line_text) = self.model.line(line_idx) {
                // For Phase 1, use simple text rendering without layout cache
                // (Will be optimized with layout cache in later iterations)

                let text_pos = Point::new(x as f64 + 10.0, y as f64 + 4.0);

                // Draw line text
                let text_style = TextStyle::new()
                    .size(self.config.font_size)
                    .color(Color::rgb(0.85, 0.85, 0.85));

                ctx.draw_text(
                    &line_text.trim_end_matches('\n'),
                    &text_style,
                    text_pos,
                    None, // No max width
                );
            }

            y += self.line_height;

            // Stop if we're below the viewport
            if y > (bounds.origin.y + bounds.size.height) as f32 {
                break;
            }
        }

        // Draw cursor (if focused)
        if self.is_focused {
            if let Some((line_idx, _)) = self.model.cursor().line_info() {
                if line_idx >= visible_lines.start && line_idx < visible_lines.end {
                    // Simple cursor rendering (vertical line)
                    let cursor_x = x + 10.0; // TODO: Calculate actual X position from text layout
                    let cursor_y = bounds.origin.y as f32 + (line_idx - self.scroll_line) as f32 * self.line_height - self.scroll_offset_y;

                    let cursor_rect = Rect::new(
                        Point::new(cursor_x as f64, cursor_y as f64 + 4.0),
                        Size::new(2.0, self.line_height as f64 - 4.0),
                    );

                    ctx.draw_styled_rect(cursor_rect, ShapeStyle::solid(Color::WHITE));
                }
            }
        }
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
