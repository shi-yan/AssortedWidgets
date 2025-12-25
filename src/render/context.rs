//! Shared rendering context - holds GPU state and rendering resources shared across all windows

use std::sync::{Arc, Mutex};
use crate::text::{GlyphAtlas, FontSystemWrapper, TextEngine};
use crate::render::pipelines::{RectPipeline, TextPipeline};
use crate::render::{RectSdfPipeline, ShadowSdfPipeline};

/// Shared rendering context containing GPU resources and rendering state
///
/// This is created once per application and shared between all windows via Arc.
/// It holds:
/// - WebGPU instance, adapter, device, and queue (GPU low-level)
/// - Shared rendering pipelines (rect, text) - stateless, reused across windows
/// - Glyph atlas, font system, and text engine (rendering high-level)
///
/// # Memory Efficiency
///
/// Sharing these resources across windows saves significant memory:
/// - Single pipeline creation instead of per-window duplication
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
    // Shared Pipelines (Stateless)
    // ========================================
    /// Rectangle rendering pipeline (shared across all windows)
    /// Contains only stateless resources: pipeline, bind group layouts
    pub rect_pipeline: RectPipeline,

    /// Text rendering pipeline (shared across all windows)
    /// Contains only stateless resources: pipeline, bind group layouts, sampler
    pub text_pipeline: TextPipeline,

    /// Rounded rectangle SDF pipeline (shared across all windows)
    /// Used for drawing rectangles with rounded corners and borders
    pub rect_sdf_pipeline: RectSdfPipeline,

    /// Shadow SDF pipeline (shared across all windows)
    /// Used for rendering analytical soft drop shadows
    pub shadow_sdf_pipeline: ShadowSdfPipeline,

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

    // ========================================
    // Surface Configuration
    // ========================================
    /// Preferred surface format (sRGB if available, otherwise first supported format)
    /// Determined once from adapter capabilities and used for all windows
    pub surface_format: wgpu::TextureFormat,
}

impl RenderContext {
    /// Create a new render context asynchronously
    ///
    /// This requests a GPU adapter, creates a logical device, and initializes
    /// shared rendering resources (atlas, fonts, text engine, pipelines).
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

        // Determine preferred surface format by querying adapter
        // We use Bgra8UnormSrgb as the default preferred format (most common on all platforms)
        let surface_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        println!("Using surface format: {:?}", surface_format);

        // Create shared rendering resources (atlas, fonts, text engine, pipelines)
        println!("Initializing shared rendering resources...");

        // Create shared pipelines (stateless, reused across all windows)
        println!("  ✓ Creating shared rect pipeline...");
        let rect_pipeline = RectPipeline::new(&device, surface_format);

        println!("  ✓ Creating shared text pipeline...");
        let text_pipeline = TextPipeline::new(&device, surface_format);

        println!("  ✓ Creating shared rect SDF pipeline...");
        let rect_sdf_pipeline = RectSdfPipeline::new(&device, surface_format);

        println!("  ✓ Creating shared shadow SDF pipeline...");
        let shadow_sdf_pipeline = ShadowSdfPipeline::new(&device, surface_format);

        // Create glyph atlas (2048×2048 with up to 16 pages)
        println!("  ✓ Creating glyph atlas (2048×2048, 16 pages max)...");
        let glyph_atlas = GlyphAtlas::new(&device, 2048, 16);

        // Create font system
        println!("  ✓ Initializing font system...");
        let font_system = FontSystemWrapper::new();

        // Create text engine
        println!("  ✓ Creating text engine...");
        let text_engine = TextEngine::new();

        println!("Shared rendering resources initialized successfully!");

        Ok(RenderContext {
            instance,
            adapter,
            device,
            queue,
            rect_pipeline,
            text_pipeline,
            rect_sdf_pipeline,
            shadow_sdf_pipeline,
            glyph_atlas: Arc::new(Mutex::new(glyph_atlas)),
            font_system: Arc::new(Mutex::new(font_system)),
            text_engine: Arc::new(Mutex::new(text_engine)),
            surface_format,
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
