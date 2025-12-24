//! Text rendering system for AssortedWidgets
//!
//! This module implements a production-quality text rendering system using:
//! - cosmic-text: Font discovery, fallback, shaping, and rasterization
//! - etagere: 2D bin packing for glyph atlas
//! - Multi-page RGBA8 texture atlas for efficient GPU rendering
//!
//! ## Two-Tier API Architecture
//!
//! ### High-Level Managed API (For Simple Widgets)
//! ```rust,ignore
//! ctx.draw_text("Hello", &TextStyle::default(), Point::new(0, 0), None);
//! ```
//! - Automatic LRU caching
//! - Deduplication (multiple widgets with same text share cache)
//! - Generational eviction (unused text auto-removed)
//!
//! ### Low-Level Manual API (For Advanced Widgets)
//! ```rust,ignore
//! let layout = engine.create_layout("Hello", &style, None, Truncate::None);
//! ctx.draw_layout(&layout, Point::new(0, 0), Color::BLACK);
//! ```
//! - Widget owns the TextLayout
//! - No caching overhead
//! - Precise invalidation control
//!
//! Architecture:
//! - `FontSystemWrapper`: Wrapper around cosmic-text's FontSystem and SwashCache
//! - `GlyphAtlas`: GPU texture atlas with bin packing and caching
//! - `WindowRenderer`: Instanced quad renderer with shared pipelines (in render module)
//! - `TextEngine`: Dual-mode caching (managed + manual)
//! - `TextLayout`: Pre-shaped text ready for rendering
//! - `TextStyle`: Font configuration

mod atlas;
mod engine;
mod font_system;
mod layout;
mod style;

pub use atlas::{GlyphAtlas, GlyphKey, GlyphLocation, UvRect};
pub use engine::{TextEngine, CacheStats};
pub use font_system::{FontSystemWrapper, RasterizedGlyph};
pub use layout::TextLayout;
pub use style::{TextStyle, TextAlign, Truncate};

// Import Color for TextInstance::new()
use crate::paint::Color;

// TextInstance is defined inline since it's a simple data structure
// used by WindowRenderer for instanced text rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextInstance {
    pub position: [f32; 2],
    pub glyph_size: [f32; 2],
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub color: [f32; 4],
    pub page_index: u32,
    pub glyph_type: u32,
    pub clip_rect: [f32; 4],
    pub z_order: u32,
    pub _padding: [u32; 3],  // Align to 16 bytes
}

impl TextInstance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        uv_min_x: f32,
        uv_min_y: f32,
        uv_max_x: f32,
        uv_max_y: f32,
        color: Color,
        page_index: u32,
        is_color: bool,
        clip_rect: [f32; 4],
    ) -> Self {
        Self {
            position: [x, y],
            glyph_size: [width, height],
            uv_min: [uv_min_x, uv_min_y],
            uv_max: [uv_max_x, uv_max_y],
            color: [color.r, color.g, color.b, color.a],
            page_index,
            glyph_type: if is_color { 1 } else { 0 },
            clip_rect,
            z_order: 0,
            _padding: [0, 0, 0],
        }
    }

    pub fn with_z_order(mut self, z_order: u32) -> Self {
        self.z_order = z_order;
        self
    }
}
