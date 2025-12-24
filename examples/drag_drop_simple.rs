use assorted_widgets::{Application, Element, WindowOptions};
use assorted_widgets::elements::DraggableRect;
use assorted_widgets::scene_graph::SceneNode;
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
        let mut app = pollster::block_on(async {
            Application::new().await
        })
        .expect("Failed to initialize rendering");

        // ================================================================
        // Window 1: Red rect at (50, 50)
        // ================================================================

        let window1_id = app.create_window(WindowOptions {
            bounds: Rect::new(Point::new(100.0, 100.0), Size::new(600.0, 400.0)),
            title: "Window 1 - Drag Source".to_string(),
            titlebar: None,
            borderless: false,
            transparent: false,
            always_on_top: false,
            utility: false,
        })
        .expect("Failed to create window 1");

        println!("[SETUP] Window 1 created at (100, 100) size 600x400");

        // Red rect at absolute position (50, 50) - NO LAYOUT!
        let mut red_rect = DraggableRect::new(
            WidgetId::new(1),
            Rect::new(Point::new(50.0, 50.0), Size::new(200.0, 150.0)),
            Color::rgb(0.8, 0.2, 0.2),
            "Red"
        );
        let red_id = red_rect.id();

        // Just set bounds directly - no layout needed!
        red_rect.set_bounds(Rect::new(Point::new(50.0, 50.0), Size::new(200.0, 150.0)));

        {
            let window = app.window_mut(window1_id).expect("Window 1 not found");
            window.element_manager_mut().add_element(Box::new(red_rect));

            // Create layout node with margin positioning and explicit size
            window.layout_manager_mut()
                .create_node(red_id, taffy::Style {
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
                })
                .expect("Failed to create layout node");

            // Simple scene graph - just one rect
            window.scene_graph_mut().set_root(SceneNode::new(red_id));

            // Set layout root
            window.layout_manager_mut()
                .set_root(red_id)
                .expect("Failed to set layout root");

            println!("[SETUP] Red rect added at (50, 50) size 200x150");
        }

        // ================================================================
        // Window 2: Target window at (750, 100)
        // ================================================================

        let window2_id = app.create_window(WindowOptions {
            bounds: Rect::new(Point::new(750.0, 100.0), Size::new(600.0, 400.0)),
            title: "Window 2 - Drop Target".to_string(),
            titlebar: None,
            borderless: false,
            transparent: false,
            always_on_top: false,
            utility: false,
        })
        .expect("Failed to create window 2");

        println!("[SETUP] Window 2 created at (750, 100) size 600x400");
        println!();
        println!("═══════════════════════════════════════════════════════");
        println!("  Ready! Try dragging the red rect to Window 2");
        println!("═══════════════════════════════════════════════════════");
        println!();

        // Run application event loop
        app.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
    }
}
