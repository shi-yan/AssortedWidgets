//! Rendering system using WebGPU
//!
//! ## Architecture
//!
//! - **RenderContext**: Shared GPU state and rendering resources
//!   - GPU resources: instance, adapter, device, queue
//!   - Rendering resources: glyph atlas, font system, text engine
//!   - Created once, shared across all windows via Arc
//! - **WindowRenderer**: Per-window rendering state (surface, configuration)
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

mod context;
mod pipelines;
mod window_renderer;

pub use context::RenderContext;
pub use pipelines::{RectPipeline, TextPipeline};
pub use window_renderer::WindowRenderer;
