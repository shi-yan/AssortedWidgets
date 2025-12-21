//! TextLayout object for shaped text
//!
//! The TextLayout wraps a cosmic-text Buffer and provides:
//! - Pre-shaped text ready for rendering
//! - Size information for layout integration
//! - Hit-testing APIs for text editing (stubbed for Phase 4)

use cosmic_text::Buffer;
use crate::types::{Point, Rect, Size};

/// A pre-shaped text layout ready for rendering
///
/// This object holds shaped text and provides geometric queries.
/// It's the central object in our two-tier API:
/// - High-level API: Created and cached transparently by TextEngine
/// - Low-level API: Created explicitly by widget, widget owns lifecycle
pub struct TextLayout {
    /// The cosmic-text buffer (holds shaped glyphs)
    pub(crate) buffer: Buffer,

    /// Cached size for quick access
    size: Size,
}

impl TextLayout {
    /// Create a new text layout from a cosmic-text buffer
    pub(crate) fn new(buffer: Buffer) -> Self {
        let size = Self::compute_size(&buffer);
        Self { buffer, size }
    }

    /// Get the size of the shaped text
    pub fn size(&self) -> Size {
        self.size
    }

    /// Get width in pixels
    pub fn width(&self) -> f64 {
        self.size.width
    }

    /// Get height in pixels
    pub fn height(&self) -> f64 {
        self.size.height
    }

    /// Access the underlying cosmic-text buffer (for advanced use)
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Compute the intrinsic size of the buffer
    fn compute_size(buffer: &Buffer) -> Size {
        let mut max_width = 0.0_f32;
        let mut max_height = 0.0_f32;

        for run in buffer.layout_runs() {
            max_width = max_width.max(run.line_w);
            max_height = run.line_y + run.line_height;
        }

        Size::new(max_width as f64, max_height as f64)
    }

    // ========================================================================
    // Hit-testing APIs (stubbed for Phase 4 - will be implemented with mouse events)
    // ========================================================================

    /// Hit-test: Find the character index at a pixel position
    ///
    /// Used for: Click to position cursor in text editor
    ///
    /// # Arguments
    /// * `position` - Pixel coordinate relative to layout origin
    ///
    /// # Returns
    /// Character index at that position, or None if out of bounds
    ///
    /// **Status:** Stubbed - will be implemented in Phase 4 (Input Handling)
    #[allow(unused_variables)]
    pub fn hit_test(&self, position: Point) -> Option<usize> {
        // TODO Phase 4: Implement using buffer.hit()
        // cosmic-text provides buffer.hit(x, y) -> Cursor
        // which gives us the character position
        None
    }

    /// Get the pixel rectangle for a character at the given index
    ///
    /// Used for: Drawing the blinking text cursor
    ///
    /// # Arguments
    /// * `index` - Character index in the text
    ///
    /// # Returns
    /// Rectangle where the cursor should be drawn
    ///
    /// **Status:** Stubbed - will be implemented in Phase 4 (Input Handling)
    #[allow(unused_variables)]
    pub fn cursor_rect(&self, index: usize) -> Option<Rect> {
        // TODO Phase 4: Implement by finding glyph at index
        // and returning its position/height
        None
    }

    /// Get rectangles for highlighting a text selection
    ///
    /// Used for: Drawing selection highlight in text editor
    ///
    /// # Arguments
    /// * `start` - Start character index
    /// * `end` - End character index
    ///
    /// # Returns
    /// List of rectangles to highlight (can span multiple lines)
    ///
    /// **Status:** Stubbed - will be implemented in Phase 4 (Input Handling)
    #[allow(unused_variables)]
    pub fn selection_rects(&self, start: usize, end: usize) -> Vec<Rect> {
        // TODO Phase 4: Implement by iterating layout_runs()
        // and building rects for each line segment
        Vec::new()
    }
}

impl std::fmt::Debug for TextLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextLayout")
            .field("size", &self.size)
            .finish()
    }
}
