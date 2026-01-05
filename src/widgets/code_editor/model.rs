// Data model for the code editor

use std::ops::Range;
use ropey::Rope;
use crate::widgets::Cursor;
use crate::paint::primitives::Color;

/// The underlying data model for the code editor
///
/// This struct manages:
/// - Text content (using ropey::Rope for efficient O(log N) line access)
/// - Cursor positions (supporting multiple cursors)
/// - Syntax highlighting spans (stub for Phase 3)
#[derive(Debug)]
pub struct EditorModel {
    /// The text content stored as a rope for efficient editing
    rope: Rope,

    /// Active cursors (supports multiple cursors for future multi-cursor editing)
    cursors: Vec<Cursor>,

    /// Selection ranges for each cursor (parallel to cursors vec)
    /// None = no selection, Some(start) = selection from start to cursor position
    selections: Vec<Option<usize>>,

    /// Syntax highlighting spans (stub for Phase 3)
    /// Each span is (byte_range, color)
    syntax_spans: Vec<(Range<usize>, Color)>,

    /// Dirty flag - set when text changes
    dirty: bool,
}

impl EditorModel {
    /// Create a new empty editor model
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            cursors: vec![Cursor::new()],
            selections: vec![None],
            syntax_spans: Vec::new(),
            dirty: false,
        }
    }

    /// Create an editor model with initial text
    pub fn with_text(text: &str) -> Self {
        let mut model = Self::new();
        model.set_text(text);
        model
    }

    // ========================================================================
    // Text Access
    // ========================================================================

    /// Get a reference to the underlying rope
    #[inline]
    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Get the total number of characters in the text
    #[inline]
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the total number of lines in the text
    #[inline]
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Check if the text is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get the text content as a String (allocates)
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Get a specific line as a String (allocates)
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.len_lines() {
            Some(self.rope.line(line_idx).to_string())
        } else {
            None
        }
    }

    /// Get a slice of the rope as a String (allocates)
    pub fn slice(&self, range: Range<usize>) -> Option<String> {
        if range.end <= self.len_chars() {
            Some(self.rope.slice(range.start..range.end).to_string())
        } else {
            None
        }
    }

    // ========================================================================
    // Text Modification
    // ========================================================================

    /// Replace all text with new content
    pub fn set_text(&mut self, text: &str) {
        self.rope = Rope::from_str(text);
        self.dirty = true;

        // Reset cursors to start
        self.cursors = vec![Cursor::new()];
        self.selections = vec![None];

        // Clear syntax highlighting
        self.syntax_spans.clear();
    }

    /// Insert text at a specific character position
    pub fn insert(&mut self, char_idx: usize, text: &str) {
        if char_idx <= self.len_chars() {
            self.rope.insert(char_idx, text);
            self.dirty = true;

            // Update cursor positions that are after the insertion point
            let inserted_len = text.chars().count();
            for cursor in &mut self.cursors {
                if cursor.char_pos() >= char_idx {
                    cursor.set_char_pos(cursor.char_pos() + inserted_len);
                }
            }
        }
    }

    /// Delete a range of characters
    pub fn delete(&mut self, range: Range<usize>) {
        let end = range.end.min(self.len_chars());
        if range.start < end {
            self.rope.remove(range.start..end);
            self.dirty = true;

            // Update cursor positions
            let deleted_len = end - range.start;
            for cursor in &mut self.cursors {
                let pos = cursor.char_pos();
                if pos >= end {
                    // Cursor is after deleted range
                    cursor.set_char_pos(pos - deleted_len);
                } else if pos > range.start {
                    // Cursor is inside deleted range
                    cursor.set_char_pos(range.start);
                }
            }
        }
    }

    /// Insert text at the primary cursor position
    pub fn insert_at_cursor(&mut self, text: &str) {
        if let Some(cursor) = self.cursors.first() {
            let pos = cursor.char_pos();
            self.insert(pos, text);
        }
    }

    /// Delete character before the primary cursor (backspace)
    pub fn delete_before_cursor(&mut self) {
        if let Some(cursor) = self.cursors.first() {
            let pos = cursor.char_pos();
            if pos > 0 {
                self.delete(pos - 1..pos);
            }
        }
    }

    /// Delete character at the primary cursor (delete key)
    pub fn delete_at_cursor(&mut self) {
        if let Some(cursor) = self.cursors.first() {
            let pos = cursor.char_pos();
            if pos < self.len_chars() {
                self.delete(pos..pos + 1);
            }
        }
    }

    // ========================================================================
    // Cursor Access
    // ========================================================================

    /// Get a reference to the primary cursor
    #[inline]
    pub fn cursor(&self) -> &Cursor {
        &self.cursors[0]
    }

    /// Get a mutable reference to the primary cursor
    #[inline]
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursors[0]
    }

    /// Get all cursors
    #[inline]
    pub fn cursors(&self) -> &[Cursor] {
        &self.cursors
    }

    /// Get the primary selection range (if any)
    #[inline]
    pub fn selection(&self) -> Option<usize> {
        self.selections[0]
    }

    /// Set the primary selection start position
    #[inline]
    pub fn set_selection(&mut self, start: Option<usize>) {
        self.selections[0] = start;
    }

    /// Update all cursor caches after text changes
    pub fn update_cursor_caches(&mut self) {
        let text = self.text();
        for cursor in &mut self.cursors {
            cursor.update_caches(&text);
        }
    }

    // ========================================================================
    // State
    // ========================================================================

    /// Check if the text has been modified
    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Clear the dirty flag
    #[inline]
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Mark the model as dirty
    #[inline]
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    // ========================================================================
    // Syntax Highlighting (Stub for Phase 3)
    // ========================================================================

    /// Get syntax highlighting spans
    #[inline]
    pub fn syntax_spans(&self) -> &[(Range<usize>, Color)] {
        &self.syntax_spans
    }

    /// Set syntax highlighting spans (for testing/future implementation)
    #[inline]
    pub fn set_syntax_spans(&mut self, spans: Vec<(Range<usize>, Color)>) {
        self.syntax_spans = spans;
    }
}

impl Default for EditorModel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_model() {
        let model = EditorModel::new();
        assert_eq!(model.len_chars(), 0);
        assert_eq!(model.len_lines(), 1); // Rope always has at least 1 line
        assert!(model.is_empty());
    }

    #[test]
    fn test_create_with_text() {
        let model = EditorModel::with_text("Hello\nWorld");
        assert_eq!(model.len_chars(), 11);
        assert_eq!(model.len_lines(), 2);
        assert_eq!(model.text(), "Hello\nWorld");
    }

    #[test]
    fn test_insert_text() {
        let mut model = EditorModel::with_text("Hello World");
        model.insert(5, ", Rust");
        assert_eq!(model.text(), "Hello, Rust World");
        assert!(model.is_dirty());
    }

    #[test]
    fn test_delete_text() {
        let mut model = EditorModel::with_text("Hello, Rust World");
        model.delete(5..11);
        assert_eq!(model.text(), "Hello World");
        assert!(model.is_dirty());
    }

    #[test]
    fn test_cursor_position_after_insert() {
        let mut model = EditorModel::with_text("Hello");
        model.cursor_mut().set_char_pos(5);

        // Insert before cursor
        model.insert(0, "XXX");
        assert_eq!(model.cursor().char_pos(), 8); // Cursor moved by 3
    }

    #[test]
    fn test_cursor_position_after_delete() {
        let mut model = EditorModel::with_text("Hello World");
        model.cursor_mut().set_char_pos(10);

        // Delete before cursor
        model.delete(0..5);
        assert_eq!(model.cursor().char_pos(), 5); // Cursor moved back by 5
    }

    #[test]
    fn test_line_access() {
        let model = EditorModel::with_text("Line 1\nLine 2\nLine 3");
        assert_eq!(model.line(0), Some("Line 1\n".to_string()));
        assert_eq!(model.line(1), Some("Line 2\n".to_string()));
        assert_eq!(model.line(2), Some("Line 3".to_string()));
        assert_eq!(model.line(3), None);
    }

    #[test]
    fn test_slice_access() {
        let model = EditorModel::with_text("Hello World");
        assert_eq!(model.slice(0..5), Some("Hello".to_string()));
        assert_eq!(model.slice(6..11), Some("World".to_string()));
        assert_eq!(model.slice(0..100), None); // Out of bounds
    }
}
