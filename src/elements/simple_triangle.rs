//! Simple Triangle Widget - Minimal test for RawSurface architecture
//!
//! This widget renders a single colored triangle to its own framebuffer
//! to test the RawSurface system before implementing complex 3D scenes.

use crate::layout::Style;
use crate::paint::PaintContext;
use crate::raw_surface::{RawSurface, RawSurfaceFramebuffer};
use crate::types::{DeferredCommand, GuiMessage, Rect, Size, WidgetId};
use crate::widget::Widget;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Instant;

pub struct SimpleTriangle {
    id: WidgetId,
    bounds: Rect,
    pipeline: Arc<wgpu::RenderPipeline>,
    pub framebuffer: RefCell<Option<RawSurfaceFramebuffer>>, // Public for Window access

    // Stored for framebuffer management
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    format: wgpu::TextureFormat,
    _sample_count: u32,

    // FPS tracking
    last_frame_time: RefCell<Option<Instant>>,
    frame_times: RefCell<Vec<f64>>, // Circular buffer of recent frame times
}

impl SimpleTriangle {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let id = WidgetId::new(0); // Placeholder, set by Window
        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Simple Triangle Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/simple_triangle.wgsl").into()),
        });

        // Create pipeline (no vertex buffers, no uniforms - all hardcoded in shader)
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Simple Triangle Pipeline"),
            layout: None, // Auto layout
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // No vertex buffers
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // No culling for simplicity
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // No MSAA - must match framebuffer
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
            framebuffer: RefCell::new(None),
            device,
            queue,
            format,
            _sample_count: sample_count,
            last_frame_time: RefCell::new(None),
            frame_times: RefCell::new(Vec::with_capacity(60)), // Store 60 frames
        }
    }

    fn ensure_framebuffer(&self) {
        let size = Size::new(self.bounds.size.width, self.bounds.size.height);

        if size.width > 0.0 && size.height > 0.0 {
            let mut fb_borrow = self.framebuffer.borrow_mut();
            if let Some(ref mut fb) = *fb_borrow {
                fb.resize(&self.device, size, self.format);
            } else {
                *fb_borrow = Some(RawSurfaceFramebuffer::new(
                    &self.device,
                    size,
                    self.format,
                    1, // No MSAA - framebuffer must be non-multisampled for texture binding
                ));
            }
        }
    }

    /// Render triangle to framebuffer
    fn render_to_framebuffer(&self) {
        let fb_borrow = self.framebuffer.borrow();
        let Some(ref framebuffer) = *fb_borrow else {
            return;
        };

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Triangle Framebuffer Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Triangle Framebuffer Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &framebuffer.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            self.paint_raw(&mut render_pass, framebuffer.size);
        }

        self.queue.submit([encoder.finish()]);
    }
}

impl RawSurface for SimpleTriangle {
    fn widget_id(&self) -> WidgetId {
        self.id
    }

    fn paint_raw(&self, render_pass: &mut wgpu::RenderPass, _size: Size) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.draw(0..3, 0..1); // Draw 3 vertices (1 triangle)
    }

    fn on_resize(&mut self, _new_size: Size) {
        // No size-dependent resources for this simple triangle
    }

    fn framebuffer_size(&self) -> Size {
        Size::new(self.bounds.size.width, self.bounds.size.height)
    }

    fn framebuffer_view(&self) -> Option<&wgpu::TextureView> {
        // Can't return a reference into RefCell borrow - Window accesses framebuffer directly
        None
    }
}

impl Widget for SimpleTriangle {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
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

    fn set_dirty(&mut self, _dirty: bool) {}

    fn is_dirty(&self) -> bool {
        true // Always redraw
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
        // Calculate FPS
        let now = Instant::now();
        let mut last_time = self.last_frame_time.borrow_mut();
        let mut frame_times = self.frame_times.borrow_mut();

        if let Some(last) = *last_time {
            let delta = now.duration_since(last).as_secs_f64();
            frame_times.push(delta);

            // Keep only last 60 frames
            if frame_times.len() > 60 {
                frame_times.remove(0);
            }

            // Calculate average FPS
            if !frame_times.is_empty() {
                let avg_delta: f64 = frame_times.iter().sum::<f64>() / frame_times.len() as f64;
                let fps = if avg_delta > 0.0 { 1.0 / avg_delta } else { 0.0 };

                // Emit FPS update signal
                ctx.emit_signal(
                    self.id,
                    crate::types::GuiMessage::Custom {
                        source: self.id,
                        signal_type: "fps_update".to_string(),
                        data: Box::new(fps),
                    },
                );
            }
        }
        *last_time = Some(now);

        // Ensure framebuffer exists and is correct size
        self.ensure_framebuffer();

        // Render triangle to framebuffer
        self.render_to_framebuffer();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_raw_surface(&self) -> Option<&dyn crate::raw_surface::RawSurface> {
        Some(self)
    }
}
