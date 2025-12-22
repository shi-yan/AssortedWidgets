//! Per-window rendering resources
//!
//! This module contains all rendering infrastructure specific to a single window.
//! Each window owns its own rendering resources (atlas, renderers, font system)
//! to allow independent rendering and simplified ownership.

use crate::paint::RectRenderer;
use crate::render::{RenderContext, WindowRenderer};
use crate::text::{FontSystemWrapper, GlyphAtlas, TextEngine, TextRenderer};

/// Bundle of rendering resources for a single window
///
/// # Architecture
///
/// This struct separates rendering concerns from event handling:
/// - **WindowRenderState**: Owns all GPU rendering resources
/// - **WindowEventLoop**: Orchestrates events, layout, and calls into render state
///
/// Each window gets its own render state because:
/// 1. Different windows may have different DPI/scale factors
/// 2. Each window has its own surface (WindowRenderer)
/// 3. Simpler ownership (no Arc/Mutex needed for rendering resources)
/// 4. Windows can render independently (future: parallel rendering)
///
/// # GPU Resources
///
/// While this is per-window, the GPU device/queue (RenderContext) is shared
/// across all windows for efficiency.
pub struct WindowRenderState {
    /// Window surface and format management
    pub renderer: WindowRenderer,

    /// Rectangle batching renderer
    pub rect_renderer: RectRenderer,

    /// Text instanced renderer
    pub text_renderer: TextRenderer,

    /// Multi-page glyph atlas (RGBA8 texture array)
    pub glyph_atlas: GlyphAtlas,

    /// Font discovery and rasterization system
    pub font_system: FontSystemWrapper,

    /// Text layout engine with dual-mode caching
    /// - High-level: Managed cache (for buttons, labels, menus)
    /// - Low-level: Manual TextLayout ownership (for editors, terminals)
    pub text_engine: TextEngine,
}

impl WindowRenderState {
    /// Create a new render state for a window
    pub fn new(
        renderer: WindowRenderer,
        rect_renderer: RectRenderer,
        text_renderer: TextRenderer,
        glyph_atlas: GlyphAtlas,
        font_system: FontSystemWrapper,
        text_engine: TextEngine,
    ) -> Self {
        Self {
            renderer,
            rect_renderer,
            text_renderer,
            glyph_atlas,
            font_system,
            text_engine,
        }
    }

    /// Begin a new rendering frame
    ///
    /// This should be called at the start of each frame to:
    /// - Increment frame counters for cache eviction
    /// - Mark all atlas glyphs as potentially evictable
    pub fn begin_frame(&mut self) {
        self.glyph_atlas.begin_frame();
        self.text_engine.begin_frame();
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
