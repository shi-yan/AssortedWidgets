//! Per-window rendering resources
//!
//! This module contains all rendering infrastructure specific to a single window.
//! Windows hold references to shared resources (atlas, fonts) via Arc<Mutex<>>
//! to avoid duplication while maintaining independent rendering state.

use crate::paint::RectRenderer;
use crate::render::{WindowRenderer, SharedRenderState};
use crate::text::TextRenderer;
use std::sync::Arc;

/// Bundle of rendering resources for a single window
///
/// # Architecture
///
/// This struct separates rendering concerns from event handling:
/// - **WindowRenderState**: Owns window-specific resources, references shared state
/// - **WindowEventLoop**: Orchestrates events, layout, and calls into render state
///
/// ## Per-Window Resources (owned directly)
/// 1. **WindowRenderer**: Each window has its own surface
/// 2. **RectRenderer**: Stateless renderer (just pipeline/shaders)
/// 3. **TextRenderer**: Stateless renderer (just pipeline/shaders)
/// 4. **scale_factor**: Current DPI scale (1.0x, 2.0x for Retina, etc.)
///
/// ## Shared Resources (via Arc<Mutex<>>)
/// 1. **GlyphAtlas**: Single texture cache across all windows
///    - Uses scale_factor in GlyphKey for multi-DPI support
///    - Avoids ~16MB duplication per window
/// 2. **FontSystem**: Font loading and rasterization
/// 3. **TextEngine**: Text layout and shaping cache
///
/// # Benefits
///
/// **Shared atlas with scale_factor in GlyphKey:**
/// - Window moves from 1.0x to 2.0x display? Both cached, no invalidation!
/// - 5 windows use same text? Single glyph cache instead of 5× duplication
/// - Memory: 1 atlas (~16MB) vs per-window (~80MB for 5 windows)
///
/// **Stateless renderers:**
/// - Projection matrix passed as parameter, not embedded
/// - No per-window duplication of pipelines/shaders
pub struct WindowRenderState {
    /// Window surface and format management
    pub renderer: WindowRenderer,

    /// Rectangle batching renderer (stateless)
    pub rect_renderer: RectRenderer,

    /// Text instanced renderer (stateless)
    pub text_renderer: TextRenderer,

    /// Current DPI scale factor (1.0 = standard, 2.0 = Retina)
    /// Used in GlyphKey to support multiple scales in single atlas
    pub scale_factor: f32,

    /// Shared rendering resources (atlas, fonts, text engine)
    /// Wrapped in Arc for cheap cloning, Mutex for thread-safety
    pub shared: Arc<SharedRenderState>,
}

impl WindowRenderState {
    /// Create a new render state for a window
    ///
    /// # Arguments
    /// * `renderer` - Window surface renderer
    /// * `rect_renderer` - Stateless rectangle renderer
    /// * `text_renderer` - Stateless text renderer
    /// * `scale_factor` - Initial DPI scale (1.0 = standard, 2.0 = Retina)
    /// * `shared` - Arc to shared render state (atlas, fonts, text engine)
    pub fn new(
        renderer: WindowRenderer,
        rect_renderer: RectRenderer,
        text_renderer: TextRenderer,
        scale_factor: f32,
        shared: Arc<SharedRenderState>,
    ) -> Self {
        Self {
            renderer,
            rect_renderer,
            text_renderer,
            scale_factor,
            shared,
        }
    }

    /// Begin a new rendering frame
    ///
    /// This should be called at the start of each frame to:
    /// - Increment frame counters for cache eviction
    /// - Mark all atlas glyphs as potentially evictable
    pub fn begin_frame(&mut self) {
        // Lock shared resources briefly to update frame counters
        self.shared.glyph_atlas.lock().unwrap().begin_frame();
        self.shared.text_engine.lock().unwrap().begin_frame();
    }

    /// Get a reference to the window surface renderer
    pub fn window_renderer(&self) -> &WindowRenderer {
        &self.renderer
    }

    /// Get a mutable reference to the window surface renderer
    pub fn window_renderer_mut(&mut self) -> &mut WindowRenderer {
        &mut self.renderer
    }
}
