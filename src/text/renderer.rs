//! GPU-accelerated text renderer using instanced quads
//!
//! Renders all text in the UI with a single draw call using instancing.
//! Supports both monochrome text and color emoji via glyph type flag.

use crate::render::RenderContext;
use crate::types::Size;
use crate::paint::Color;
use wgpu::util::DeviceExt;

/// Instance data for rendering a single glyph quad
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextInstance {
    /// World-space position (top-left corner)
    pub position: [f32; 2],

    /// Glyph size in pixels
    pub glyph_size: [f32; 2],

    /// UV coordinates (min)
    pub uv_min: [f32; 2],

    /// UV coordinates (max)
    pub uv_max: [f32; 2],

    /// RGBA color
    pub color: [f32; 4],

    /// Which page in texture array
    pub page_index: u32,

    /// Glyph type: 0 = monochrome, 1 = color emoji
    pub glyph_type: u32,

    /// Clipping rectangle (x, y, width, height)
    pub clip_rect: [f32; 4],
}

impl TextInstance {
    /// Create a new text instance
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        uv_min_x: f32,
        uv_min_y: f32,
        uv_max_x: f32,
        uv_max_y: f32,
        color: Color,
        page_index: u32,
        is_color: bool,
        clip_rect: [f32; 4],
    ) -> Self {
        Self {
            position: [x, y],
            glyph_size: [width, height],
            uv_min: [uv_min_x, uv_min_y],
            uv_max: [uv_max_x, uv_max_y],
            color: [color.r, color.g, color.b, color.a],
            page_index,
            glyph_type: if is_color { 1 } else { 0 },
            clip_rect,
        }
    }
}

/// Uniforms for the text shader
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TextUniforms {
    screen_size: [f32; 2],
    _padding: [f32; 2],  // Align to 16 bytes
}

/// Renderer for batched text glyphs
pub struct TextRenderer {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    instance_buffer: Option<wgpu::Buffer>,
    instance_capacity: usize,
    sampler: wgpu::Sampler,
}

impl TextRenderer {
    /// Create a new text renderer
    pub fn new(context: &RenderContext, surface_format: wgpu::TextureFormat) -> Self {
        let device = context.device();

        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text.wgsl").into()),
        });

        // Create uniform buffer
        let uniforms = TextUniforms {
            screen_size: [800.0, 600.0],  // Will be updated on resize
            _padding: [0.0, 0.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Text Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create uniform bind group layout (group 0)
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Text Uniform Bind Group Layout"),
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

        // Create uniform bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create texture bind group layout (group 1)
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Text Texture Bind Group Layout"),
            entries: &[
                // Texture array
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Text Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[
                &uniform_bind_group_layout,
                &texture_bind_group_layout,
            ],
            immediate_size: 0,
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<TextInstance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        // position
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        // glyph_size
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 8,
                            shader_location: 1,
                        },
                        // uv_min
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 16,
                            shader_location: 2,
                        },
                        // uv_max
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 24,
                            shader_location: 3,
                        },
                        // color
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 32,
                            shader_location: 4,
                        },
                        // page_index
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Uint32,
                            offset: 48,
                            shader_location: 5,
                        },
                        // glyph_type
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Uint32,
                            offset: 52,
                            shader_location: 6,
                        },
                        // clip_rect
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 56,
                            shader_location: 7,
                        },
                    ],
                }],
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
                topology: wgpu::PrimitiveTopology::TriangleStrip,
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
            uniform_buffer,
            uniform_bind_group,
            texture_bind_group_layout,
            instance_buffer: None,
            instance_capacity: 0,
            sampler,
        }
    }

    /// Update screen size uniforms
    pub fn update_screen_size(&mut self, context: &RenderContext, size: Size) {
        let uniforms = TextUniforms {
            screen_size: [size.width as f32, size.height as f32],
            _padding: [0.0, 0.0],
        };
        context
            .queue()
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    /// Render text instances
    pub fn render(
        &mut self,
        context: &RenderContext,
        render_pass: &mut wgpu::RenderPass,
        instances: &[TextInstance],
        atlas_texture_view: &wgpu::TextureView,
    ) {
        if instances.is_empty() {
            return;
        }

        let device = context.device();

        // Create or resize instance buffer if needed
        let needed_capacity = instances.len();
        if self.instance_buffer.is_none() || needed_capacity > self.instance_capacity {
            self.instance_capacity = needed_capacity.max(128);
            self.instance_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Text Instance Buffer"),
                size: (self.instance_capacity * std::mem::size_of::<TextInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }

        // Upload instance data
        let instance_buffer = self.instance_buffer.as_ref().unwrap();
        context
            .queue()
            .write_buffer(instance_buffer, 0, bytemuck::cast_slice(instances));

        // Create texture bind group
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Texture Bind Group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(atlas_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        // Render all text in one draw call
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
        render_pass.draw(0..4, 0..instances.len() as u32);  // 4 vertices per quad, N instances
    }
}
