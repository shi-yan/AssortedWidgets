//! Per-window rendering state
//!
//! This module contains all rendering infrastructure specific to a single window.
//! Windows share pipelines from RenderContext but have their own surfaces and uniforms.

use crate::paint::RectInstance;
use crate::platform::PlatformWindow;
use crate::render::RenderContext;
use crate::text::TextInstance;
use crate::types::{Rect, Size};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Window uniforms shared by all rendering pipelines (per-window - contains projection matrix)
///
/// This uniform buffer is shared across all pipelines (rect, text, path, rect_sdf, shadow_sdf, image)
/// since they all need the same projection matrix for coordinate transformation.
/// The projection matrix converts from physical pixel coordinates to NDC.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct WindowUniforms {
    projection: [[f32; 4]; 4], // mat4x4 orthogonal projection matrix
}

/// Create an orthogonal projection matrix for 2D rendering
///
/// Maps logical pixel coordinates (0, 0) to NDC (-1, 1) at top-left,
/// and (width_logical, height_logical) to NDC (1, -1) at bottom-right.
///
/// The viewport (set to physical size) handles DPI scaling automatically.
///
/// The matrix is:
/// ```
/// [2/width,    0,         0,   -1]
/// [0,         -2/height,  0,    1]
/// [0,          0,        -1,    0]
/// [0,          0,         0,    1]
/// ```
fn create_orthogonal_projection(logical_width: f32, logical_height: f32) -> [[f32; 4]; 4] {
    [
        [2.0 / logical_width, 0.0, 0.0, 0.0],
        [0.0, -2.0 / logical_height, 0.0, 0.0],
        [0.0, 0.0, -1.0, 0.0],
        [-1.0, 1.0, 0.0, 1.0],
    ]
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
    /// Shared window uniform buffer (contains screen_size for coordinate transformation)
    /// This buffer is shared by ALL pipelines: rect, text, path, rect_sdf, shadow_sdf, image
    window_uniform_buffer: wgpu::Buffer,

    /// Shared window uniform bind group (@group(0) @binding(0) for all pipelines)
    /// All pipelines now use VERTEX | FRAGMENT visibility, so one bind group works for all
    window_uniform_bind_group: wgpu::BindGroup,

    /// SDF rect clip uniforms (clip regions for fragment shader) - DIFFERENT data, separate buffer
    _rect_sdf_clip_uniform_buffer: wgpu::Buffer,
    rect_sdf_clip_uniform_bind_group: wgpu::BindGroup,

    // ========================================
    // MSAA (Multisample Anti-Aliasing)
    // ========================================
    /// Multisampled texture for MSAA (rendering target)
    /// This is rendered to first, then resolved to the surface texture
    msaa_texture: Option<wgpu::Texture>,

    /// MSAA texture view (public for render pass configuration)
    pub msaa_view: Option<wgpu::TextureView>,

    // ========================================
    // Depth Buffer (For Z-Ordering)
    // ========================================
    /// Depth texture for proper layering of overlapping UI elements
    depth_texture: wgpu::Texture,

    /// Depth texture view (public for render pass configuration)
    pub depth_view: wgpu::TextureView,

    // ========================================
    // Per-Window Instance Buffers (Dynamic)
    // ========================================
    /// Rectangle instance buffer (reused each frame)
    rect_instance_buffer: Option<wgpu::Buffer>,
    rect_instance_capacity: usize,

    /// Text instance buffer (reused each frame)
    text_instance_buffer: Option<wgpu::Buffer>,
    text_instance_capacity: usize,

    /// SDF rectangle instance buffer (reused each frame)
    sdf_instance_buffer: Option<wgpu::Buffer>,
    sdf_instance_capacity: usize,

    /// Shadow instance buffer (reused each frame)
    shadow_instance_buffer: Option<wgpu::Buffer>,
    shadow_instance_capacity: usize,

    /// Path vertex buffer (reused each frame, for Lyon tessellation)
    path_vertex_buffer: Option<wgpu::Buffer>,
    path_vertex_capacity: usize,

    /// Path index buffer (reused each frame, for Lyon tessellation)
    path_index_buffer: Option<wgpu::Buffer>,
    path_index_capacity: usize,

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

        println!(
            "Window logical size: {}x{}, scale factor: {}, physical pixels: {}x{}",
            bounds.size.width, bounds.size.height, scale_factor, width, height
        );

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

        // Create SHARED window uniform buffer (projection matrix for coordinate transformation)
        // Projection maps LOGICAL coordinates to NDC (viewport handles physical size)
        // This single buffer is shared by ALL pipelines: rect, text, path, rect_sdf, shadow_sdf, image
        let logical_width = bounds.size.width as f32;
        let logical_height = bounds.size.height as f32;
        let projection = create_orthogonal_projection(logical_width, logical_height);

        let window_uniforms = WindowUniforms { projection };

        let window_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Window Uniform Buffer (Shared)"),
            contents: bytemuck::cast_slice(&[window_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create SHARED bind group for window uniforms (@group(0) @binding(0) for all pipelines)
        // Since all pipelines now have identical bind group layouts (VERTEX | FRAGMENT visibility),
        // we can use ANY pipeline's layout - they're all the same!
        let window_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Window Uniform Bind Group (Shared)"),
            layout: &context.rect_pipeline.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: window_uniform_buffer.as_entire_binding(),
            }],
        });

        // Create SDF rect clip uniforms and bind group (DIFFERENT data - separate buffer)
        let rect_sdf_clip_uniform_buffer =
            context.rect_sdf_pipeline.create_clip_uniform_buffer(device);
        let rect_sdf_clip_uniform_bind_group = context
            .rect_sdf_pipeline
            .create_clip_bind_group(device, &rect_sdf_clip_uniform_buffer);

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

        // Create depth texture for z-ordering
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: context.sample_count, // Match MSAA sample count
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(WindowRenderer {
            surface,
            config,
            format,
            window_uniform_buffer,
            window_uniform_bind_group,
            _rect_sdf_clip_uniform_buffer: rect_sdf_clip_uniform_buffer,
            rect_sdf_clip_uniform_bind_group,
            msaa_texture,
            msaa_view,
            depth_texture,
            depth_view,
            rect_instance_buffer: None,
            rect_instance_capacity: 0,
            text_instance_buffer: None,
            text_instance_capacity: 0,
            sdf_instance_buffer: None,
            sdf_instance_capacity: 0,
            shadow_instance_buffer: None,
            shadow_instance_capacity: 0,
            path_vertex_buffer: None,
            path_vertex_capacity: 0,
            path_index_buffer: None,
            path_index_capacity: 0,
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
            self.surface
                .configure(self.render_context.device(), &self.config);

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

            // Recreate depth texture with new size
            let device = self.render_context.device();
            self.depth_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: self.render_context.sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            self.depth_view = self.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

            // Update uniform buffers with new screen size
            self.update_screen_size(new_bounds.size, scale_factor as f32);

            println!(
                "Surface resized to {}x{} physical pixels (logical: {}x{}, scale: {})",
                width, height, new_bounds.size.width, new_bounds.size.height, scale_factor
            );
        }
    }

    /// Update screen size in uniform buffers
    pub fn update_screen_size(&mut self, size: Size, scale_factor: f32) {
        // Projection maps LOGICAL coordinates to NDC (viewport handles physical size)
        let logical_width = size.width as f32;
        let logical_height = size.height as f32;
        let projection = create_orthogonal_projection(logical_width, logical_height);

        // Update SHARED window uniform buffer (used by all pipelines)
        let window_uniforms = WindowUniforms { projection };
        self.render_context.queue().write_buffer(
            &self.window_uniform_buffer,
            0,
            bytemuck::cast_slice(&[window_uniforms]),
        );

        let physical_width = logical_width * scale_factor;
        let physical_height = logical_height * scale_factor;
        println!(
            "[WindowRenderer] Updated projection matrix: logical = {:.0}x{:.0}, scale = {:.1}x, physical = {:.0}x{:.0}",
            logical_width, logical_height, scale_factor, physical_width, physical_height
        );
    }

    /// Get the current surface texture for rendering
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    /// Render rectangles using shared pipeline
    pub fn render_rects(&mut self, render_pass: &mut wgpu::RenderPass, instances: &[RectInstance]) {
        if instances.is_empty() {
            return;
        }

        println!("Rendering {} rect instances", instances.len());

        println!("Rect instances: {:?}", instances);

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
        render_pass.set_bind_group(0, &self.window_uniform_bind_group, &[]);
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
                    resource: wgpu::BindingResource::Sampler(
                        &self.render_context.text_pipeline.sampler,
                    ),
                },
            ],
        });

        // Render using shared pipeline
        render_pass.set_pipeline(&self.render_context.text_pipeline.pipeline);
        render_pass.set_bind_group(0, &self.window_uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
        render_pass.draw(0..4, 0..instances.len() as u32);
    }

    /// Render shadows for shapes (rendered FIRST, at z=SHADOW layer)
    pub fn render_shadows(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        batcher: &crate::paint::PrimitiveBatcher,
        layered_bounds_tree: &mut crate::paint::LayeredBoundsTree,
    ) {
        if batcher.is_empty() {
            return;
        }

        // Count how many shapes have shadows (estimate for buffer sizing)
        let shadow_count = batcher
            .commands()
            .iter()
            .filter(|cmd| {
                if let crate::paint::DrawCommand::Rect { style, .. } = cmd {
                    style.shadow.is_some()
                } else {
                    false
                }
            })
            .count();

        if shadow_count == 0 {
            return;
        }

        let device = self.render_context.device();

        // ShadowInstance is 64 bytes (4 vec4 + 1 vec2 + 2 floats with padding)
        const SHADOW_INSTANCE_SIZE: usize = 64;

        // Create or resize instance buffer if needed
        let needed_capacity = shadow_count;
        if self.shadow_instance_buffer.is_none() || needed_capacity > self.shadow_instance_capacity {
            self.shadow_instance_capacity = needed_capacity.max(128);
            self.shadow_instance_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Shadow Instance Buffer"),
                size: (self.shadow_instance_capacity * SHADOW_INSTANCE_SIZE) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        // Render using the pre-allocated buffer
        let instance_buffer = self.shadow_instance_buffer.as_ref().unwrap();
        let instances_rendered = self.render_context.shadow_sdf_pipeline.render(
            &self.render_context.queue,
            render_pass,
            &self.window_uniform_bind_group,
            &self.rect_sdf_clip_uniform_bind_group,
            instance_buffer,
            batcher,
            layered_bounds_tree,
        );

        if instances_rendered > 0 {
            println!(
                "[WindowRenderer] Rendered {} shadow instances",
                instances_rendered
            );
        }
    }

    /// Render SDF rectangles (rounded rects with borders) using the SDF pipeline
    pub fn render_sdf_rects(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        batcher: &crate::paint::PrimitiveBatcher,
        layered_bounds_tree: &mut crate::paint::LayeredBoundsTree,
    ) {
        if batcher.is_empty() {
            return;
        }

        // Count how many rect commands we have (estimate for buffer sizing)
        let rect_count = batcher
            .commands()
            .iter()
            .filter(|cmd| matches!(cmd, crate::paint::DrawCommand::Rect { .. }))
            .count();

        if rect_count == 0 {
            return;
        }

        let device = self.render_context.device();

        // SDF RectInstance is 240 bytes (16 attributes × 16 bytes alignment)
        const SDF_RECT_INSTANCE_SIZE: usize = 240;

        // Create or resize instance buffer if needed
        let needed_capacity = rect_count;
        if self.sdf_instance_buffer.is_none() || needed_capacity > self.sdf_instance_capacity {
            self.sdf_instance_capacity = needed_capacity.max(128);
            self.sdf_instance_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("SDF Rect Instance Buffer"),
                size: (self.sdf_instance_capacity * SDF_RECT_INSTANCE_SIZE) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        // Render using the pre-allocated buffer
        let instance_buffer = self.sdf_instance_buffer.as_ref().unwrap();
        let instances_rendered = self.render_context.rect_sdf_pipeline.render(
            &self.render_context.queue,
            render_pass,
            &self.window_uniform_bind_group,
            &self.rect_sdf_clip_uniform_bind_group,
            instance_buffer,
            batcher,
            layered_bounds_tree,
        );

        if instances_rendered > 0 {
            println!(
                "[WindowRenderer] Rendered {} SDF rect instances",
                instances_rendered
            );
        }
    }

    /// Render paths (lines, bezier curves) using the path pipeline
    pub fn render_paths(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        batcher: &crate::paint::PrimitiveBatcher,
    ) {
        use crate::paint::DrawCommand;

        // Count path/line commands to estimate buffer size needed
        let path_command_count = batcher
            .commands()
            .iter()
            .filter(|cmd| matches!(cmd, DrawCommand::Line { .. } | DrawCommand::Path { .. }))
            .count();

        if path_command_count == 0 {
            return;
        }

        // Estimate needed capacity (conservative: ~100 vertices and ~200 indices per command)
        // Lyon tessellation typically produces 10-50 vertices per simple path
        let estimated_vertex_count = path_command_count * 100;
        let estimated_index_count = path_command_count * 200;

        let device = self.render_context.device();

        // Create or resize vertex buffer if needed
        if self.path_vertex_buffer.is_none() || estimated_vertex_count > self.path_vertex_capacity {
            self.path_vertex_capacity = estimated_vertex_count.max(1024);
            // PathVertex is 24 bytes (2 floats for position + 4 floats for color)
            const PATH_VERTEX_SIZE: usize = 24;
            self.path_vertex_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Path Vertex Buffer"),
                size: (self.path_vertex_capacity * PATH_VERTEX_SIZE) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        // Create or resize index buffer if needed
        if self.path_index_buffer.is_none() || estimated_index_count > self.path_index_capacity {
            self.path_index_capacity = estimated_index_count.max(2048);
            // Indices are u16 (2 bytes each)
            self.path_index_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Path Index Buffer"),
                size: (self.path_index_capacity * std::mem::size_of::<u16>()) as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        // Render using pre-allocated buffers
        let vertex_buffer = self.path_vertex_buffer.as_ref().unwrap();
        let index_buffer = self.path_index_buffer.as_ref().unwrap();

        self.render_context.path_pipeline.render(
            &self.render_context.queue,
            render_pass,
            &self.window_uniform_bind_group,
            vertex_buffer,
            index_buffer,
            batcher,
        );
    }

    /// Begin a new rendering frame
    pub fn begin_frame(&mut self) {
        // Lock shared resources briefly to update frame counters
        self.render_context
            .glyph_atlas
            .lock()
            .unwrap()
            .begin_frame();
        self.render_context
            .text_engine
            .lock()
            .unwrap()
            .begin_frame();
    }

    /// Render a single image (Phase 5)
    ///
    /// This renders an image using the ImagePipeline with a textured quad.
    ///
    /// # Arguments
    /// * `render_pass` - Active render pass
    /// * `instance` - Image instance data (position, size, tint, clip)
    /// * `texture` - GPU texture containing the image data
    /// * `texture_view` - Texture view for binding
    /// * `image_pipeline` - Shared image rendering pipeline
    pub fn render_image(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        instance: &crate::render::ImageInstance,
        _texture: &wgpu::Texture,
        texture_view: &wgpu::TextureView,
        image_pipeline: &crate::render::ImagePipeline,
    ) {
        let device = self.render_context.device();

        // Create instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image Instance Buffer"),
            contents: bytemuck::cast_slice(&[*instance]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create texture bind group for this specific image
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Image Texture Bind Group"),
            layout: &image_pipeline.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&image_pipeline.sampler),
                },
            ],
        });

        // Render the image
        render_pass.set_pipeline(&image_pipeline.pipeline);
        render_pass.set_bind_group(0, &self.window_uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
        render_pass.draw(0..4, 0..1); // 4 vertices for quad (triangle strip)
    }
}

// Re-export traits needed for WindowRenderer::new()
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
