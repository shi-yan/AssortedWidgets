// Terminal minimap page storage
//
// Each page represents a fixed number of grid lines (512 by default).
// Unlike code editor, pages may not align with newline boundaries
// because we don't control the grid layout.

use super::color::{encode_rgb565, decode_rgb565};

/// Status of a minimap page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageStatus {
    /// Page is up-to-date
    Clean,
    /// Page needs rasterization (content changed)
    Dirty,
    /// Page is currently being rasterized in background thread
    Rasterizing,
    /// Page needs complete regeneration (grid reflowed)
    Stale,
}

/// A terminal minimap page containing rasterized grid content
///
/// Storage format:
/// - Intensity channel: grayscale pixel values (0-255) for anti-aliasing
/// - Color data: RGB565 encoded colors (16-bit)
///
/// Memory: 3 bytes per pixel (1 byte intensity + 2 bytes RGB565)
pub struct TerminalMinimapPage {
    /// Start line number (grid line index, including scrollback)
    pub start_line: usize,

    /// Number of grid lines in this page
    pub line_count: usize,

    /// Width in pixels
    pub width_pixels: usize,

    /// Height in pixels (2 pixels per grid line)
    pub height_pixels: usize,

    /// Intensity channel (width * height bytes)
    /// Each byte is a grayscale intensity value (0-255)
    pub intensity: Vec<u8>,

    /// Color data (width * height * 2 bytes, RGB565 encoded)
    /// Each pixel is stored as u16 (16-bit RGB565)
    pub color_rgb565: Vec<u16>,

    /// Page status
    pub status: PageStatus,

    /// Grid generation counter (for cache invalidation)
    pub grid_generation: usize,
}

impl TerminalMinimapPage {
    /// Create a new empty page
    ///
    /// # Arguments
    /// * `start_line` - First grid line number in this page
    /// * `line_count` - Number of grid lines in this page
    /// * `width_pixels` - Width in pixels
    /// * `grid_generation` - Current grid generation counter
    pub fn new(
        start_line: usize,
        line_count: usize,
        width_pixels: usize,
        grid_generation: usize,
    ) -> Self {
        let height_pixels = line_count * 2; // 2 pixels per grid line
        let pixel_count = width_pixels * height_pixels;

        Self {
            start_line,
            line_count,
            width_pixels,
            height_pixels,
            intensity: vec![0; pixel_count],
            color_rgb565: vec![0; pixel_count],
            status: PageStatus::Dirty,
            grid_generation,
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
        // 1 byte intensity + 2 bytes RGB565 per pixel
        self.pixel_count() * 3
    }

    /// Clear the page (set all pixels to black/transparent)
    pub fn clear(&mut self) {
        self.intensity.fill(0);
        self.color_rgb565.fill(0);
        self.status = PageStatus::Dirty;
    }

    /// Set a pixel's intensity and color
    ///
    /// # Arguments
    /// * `x` - X coordinate (0 to width-1)
    /// * `y` - Y coordinate (0 to height-1)
    /// * `intensity` - Grayscale intensity (0-255)
    /// * `color_rgb565` - RGB565 encoded color (16-bit)
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, intensity: u8, color_rgb565: u16) {
        if x < self.width_pixels && y < self.height_pixels {
            let index = y * self.width_pixels + x;
            self.intensity[index] = intensity;
            self.color_rgb565[index] = color_rgb565;
        }
    }

    /// Set a pixel with RGB888 color (converts to RGB565)
    ///
    /// # Arguments
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `intensity` - Grayscale intensity (0-255)
    /// * `r`, `g`, `b` - RGB color components (0-255 each)
    #[inline]
    pub fn set_pixel_rgb(&mut self, x: usize, y: usize, intensity: u8, r: u8, g: u8, b: u8) {
        let color_rgb565 = encode_rgb565(r, g, b);
        self.set_pixel(x, y, intensity, color_rgb565);
    }

    /// Get a pixel's intensity and color
    ///
    /// Returns None if coordinates are out of bounds.
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<(u8, u16)> {
        if x < self.width_pixels && y < self.height_pixels {
            let index = y * self.width_pixels + x;
            Some((self.intensity[index], self.color_rgb565[index]))
        } else {
            None
        }
    }

    /// Get a pixel as RGB888 (decodes RGB565)
    pub fn get_pixel_rgb(&self, x: usize, y: usize) -> Option<(u8, u8, u8, u8)> {
        self.get_pixel(x, y).map(|(intensity, color565)| {
            let (r, g, b) = decode_rgb565(color565);
            (intensity, r, g, b)
        })
    }

    /// Mark page as clean (rasterization complete)
    pub fn mark_clean(&mut self) {
        self.status = PageStatus::Clean;
    }

    /// Mark page as dirty (needs rasterization)
    pub fn mark_dirty(&mut self) {
        self.status = PageStatus::Dirty;
    }

    /// Mark page as stale (needs complete regeneration)
    pub fn mark_stale(&mut self) {
        self.status = PageStatus::Stale;
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
        let page = TerminalMinimapPage::new(0, 512, 100, 0);

        assert_eq!(page.start_line, 0);
        assert_eq!(page.line_count, 512);
        assert_eq!(page.width_pixels, 100);
        assert_eq!(page.height_pixels, 1024); // 512 lines * 2 pixels
        assert_eq!(page.pixel_count(), 102400); // 100 * 1024
        assert_eq!(page.status, PageStatus::Dirty);
        assert_eq!(page.grid_generation, 0);
    }

    #[test]
    fn test_byte_size() {
        let page = TerminalMinimapPage::new(0, 512, 100, 0);
        // 102400 pixels * 3 bytes (1 intensity + 2 RGB565)
        assert_eq!(page.byte_size(), 307200);
    }

    #[test]
    fn test_set_get_pixel() {
        let mut page = TerminalMinimapPage::new(0, 10, 50, 0);

        page.set_pixel(10, 5, 200, 0xF800); // Red

        let (intensity, color) = page.get_pixel(10, 5).unwrap();
        assert_eq!(intensity, 200);
        assert_eq!(color, 0xF800);
    }

    #[test]
    fn test_set_get_pixel_rgb() {
        let mut page = TerminalMinimapPage::new(0, 10, 50, 0);

        page.set_pixel_rgb(10, 5, 200, 255, 0, 0); // Red

        let (intensity, r, g, b) = page.get_pixel_rgb(10, 5).unwrap();
        assert_eq!(intensity, 200);
        // Allow some precision loss in RGB565
        assert!(r >= 248);
        assert!(g <= 7);
        assert!(b <= 7);
    }

    #[test]
    fn test_clear_page() {
        let mut page = TerminalMinimapPage::new(0, 10, 50, 0);

        page.set_pixel(10, 5, 200, 0xFFFF);
        page.clear();

        let (intensity, color) = page.get_pixel(10, 5).unwrap();
        assert_eq!(intensity, 0);
        assert_eq!(color, 0);
        assert_eq!(page.status, PageStatus::Dirty);
    }

    #[test]
    fn test_out_of_bounds() {
        let page = TerminalMinimapPage::new(0, 10, 50, 0);

        assert!(page.get_pixel(100, 5).is_none());
        assert!(page.get_pixel(10, 100).is_none());
    }

    #[test]
    fn test_status_transitions() {
        let mut page = TerminalMinimapPage::new(0, 10, 50, 0);

        assert_eq!(page.status, PageStatus::Dirty);

        page.mark_clean();
        assert_eq!(page.status, PageStatus::Clean);

        page.mark_dirty();
        assert_eq!(page.status, PageStatus::Dirty);

        page.mark_stale();
        assert_eq!(page.status, PageStatus::Stale);
    }

    #[test]
    fn test_grid_generation() {
        let page1 = TerminalMinimapPage::new(0, 512, 100, 0);
        let page2 = TerminalMinimapPage::new(0, 512, 100, 5);

        assert_eq!(page1.grid_generation, 0);
        assert_eq!(page2.grid_generation, 5);
    }
}
