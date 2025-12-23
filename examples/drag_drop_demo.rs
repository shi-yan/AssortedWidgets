use assorted_widgets::{Application, Element, WindowOptions};
use assorted_widgets::elements::DraggableRect;
use assorted_widgets::scene_graph::SceneNode;
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

        let mut app = pollster::block_on(async {
            Application::new().await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!");
        println!();

        // ================================================================
        // Create Window 1: Red and Blue rects
        // ================================================================

        let window1_id = app.create_window(WindowOptions {
            bounds: Rect::new(Point::new(100.0, 100.0), Size::new(600.0, 400.0)),
            title: "Drag Demo - Window 1".to_string(),
            titlebar: None,
        })
        .expect("Failed to create window 1");

        println!("Window 1 created!");

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
        let blue_id = blue_rect.id();

        // Setup Window 1
        {
            let window = app.window_mut(window1_id).expect("Window 1 not found");

            window.element_manager_mut().add_element(Box::new(red_rect));
            window.element_manager_mut().add_element(Box::new(blue_rect));

            window.layout_manager_mut().create_node(red_id, taffy::Style::default())
                .expect("Failed to create layout node");
            window.layout_manager_mut().create_node(blue_id, taffy::Style::default())
                .expect("Failed to create layout node");

            let mut root = SceneNode::new(red_id);
            root.add_child(SceneNode::new(blue_id));

            window.scene_graph_mut().set_root(root);
            window.layout_manager_mut().set_root(red_id)
                .expect("Failed to set layout root");
        }

        // ================================================================
        // Create Window 2: Green and Yellow rects
        // ================================================================

        let window2_id = app.create_window(WindowOptions {
            bounds: Rect::new(Point::new(750.0, 100.0), Size::new(600.0, 400.0)),
            title: "Drag Demo - Window 2".to_string(),
            titlebar: None,
        })
        .expect("Failed to create window 2");

        println!("Window 2 created!");

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
        let yellow_id = yellow_rect.id();

        // Setup Window 2
        {
            let window = app.window_mut(window2_id).expect("Window 2 not found");

            window.element_manager_mut().add_element(Box::new(green_rect));
            window.element_manager_mut().add_element(Box::new(yellow_rect));

            window.layout_manager_mut().create_node(green_id, taffy::Style::default())
                .expect("Failed to create layout node");
            window.layout_manager_mut().create_node(yellow_id, taffy::Style::default())
                .expect("Failed to create layout node");

            let mut root = SceneNode::new(green_id);
            root.add_child(SceneNode::new(yellow_id));

            window.scene_graph_mut().set_root(root);
            window.layout_manager_mut().set_root(green_id)
                .expect("Failed to set layout root");
        }

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

        // Run application event loop
        app.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
