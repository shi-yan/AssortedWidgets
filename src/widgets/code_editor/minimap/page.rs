// Minimap page storage
//
// Each page represents a fixed number of logical lines (512 by default).
// Pixel data is stored in dual-channel format: intensity + color index.

use super::char_sheet::CharSheet;
use super::palette::COLOR_DEFAULT;

/// Status of a minimap page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageStatus {
    /// Page is up-to-date
    Clean,
    /// Page needs rasterization (text changed)
    Dirty,
    /// Page needs reflow (wrapping changed, but text didn't)
    NeedsReflow,
    /// Page is currently being rasterized in background thread
    Rasterizing,
}

/// A minimap page containing rasterized text
///
/// Each page stores:
/// - Intensity channel: grayscale pixel values (0-255)
/// - Color index channel: palette index for each pixel (0-255)
///
/// Pages use strict newline cutoff to prevent pixel migration during reflow.
pub struct MinimapPage {
    /// Start line number (logical line index)
    pub start_line: usize,

    /// Number of lines in this page
    pub line_count: usize,

    /// Width in pixels
    pub width_pixels: usize,

    /// Height in pixels (2 pixels per line)
    pub height_pixels: usize,

    /// Intensity channel (width * height bytes)
    /// Each byte is a grayscale intensity value (0-255)
    pub intensity: Vec<u8>,

    /// Color index channel (width * height bytes)
    /// Each byte is an index into the color palette (0-255)
    pub color_index: Vec<u8>,

    /// Page status
    pub status: PageStatus,
}

impl MinimapPage {
    /// Create a new empty page
    ///
    /// # Arguments
    /// * `start_line` - First logical line number in this page
    /// * `line_count` - Number of lines in this page
    /// * `width_pixels` - Width in pixels (typically editor width / char_width)
    pub fn new(start_line: usize, line_count: usize, width_pixels: usize) -> Self {
        let height_pixels = line_count * 2; // 2 pixels per line
        let pixel_count = width_pixels * height_pixels;

        Self {
            start_line,
            line_count,
            width_pixels,
            height_pixels,
            intensity: vec![0; pixel_count],
            color_index: vec![COLOR_DEFAULT; pixel_count],
            status: PageStatus::Dirty,
        }
    }

    /// Get the total number of pixels in this page
    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.width_pixels * self.height_pixels
    }

    /// Get byte size of this page (for memory tracking)
    #[inline]
    pub fn byte_size(&self) -> usize {
        // 2 bytes per pixel (intensity + color_index)
        self.pixel_count() * 2
    }

    /// Clear the page (set all pixels to background)
    pub fn clear(&mut self) {
        self.intensity.fill(0);
        self.color_index.fill(COLOR_DEFAULT);
        self.status = PageStatus::Dirty;
    }

    /// Set a pixel's intensity and color
    ///
    /// # Arguments
    /// * `x` - X coordinate (0 to width-1)
    /// * `y` - Y coordinate (0 to height-1)
    /// * `intensity` - Grayscale intensity (0-255)
    /// * `color` - Palette color index (0-255)
    pub fn set_pixel(&mut self, x: usize, y: usize, intensity: u8, color: u8) {
        if x < self.width_pixels && y < self.height_pixels {
            let index = y * self.width_pixels + x;
            self.intensity[index] = intensity;
            self.color_index[index] = color;
        }
    }

    /// Get a pixel's intensity and color
    ///
    /// Returns None if coordinates are out of bounds.
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<(u8, u8)> {
        if x < self.width_pixels && y < self.height_pixels {
            let index = y * self.width_pixels + x;
            Some((self.intensity[index], self.color_index[index]))
        } else {
            None
        }
    }

    /// Rasterize a line of text onto the page
    ///
    /// # Arguments
    /// * `line_idx` - Line index within this page (0 to line_count-1)
    /// * `text` - Text content of the line
    /// * `char_sheet` - Character sheet for glyph rendering
    /// * `color` - Color index to use for all characters
    pub fn rasterize_line(
        &mut self,
        line_idx: usize,
        text: &str,
        char_sheet: &CharSheet,
        color: u8,
    ) {
        if line_idx >= self.line_count {
            return;
        }

        let y_start = line_idx * 2; // 2 pixels per line
        let mut x = 0;

        for ch in text.chars() {
            if x >= self.width_pixels {
                break; // Line is full
            }

            let (pixels, width) = char_sheet.render_char(ch);

            if width == 1 {
                // Narrow glyph (1x2 pixels)
                if x < self.width_pixels {
                    self.set_pixel(x, y_start, pixels[0], color);
                    self.set_pixel(x, y_start + 1, pixels[1], color);
                    x += 1;
                }
            } else {
                // Wide glyph (2x2 pixels)
                if x + 1 < self.width_pixels {
                    self.set_pixel(x, y_start, pixels[0], color);
                    self.set_pixel(x + 1, y_start, pixels[1], color);
                    self.set_pixel(x, y_start + 1, pixels[2], color);
                    self.set_pixel(x + 1, y_start + 1, pixels[3], color);
                    x += 2;
                }
            }
        }
    }

    /// Mark page as clean (rasterization complete)
    pub fn mark_clean(&mut self) {
        self.status = PageStatus::Clean;
    }

    /// Mark page as dirty (needs rasterization)
    pub fn mark_dirty(&mut self) {
        self.status = PageStatus::Dirty;
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_page() {
        let page = MinimapPage::new(0, 512, 100);

        assert_eq!(page.start_line, 0);
        assert_eq!(page.line_count, 512);
        assert_eq!(page.width_pixels, 100);
        assert_eq!(page.height_pixels, 1024); // 512 lines * 2 pixels
        assert_eq!(page.pixel_count(), 102400); // 100 * 1024
        assert_eq!(page.status, PageStatus::Dirty);
    }

    #[test]
    fn test_byte_size() {
        let page = MinimapPage::new(0, 512, 100);
        assert_eq!(page.byte_size(), 204800); // 102400 pixels * 2 channels
    }

    #[test]
    fn test_set_get_pixel() {
        let mut page = MinimapPage::new(0, 10, 50);

        page.set_pixel(10, 5, 200, 3);

        let (intensity, color) = page.get_pixel(10, 5).unwrap();
        assert_eq!(intensity, 200);
        assert_eq!(color, 3);
    }

    #[test]
    fn test_clear_page() {
        let mut page = MinimapPage::new(0, 10, 50);

        page.set_pixel(10, 5, 200, 3);
        page.clear();

        let (intensity, color) = page.get_pixel(10, 5).unwrap();
        assert_eq!(intensity, 0);
        assert_eq!(color, COLOR_DEFAULT);
        assert_eq!(page.status, PageStatus::Dirty);
    }

    #[test]
    fn test_rasterize_line() {
        let char_sheet = CharSheet::with_defaults();
        let mut page = MinimapPage::new(0, 10, 100);

        page.rasterize_line(0, "Hello", &char_sheet, COLOR_DEFAULT);

        // First line should have pixels set
        let (intensity, _) = page.get_pixel(0, 0).unwrap();
        assert!(intensity > 0); // Should have some intensity

        page.mark_clean();
        assert_eq!(page.status, PageStatus::Clean);
    }

    #[test]
    fn test_out_of_bounds() {
        let page = MinimapPage::new(0, 10, 50);

        assert!(page.get_pixel(100, 5).is_none());
        assert!(page.get_pixel(10, 100).is_none());
    }

    #[test]
    fn test_status_transitions() {
        let mut page = MinimapPage::new(0, 10, 50);

        assert_eq!(page.status, PageStatus::Dirty);

        page.mark_clean();
        assert_eq!(page.status, PageStatus::Clean);

        page.mark_dirty();
        assert_eq!(page.status, PageStatus::Dirty);
    }
}
