//! Test element for Phase 3.1 text rendering
//!
//! Renders individual characters with manual positioning (no shaping yet).
//! Demonstrates English, Chinese, and emoji rendering.

use crate::element::Element;
use crate::paint::{Color, PaintContext};
use crate::text::{GlyphAtlas, GlyphKey};
use crate::text::FontSystemWrapper;
use crate::types::{Rect, WidgetId};
use cosmic_text::{Attrs, CacheKey, Family, Style, Weight};

/// Test element that renders text glyphs manually
pub struct TextTestElement {
    id: WidgetId,
    bounds: Rect,
    /// Characters to render
    text: String,
    /// Font size
    font_size: f32,
    /// Text color
    color: Color,
    /// X position for text
    x: f32,
    /// Y position for text
    y: f32,
}

impl TextTestElement {
    /// Create a new text test element
    pub fn new(id: WidgetId, text: String, font_size: f32, x: f32, y: f32, color: Color) -> Self {
        Self {
            id,
            bounds: Rect::default(),
            text,
            font_size,
            color,
            x,
            y,
        }
    }

    /// Render text using the glyph atlas and font system
    pub fn render_text(
        &self,
        paint_ctx: &mut PaintContext,
        font_system: &mut FontSystemWrapper,
        glyph_atlas: &mut GlyphAtlas,
        queue: &wgpu::Queue,
    ) {
        // Phase 3.1: Manual character positioning (no shaping)
        // Just place characters horizontally with fixed spacing

        let mut x_offset = self.x;
        let y_baseline = self.y;

        // Create cosmic-text attributes for font selection
        let attrs = Attrs::new()
            .family(Family::SansSerif)
            .weight(Weight::NORMAL)
            .style(Style::Normal);

        for ch in self.text.chars() {
            // Get font for this character (cosmic-text handles fallback)
            let font_system = font_system.font_system_mut();

            // Find a font that supports this character
            let font_id = if let Some((font_id, _)) = font_system
                .get_font_matches(attrs)
                .find(|(id, _)| {
                    let font = font_system.get_font(*id).unwrap();
                    font.with_face(|face| face.has_glyph(ch as u32))
                })
            {
                font_id.0
            } else {
                // Fallback to first font
                0
            };

            // Create cache key for rasterization
            let cache_key = CacheKey {
                font_id: cosmic_text::fontdb::ID(font_id),
                glyph_id: 0, // Will be looked up by cosmic-text
                font_size_bits: (self.font_size * 64.0) as u32,
                x_bin: 0,
                y_bin: 0,
            };

            // Get glyph ID from font
            let glyph_id = font_system.get_font(cosmic_text::fontdb::ID(font_id))
                .and_then(|font| font.with_face(|face| face.glyph_index(ch as u32)))
                .unwrap_or(0);

            let cache_key = CacheKey {
                font_id: cosmic_text::fontdb::ID(font_id),
                glyph_id,
                font_size_bits: (self.font_size * 64.0) as u32,
                x_bin: 0,
                y_bin: 0,
            };

            // Rasterize glyph
            if let Some(rasterized) = font_system.rasterize_glyph(cache_key) {
                // Create glyph key for atlas
                let glyph_key = GlyphKey::new(font_id, self.font_size, ch, 0);

                // Check if glyph is already in atlas
                let location = if let Some(loc) = glyph_atlas.get(&glyph_key) {
                    *loc
                } else {
                    // Insert into atlas
                    match glyph_atlas.insert(
                        queue,
                        glyph_key,
                        &rasterized.pixels,
                        rasterized.width,
                        rasterized.height,
                        rasterized.offset_x,
                        rasterized.offset_y,
                        rasterized.is_color,
                    ) {
                        Ok(loc) => loc,
                        Err(e) => {
                            eprintln!("Failed to insert glyph '{}': {}", ch, e);
                            continue;
                        }
                    }
                };

                // Calculate position (manual positioning for Phase 3.1)
                let glyph_x = x_offset + location.offset_x as f32;
                let glyph_y = y_baseline + location.offset_y as f32;

                // Draw glyph
                paint_ctx.draw_glyph(
                    glyph_x,
                    glyph_y,
                    location.width as f32,
                    location.height as f32,
                    location.uv_rect.min_x,
                    location.uv_rect.min_y,
                    location.uv_rect.max_x,
                    location.uv_rect.max_y,
                    self.color,
                    location.page_index,
                    location.is_color,
                );

                // Advance cursor (rough approximation - proper metrics in Phase 3.2)
                x_offset += location.width as f32 + 2.0;
            }
        }
    }
}

impl Element for TextTestElement {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn paint(&self, _ctx: &mut PaintContext) {
        // Note: We can't paint here because we need mutable access to font_system and glyph_atlas
        // which are owned by GuiEventLoop. We'll need to call render_text() separately.
        // This is a limitation of Phase 3.1 - will be resolved in Phase 3.2 with better architecture.
    }

    fn needs_measure(&self) -> bool {
        false
    }

    fn measure(&self, _known_dimensions: taffy::Size<Option<f32>>, _available_space: taffy::Size<taffy::AvailableSpace>) -> Option<taffy::Size<f32>> {
        None
    }
}
