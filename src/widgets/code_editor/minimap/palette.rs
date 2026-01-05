// Color palette for minimap rendering
//
// Maps color indices (0-255) to foreground/background color pairs.
// Used for syntax highlighting in the minimap.

use crate::paint::primitives::Color;

/// Maximum number of palette entries (limited by u8 index)
pub const MAX_PALETTE_ENTRIES: usize = 256;

/// A color palette entry with foreground and background colors
#[derive(Debug, Clone, Copy)]
pub struct PaletteEntry {
    /// Foreground color (text color)
    pub fg: Color,
    /// Background color
    pub bg: Color,
}

impl PaletteEntry {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self { fg, bg }
    }
}

/// Color palette for minimap rendering
///
/// Stores up to 256 foreground/background color pairs.
/// Each pixel in the minimap has an intensity (0-255) and a color index (0-255).
///
/// The final pixel color is computed as:
/// ```glsl
/// color = mix(bg, fg, intensity)
/// ```
#[derive(Debug, Clone)]
pub struct MinimapPalette {
    /// Palette entries (indexed 0-255)
    entries: Vec<PaletteEntry>,
}

impl MinimapPalette {
    /// Create a new empty palette
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Create a palette with default syntax highlighting colors
    pub fn with_defaults() -> Self {
        let mut palette = Self::new();

        // Index 0: Default text (light gray on dark background)
        palette.add_entry(
            Color::rgb(0.85, 0.85, 0.85), // Light gray text
            Color::rgb(0.11, 0.12, 0.13), // Dark background
        );

        // Index 1: Keywords (blue)
        palette.add_entry(
            Color::rgb(0.4, 0.6, 1.0),
            Color::rgb(0.11, 0.12, 0.13),
        );

        // Index 2: Strings (green)
        palette.add_entry(
            Color::rgb(0.6, 0.9, 0.6),
            Color::rgb(0.11, 0.12, 0.13),
        );

        // Index 3: Comments (gray)
        palette.add_entry(
            Color::rgb(0.5, 0.5, 0.5),
            Color::rgb(0.11, 0.12, 0.13),
        );

        // Index 4: Numbers (orange)
        palette.add_entry(
            Color::rgb(1.0, 0.7, 0.4),
            Color::rgb(0.11, 0.12, 0.13),
        );

        // Index 5: Functions (yellow)
        palette.add_entry(
            Color::rgb(1.0, 0.9, 0.4),
            Color::rgb(0.11, 0.12, 0.13),
        );

        // Index 6: Selected text (highlighted)
        palette.add_entry(
            Color::rgb(1.0, 1.0, 1.0), // White text
            Color::rgb(0.2, 0.4, 0.8), // Blue background
        );

        palette
    }

    /// Add a color entry to the palette
    ///
    /// Returns the index of the added entry, or None if palette is full.
    pub fn add_entry(&mut self, fg: Color, bg: Color) -> Option<u8> {
        if self.entries.len() >= MAX_PALETTE_ENTRIES {
            return None;
        }

        let index = self.entries.len() as u8;
        self.entries.push(PaletteEntry::new(fg, bg));
        Some(index)
    }

    /// Get a palette entry by index
    pub fn get(&self, index: u8) -> Option<&PaletteEntry> {
        self.entries.get(index as usize)
    }

    /// Get the number of entries in the palette
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the palette is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get all entries as a slice
    pub fn entries(&self) -> &[PaletteEntry] {
        &self.entries
    }

    /// Convert palette to GPU-compatible format
    ///
    /// Returns two parallel arrays:
    /// - fg_colors: foreground colors as [r, g, b, a] floats
    /// - bg_colors: background colors as [r, g, b, a] floats
    pub fn to_gpu_format(&self) -> (Vec<[f32; 4]>, Vec<[f32; 4]>) {
        let mut fg_colors = Vec::with_capacity(MAX_PALETTE_ENTRIES);
        let mut bg_colors = Vec::with_capacity(MAX_PALETTE_ENTRIES);

        for entry in &self.entries {
            fg_colors.push([entry.fg.r, entry.fg.g, entry.fg.b, entry.fg.a]);
            bg_colors.push([entry.bg.r, entry.bg.g, entry.bg.b, entry.bg.a]);
        }

        // Pad to 256 entries
        while fg_colors.len() < MAX_PALETTE_ENTRIES {
            fg_colors.push([0.0, 0.0, 0.0, 1.0]);
            bg_colors.push([0.0, 0.0, 0.0, 1.0]);
        }

        (fg_colors, bg_colors)
    }
}

impl Default for MinimapPalette {
    fn default() -> Self {
        Self::with_defaults()
    }
}

// ============================================================================
// Color Index Constants
// ============================================================================

/// Default text color index
pub const COLOR_DEFAULT: u8 = 0;

/// Keyword color index
pub const COLOR_KEYWORD: u8 = 1;

/// String color index
pub const COLOR_STRING: u8 = 2;

/// Comment color index
pub const COLOR_COMMENT: u8 = 3;

/// Number color index
pub const COLOR_NUMBER: u8 = 4;

/// Function color index
pub const COLOR_FUNCTION: u8 = 5;

/// Selection color index
pub const COLOR_SELECTION: u8 = 6;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_palette() {
        let palette = MinimapPalette::new();
        assert_eq!(palette.len(), 0);
        assert!(palette.is_empty());
    }

    #[test]
    fn test_create_with_defaults() {
        let palette = MinimapPalette::with_defaults();
        assert!(palette.len() > 0);
        assert!(!palette.is_empty());

        // Should have at least the default entry
        assert!(palette.get(COLOR_DEFAULT).is_some());
        assert!(palette.get(COLOR_KEYWORD).is_some());
    }

    #[test]
    fn test_add_entry() {
        let mut palette = MinimapPalette::new();

        let fg = Color::rgb(1.0, 0.0, 0.0);
        let bg = Color::rgb(0.0, 0.0, 0.0);

        let index = palette.add_entry(fg, bg).unwrap();
        assert_eq!(index, 0);

        let entry = palette.get(index).unwrap();
        assert_eq!(entry.fg.r, 1.0);
        assert_eq!(entry.bg.r, 0.0);
    }

    #[test]
    fn test_to_gpu_format() {
        let palette = MinimapPalette::with_defaults();

        let (fg_colors, bg_colors) = palette.to_gpu_format();

        assert_eq!(fg_colors.len(), MAX_PALETTE_ENTRIES);
        assert_eq!(bg_colors.len(), MAX_PALETTE_ENTRIES);

        // Check first entry
        assert!(fg_colors[0][0] > 0.0); // Should have some red component
    }

    #[test]
    fn test_palette_constants() {
        assert_eq!(COLOR_DEFAULT, 0);
        assert_eq!(COLOR_KEYWORD, 1);
        assert_eq!(COLOR_STRING, 2);
        assert_eq!(COLOR_COMMENT, 3);
    }
}
