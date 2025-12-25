use crate::paint::{DrawCommand, PrimitiveBatcher, Shadow};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// GPU representation of a shadow instance (matches WGSL struct)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ShadowInstance {
    rect: [f32; 4],           // x, y, width, height (of shape casting shadow)
    corner_radius: [f32; 4],  // top_left, top_right, bottom_right, bottom_left
    shadow_color: [f32; 4],   // rgba
    offset: [f32; 2],         // Shadow offset (x, y)
    blur_radius: f32,
    spread_radius: f32,
}

/// Uniform buffer for screen size
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    screen_size: [f32; 2],
    _padding: [f32; 2], // Align to 16 bytes
}

/// Pipeline for rendering drop shadows using analytical SDF
pub struct ShadowSdfPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    clip_bind_group_layout: wgpu::BindGroupLayout,
}

impl ShadowSdfPipeline {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat, sample_count: u32) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shadow SDF Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/shadow_sdf.wgsl").into()),
        });

        // Bind group layout for uniforms (group 0)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow SDF Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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
            label: Some("Shadow SDF Clip Bind Group Layout"),
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
            label: Some("Shadow SDF Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout, &clip_bind_group_layout],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shadow SDF Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    // Instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<ShadowInstance>() as wgpu::BufferAddress,
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
                            // shadow_color: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 32,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // offset: vec2<f32>
                            wgpu::VertexAttribute {
                                offset: 48,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            // blur_radius: f32
                            wgpu::VertexAttribute {
                                offset: 56,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32,
                            },
                            // spread_radius: f32
                            wgpu::VertexAttribute {
                                offset: 60,
                                shader_location: 5,
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
                count: sample_count,
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

    /// Render shadows for shapes that have shadows
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

        // Convert draw commands with shadows to shadow instances
        let instances: Vec<ShadowInstance> = batcher
            .commands()
            .iter()
            .filter_map(|cmd| match cmd {
                DrawCommand::Rect { rect, style, .. } => {
                    // Only render shadows for shapes that have them
                    style.shadow.as_ref().map(|shadow| {
                        ShadowInstance {
                            rect: [rect.origin.x as f32, rect.origin.y as f32, rect.size.width as f32, rect.size.height as f32],
                            corner_radius: style.corner_radius.to_array(),
                            shadow_color: shadow.color.to_array(),
                            offset: [shadow.offset.0, shadow.offset.1],
                            blur_radius: shadow.blur_radius,
                            spread_radius: shadow.spread_radius,
                        }
                    })
                }
                // Other commands don't cast shadows (lines and paths could, but not implemented yet)
                DrawCommand::Line { .. }
                | DrawCommand::Path { .. }
                | DrawCommand::Icon { .. }
                | DrawCommand::Image { .. }
                | DrawCommand::PushClip { .. }
                | DrawCommand::PopClip => None,
            })
            .collect();

        if instances.is_empty() {
            return;
        }

        // Create instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shadow Instance Buffer"),
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

    /// Create uniform buffer for screen size (reuse from rect pipeline)
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
            label: Some("Shadow SDF Uniform Buffer"),
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
            label: Some("Shadow SDF Bind Group"),
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

    /// Reference to clip bind group layout (same as rect pipeline)
    pub fn clip_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.clip_bind_group_layout
    }
}
