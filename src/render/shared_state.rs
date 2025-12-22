//! Shared rendering resources across all windows
//!
//! This module contains rendering resources that are expensive to create
//! and can be safely shared across multiple windows.

use std::sync::{Arc, Mutex};
use crate::text::{GlyphAtlas, FontSystemWrapper, TextEngine};
use crate::render::RenderContext;

/// Shared rendering resources across all windows
///
/// # Architecture
///
/// These resources are wrapped in Arc<Mutex<>> to allow safe sharing:
///
/// 1. **GlyphAtlas**: Single texture cache for all windows
///    - Uses scale_factor in GlyphKey to support multiple DPIs
///    - Avoids ~16MB duplication per window
///    - Handles windows moving between Retina and non-Retina displays
///
/// 2. **FontSystem**: Font loading and rasterization
///    - Font database is expensive to initialize
///    - All windows typically use the same fonts
///
/// 3. **TextEngine**: Text layout and shaping cache
///    - Shaped text results can be reused across windows
///    - Dual-mode: managed cache + manual TextLayout ownership
///
/// # Thread Safety
///
/// All fields use Mutex for interior mutability. Lock guards should be
/// acquired and released quickly to avoid blocking other windows.
///
/// # Memory Efficiency
///
/// Without sharing:
/// - 3 windows × 2048×2048 RGBA atlas = ~48MB
/// - 3 windows × font database = ~30MB
///
/// With sharing:
/// - 1 atlas = ~16MB
/// - 1 font database = ~10MB
pub struct SharedRenderState {
    /// Multi-page glyph atlas (thread-safe, shared across windows)
    /// Contains glyphs at all scale factors (1.0x, 2.0x, etc.)
    pub glyph_atlas: Arc<Mutex<GlyphAtlas>>,

    /// Font system for discovery and rasterization
    /// Expensive to initialize, shared across all windows
    pub font_system: Arc<Mutex<FontSystemWrapper>>,

    /// Text layout engine with dual-mode caching
    /// Shaped text results shared across windows
    pub text_engine: Arc<Mutex<TextEngine>>,
}

impl SharedRenderState {
    /// Create new shared render state
    ///
    /// # Arguments
    /// * `render_ctx` - Shared GPU context (for creating atlas texture)
    ///
    /// This should be called once at application startup.
    pub fn new(render_ctx: &RenderContext) -> Self {
        // Create glyph atlas (2048×2048 with up to 16 pages)
        let glyph_atlas = GlyphAtlas::new(&render_ctx.device, 2048, 16);

        // Create font system
        let font_system = FontSystemWrapper::new();

        // Create text engine
        let text_engine = TextEngine::new();

        Self {
            glyph_atlas: Arc::new(Mutex::new(glyph_atlas)),
            font_system: Arc::new(Mutex::new(font_system)),
            text_engine: Arc::new(Mutex::new(text_engine)),
        }
    }

    /// Clone the Arc references for use in another window
    ///
    /// This is cheap (just incrementing reference counts).
    pub fn clone_refs(&self) -> Self {
        Self {
            glyph_atlas: Arc::clone(&self.glyph_atlas),
            font_system: Arc::clone(&self.font_system),
            text_engine: Arc::clone(&self.text_engine),
        }
    }
}
