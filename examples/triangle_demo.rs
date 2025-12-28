///! Simple Triangle Demo - Test for RawSurface Architecture
///!
///! This is a minimal test to verify the new RawSurface architecture works.
///! Shows a single colored triangle rendered to a framebuffer texture.
///!
///! Goal: See SOMETHING render before tackling complex 3D scenes.

use assorted_widgets::Application;
use assorted_widgets::elements::{Container, SimpleTriangle, TextLabel};
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
        // ✨ New ergonomic API
        Application::launch(|app| {
            println!("WebGPU initialized successfully!\n");

            // Get render context for creating GPU resources
            let render_ctx = app.render_context();
            let device = render_ctx.device.clone();  // Clone the Arc, not the Device
            let queue = render_ctx.queue.clone();    // Clone the Arc, not the Queue
            let surface_format = render_ctx.surface_format;
            let sample_count = render_ctx.sample_count;

            // Create simple triangle widget
            println!("Creating triangle widget...");
            let triangle = SimpleTriangle::new(
                device,
                queue,
                surface_format,
                sample_count,
            );
            println!("  ✓ Pipeline created\n");

            app.spawn_window("Triangle Demo - RawSurface Test", 600.0, 600.0, |window| {
                // Create container with layout style
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

                // ✅ Capture the container ID returned by add_to_root()
                let container_id = window.add_to_root(
                    Box::new(Container::new(taffy::Style::default())),
                    container_style
                ).expect("Failed to add container");

                // Add title
                let title = TextLabel::new("Simple Triangle Test")
                    .with_style(TextStyle::new().size(24.0).color(Color::WHITE));

                window.add_child(Box::new(title), taffy::Style::default(), container_id)
                    .expect("Failed to add title");

                // Add FPS counter - capture the ID for signal connection
                let fps_label = TextLabel::new("FPS: 0")
                    .with_style(TextStyle::new().size(20.0).color(Color::rgb(0.0, 1.0, 0.0)));

                let fps_label_id = window.add_child(Box::new(fps_label), taffy::Style::default(), container_id)
                    .expect("Failed to add FPS label");

                // Add triangle with explicit size - capture the ID for signal connection
                let triangle_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(400.0),
                        height: taffy::Dimension::length(400.0),
                    },
                    ..Default::default()
                };

                let triangle_id = window.add_child(Box::new(triangle), triangle_style, container_id)
                    .expect("Failed to add triangle");

                // Connect triangle's FPS signal to FPS label (testing signal/slot system)
                window.connect(triangle_id, "fps_update".to_string(), fps_label_id);

                println!("✅ Demo setup complete!\n");
                println!("══════════════════════════════════════");
                println!("If you see a colored triangle, the");
                println!("RawSurface architecture is working!");
                println!("Signal/slot system: Triangle → FPS Label");
                println!("══════════════════════════════════════\n");
            });
        });
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
    }
}
