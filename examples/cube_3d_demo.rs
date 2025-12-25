///! 3D Cube Demo - Low-Level WebGPU RenderPass Access
///!
///! This demo showcases the Tier 2 rendering system, allowing widgets to directly
///! access the WebGPU RenderPass for custom 3D graphics.
///!
///! Features demonstrated:
///! - Custom 3D rendering with own GPU pipelines and buffers
///! - Direct WebGPU RenderPass access via register_custom_render()
///! - Integration of 3D widgets with 2D UI elements (FPS counter)
///! - Continuous animation with frame timing
///!
///! Unlike frameworks like gpui or iced, AssortedWidgets provides BOTH:
///! 1. High-level primitives (text, rectangles, paths) - Tier 1
///! 2. Low-level WebGPU access (custom pipelines) - Tier 2
///!
///! This makes it suitable for:
///! - 3D applications with embedded UI
///! - Game engines
///! - Scientific visualization
///! - Any application requiring custom GPU rendering

use assorted_widgets::{Application, WindowOptions};
use assorted_widgets::elements::{Container, Cube3D, TextLabel};
use assorted_widgets::types::{Point, Rect, Size};
use assorted_widgets::paint::Color;
use assorted_widgets::text::TextStyle;

fn main() {
    println!("\n═══════════════════════════════════════════════════════");
    println!("  3D Cube Demo - Low-Level WebGPU Access");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("This demo demonstrates AssortedWidgets' unique capability:");
    println!("  ✓ Tier 1: High-level 2D primitives (FPS text)");
    println!("  ✓ Tier 2: Low-level 3D rendering (rotating cube)");
    println!();
    println!("The cube widget:");
    println!("  • Creates its own GPU buffers and pipeline");
    println!("  • Directly accesses WebGPU RenderPass");
    println!("  • Renders alongside standard UI elements");
    println!();
    println!("This is NOT possible in frameworks like gpui or iced!");
    println!("═══════════════════════════════════════════════════════");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        let mut app = pollster::block_on(async {
            Application::new().await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!");
        println!();

        // Create window
        let window_id = app.create_window(WindowOptions {
            bounds: Rect::new(Point::new(100.0, 100.0), Size::new(800.0, 700.0)),
            title: "3D Cube Demo - Low-Level WebGPU Access".to_string(),
            titlebar: None,
            borderless: false,
            transparent: false,
            always_on_top: false,
            utility: false,
        })
        .expect("Failed to create window");

        println!("Window created!");
        println!();

        // Get render context for creating GPU resources
        let render_ctx = app.render_context();
        let device = render_ctx.device();
        let surface_format = render_ctx.surface_format;

        // Create 3D cube widget with custom GPU resources
        println!("Creating 3D cube widget...");
        let sample_count = render_ctx.sample_count;
        let cube = Cube3D::new(
            assorted_widgets::types::WidgetId::new(1),
            device,
            surface_format,
            sample_count,
        );
        println!("  ✓ GPU pipeline created");
        println!("  ✓ Vertex and index buffers allocated");
        println!("  ✓ Uniform buffer initialized");
        println!();

        // Create container for all widgets
        let container = Container::new(
            assorted_widgets::types::WidgetId::new(100),
            taffy::Style::default(),
        );

        // Get mutable reference to the window to set up UI
        let window = app.window_mut(window_id).expect("Window not found");

        // Create layout style for vertical stacking
        let container_style = taffy::Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            align_items: Some(taffy::AlignItems::Center),
            justify_content: Some(taffy::JustifyContent::Center),
            gap: taffy::Size {
                width: taffy::LengthPercentage::length(20.0),
                height: taffy::LengthPercentage::length(20.0),
            },
            padding: taffy::Rect {
                left: taffy::LengthPercentage::length(50.0),
                right: taffy::LengthPercentage::length(50.0),
                top: taffy::LengthPercentage::length(30.0),
                bottom: taffy::LengthPercentage::length(50.0),
            },
            size: taffy::Size {
                width: taffy::Dimension::length(800.0),
                height: taffy::Dimension::length(700.0),
            },
            ..Default::default()
        };

        // Add container as root
        window.add_root(Box::new(container), container_style)
            .expect("Failed to add container");

        let container_id = assorted_widgets::types::WidgetId::new(100);

        // Create title label
        let title_label = TextLabel::new(
            assorted_widgets::types::WidgetId::new(3),
            "3D Cube Demo - Tier 2 Rendering",
        ).with_style(TextStyle::new().size(28.0).color(Color::WHITE));

        // Add title as child of container
        window.add_child(Box::new(title_label), taffy::Style::default(), container_id)
            .expect("Failed to add title");

        // Create instructions label
        let instructions = TextLabel::new(
            assorted_widgets::types::WidgetId::new(4),
            "Watch the cube rotate! This uses direct WebGPU RenderPass access.",
        ).with_style(TextStyle::new().size(16.0).color(Color::rgb(0.7, 0.7, 0.7)));

        // Add instructions as child of container
        window.add_child(Box::new(instructions), taffy::Style::default(), container_id)
            .expect("Failed to add instructions");

        // Add 3D cube as child of container with explicit size
        let cube_style = taffy::Style {
            size: taffy::Size {
                width: taffy::Dimension::length(400.0),
                height: taffy::Dimension::length(400.0),
            },
            ..Default::default()
        };
        window.add_child(Box::new(cube), cube_style, container_id)
            .expect("Failed to add cube");

        // Create FPS counter label
        let fps_label = TextLabel::new(
            assorted_widgets::types::WidgetId::new(2),
            "FPS: 0",
        ).with_style(TextStyle::new().size(24.0).color(Color::rgb(0.0, 1.0, 0.0)));

        // Add FPS counter as child of container
        window.add_child(Box::new(fps_label), taffy::Style::default(), container_id)
            .expect("Failed to add FPS counter");

        println!("✅ Demo setup complete!");
        println!();
        println!("═══════════════════════════════════════════════════════");
        println!("  RENDERING ARCHITECTURE");
        println!("═══════════════════════════════════════════════════════");
        println!();
        println!("Tier 1 (High-Level):");
        println!("  • FPS text rendered via TextPipeline");
        println!("  • Uses shared glyph atlas");
        println!("  • Batched with other UI elements");
        println!();
        println!("Tier 2 (Low-Level):");
        println!("  • Cube rendered via custom pipeline");
        println!("  • Direct RenderPass access");
        println!("  • Own vertex/index buffers");
        println!("  • Custom shaders (cube_3d.wgsl)");
        println!();
        println!("Both tiers render to the SAME RenderPass!");
        println!("═══════════════════════════════════════════════════════");
        println!();

        // Run application event loop
        app.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
