//! Shared rendering context - holds GPU state and rendering resources shared across all windows

use std::sync::{Arc, Mutex};
use crate::text::{GlyphAtlas, FontSystemWrapper, TextEngine};

/// Shared rendering context containing GPU resources and rendering state
///
/// This is created once per application and shared between all windows via Arc.
/// It holds:
/// - WebGPU instance, adapter, device, and queue (GPU low-level)
/// - Glyph atlas, font system, and text engine (rendering high-level)
///
/// # Memory Efficiency
///
/// Sharing these resources across windows saves significant memory:
/// - Single glyph atlas (~16MB) instead of per-window duplication (~80MB for 5 windows)
/// - Single font database (~10MB) instead of ~50MB for 5 windows
/// - Shared text shaping cache
pub struct RenderContext {
    // ========================================
    // GPU Resources (Low-Level)
    // ========================================
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,

    // ========================================
    // Rendering Resources (High-Level)
    // ========================================
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

impl RenderContext {
    /// Create a new render context asynchronously
    ///
    /// This requests a GPU adapter, creates a logical device, and initializes
    /// shared rendering resources (atlas, fonts, text engine).
    ///
    /// Use `pollster::block_on(RenderContext::new())` in main() for simple blocking init.
    pub async fn new() -> Result<Self, String> {
        // Create wgpu instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Request adapter (represents a physical GPU)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok()
            .ok_or_else(|| "Failed to find a suitable GPU adapter".to_string())?;

        // Log adapter info
        let info = adapter.get_info();
        println!("Using GPU: {} ({:?})", info.name, info.backend);

        // Request device and queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("AssortedWidgets Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                experimental_features: Default::default(),
                trace: Default::default(),
            })
            .await
            .map_err(|e| format!("Failed to create device: {}", e))?;

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        // Create shared rendering resources (atlas, fonts, text engine)
        println!("Initializing shared rendering resources (atlas, fonts, text engine)");

        // Create glyph atlas (2048×2048 with up to 16 pages)
        let glyph_atlas = GlyphAtlas::new(&device, 2048, 16);

        // Create font system
        let font_system = FontSystemWrapper::new();

        // Create text engine
        let text_engine = TextEngine::new();

        Ok(RenderContext {
            instance,
            adapter,
            device,
            queue,
            glyph_atlas: Arc::new(Mutex::new(glyph_atlas)),
            font_system: Arc::new(Mutex::new(font_system)),
            text_engine: Arc::new(Mutex::new(text_engine)),
        })
    }

    /// Get device reference
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Get queue reference
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}
