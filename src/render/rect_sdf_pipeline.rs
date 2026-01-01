use crate::paint::{DrawCommand, PrimitiveBatcher};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// GPU representation of a rectangle instance (matches WGSL struct)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RectInstance {
    rect: [f32; 4],           // x, y, width, height
    corner_radius: [f32; 4],  // top_left, top_right, bottom_right, bottom_left
    fill_type: u32,           // 0 = solid, 1 = linear gradient, 2 = radial gradient
    stop_count: u32,          // Number of gradient stops (2-8)
    _padding1: [u32; 2],      // Align to 16 bytes
    fill_color: [f32; 4],     // rgba for solid, unused for gradients
    border_color: [f32; 4],   // rgba
    border_width: f32,
    depth: f32,               // Z-depth for painter's algorithm (0.0 = back, 1.0 = front)
    _padding2: [f32; 2],      // Align to 16 bytes
    gradient_start_end: [f32; 4], // start.xy, end.xy (linear) or center.xy, radius, _
    gradient_stop_0: [f32; 4], // offset, r, g, b
    gradient_stop_1: [f32; 4],
    gradient_stop_2: [f32; 4],
    gradient_stop_3: [f32; 4],
    gradient_stop_4: [f32; 4],
    gradient_stop_5: [f32; 4],
    gradient_stop_6: [f32; 4],
    gradient_stop_7: [f32; 4],
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
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat, sample_count: u32) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Rect SDF Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/rect_sdf.wgsl").into()),
        });

        // Bind group layout for uniforms (group 0)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Rect SDF Bind Group Layout"),
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
                            // fill_type: u32
                            wgpu::VertexAttribute {
                                offset: 32,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Uint32,
                            },
                            // stop_count: u32
                            wgpu::VertexAttribute {
                                offset: 36,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Uint32,
                            },
                            // fill_color: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 48,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // border_color: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 64,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // border_width_and_depth: vec2<f32> (border_width, depth)
                            wgpu::VertexAttribute {
                                offset: 80,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            // gradient_start_end: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 96,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // gradient_stop_0: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 112,
                                shader_location: 8,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // gradient_stop_1: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 128,
                                shader_location: 9,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // gradient_stop_2: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 144,
                                shader_location: 10,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // gradient_stop_3: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 160,
                                shader_location: 11,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // gradient_stop_4: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 176,
                                shader_location: 12,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // gradient_stop_5: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 192,
                                shader_location: 13,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // gradient_stop_6: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 208,
                                shader_location: 14,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // gradient_stop_7: vec4<f32>
                            wgpu::VertexAttribute {
                                offset: 224,
                                shader_location: 15,
                                format: wgpu::VertexFormat::Float32x4,
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
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

    /// Render batched rectangles using a pre-allocated instance buffer
    ///
    /// This method converts draw commands to instances and uploads them to the provided buffer,
    /// then executes the draw call. The caller is responsible for managing the buffer lifecycle.
    ///
    /// Returns the number of instances rendered.
    pub fn render(
        &self,
        queue: &Arc<wgpu::Queue>,
        render_pass: &mut wgpu::RenderPass,
        uniform_bind_group: &wgpu::BindGroup,
        clip_bind_group: &wgpu::BindGroup,
        instance_buffer: &wgpu::Buffer,
        batcher: &PrimitiveBatcher,
        layered_bounds_tree: &mut crate::paint::LayeredBoundsTree,
    ) -> usize {
        if batcher.is_empty() {
            return 0;
        }

        // Convert draw commands to instances
        let instances: Vec<RectInstance> = batcher
            .commands()
            .iter()
            .enumerate()
            .filter_map(|(_idx, cmd)| match cmd {
                DrawCommand::Rect { rect, style, z_index } => {
                    use crate::paint::Brush;

                    let border_color = style.border.as_ref()
                        .map(|b| b.color)
                        .unwrap_or_else(|| crate::paint::Color::rgb(0.0, 0.0, 0.0));
                    let border_width = style.border.as_ref()
                        .map(|b| b.width)
                        .unwrap_or(0.0);

                    // Phase 2: BoundsTree-based depth assignment
                    // Insert rect into layered tree → automatic overlap detection
                    // Non-overlapping rects get same depth → batching!
                    let depth = layered_bounds_tree.insert(*rect, *z_index);

                    let mut instance = RectInstance {
                        rect: [rect.origin.x as f32, rect.origin.y as f32, rect.size.width as f32, rect.size.height as f32],
                        corner_radius: style.corner_radius.to_array(),
                        fill_type: 0,         // Will be set based on brush type
                        stop_count: 0,        // Will be set for gradients
                        _padding1: [0; 2],
                        fill_color: [0.0; 4],
                        border_color: border_color.to_array(),
                        border_width,
                        depth,                // Z-depth for proper layering
                        _padding2: [0.0; 2],
                        gradient_start_end: [0.0; 4],
                        gradient_stop_0: [0.0; 4],
                        gradient_stop_1: [0.0; 4],
                        gradient_stop_2: [0.0; 4],
                        gradient_stop_3: [0.0; 4],
                        gradient_stop_4: [0.0; 4],
                        gradient_stop_5: [0.0; 4],
                        gradient_stop_6: [0.0; 4],
                        gradient_stop_7: [0.0; 4],
                    };

                    // Encode brush data
                    match &style.fill {
                        Brush::Solid(color) => {
                            instance.fill_type = 0;  // Solid color
                            instance.fill_color = color.to_array();
                        }
                        Brush::LinearGradient(gradient) => {
                            instance.fill_type = 1;  // Linear gradient
                            instance.stop_count = gradient.stops.len() as u32;

                            // Encode start and end points (normalized 0-1 coordinates)
                            instance.gradient_start_end = [
                                gradient.start.x as f32,
                                gradient.start.y as f32,
                                gradient.end.x as f32,
                                gradient.end.y as f32,
                            ];

                            // Encode color stops (offset, r, g, b)
                            let stops = [
                                &mut instance.gradient_stop_0,
                                &mut instance.gradient_stop_1,
                                &mut instance.gradient_stop_2,
                                &mut instance.gradient_stop_3,
                                &mut instance.gradient_stop_4,
                                &mut instance.gradient_stop_5,
                                &mut instance.gradient_stop_6,
                                &mut instance.gradient_stop_7,
                            ];

                            for (i, stop) in gradient.stops.iter().enumerate() {
                                if i < 8 {
                                    let color_array = stop.color.to_array();
                                    stops[i][0] = stop.offset;
                                    stops[i][1] = color_array[0];
                                    stops[i][2] = color_array[1];
                                    stops[i][3] = color_array[2];
                                }
                            }
                        }
                        Brush::RadialGradient(gradient) => {
                            instance.fill_type = 2;  // Radial gradient
                            instance.stop_count = gradient.stops.len() as u32;

                            // Encode center and radius (normalized 0-1 coordinates)
                            instance.gradient_start_end = [
                                gradient.center.x as f32,
                                gradient.center.y as f32,
                                gradient.radius,
                                0.0,
                            ];

                            // Encode color stops (offset, r, g, b)
                            let stops = [
                                &mut instance.gradient_stop_0,
                                &mut instance.gradient_stop_1,
                                &mut instance.gradient_stop_2,
                                &mut instance.gradient_stop_3,
                                &mut instance.gradient_stop_4,
                                &mut instance.gradient_stop_5,
                                &mut instance.gradient_stop_6,
                                &mut instance.gradient_stop_7,
                            ];

                            for (i, stop) in gradient.stops.iter().enumerate() {
                                if i < 8 {
                                    let color_array = stop.color.to_array();
                                    stops[i][0] = stop.offset;
                                    stops[i][1] = color_array[0];
                                    stops[i][2] = color_array[1];
                                    stops[i][3] = color_array[2];
                                }
                            }
                        }
                    }

                    Some(instance)
                }
                // Other commands are not rendered by this pipeline
                DrawCommand::Line { .. }
                | DrawCommand::Path { .. }
                | DrawCommand::Icon { .. }
                | DrawCommand::Image { .. }
                | DrawCommand::PushClip { .. }
                | DrawCommand::PopClip => None,
            })
            .collect();

        if instances.is_empty() {
            return 0;
        }

        //println!(
        //    "[RectSdfPipeline::render] Rendering {:?} rect instances",
        //    instances
        //);

        // Upload instance data to the pre-allocated buffer
        queue.write_buffer(
            instance_buffer,
            0,
            bytemuck::cast_slice(&instances),
        );

        // Render
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, uniform_bind_group, &[]);
        render_pass.set_bind_group(1, clip_bind_group, &[]);
        render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
        render_pass.draw(0..6, 0..instances.len() as u32); // 6 vertices per quad (2 triangles)

        instances.len()
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

impl crate::render::Pipeline for RectSdfPipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    fn label(&self) -> &'static str {
        "Rect SDF Pipeline"
    }
}
