//! Path rendering pipeline using Lyon tessellation
//!
//! Converts vector paths (lines, bezier curves) into triangles for GPU rendering.

use crate::paint::{Color, DrawCommand, LineCap, LineJoin, PrimitiveBatcher, Stroke};
use lyon::tessellation::{BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator, VertexBuffers};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Vertex for path rendering (position + color)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PathVertex {
    position: [f32; 2],
    color: [f32; 4],
}

/// Uniform buffer for screen transformation
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    screen_size: [f32; 2],
    _padding: [f32; 2],
}

/// Pipeline for rendering tessellated paths
pub struct PathPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl PathPipeline {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat, sample_count: u32) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Path Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/path.wgsl").into()),
        });

        // Bind group layout for uniforms
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Path Bind Group Layout"),
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Path Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Path Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<PathVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        // position: vec2<f32>
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        // color: vec4<f32>
                        wgpu::VertexAttribute {
                            offset: 8,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
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
        }
    }

    /// Render paths from batcher
    pub fn render(
        &self,
        device: &Arc<wgpu::Device>,
        render_pass: &mut wgpu::RenderPass,
        uniform_bind_group: &wgpu::BindGroup,
        batcher: &PrimitiveBatcher,
    ) {
        // Collect lines and paths
        let mut geometry = VertexBuffers::new();

        for cmd in batcher.commands() {
            match cmd {
                DrawCommand::Line { p1, p2, stroke, .. } => {
                    self.tessellate_line(&mut geometry, *p1, *p2, stroke);
                }
                DrawCommand::Path { path, fill, stroke, .. } => {
                    if let Some(fill_color) = fill {
                        self.tessellate_fill(&mut geometry, path, *fill_color);
                    }
                    if let Some(stroke_style) = stroke {
                        self.tessellate_stroke(&mut geometry, path, stroke_style);
                    }
                }
                _ => {}
            }
        }

        if geometry.vertices.is_empty() {
            return;
        }

        // Create vertex and index buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Path Vertex Buffer"),
            contents: bytemuck::cast_slice(&geometry.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Path Index Buffer"),
            contents: bytemuck::cast_slice(&geometry.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Render
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..geometry.indices.len() as u32, 0, 0..1);
    }

    /// Tessellate a line segment
    fn tessellate_line(
        &self,
        geometry: &mut VertexBuffers<PathVertex, u16>,
        p1: crate::types::Point,
        p2: crate::types::Point,
        stroke: &Stroke,
    ) {
        use lyon::geom::point;
        use lyon::path::Path;

        let mut path_builder = Path::builder();
        path_builder.begin(point(p1.x as f32, p1.y as f32));
        path_builder.line_to(point(p2.x as f32, p2.y as f32));
        path_builder.end(false);
        let path = path_builder.build();

        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::default()
            .with_line_width(stroke.width)
            .with_line_cap(stroke.cap.to_lyon())
            .with_line_join(stroke.join.to_lyon());

        let color = stroke.color.to_array();

        tessellator
            .tessellate_path(
                &path,
                &options,
                &mut BuffersBuilder::new(geometry, |vertex: lyon::tessellation::StrokeVertex| {
                    PathVertex {
                        position: vertex.position().to_array(),
                        color,
                    }
                }),
            )
            .unwrap();
    }

    /// Tessellate a filled path
    fn tessellate_fill(
        &self,
        geometry: &mut VertexBuffers<PathVertex, u16>,
        path: &crate::paint::Path,
        fill_color: Color,
    ) {
        let lyon_path = path.to_lyon_path();

        let mut tessellator = FillTessellator::new();
        let options = FillOptions::default();

        let color = fill_color.to_array();

        tessellator
            .tessellate_path(
                &lyon_path,
                &options,
                &mut BuffersBuilder::new(geometry, |vertex: lyon::tessellation::FillVertex| {
                    PathVertex {
                        position: vertex.position().to_array(),
                        color,
                    }
                }),
            )
            .unwrap();
    }

    /// Tessellate a stroked path
    fn tessellate_stroke(
        &self,
        geometry: &mut VertexBuffers<PathVertex, u16>,
        path: &crate::paint::Path,
        stroke: &Stroke,
    ) {
        let lyon_path = path.to_lyon_path();

        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::default()
            .with_line_width(stroke.width)
            .with_line_cap(stroke.cap.to_lyon())
            .with_line_join(stroke.join.to_lyon());

        let color = stroke.color.to_array();

        tessellator
            .tessellate_path(
                &lyon_path,
                &options,
                &mut BuffersBuilder::new(geometry, |vertex: lyon::tessellation::StrokeVertex| {
                    PathVertex {
                        position: vertex.position().to_array(),
                        color,
                    }
                }),
            )
            .unwrap();
    }

    /// Create uniform buffer for screen size
    pub fn create_uniform_buffer(&self, device: &wgpu::Device, width: u32, height: u32) -> wgpu::Buffer {
        let uniforms = Uniforms {
            screen_size: [width as f32, height as f32],
            _padding: [0.0; 2],
        };

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Path Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    /// Create bind group for uniforms
    pub fn create_bind_group(&self, device: &wgpu::Device, uniform_buffer: &wgpu::Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Path Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        })
    }

    /// Update uniform buffer with new screen size
    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniform_buffer: &wgpu::Buffer, width: u32, height: u32) {
        let uniforms = Uniforms {
            screen_size: [width as f32, height as f32],
            _padding: [0.0; 2],
        };

        queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
}
