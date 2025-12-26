//! TextLayout object for shaped text
//!
//! The TextLayout wraps a cosmic-text Buffer and provides:
//! - Pre-shaped text ready for rendering
//! - Size information for layout integration
//! - Hit-testing APIs for text editing (stubbed for Phase 4)

use cosmic_text::Buffer;
use crate::types::{Point, Rect, Size};
use crate::text::TextAlign;

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

    /// Text alignment
    pub(crate) alignment: TextAlign,

    /// Max width constraint (needed for center/right alignment calculation)
    pub(crate) max_width: Option<f32>,

    /// Y offset of the first line's baseline (for proper vertical alignment)
    /// This is the baseline offset that should be subtracted when rendering
    /// to ensure text starts at the specified position without extra top padding
    pub(crate) first_line_y: f32,
}

impl TextLayout {
    /// Create a new text layout from a cosmic-text buffer
    pub(crate) fn new(buffer: Buffer, alignment: TextAlign, max_width: Option<f32>) -> Self {
        let (size, first_line_y) = Self::compute_size(&buffer);
        Self { buffer, size, alignment, max_width, first_line_y }
    }

    /// Get text alignment
    pub fn alignment(&self) -> TextAlign {
        self.alignment
    }

    /// Get max width constraint
    pub fn max_width(&self) -> Option<f32> {
        self.max_width
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

    /// Get the first line's Y offset (baseline position)
    /// This should be subtracted from the rendering position to ensure
    /// text starts at the specified Y coordinate without extra top padding
    pub(crate) fn first_line_y(&self) -> f32 {
        self.first_line_y
    }

    /// Access the underlying cosmic-text buffer (for advanced use)
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Compute the intrinsic size of the buffer
    fn compute_size(buffer: &Buffer) -> (Size, f32) {
        let mut max_width = 0.0_f32;
        let mut max_height = 0.0_f32;
        let mut first_line_y = 0.0_f32;
        let mut first_run = true;

        for run in buffer.layout_runs() {
            if first_run {
                first_line_y = run.line_y;
                first_run = false;
            }
            max_width = max_width.max(run.line_w);
            max_height = run.line_y + run.line_height;
        }

        // Subtract first_line_y to get the actual text height (excluding the top baseline offset)
        // This ensures symmetric padding when text is rendered at position.y + line_y
        let actual_height = max_height - first_line_y;

        (Size::new(max_width as f64, actual_height as f64), first_line_y)
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
