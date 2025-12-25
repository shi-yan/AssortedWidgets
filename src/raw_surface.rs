//! RawSurface - Base for widgets that need direct GPU rendering access
//!
//! This module provides the infrastructure for widgets that want to do custom
//! GPU rendering (3D graphics, custom shaders, etc.) while properly integrating
//! with the UI system's z-ordering and layout.
//!
//! Architecture:
//! 1. Each RawSurface widget gets its own framebuffer texture
//! 2. Widget renders to texture via paint_raw()
//! 3. Texture is composited as a quad at widget's bounds with proper z-order
//! 4. Framework handles texture resize when layout changes

use crate::types::{Size, WidgetId};
use std::sync::Arc;

/// Manages a framebuffer texture for a RawSurface widget
pub struct RawSurfaceFramebuffer {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub size: Size,
    pub sample_count: u32,
}

impl RawSurfaceFramebuffer {
    /// Create a new framebuffer with the given size
    pub fn new(
        device: &wgpu::Device,
        size: Size,
        format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RawSurface Framebuffer"),
            size: wgpu::Extent3d {
                width: size.width.max(1.0) as u32,
                height: size.height.max(1.0) as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            view,
            size,
            sample_count,
        }
    }

    /// Resize the framebuffer (recreates texture)
    pub fn resize(&mut self, device: &wgpu::Device, new_size: Size, format: wgpu::TextureFormat) {
        if (self.size.width - new_size.width).abs() < 1.0
            && (self.size.height - new_size.height).abs() < 1.0
        {
            return; // No significant change
        }

        *self = Self::new(device, new_size, format, self.sample_count);
    }
}

/// Trait for widgets that need direct GPU rendering access
pub trait RawSurface {
    /// Get the widget's ID
    fn widget_id(&self) -> WidgetId;

    /// Called when the widget needs to render to its framebuffer
    ///
    /// The render_pass is already configured for the widget's texture.
    /// The widget should issue draw calls but NOT begin/end the pass.
    fn paint_raw(&self, render_pass: &mut wgpu::RenderPass, size: Size);

    /// Called when the widget's size changes
    ///
    /// Widget should recreate any size-dependent resources (uniforms, etc.)
    fn on_resize(&mut self, new_size: Size);

    /// Get current size for framebuffer management
    fn framebuffer_size(&self) -> Size;

    /// Get the framebuffer texture view for compositing
    ///
    /// Returns None if framebuffer hasn't been created yet.
    /// The framework uses this to composite the widget's rendering to the main screen.
    fn framebuffer_view(&self) -> Option<&wgpu::TextureView>;
}
