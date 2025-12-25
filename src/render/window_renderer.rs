//! Per-window rendering state
//!
//! This module contains all rendering infrastructure specific to a single window.
//! Windows share pipelines from RenderContext but have their own surfaces and uniforms.

use crate::platform::PlatformWindow;
use crate::render::RenderContext;
use crate::types::{Rect, Size};
use crate::paint::RectInstance;
use crate::text::TextInstance;
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Uniforms for the rect shader (per-window - contains screen_size)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RectUniforms {
    screen_size: [f32; 2],
    _padding: [f32; 2],  // Align to 16 bytes
}

/// Uniforms for the text shader (per-window - contains screen_size)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TextUniforms {
    screen_size: [f32; 2],
    _padding: [f32; 2],  // Align to 16 bytes
}

/// Uniforms for the path shader (per-window - contains screen_size)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PathUniforms {
    screen_size: [f32; 2],
    _padding: [f32; 2],  // Align to 16 bytes
}

/// Per-window rendering resources
///
/// This struct consolidates all window-specific rendering state.
/// Previously split between WindowRenderer and WindowRenderState, now unified.
///
/// Each window has its own:
/// - Surface (unique to each window)
/// - Uniform buffers (screen_size varies per window)
/// - Instance buffers (dynamic, reused per frame)
/// - Scale factor (for DPI)
///
/// Windows share (via Arc<RenderContext>):
/// - Pipelines (rect, text) - stateless, created once
/// - Glyph atlas - single texture cache for all windows
/// - Font system - initialized once, shared
/// - Text engine - shaping cache shared
pub struct WindowRenderer {
    // ========================================
    // Surface Management (Per-Window)
    // ========================================
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub format: wgpu::TextureFormat,

    // ========================================
    // Per-Window Uniforms (Screen Size)
    // ========================================
    /// Rectangle uniforms (contains screen_size specific to this window)
    rect_uniform_buffer: wgpu::Buffer,
    rect_uniform_bind_group: wgpu::BindGroup,

    /// Text uniforms (contains screen_size specific to this window)
    text_uniform_buffer: wgpu::Buffer,
    text_uniform_bind_group: wgpu::BindGroup,

    /// SDF rect uniforms (contains screen_size specific to this window)
    rect_sdf_uniform_buffer: wgpu::Buffer,
    rect_sdf_uniform_bind_group: wgpu::BindGroup,

    /// SDF rect clip uniforms (clip regions for fragment shader)
    rect_sdf_clip_uniform_buffer: wgpu::Buffer,
    rect_sdf_clip_uniform_bind_group: wgpu::BindGroup,

    /// Path uniforms (contains screen_size specific to this window)
    path_uniform_buffer: wgpu::Buffer,
    path_uniform_bind_group: wgpu::BindGroup,

    // ========================================
    // MSAA (Multisample Anti-Aliasing)
    // ========================================
    /// Multisampled texture for MSAA (rendering target)
    /// This is rendered to first, then resolved to the surface texture
    msaa_texture: Option<wgpu::Texture>,

    /// MSAA texture view (public for render pass configuration)
    pub msaa_view: Option<wgpu::TextureView>,

    // ========================================
    // Per-Window Instance Buffers (Dynamic)
    // ========================================
    /// Rectangle instance buffer (reused each frame)
    rect_instance_buffer: Option<wgpu::Buffer>,
    rect_instance_capacity: usize,

    /// Text instance buffer (reused each frame)
    text_instance_buffer: Option<wgpu::Buffer>,
    text_instance_capacity: usize,

    // ========================================
    // Window State
    // ========================================
    /// Current DPI scale factor (1.0 = standard, 2.0 = Retina)
    pub scale_factor: f32,

    /// Reference to shared rendering context
    /// (GPU resources, pipelines, atlas, fonts)
    pub render_context: Arc<RenderContext>,
}

impl WindowRenderer {
    /// Create a new window renderer for the given window
    pub fn new<W: PlatformWindow + HasWindowHandle + HasDisplayHandle>(
        context: Arc<RenderContext>,
        window: &W,
    ) -> Result<Self, String> {
        let device = context.device();

        // Create surface from window handles
        let surface = unsafe {
            let target = wgpu::SurfaceTargetUnsafe::from_window(window)
                .map_err(|e| format!("Failed to create surface target: {}", e))?;

            context
                .instance
                .create_surface_unsafe(target)
                .map_err(|e| format!("Failed to create surface: {}", e))?
        };

        // Use the shared surface format from RenderContext
        let format = context.surface_format;

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

        surface.configure(device, &config);

        // Create per-window uniform buffers (screen_size specific to this window)
        let physical_size = [width as f32, height as f32];

        let rect_uniforms = RectUniforms {
            screen_size: physical_size,
            _padding: [0.0, 0.0],
        };

        let rect_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Uniform Buffer"),
            contents: bytemuck::cast_slice(&[rect_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let rect_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Rect Bind Group"),
            layout: &context.rect_pipeline.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: rect_uniform_buffer.as_entire_binding(),
            }],
        });

        let text_uniforms = TextUniforms {
            screen_size: physical_size,
            _padding: [0.0, 0.0],
        };

        let text_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Text Uniform Buffer"),
            contents: bytemuck::cast_slice(&[text_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let text_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Uniform Bind Group"),
            layout: &context.text_pipeline.uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: text_uniform_buffer.as_entire_binding(),
            }],
        });

        // Create SDF rect uniforms and bind group
        let rect_sdf_uniform_buffer = context.rect_sdf_pipeline.create_uniform_buffer(device, width, height);
        let rect_sdf_uniform_bind_group = context.rect_sdf_pipeline.create_bind_group(device, &rect_sdf_uniform_buffer);

        // Create SDF rect clip uniforms and bind group (initially empty)
        let rect_sdf_clip_uniform_buffer = context.rect_sdf_pipeline.create_clip_uniform_buffer(device);
        let rect_sdf_clip_uniform_bind_group = context.rect_sdf_pipeline.create_clip_bind_group(device, &rect_sdf_clip_uniform_buffer);

        // Create path uniforms and bind group
        let path_uniform_buffer = context.path_pipeline.create_uniform_buffer(device, width, height);
        let path_uniform_bind_group = context.path_pipeline.create_bind_group(device, &path_uniform_buffer);

        // Create MSAA texture if sample count > 1
        let (msaa_texture, msaa_view) = if context.sample_count > 1 {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("MSAA Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: context.sample_count,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            (Some(texture), Some(view))
        } else {
            (None, None)
        };

        Ok(WindowRenderer {
            surface,
            config,
            format,
            rect_uniform_buffer,
            rect_uniform_bind_group,
            text_uniform_buffer,
            text_uniform_bind_group,
            rect_sdf_uniform_buffer,
            rect_sdf_uniform_bind_group,
            rect_sdf_clip_uniform_buffer,
            rect_sdf_clip_uniform_bind_group,
            path_uniform_buffer,
            path_uniform_bind_group,
            msaa_texture,
            msaa_view,
            rect_instance_buffer: None,
            rect_instance_capacity: 0,
            text_instance_buffer: None,
            text_instance_capacity: 0,
            scale_factor: scale_factor as f32,
            render_context: context,
        })
    }

    /// Reconfigure surface when window is resized
    pub fn resize(&mut self, new_bounds: Rect, scale_factor: f64) {
        // Use physical pixels for Retina displays
        let width = (new_bounds.size.width * scale_factor).max(1.0) as u32;
        let height = (new_bounds.size.height * scale_factor).max(1.0) as u32;

        if width != self.config.width || height != self.config.height {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(self.render_context.device(), &self.config);

            // Update scale factor
            self.scale_factor = scale_factor as f32;

            // Recreate MSAA texture with new size
            if self.render_context.sample_count > 1 {
                let device = self.render_context.device();
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("MSAA Texture"),
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: self.render_context.sample_count,
                    dimension: wgpu::TextureDimension::D2,
                    format: self.format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                });
                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                self.msaa_texture = Some(texture);
                self.msaa_view = Some(view);
            }

            // Update uniform buffers with new screen size
            self.update_screen_size(new_bounds.size, scale_factor as f32);

            println!("Surface resized to {}x{} physical pixels (logical: {}x{}, scale: {})",
                width, height, new_bounds.size.width, new_bounds.size.height, scale_factor);
        }
    }

    /// Update screen size in uniform buffers
    pub fn update_screen_size(&mut self, size: Size, scale_factor: f32) {
        // Scale logical size by scale_factor to match physical viewport
        let physical_size = [size.width as f32 * scale_factor, size.height as f32 * scale_factor];

        // Update rect uniforms
        let rect_uniforms = RectUniforms {
            screen_size: physical_size,
            _padding: [0.0, 0.0],
        };
        self.render_context.queue().write_buffer(
            &self.rect_uniform_buffer,
            0,
            bytemuck::cast_slice(&[rect_uniforms]),
        );

        // Update text uniforms
        let text_uniforms = TextUniforms {
            screen_size: physical_size,
            _padding: [0.0, 0.0],
        };
        self.render_context.queue().write_buffer(
            &self.text_uniform_buffer,
            0,
            bytemuck::cast_slice(&[text_uniforms]),
        );

        // Update SDF rect uniforms
        let width = (size.width * scale_factor as f64) as u32;
        let height = (size.height * scale_factor as f64) as u32;
        self.render_context.rect_sdf_pipeline.update_uniforms(
            self.render_context.queue(),
            &self.rect_sdf_uniform_buffer,
            width,
            height,
        );

        // Update path uniforms
        self.render_context.path_pipeline.update_uniforms(
            self.render_context.queue(),
            &self.path_uniform_buffer,
            width,
            height,
        );

        println!("[WindowRenderer] Updated uniforms: logical = {:.0}x{:.0}, scale = {:.1}x, physical = {:.0}x{:.0}",
            size.width, size.height, scale_factor, physical_size[0], physical_size[1]);
    }

    /// Get the current surface texture for rendering
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    /// Render rectangles using shared pipeline
    pub fn render_rects(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        instances: &[RectInstance],
    ) {
        if instances.is_empty() {
            return;
        }

        let device = self.render_context.device();

        // Create or resize instance buffer if needed
        let needed_capacity = instances.len();
        if self.rect_instance_buffer.is_none() || needed_capacity > self.rect_instance_capacity {
            self.rect_instance_capacity = needed_capacity.max(128);
            self.rect_instance_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Rect Instance Buffer"),
                size: (self.rect_instance_capacity * std::mem::size_of::<RectInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        // Upload instance data
        let instance_buffer = self.rect_instance_buffer.as_ref().unwrap();
        self.render_context.queue().write_buffer(
            instance_buffer,
            0,
            bytemuck::cast_slice(instances),
        );

        // Render using shared pipeline
        render_pass.set_pipeline(&self.render_context.rect_pipeline.pipeline);
        render_pass.set_bind_group(0, &self.rect_uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
        render_pass.draw(0..4, 0..instances.len() as u32);
    }

    /// Render text using shared pipeline
    pub fn render_text(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        instances: &[TextInstance],
        atlas_texture_view: &wgpu::TextureView,
    ) {
        if instances.is_empty() {
            return;
        }

        let device = self.render_context.device();

        // Create or resize instance buffer if needed
        let needed_capacity = instances.len();
        if self.text_instance_buffer.is_none() || needed_capacity > self.text_instance_capacity {
            self.text_instance_capacity = needed_capacity.max(128);
            self.text_instance_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Text Instance Buffer"),
                size: (self.text_instance_capacity * std::mem::size_of::<TextInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        // Upload instance data
        let instance_buffer = self.text_instance_buffer.as_ref().unwrap();
        self.render_context.queue().write_buffer(
            instance_buffer,
            0,
            bytemuck::cast_slice(instances),
        );

        // Create texture bind group
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Texture Bind Group"),
            layout: &self.render_context.text_pipeline.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(atlas_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.render_context.text_pipeline.sampler),
                },
            ],
        });

        // Render using shared pipeline
        render_pass.set_pipeline(&self.render_context.text_pipeline.pipeline);
        render_pass.set_bind_group(0, &self.text_uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
        render_pass.draw(0..4, 0..instances.len() as u32);
    }

    /// Render shadows for shapes (rendered FIRST, at z=SHADOW layer)
    pub fn render_shadows(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        batcher: &crate::paint::PrimitiveBatcher,
    ) {
        // Shadows use the same clip uniform buffer as rectangles
        self.render_context.shadow_sdf_pipeline.render(
            &self.render_context.device,
            &self.render_context.queue,
            render_pass,
            &self.rect_sdf_uniform_buffer,  // Reuse rect uniforms (same screen_size)
            &self.rect_sdf_uniform_bind_group,
            &self.rect_sdf_clip_uniform_bind_group,
            batcher,
        );
    }

    /// Render SDF rectangles (rounded rects with borders) using the SDF pipeline
    pub fn render_sdf_rects(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        batcher: &crate::paint::PrimitiveBatcher,
    ) {
        // TODO: Extract clip stack from batcher and update clip uniform buffer
        // For now, we render with empty clip stack (will be integrated in visual test)

        self.render_context.rect_sdf_pipeline.render(
            &self.render_context.device,
            &self.render_context.queue,
            render_pass,
            &self.rect_sdf_uniform_buffer,
            &self.rect_sdf_uniform_bind_group,
            &self.rect_sdf_clip_uniform_bind_group,
            batcher,
        );
    }

    /// Render paths (lines, bezier curves) using the path pipeline
    pub fn render_paths(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        batcher: &crate::paint::PrimitiveBatcher,
    ) {
        self.render_context.path_pipeline.render(
            &self.render_context.device,
            render_pass,
            &self.path_uniform_bind_group,
            batcher,
        );
    }

    /// Begin a new rendering frame
    pub fn begin_frame(&mut self) {
        // Lock shared resources briefly to update frame counters
        self.render_context.glyph_atlas.lock().unwrap().begin_frame();
        self.render_context.text_engine.lock().unwrap().begin_frame();
    }
}

// Re-export traits needed for WindowRenderer::new()
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
