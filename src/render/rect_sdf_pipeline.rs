use crate::paint::{DrawCommand, PrimitiveBatcher};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// GPU representation of a rectangle instance (matches WGSL struct)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RectInstance {
    rect: [f32; 4],           // x, y, width, height
    corner_radius: [f32; 4],  // top_left, top_right, bottom_right, bottom_left
    fill_color: [f32; 4],     // rgba
    border_color: [f32; 4],   // rgba
    border_width: f32,
    _padding: [f32; 3],       // Align to 16 bytes
}

/// Uniform buffer for screen size
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    screen_size: [f32; 2],
    _padding: [f32; 2], // Align to 16 bytes
}

/// Pipeline for rendering rounded rectangles using SDF
pub struct RectSdfPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    clip_bind_group_layout: wgpu::BindGroupLayout,
}

impl RectSdfPipeline {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Rect SDF Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/rect_sdf.wgsl").into()),
        });

        // Bind group layout for uniforms (group 0)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Rect SDF Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Bind group layout for clip uniforms (group 1)
        let clip_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Rect SDF Clip Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Rect SDF Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout, &clip_bind_group_layout],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rect SDF Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    // Instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<RectInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            // rect: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // corner_radius: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 16,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // fill_color: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 32,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // border_color: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 48,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // border_width: f32
                            wgpu::VertexAttribute {
                                offset: 64,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32,
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
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group_layout,
            clip_bind_group_layout,
        }
    }

    /// Render batched rectangles
    pub fn render(
        &self,
        device: &Arc<wgpu::Device>,
        _queue: &Arc<wgpu::Queue>,
        render_pass: &mut wgpu::RenderPass,
        _uniform_buffer: &wgpu::Buffer,
        uniform_bind_group: &wgpu::BindGroup,
        clip_bind_group: &wgpu::BindGroup,
        batcher: &PrimitiveBatcher,
    ) {
        if batcher.is_empty() {
            return;
        }

        // Convert draw commands to instances
        let instances: Vec<RectInstance> = batcher
            .commands()
            .iter()
            .filter_map(|cmd| match cmd {
                DrawCommand::Rect { rect, style, .. } => {
                    let fill_color = style.fill.to_color();
                    let border_color = style.border.as_ref()
                        .map(|b| b.color)
                        .unwrap_or(fill_color);
                    let border_width = style.border.as_ref()
                        .map(|b| b.width)
                        .unwrap_or(0.0);

                    Some(RectInstance {
                        rect: [rect.origin.x as f32, rect.origin.y as f32, rect.size.width as f32, rect.size.height as f32],
                        corner_radius: style.corner_radius.to_array(),
                        fill_color: fill_color.to_array(),
                        border_color: border_color.to_array(),
                        border_width,
                        _padding: [0.0; 3],
                    })
                }
                // Clip commands are not rendered as rectangles
                DrawCommand::PushClip { .. } | DrawCommand::PopClip => None,
            })
            .collect();

        if instances.is_empty() {
            return;
        }

        // Create instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Render
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, uniform_bind_group, &[]);
        render_pass.set_bind_group(1, clip_bind_group, &[]);
        render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
        render_pass.draw(0..6, 0..instances.len() as u32); // 6 vertices per quad (2 triangles)
    }

    /// Create uniform buffer for screen size
    pub fn create_uniform_buffer(
        &self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> wgpu::Buffer {
        let uniforms = Uniforms {
            screen_size: [width as f32, height as f32],
            _padding: [0.0; 2],
        };

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect SDF Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    /// Create bind group for uniforms
    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        uniform_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Rect SDF Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        })
    }

    /// Update uniform buffer with new screen size
    pub fn update_uniforms(
        &self,
        queue: &wgpu::Queue,
        uniform_buffer: &wgpu::Buffer,
        width: u32,
        height: u32,
    ) {
        let uniforms = Uniforms {
            screen_size: [width as f32, height as f32],
            _padding: [0.0; 2],
        };

        queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    /// Create clip uniform buffer (initially with no active clips)
    pub fn create_clip_uniform_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        // Buffer size to match WGSL struct layout with alignment:
        // struct ClipUniforms {
        //     count: u32,                         // 4 bytes
        //     _padding: vec3<u32>,                // 12 bytes -> 16 total (aligned)
        //     regions: array<ClipRegion, 8>,      // 8 * 32 = 256 bytes
        // }
        // Shader expects: 16 (header) + 272 (8 regions * 34 bytes with padding) = 288 bytes
        let size = 288;

        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rect SDF Clip Uniform Buffer"),
            size: size as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    /// Create bind group for clip uniforms
    pub fn create_clip_bind_group(
        &self,
        device: &wgpu::Device,
        clip_uniform_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Rect SDF Clip Bind Group"),
            layout: &self.clip_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: clip_uniform_buffer.as_entire_binding(),
            }],
        })
    }

    /// Update clip uniform buffer from ClipStack
    pub fn update_clip_uniforms(
        &self,
        queue: &wgpu::Queue,
        clip_uniform_buffer: &wgpu::Buffer,
        clip_data: &[f32],
    ) {
        queue.write_buffer(clip_uniform_buffer, 0, bytemuck::cast_slice(clip_data));
    }
}
