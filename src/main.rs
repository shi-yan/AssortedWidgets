use assorted_widgets::{GuiEventLoop, WindowOptions};

fn main() {
    println!("AssortedWidgets - WebGPU Triangle Demo");
    println!("=======================================");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        let mut event_loop = pollster::block_on(async {
            GuiEventLoop::new_with_window(WindowOptions {
                title: "AssortedWidgets - WebGPU Triangle".to_string(),
                ..Default::default()
            })
            .await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!");
        println!();

        // Create triangle pipeline
        println!("Creating triangle render pipeline...");
        let pipeline = create_triangle_pipeline(&event_loop);
        println!("Pipeline created!");
        println!();

        println!("Starting continuous rendering...");
        println!("You should see a colorful triangle (red, green, blue vertices)");
        println!("The triangle will redraw continuously at 60fps");
        println!("Press Cmd+Q to quit.");
        println!();

        // Set up render function that captures the pipeline
        event_loop.set_render_fn(move |renderer, ctx| {
            render_triangle_frame(renderer, ctx, &pipeline);
        });

        // Run event loop (never returns)
        event_loop.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo only runs on macOS currently.");
    }
}

#[cfg(target_os = "macos")]
fn create_triangle_pipeline(event_loop: &GuiEventLoop) -> wgpu::RenderPipeline {
    let ctx = event_loop.render_context();

    // Load shader
    let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Triangle Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/triangle.wgsl").into()),
    });

    // Create pipeline layout
    let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Triangle Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    // Get surface format from renderer
    let format = event_loop.renderer().unwrap().format;

    // Create render pipeline
    ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Triangle Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

#[cfg(target_os = "macos")]
fn render_triangle_frame(
    renderer: &assorted_widgets::render::WindowRenderer,
    ctx: &assorted_widgets::render::RenderContext,
    pipeline: &wgpu::RenderPipeline,
) {
    // Get surface texture
    let surface_texture = match renderer.get_current_texture() {
        Ok(texture) => texture,
        Err(e) => {
            eprintln!("Failed to get surface texture: {:?}", e);
            return;
        }
    };

    // Create texture view with sRGB format
    let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(renderer.format.add_srgb_suffix()),
        ..Default::default()
    });

    // Create command encoder
    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Triangle Encoder"),
    });

    // Render pass
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Triangle Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(pipeline);
        render_pass.draw(0..3, 0..1); // Draw 3 vertices (the triangle!)
    }

    // Submit commands
    ctx.queue.submit([encoder.finish()]);

    // Present the frame
    surface_texture.present();
}
