//! Per-window rendering state

use crate::platform::PlatformWindow;
use crate::render::RenderContext;
use crate::types::Rect;

/// Per-window renderer
///
/// Each window has its own surface and configuration,
/// but shares the device/queue from RenderContext.
pub struct WindowRenderer {
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub format: wgpu::TextureFormat,
}

impl WindowRenderer {
    /// Create a new window renderer for the given window
    pub fn new<W: PlatformWindow + HasWindowHandle + HasDisplayHandle>(
        context: &RenderContext,
        window: &W,
    ) -> Result<Self, String> {
        // Create surface from window handles (using unsafe API for raw handles)
        let surface = unsafe {
            let target = wgpu::SurfaceTargetUnsafe::from_window(window)
                .map_err(|e| format!("Failed to create surface target: {}", e))?;

            context
                .instance
                .create_surface_unsafe(target)
                .map_err(|e| format!("Failed to create surface: {}", e))?
        };

        // Get surface capabilities
        let caps = surface.get_capabilities(&context.adapter);
        let format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);

        println!("Surface format: {:?}", format);

        // Get window size in PHYSICAL pixels (for Retina displays)
        let bounds = window.content_bounds();
        let scale_factor = window.scale_factor();
        let width = (bounds.size.width * scale_factor).max(1.0) as u32;
        let height = (bounds.size.height * scale_factor).max(1.0) as u32;

        println!("Window logical size: {}x{}, scale factor: {}, physical pixels: {}x{}",
            bounds.size.width, bounds.size.height, scale_factor, width, height);

        // Configure surface with physical pixel dimensions
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![format.add_srgb_suffix()],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(context.device(), &config);

        Ok(WindowRenderer {
            surface,
            config,
            format,
        })
    }

    /// Reconfigure surface when window is resized
    pub fn resize(&mut self, context: &RenderContext, new_bounds: Rect, scale_factor: f64) {
        // Use physical pixels for Retina displays
        let width = (new_bounds.size.width * scale_factor).max(1.0) as u32;
        let height = (new_bounds.size.height * scale_factor).max(1.0) as u32;

        if width != self.config.width || height != self.config.height {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(context.device(), &self.config);
            println!("Surface resized to {}x{} physical pixels (logical: {}x{}, scale: {})",
                width, height, new_bounds.size.width, new_bounds.size.height, scale_factor);
        }
    }

    /// Get the current surface texture for rendering
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }
}

// Re-export traits needed for WindowRenderer::new()
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
