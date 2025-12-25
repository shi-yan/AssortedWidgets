use assorted_widgets::{Application, Widget};
use assorted_widgets::elements::DraggableRect;
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::paint::Color;

fn main() {
    println!("AssortedWidgets - Mouse Capture & Drag Demo");
    println!("============================================");
    println!();
    println!("This demo showcases:");
    println!("  ✓ Mouse capture during drag operations");
    println!("  ✓ Dragging elements within a window");
    println!("  ✓ Mouse events delivered even outside window bounds");
    println!("  ✓ Two windows to test independently");
    println!();
    println!("Instructions:");
    println!("  1. Click and hold on a colored rectangle");
    println!("  2. Drag the mouse (even outside the window!)");
    println!("  3. Release to drop");
    println!("  4. Watch terminal for capture/release events");
    println!();
    println!("Note: Cross-window drag-drop with floating proxy requires");
    println!("      additional infrastructure (future enhancement)");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        Application::launch(|app| {
            println!("WebGPU initialized successfully!");
            println!();

            // ================================================================
            // Create Window 1: Red and Blue rects
            // ================================================================

            app.spawn_window("Drag Demo - Window 1", 600.0, 400.0, |window| {
                println!("Window 1 created!");

                use assorted_widgets::layout::Style;

                // Red draggable rect
                let red_rect = DraggableRect::new(
                    WidgetId::new(1),
                    Rect::new(Point::new(50.0, 50.0), Size::new(200.0, 150.0)),
                    Color::rgb(0.8, 0.2, 0.2),
                    "Red Rect"
                );
                let red_id = red_rect.id();

                // Blue draggable rect
                let blue_rect = DraggableRect::new(
                    WidgetId::new(2),
                    Rect::new(Point::new(300.0, 50.0), Size::new(200.0, 150.0)),
                    Color::rgb(0.2, 0.2, 0.8),
                    "Blue Rect"
                );

                // Add red rect as root with absolute positioning
                window.add_root(
                    Box::new(red_rect),
                    Style {
                        position: taffy::Position::Absolute,
                        inset: taffy::Rect {
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
                )
                .expect("Failed to add red rect as root");

                // Add blue rect as child with absolute positioning
                window.add_child(
                    Box::new(blue_rect),
                    Style {
                        position: taffy::Position::Absolute,
                        inset: taffy::Rect {
                            left: taffy::LengthPercentageAuto::length(300.0),
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
                    red_id,
                )
                .expect("Failed to add blue rect as child");
            });

            // ================================================================
            // Create Window 2: Green and Yellow rects
            // ================================================================

            app.spawn_window("Drag Demo - Window 2", 600.0, 400.0, |window| {
                println!("Window 2 created!");

                use assorted_widgets::layout::Style;

                // Green draggable rect
                let green_rect = DraggableRect::new(
                    WidgetId::new(3),
                    Rect::new(Point::new(50.0, 50.0), Size::new(200.0, 150.0)),
                    Color::rgb(0.2, 0.8, 0.2),
                    "Green Rect"
                );
                let green_id = green_rect.id();

                // Yellow draggable rect
                let yellow_rect = DraggableRect::new(
                    WidgetId::new(4),
                    Rect::new(Point::new(300.0, 50.0), Size::new(200.0, 150.0)),
                    Color::rgb(0.9, 0.9, 0.2),
                    "Yellow Rect"
                );

                // Add green rect as root with absolute positioning
                window.add_root(
                    Box::new(green_rect),
                    Style {
                        position: taffy::Position::Absolute,
                        inset: taffy::Rect {
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
                )
                .expect("Failed to add green rect as root");

                // Add yellow rect as child with absolute positioning
                window.add_child(
                    Box::new(yellow_rect),
                    Style {
                        position: taffy::Position::Absolute,
                        inset: taffy::Rect {
                            left: taffy::LengthPercentageAuto::length(300.0),
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
                    green_id,
                )
                .expect("Failed to add yellow rect as child");
            });

            println!();
            println!("✅ Demo setup complete!");
            println!();
            println!("═══════════════════════════════════════════════════════");
            println!("  TEST INSTRUCTIONS");
            println!("═══════════════════════════════════════════════════════");
            println!();
            println!("Window 1: Red and Blue draggable rectangles");
            println!("Window 2: Green and Yellow draggable rectangles");
            println!();
            println!("Try this:");
            println!("  1. Click and drag a rectangle");
            println!("  2. Drag your mouse OUTSIDE the window");
            println!("  3. The rectangle still follows (mouse is captured!)");
            println!("  4. Release to drop");
            println!();
            println!("Watch the terminal for:");
            println!("  • Mouse capture events");
            println!("  • Drag position updates");
            println!("  • Mouse release events");
            println!("═══════════════════════════════════════════════════════");
            println!();
        });
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
