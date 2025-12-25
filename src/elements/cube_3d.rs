//! 3D Cube Widget - Demonstrates low-level WebGPU RenderPass access
//!
//! This widget shows how to use the Tier 2 rendering API for custom 3D graphics.
//! It creates its own GPU resources (buffers, pipelines) and uses
//! `register_custom_render()` to access the WebGPU RenderPass directly.

use crate::widget::Widget;
use crate::layout::Style;
use crate::paint::PaintContext;
use crate::types::{DeferredCommand, GuiMessage, Rect, WidgetId};
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],  // Model-View-Projection matrix
}

/// 3D rotating cube widget
///
/// This demonstrates custom 3D rendering using low-level WebGPU access.
/// The cube rotates continuously and can be placed anywhere in the UI.
pub struct Cube3D {
    id: WidgetId,
    bounds: Rect,

    // GPU Resources (created once, reused every frame)
    pipeline: Arc<wgpu::RenderPipeline>,
    bind_group: Arc<wgpu::BindGroup>,
    vertex_buffer: Arc<wgpu::Buffer>,
    index_buffer: Arc<wgpu::Buffer>,
    uniform_buffer: Arc<wgpu::Buffer>,

    // Animation state
    rotation: f32,  // Rotation angle in radians
}

impl Cube3D {
    /// Create a new 3D cube widget
    ///
    /// This initializes all GPU resources needed for rendering.
    /// Call this during application setup with access to the device.
    pub fn new(
        id: WidgetId,
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        // Define cube vertices (8 vertices, 6 faces with different colors)
        let vertices = [
            // Front face (red)
            Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
            // Back face (green)
            Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
            // Top face (blue)
            Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] },
            // Bottom face (yellow)
            Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
            Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
            Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0] },
            Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0] },
            // Right face (cyan)
            Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0] },
            Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 1.0] },
            Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 1.0, 1.0] },
            Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 1.0] },
            // Left face (magenta)
            Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0] },
            Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 1.0] },
            Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 1.0] },
            Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 1.0] },
        ];

        // Define indices for the 12 triangles (6 faces * 2 triangles each)
        #[allow(clippy::zero_prefixed_literal)]
        let indices: [u16; 36] = [
            00, 01, 02,  02, 03, 00,  // Front
            04, 06, 05,  06, 04, 07,  // Back
            08, 09, 10,  10, 11, 08,  // Top
            12, 14, 13,  14, 12, 15,  // Bottom
            16, 17, 18,  18, 19, 16,  // Right
            20, 22, 21,  22, 20, 23,  // Left
        ];

        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create uniform buffer (will be updated each frame)
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Cube Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cube Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/cube_3d.wgsl").into()),
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Cube Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cube Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cube Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        // Create render pipeline with depth testing
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            // position
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            // color
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as u64,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,  // We'll skip depth buffer for simplicity
            multisample: wgpu::MultisampleState {
                count: sample_count,  // Match framework's MSAA setting
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self {
            id,
            bounds: Rect::default(),
            pipeline: Arc::new(pipeline),
            bind_group: Arc::new(bind_group),
            vertex_buffer: Arc::new(vertex_buffer),
            index_buffer: Arc::new(index_buffer),
            uniform_buffer: Arc::new(uniform_buffer),
            rotation: 0.0,
        }
    }

    /// Update animation (call this each frame)
    pub fn update(&mut self, delta_time: f32) {
        self.rotation += delta_time * 1.0;  // Rotate 1 radian per second
    }

    /// Create MVP matrix for the cube (static method for use in paint)
    fn create_mvp_matrix_static(rotation: f32, aspect_ratio: f32) -> [[f32; 4]; 4] {
        // Model matrix (rotation around Y and X axes)
        let cos_y = rotation.cos();
        let sin_y = rotation.sin();
        let cos_x = (rotation * 0.5).cos();
        let sin_x = (rotation * 0.5).sin();

        // Combined rotation matrix (Y * X)
        let model = [
            [cos_y, sin_x * sin_y, cos_x * sin_y, 0.0],
            [0.0, cos_x, -sin_x, 0.0],
            [-sin_y, sin_x * cos_y, cos_x * cos_y, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        // View matrix (camera at [0, 0, 3] looking at origin)
        let view = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, -3.0, 1.0],
        ];

        // Projection matrix (perspective)
        let fov = 45.0_f32.to_radians();
        let near = 0.1;
        let far = 100.0;
        let f = 1.0 / (fov / 2.0).tan();

        let projection = [
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, -f, 0.0, 0.0],  // Flip Y for correct orientation
            [0.0, 0.0, far / (near - far), -1.0],
            [0.0, 0.0, (near * far) / (near - far), 0.0],
        ];

        // Multiply matrices: P * V * M
        multiply_mat4(projection, multiply_mat4(view, model))
    }
}

impl Widget for Cube3D {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &crate::event::OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn set_dirty(&mut self, _dirty: bool) {
        // Always dirty (animating)
    }

    fn is_dirty(&self) -> bool {
        true  // Always render (continuously animating)
    }

    fn layout(&self) -> Style {
        Style {
            size: taffy::Size {
                width: taffy::Dimension::length(400.0),
                height: taffy::Dimension::length(400.0),
            },
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        println!("[Cube3D] paint() called - bounds: {:?}", self.bounds);

        // Skip rendering if bounds are invalid
        if self.bounds.size.width <= 0.0 || self.bounds.size.height <= 0.0 {
            println!("[Cube3D] Skipping - invalid bounds");
            return;
        }

        // Update rotation (will be mutable via interior mutability workaround)
        // For now, use a simple time-based rotation
        use std::time::{SystemTime, UNIX_EPOCH};
        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f32();
        let rotation = elapsed * 1.0;  // 1 radian per second

        // Calculate aspect ratio from bounds
        let aspect_ratio = self.bounds.size.width as f32 / self.bounds.size.height as f32;

        // Create MVP matrix using current rotation
        let mvp = Self::create_mvp_matrix_static(rotation, aspect_ratio);
        let uniforms = Uniforms { mvp };

        // Debug: Print MVP matrix first frame only
        static PRINTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !PRINTED.swap(true, std::sync::atomic::Ordering::Relaxed) {
            println!("[Cube3D] MVP matrix (first frame):");
            for (i, row) in mvp.iter().enumerate() {
                println!("  [{:8.4}, {:8.4}, {:8.4}, {:8.4}]", row[0], row[1], row[2], row[3]);
            }
            println!("[Cube3D] Rotation: {:.2}, Aspect: {:.2}", rotation, aspect_ratio);
        }

        // Get queue for uploading uniforms
        let queue = ctx.queue();
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        // Clone Arc references for use in the closure
        let pipeline = Arc::clone(&self.pipeline);
        let bind_group = Arc::clone(&self.bind_group);
        let vertex_buffer = Arc::clone(&self.vertex_buffer);
        let index_buffer = Arc::clone(&self.index_buffer);

        // Capture bounds for viewport
        let x = self.bounds.origin.x as f32;
        let y = self.bounds.origin.y as f32;
        let w = self.bounds.size.width as f32;
        let h = self.bounds.size.height as f32;

        // Register custom render callback for 3D rendering
        ctx.register_custom_render(move |render_pass| {
            println!("[Cube3D] Callback executing! Viewport: {}x{} at ({}, {})", w, h, x, y);

            // CRITICAL: Set viewport to widget bounds!
            // Without this, cube renders in NDC space to entire window
            render_pass.set_viewport(x, y, w, h, 0.0, 1.0);

            render_pass.set_pipeline(&*pipeline);
            render_pass.set_bind_group(0, Some(&*bind_group), &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..36, 0, 0..1);
            println!("[Cube3D] Draw call issued!");
        });
        println!("[Cube3D] Callback registered");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Helper function to multiply 4x4 matrices
fn multiply_mat4(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}
