///! Simple Triangle Demo - Test for RawSurface Architecture
///!
///! This is a minimal test to verify the new RawSurface architecture works.
///! Shows a single colored triangle rendered to a framebuffer texture.
///!
///! Goal: See SOMETHING render before tackling complex 3D scenes.

use assorted_widgets::{Application, WindowOptions};
use assorted_widgets::elements::{Container, SimpleTriangle, TextLabel};
use assorted_widgets::types::{Point, Rect, Size};
use assorted_widgets::paint::Color;
use assorted_widgets::text::TextStyle;

fn main() {
    println!("\n══════════════════════════════════════");
    println!("  Simple Triangle Demo");
    println!("══════════════════════════════════════");
    println!();
    println!("Testing RawSurface architecture:");
    println!("  • Triangle renders to framebuffer");
    println!("  • Texture composited with UI");
    println!("  • Proper z-ordering");
    println!("══════════════════════════════════════");
    println!();

    #[cfg(target_os = "macos")]
    {
        let mut app = pollster::block_on(async {
            Application::new().await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!\n");

        // Create window
        let window_id = app.create_window(WindowOptions {
            bounds: Rect::new(Point::new(100.0, 100.0), Size::new(600.0, 600.0)),
            title: "Triangle Demo - RawSurface Test".to_string(),
            titlebar: None,
            borderless: false,
            transparent: false,
            always_on_top: false,
            utility: false,
        })
        .expect("Failed to create window");

        // Get render context for creating GPU resources
        let render_ctx = app.render_context();
        let device = render_ctx.device.clone();  // Clone the Arc, not the Device
        let queue = render_ctx.queue.clone();    // Clone the Arc, not the Queue
        let surface_format = render_ctx.surface_format;
        let sample_count = render_ctx.sample_count;

        // Create simple triangle widget
        println!("Creating triangle widget...");
        let triangle = SimpleTriangle::new(
            assorted_widgets::types::WidgetId::new(1),
            device,
            queue,
            surface_format,
            sample_count,
        );
        println!("  ✓ Pipeline created\n");

        // Get mutable reference to window
        let window = app.window_mut(window_id).expect("Window not found");

        // Create container
        let container = Container::new(
            assorted_widgets::types::WidgetId::new(100),
            taffy::Style::default(),
        );

        let container_style = taffy::Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            align_items: Some(taffy::AlignItems::Center),
            justify_content: Some(taffy::JustifyContent::Center),
            size: taffy::Size {
                width: taffy::Dimension::length(600.0),
                height: taffy::Dimension::length(600.0),
            },
            ..Default::default()
        };

        window.add_root(Box::new(container), container_style)
            .expect("Failed to add container");

        let container_id = assorted_widgets::types::WidgetId::new(100);

        // Add title
        let title = TextLabel::new(
            assorted_widgets::types::WidgetId::new(2),
            "Simple Triangle Test",
        ).with_style(TextStyle::new().size(24.0).color(Color::WHITE));

        window.add_child(Box::new(title), taffy::Style::default(), container_id)
            .expect("Failed to add title");

        // Add FPS counter (will be updated via signal/slot system)
        let fps_label = TextLabel::new(
            assorted_widgets::types::WidgetId::new(3),
            "FPS: 0",
        ).with_style(TextStyle::new().size(20.0).color(Color::rgb(0.0, 1.0, 0.0)));

        window.add_child(Box::new(fps_label), taffy::Style::default(), container_id)
            .expect("Failed to add FPS label");

        // Add triangle with explicit size
        let triangle_style = taffy::Style {
            size: taffy::Size {
                width: taffy::Dimension::length(400.0),
                height: taffy::Dimension::length(400.0),
            },
            ..Default::default()
        };

        window.add_child(Box::new(triangle), triangle_style, container_id)
            .expect("Failed to add triangle");

        // Connect triangle's FPS signal to FPS label (testing signal/slot system)
        let triangle_id = assorted_widgets::types::WidgetId::new(1);
        let fps_label_id = assorted_widgets::types::WidgetId::new(3);
        window.connect(triangle_id, "fps_update".to_string(), fps_label_id);

        println!("✅ Demo setup complete!\n");
        println!("══════════════════════════════════════");
        println!("If you see a colored triangle, the");
        println!("RawSurface architecture is working!");
        println!("Signal/slot system: Triangle → FPS Label");
        println!("══════════════════════════════════════\n");

        // Run application
        app.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
    }
}
