use crate::text::{FontSystemWrapper, GlyphAtlas};
use cosmic_text::fontdb;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Icon rendering engine - direct rasterization from icon font
///
/// Icons are rasterized directly from the embedded Material Icons font,
/// bypassing the font fallback system. Rasterized glyphs are uploaded to
/// the shared GlyphAtlas and rendered through TextPipeline.
pub struct IconEngine {
    /// Icon ID → Unicode codepoint mapping
    mapping: HashMap<String, char>,

    /// Font face ID in the font database (for direct rasterization)
    font_id: fontdb::ID,

    /// Reference to font system for rasterization
    font_system: Arc<Mutex<FontSystemWrapper>>,
}

impl IconEngine {
    /// Create a new IconEngine with embedded Material Icons font
    pub fn new(
        font_system: Arc<Mutex<FontSystemWrapper>>,
        _glyph_atlas: Arc<Mutex<GlyphAtlas>>,  // Not used, icons use text rendering
    ) -> Self {
        // Load embedded icon font (TTF format)
        let font_data = super::embedded::IconAssets::icon_font();

        println!("=== Icon Font Loading (Direct Rasterization) ===");
        println!("  Font data size: {} bytes", font_data.len());

        // Register font with cosmic-text
        // Note: In cosmic-text 0.15.0, load_font_data() returns () not Vec<ID>
        let mut fs = font_system.lock().unwrap();

        // Get face count before loading
        let db = fs.font_system().db();
        let face_count_before = db.faces().count();
        println!("  Face count before loading: {}", face_count_before);

        // Load the font
        fs.font_system_mut().db_mut().load_font_data(font_data);

        // Check face count after loading
        let db = fs.font_system().db();
        let face_count_after = db.faces().count();
        println!("  Face count after loading: {}", face_count_after);

        // Find the newly loaded font by querying for new faces
        let db = fs.font_system().db();
        let mut font_id = None;
        let mut faces_loaded = 0;

        for face in db.faces().skip(face_count_before) {
            if font_id.is_none() {
                font_id = Some(face.id);
            }
            faces_loaded += 1;

            // Debug info for first face
            if faces_loaded == 1 {
                if let Some((family, _)) = face.families.first() {
                    println!("  ✅ Font family: \"{}\"", family);
                    println!("  Font style: {:?}", face.style);
                    println!("  Font weight: {:?}", face.weight);
                    println!("  Font ID: {:?}", face.id);
                }
            }
        }

        println!("  Loaded {} font face(s) from icon.ttf", faces_loaded);

        let font_id = font_id.expect("Failed to load icon.ttf - no font faces returned");

        drop(fs); // Release lock before loading mapping

        // Load icon ID → codepoint mapping
        let mapping = super::embedded::IconAssets::icon_mapping();

        println!(
            "  ✅ IconEngine initialized: {} icons loaded",
            mapping.len(),
        );
        println!("  Mode: Direct rasterization (bypass font fallback)\n");

        Self {
            mapping,
            font_id,
            font_system,
        }
    }

    /// Get the Unicode character for an icon ID
    ///
    /// # Arguments
    /// * `icon_id` - Human-readable icon identifier (e.g., "search", "home")
    ///
    /// # Returns
    /// `Some(char)` if icon exists, `None` otherwise
    ///
    /// # Example
    /// ```ignore
    /// let icon_char = icon_engine.get_icon_char("search")?;
    /// // Use with TextEngine to render: text_engine.render_text(&icon_char.to_string(), ...)
    /// ```
    pub fn get_icon_char(&self, icon_id: &str) -> Option<char> {
        //println!("[IconEngine] get_icon_char called for: {}", icon_id);
        let result = self.mapping.get(icon_id).copied();
        //println!("[IconEngine] get_icon_char result: {:?}", result);
        result
    }

    /// Check if an icon ID exists in the mapping
    pub fn has_icon(&self, icon_id: &str) -> bool {
        self.mapping.contains_key(icon_id)
    }

    /// Get all available icon IDs
    pub fn available_icons(&self) -> Vec<String> {
        self.mapping.keys().cloned().collect()
    }

    /// Rasterize an icon glyph for visual bounding box positioning
    ///
    /// This uses cosmic-text's shaping to get a cache key, but the caller should
    /// ignore bearing/offset values when positioning. Icons should be positioned
    /// by their visual bounding box, not text baseline metrics.
    ///
    /// # Arguments
    /// * `font_system_wrapper` - Already-locked FontSystemWrapper
    /// * `icon_char` - The Unicode character to rasterize
    /// * `size` - Font size in points
    ///
    /// # Returns
    /// Tuple of (RasterizedGlyph, CacheKey) where the glyph should be positioned
    /// with zero bearing offsets (ignore offset_x/offset_y in RasterizedGlyph)
    pub fn rasterize_icon(
        &self,
        font_system_wrapper: &mut FontSystemWrapper,
        icon_char: char,
        size: f32,
    ) -> Option<(crate::text::RasterizedGlyph, cosmic_text::CacheKey)> {
        // Get cache key via shaping (needed for cosmic-text API)
        let cache_key = self.get_cache_key(font_system_wrapper, icon_char, size as f64)?;

        // Rasterize the glyph
        let rasterized = font_system_wrapper.rasterize_glyph(cache_key)?;

        Some((rasterized, cache_key))
    }

    /// Get cache key for an icon glyph (internal method)
    ///
    /// This is used internally by `rasterize_icon()`. Direct callers should prefer
    /// `rasterize_icon()` which provides both the cache key and rasterized glyph.
    fn get_cache_key(&self, font_system_wrapper: &mut FontSystemWrapper, icon_char: char, size: f64) -> Option<cosmic_text::CacheKey> {
        use cosmic_text::{Attrs, Buffer, Metrics, Shaping, Family};

        let font_system = font_system_wrapper.font_system_mut();

        // Create a buffer with forced font using Family::Name
        // Query the font name from the database using our font_id
        let font_name = if let Some(face_info) = font_system.db().face(self.font_id) {
            if let Some((family, _)) = face_info.families.first() {
                family.clone()
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Create attrs that force our specific icon font
        let attrs = Attrs::new().family(Family::Name(&font_name));

        let mut buffer = Buffer::new(font_system, Metrics::new(size as f32, size as f32));

        buffer.set_text(font_system, &icon_char.to_string(), &attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(font_system, false);

        // Extract the cache_key from the shaped glyph
        for line in buffer.lines.iter() {
            if let Some(layout_lines) = line.layout_opt() {
                for layout_line in layout_lines {
                    for glyph in &layout_line.glyphs {
                        let physical = glyph.physical((0.0, 0.0), 1.0);
                        return Some(physical.cache_key);
                    }
                }
            }
        }

        None
    }
}
