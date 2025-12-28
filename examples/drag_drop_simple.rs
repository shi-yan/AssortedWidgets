use assorted_widgets::Application;
use assorted_widgets::elements::DraggableRect;
use assorted_widgets::layout::Style;
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::paint::Color;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  DRAG-DROP TEST - Simplified");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("Testing:");
    println!("  1. Mouse drag detection");
    println!("  2. Proxy window creation (floating, semi-transparent)");
    println!("  3. Global mouse position tracking");
    println!("  4. Target window detection");
    println!("  5. Widget transfer between windows");
    println!();
    println!("═══════════════════════════════════════════════════════");
    println!();

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            // ================================================================
            // Window 1: Red rect at (50, 50)
            // ================================================================

            app.spawn_window("Window 1 - Drag Source", 600.0, 400.0, |window| {
                println!("[SETUP] Window 1 created at (100, 100) size 600x400");

                // Red rect at absolute position (50, 50)
                let red_rect = DraggableRect::new(
                    WidgetId::new(1),
                    Rect::new(Point::new(50.0, 50.0), Size::new(200.0, 150.0)),
                    Color::rgb(0.8, 0.2, 0.2),
                    "Red"
                );

                // Use the clean Window API - add_to_root handles all three internal systems
                window.add_to_root(
                    Box::new(red_rect),
                    Style {
                        margin: taffy::Rect {
                            left: taffy::LengthPercentageAuto::length(50.0),
                            top: taffy::LengthPercentageAuto::length(50.0),
                            right: taffy::LengthPercentageAuto::auto(),
                            bottom: taffy::LengthPercentageAuto::auto(),
                        },
                        size: taffy::Size {
                            width: taffy::Dimension::length(200.0),
                            height: taffy::Dimension::length(150.0),
                        },
                        ..Default::default()
                    },
                ).expect("Failed to add red rect");

                println!("[SETUP] Red rect added at (50, 50) size 200x150");
            });

            // ================================================================
            // Window 2: Target window at (750, 100)
            // ================================================================

            app.spawn_window("Window 2 - Drop Target", 600.0, 400.0, |_window| {
                println!("[SETUP] Window 2 created at (750, 100) size 600x400");
                println!();
                println!("═══════════════════════════════════════════════════════");
                println!("  Ready! Try dragging the red rect to Window 2");
                println!("═══════════════════════════════════════════════════════");
                println!();
            });
        });
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
    }
}
