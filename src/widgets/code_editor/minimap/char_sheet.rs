// Character sheet for minimap rendering
//
// Renders characters as tiny 1x2 or 2x2 pixel glyphs for the minimap.
// This allows efficient pixel-perfect rendering of code at a small scale.

use std::collections::HashMap;

/// Width category for a character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphWidth {
    /// Narrow characters (ASCII, Latin) - 1 pixel wide
    Narrow,
    /// Wide characters (CJK, emoji) - 2 pixels wide
    Wide,
}

/// Character sheet for minimap glyphs
///
/// Stores pre-rasterized micro-glyphs:
/// - Narrow glyphs: 1x2 pixels (most ASCII/Latin characters)
/// - Wide glyphs: 2x2 pixels (CJK characters, emoji)
///
/// Each pixel is stored as a grayscale intensity value (0-255).
#[derive(Debug, Clone)]
pub struct CharSheet {
    /// 1x2 glyphs for narrow characters
    /// [top_pixel, bottom_pixel]
    narrow_glyphs: HashMap<char, [u8; 2]>,

    /// 2x2 glyphs for wide characters
    /// [top_left, top_right, bottom_left, bottom_right]
    wide_glyphs: HashMap<char, [u8; 4]>,

    /// Width classification for each character
    char_widths: HashMap<char, GlyphWidth>,
}

impl CharSheet {
    /// Create a new empty character sheet
    pub fn new() -> Self {
        Self {
            narrow_glyphs: HashMap::new(),
            wide_glyphs: HashMap::new(),
            char_widths: HashMap::new(),
        }
    }

    /// Create a character sheet with default ASCII glyphs
    ///
    /// For Phase 2, we'll create simple placeholder glyphs.
    /// In later phases, this will use actual font rasterization.
    pub fn with_defaults() -> Self {
        let mut sheet = Self::new();

        // Add common ASCII characters with placeholder glyphs
        for ch in ' '..='~' {
            sheet.add_narrow_glyph(ch, Self::generate_placeholder_narrow(ch));
        }

        // Add newline
        sheet.add_narrow_glyph('\n', [0, 0]);

        // Add common CJK range (placeholder for now)
        // In real implementation, these would be rasterized from a font
        for code in 0x4E00..=0x4E10 {
            if let Some(ch) = char::from_u32(code) {
                sheet.add_wide_glyph(ch, Self::generate_placeholder_wide(ch));
            }
        }

        sheet
    }

    /// Add a narrow (1x2) glyph
    pub fn add_narrow_glyph(&mut self, ch: char, pixels: [u8; 2]) {
        self.narrow_glyphs.insert(ch, pixels);
        self.char_widths.insert(ch, GlyphWidth::Narrow);
    }

    /// Add a wide (2x2) glyph
    pub fn add_wide_glyph(&mut self, ch: char, pixels: [u8; 4]) {
        self.wide_glyphs.insert(ch, pixels);
        self.char_widths.insert(ch, GlyphWidth::Wide);
    }

    /// Get the width category for a character
    pub fn get_width(&self, ch: char) -> GlyphWidth {
        self.char_widths
            .get(&ch)
            .copied()
            .unwrap_or(GlyphWidth::Narrow)
    }

    /// Get narrow glyph pixels (1x2)
    pub fn get_narrow(&self, ch: char) -> Option<[u8; 2]> {
        self.narrow_glyphs.get(&ch).copied()
    }

    /// Get wide glyph pixels (2x2)
    pub fn get_wide(&self, ch: char) -> Option<[u8; 4]> {
        self.wide_glyphs.get(&ch).copied()
    }

    /// Render a character to pixel data
    ///
    /// Returns (pixels, width_in_pixels)
    pub fn render_char(&self, ch: char) -> (Vec<u8>, usize) {
        match self.get_width(ch) {
            GlyphWidth::Narrow => {
                let pixels = self.get_narrow(ch).unwrap_or([128, 128]); // Default gray
                (pixels.to_vec(), 1)
            }
            GlyphWidth::Wide => {
                let pixels = self.get_wide(ch).unwrap_or([128, 128, 128, 128]);
                (pixels.to_vec(), 2)
            }
        }
    }

    // ========================================================================
    // Placeholder Generation (for Phase 2)
    // ========================================================================

    /// Generate a simple placeholder narrow glyph
    ///
    /// For Phase 2, this creates a deterministic pattern based on the character.
    /// In Phase 3+, this will use actual font rasterization with cosmic-text.
    fn generate_placeholder_narrow(ch: char) -> [u8; 2] {
        // Create a pattern based on character code
        let code = ch as u32;
        let top = ((code * 137) % 256) as u8;
        let bottom = ((code * 199) % 256) as u8;
        [top, bottom]
    }

    /// Generate a simple placeholder wide glyph
    fn generate_placeholder_wide(ch: char) -> [u8; 4] {
        let code = ch as u32;
        let tl = ((code * 137) % 256) as u8;
        let tr = ((code * 173) % 256) as u8;
        let bl = ((code * 199) % 256) as u8;
        let br = ((code * 223) % 256) as u8;
        [tl, tr, bl, br]
    }
}

impl Default for CharSheet {
    fn default() -> Self {
        Self::with_defaults()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_sheet() {
        let sheet = CharSheet::new();
        assert_eq!(sheet.get_width('a'), GlyphWidth::Narrow); // Default
        assert!(sheet.get_narrow('a').is_none());
    }

    #[test]
    fn test_create_with_defaults() {
        let sheet = CharSheet::with_defaults();

        // Should have ASCII characters
        assert!(sheet.get_narrow('a').is_some());
        assert!(sheet.get_narrow('Z').is_some());
        assert!(sheet.get_narrow(' ').is_some());

        // All ASCII should be narrow
        assert_eq!(sheet.get_width('a'), GlyphWidth::Narrow);
        assert_eq!(sheet.get_width('Z'), GlyphWidth::Narrow);
    }

    #[test]
    fn test_add_narrow_glyph() {
        let mut sheet = CharSheet::new();
        sheet.add_narrow_glyph('a', [100, 150]);

        assert_eq!(sheet.get_width('a'), GlyphWidth::Narrow);
        assert_eq!(sheet.get_narrow('a'), Some([100, 150]));
    }

    #[test]
    fn test_add_wide_glyph() {
        let mut sheet = CharSheet::new();
        sheet.add_wide_glyph('中', [10, 20, 30, 40]);

        assert_eq!(sheet.get_width('中'), GlyphWidth::Wide);
        assert_eq!(sheet.get_wide('中'), Some([10, 20, 30, 40]));
    }

    #[test]
    fn test_render_char() {
        let mut sheet = CharSheet::new();
        sheet.add_narrow_glyph('a', [100, 150]);
        sheet.add_wide_glyph('中', [10, 20, 30, 40]);

        let (pixels, width) = sheet.render_char('a');
        assert_eq!(pixels, vec![100, 150]);
        assert_eq!(width, 1);

        let (pixels, width) = sheet.render_char('中');
        assert_eq!(pixels, vec![10, 20, 30, 40]);
        assert_eq!(width, 2);
    }

    #[test]
    fn test_placeholder_generation() {
        let sheet = CharSheet::with_defaults();

        // Different characters should have different patterns
        let a_glyph = sheet.get_narrow('a').unwrap();
        let b_glyph = sheet.get_narrow('b').unwrap();
        assert_ne!(a_glyph, b_glyph);
    }
}
