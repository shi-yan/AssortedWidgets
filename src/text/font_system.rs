//! Font system wrapper for cosmic-text integration
//!
//! Wraps cosmic-text's FontSystem and SwashCache to provide:
//! - Automatic font discovery (system fonts)
//! - Font fallback for multi-language support
//! - Glyph rasterization via SwashCache

use cosmic_text::{FontSystem, SwashCache, CacheKey, SwashContent, SwashImage};

/// Wrapper around cosmic-text's font system
///
/// Handles:
/// - Font discovery and loading
/// - Font fallback for characters not in primary font
/// - Glyph rasterization (vector to bitmap)
pub struct FontSystemWrapper {
    /// cosmic-text font system (discovers and manages fonts)
    font_system: FontSystem,

    /// Swash cache for rasterizing glyphs
    swash_cache: SwashCache,
}

impl FontSystemWrapper {
    /// Create a new font system
    ///
    /// This automatically discovers all system fonts
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
    }

    /// Get a reference to the font system
    pub fn font_system(&self) -> &FontSystem {
        &self.font_system
    }

    /// Get a mutable reference to the font system
    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }

    /// Rasterize a glyph using the swash cache
    ///
    /// # Arguments
    /// * `cache_key` - Glyph cache key from cosmic-text
    ///
    /// # Returns
    /// The rasterized glyph image, or None if rasterization failed
    pub fn rasterize_glyph(&mut self, cache_key: CacheKey) -> Option<RasterizedGlyph> {
        self.swash_cache
            .get_image(&mut self.font_system, cache_key)
            .as_ref()
            .map(|image| RasterizedGlyph::from_swash_image(image))
    }
}

impl Default for FontSystemWrapper {
    fn default() -> Self {
        Self::new()
    }
}

/// A rasterized glyph with pixel data
pub struct RasterizedGlyph {
    /// RGBA8 pixel data
    pub pixels: Vec<u8>,

    /// Glyph dimensions
    pub width: u32,
    pub height: u32,

    /// Glyph bearing (offset from baseline)
    pub offset_x: i32,
    pub offset_y: i32,

    /// Is this a color glyph (emoji)?
    pub is_color: bool,
}

impl RasterizedGlyph {
    /// Convert from cosmic-text's SwashImage
    fn from_swash_image(image: &SwashImage) -> Self {
        let width = image.placement.width;
        let height = image.placement.height;
        let offset_x = image.placement.left;
        let offset_y = image.placement.top;

        let is_color = matches!(image.content, SwashContent::Color);

        // Convert pixel data to RGBA8
        let pixels = match image.content {
            SwashContent::Mask => {
                // Monochrome glyph: data is alpha channel only
                // Store in alpha channel, RGB can be colored later in shader
                let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
                for &alpha in &image.data {
                    rgba.push(255); // R (unused for mono glyphs)
                    rgba.push(255); // G (unused)
                    rgba.push(255); // B (unused)
                    rgba.push(alpha); // A (the actual glyph mask)
                }
                rgba
            }
            SwashContent::Color => {
                // Color glyph (emoji): data is already RGBA
                image.data.clone()
            }
            SwashContent::SubpixelMask => {
                // Subpixel antialiasing: RGB channels used for LCD rendering
                // For now, treat as grayscale mask (take average of RGB)
                let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
                for chunk in image.data.chunks(3) {
                    let avg = ((chunk[0] as u32 + chunk[1] as u32 + chunk[2] as u32) / 3) as u8;
                    rgba.push(255); // R
                    rgba.push(255); // G
                    rgba.push(255); // B
                    rgba.push(avg);  // A
                }
                rgba
            }
        };

        Self {
            pixels,
            width,
            height,
            offset_x,
            offset_y,
            is_color,
        }
    }
}
