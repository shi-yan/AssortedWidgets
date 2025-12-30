//! Rendering system using WebGPU
//!
//! ## Architecture
//!
//! - **RenderContext**: Shared GPU state and rendering resources
//!   - GPU resources: instance, adapter, device, queue
//!   - Rendering resources: glyph atlas, font system, text engine
//!   - Created once, shared across all windows via Arc
//! - **WindowRenderer**: Per-window rendering state (surface, configuration)
//! - **Pipeline Trait**: Common interface for all rendering pipelines
//!   - All pipelines implement this trait for consistency
//!   - Pipelines are stateless and shared across windows
//!   - Per-window state (buffers, bind groups) is managed by WindowRenderer
//!
//! ## Usage
//!
//! ```no_run
//! use assorted_widgets::render::*;
//!
//! // Initialize shared rendering context (GPU + atlas + fonts)
//! let render_ctx = pollster::block_on(RenderContext::new()).unwrap();
//!
//! // Create per-window renderer
//! let window_renderer = WindowRenderer::new(&render_ctx, &window).unwrap();
//!
//! // Render a frame
//! let surface_texture = window_renderer.get_current_texture().unwrap();
//! let view = surface_texture.texture.create_view(&Default::default());
//!
//! let mut encoder = render_ctx.device.create_command_encoder(&Default::default());
//! {
//!     let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//!         // ... render your elements
//!     });
//! }
//! render_ctx.queue.submit([encoder.finish()]);
//! surface_texture.present();
//! ```

/// Common interface for all rendering pipelines
///
/// All pipelines in AssortedWidgets implement this trait for consistency.
/// Pipelines are stateless GPU objects that are created once and shared
/// across all windows via RenderContext.
///
/// Per-window state (uniform buffers, instance buffers, bind groups) is
/// managed by WindowRenderer, not by the pipelines themselves.
///
/// ## Note on Construction
/// Pipelines have a `new()` method, but it's not part of this trait to avoid
/// conflicts with inherent implementations. All pipelines follow the convention:
/// ```ignore
/// impl SomePipeline {
///     pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat, sample_count: u32) -> Self;
/// }
/// ```
pub trait Pipeline {
    /// Get a reference to the underlying wgpu render pipeline
    ///
    /// This allows the pipeline to be set on a render pass.
    fn pipeline(&self) -> &wgpu::RenderPipeline;

    /// Get the pipeline's label for debugging
    ///
    /// Returns a human-readable name for this pipeline (e.g., "Rect Pipeline", "Text Pipeline")
    fn label(&self) -> &'static str;
}

mod context;
mod image_pipeline;
mod path_pipeline;
mod rect_sdf_pipeline;
mod shadow_sdf_pipeline;
mod window_renderer;
mod rect_pipeline;
mod text_pipeline;

pub use context::RenderContext;
pub use image_pipeline::{ImageInstance, ImagePipeline};
pub use path_pipeline::PathPipeline;
pub use rect_pipeline::RectPipeline;
pub use text_pipeline::TextPipeline;
pub use rect_sdf_pipeline::RectSdfPipeline;
pub use shadow_sdf_pipeline::ShadowSdfPipeline;
pub use window_renderer::WindowRenderer;
