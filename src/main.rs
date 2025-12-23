use assorted_widgets::{Application, Element, WindowOptions};
use assorted_widgets::elements::ClickableRect;
use assorted_widgets::scene_graph::SceneNode;
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::paint::Color;

fn main() {
    println!("AssortedWidgets - Phase 2: Event Handling & Hit Testing Demo");
    println!("=============================================================");
    println!();
    println!("This demo tests:");
    println!("  ✓ Z-order based hit testing");
    println!("  ✓ Event dispatch to interactive elements");
    println!("  ✓ Mouse event logging");
    println!("  ✓ Overlapping elements (later elements on top)");
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
            bounds: Rect::new(Point::new(100.0, 100.0), Size::new(800.0, 600.0)),
            title: "Phase 2: Hit Testing Demo".to_string(),
            titlebar: None,
        })
        .expect("Failed to create window");

        println!("Window created!");
        println!();

        // ================================================================
        // Create overlapping clickable rectangles to test z-order
        // ================================================================

        // Rectangle 1: Large red background (bottom layer, z-order = 0)
        let rect1 = ClickableRect::new(
            WidgetId::new(1),
            Rect::new(Point::new(50.0, 50.0), Size::new(300.0, 300.0)),
            Color::rgb(0.8, 0.2, 0.2), // Red
            "Red Background"
        );
        let rect1_id = rect1.id();

        // Rectangle 2: Medium blue (middle layer, z-order = 1, overlaps rect1)
        let rect2 = ClickableRect::new(
            WidgetId::new(2),
            Rect::new(Point::new(150.0, 150.0), Size::new(300.0, 300.0)),
            Color::rgb(0.2, 0.2, 0.8), // Blue
            "Blue Middle"
        );
        let rect2_id = rect2.id();

        // Rectangle 3: Small green (top layer, z-order = 2, overlaps both)
        let rect3 = ClickableRect::new(
            WidgetId::new(3),
            Rect::new(Point::new(250.0, 250.0), Size::new(200.0, 200.0)),
            Color::rgb(0.2, 0.8, 0.2), // Green
            "Green Top"
        );
        let rect3_id = rect3.id();

        // Rectangle 4: Non-overlapping yellow (z-order = 3, but separate)
        let rect4 = ClickableRect::new(
            WidgetId::new(4),
            Rect::new(Point::new(500.0, 100.0), Size::new(200.0, 200.0)),
            Color::rgb(0.9, 0.9, 0.2), // Yellow
            "Yellow Separate"
        );
        let rect4_id = rect4.id();

        // Get mutable reference to the window to set up UI
        let window = app.window_mut(window_id).expect("Window not found");

        // Add all rectangles to element manager (in order - this determines z-order)
        window.element_manager_mut().add_element(Box::new(rect1));
        window.element_manager_mut().add_element(Box::new(rect2));
        window.element_manager_mut().add_element(Box::new(rect3));
        window.element_manager_mut().add_element(Box::new(rect4));

        // Create layout nodes for all elements
        window.layout_manager_mut().create_node(rect1_id, taffy::Style::default())
            .expect("Failed to create layout node");
        window.layout_manager_mut().create_node(rect2_id, taffy::Style::default())
            .expect("Failed to create layout node");
        window.layout_manager_mut().create_node(rect3_id, taffy::Style::default())
            .expect("Failed to create layout node");
        window.layout_manager_mut().create_node(rect4_id, taffy::Style::default())
            .expect("Failed to create layout node");

        // Create scene graph: rect1 as root, others as children
        // This creates a flat hierarchy for this demo
        let mut root = SceneNode::new(rect1_id);
        root.add_child(SceneNode::new(rect2_id));
        root.add_child(SceneNode::new(rect3_id));
        root.add_child(SceneNode::new(rect4_id));

        window.scene_graph_mut().set_root(root);
        window.layout_manager_mut().set_root(rect1_id)
            .expect("Failed to set layout root");

        println!("✅ Demo setup complete!");
        println!();
        println!("═══════════════════════════════════════════════════════");
        println!("  TEST INSTRUCTIONS");
        println!("═══════════════════════════════════════════════════════");
        println!();
        println!("1. Red Background (50, 50) - 300x300 - Bottom Layer");
        println!("2. Blue Middle (150, 150) - 300x300 - Middle Layer (overlaps red)");
        println!("3. Green Top (250, 250) - 200x200 - Top Layer (overlaps both)");
        println!("4. Yellow Separate (500, 100) - 200x200 - Separate (no overlap)");
        println!();
        println!("Expected behavior:");
        println!("  • Click in overlap area (250-350, 250-350):");
        println!("    → Should hit GREEN (highest z-order)");
        println!("  • Click in blue-only area (150-250, 150-450):");
        println!("    → Should hit BLUE");
        println!("  • Click in red-only area (50-150, 50-350):");
        println!("    → Should hit RED");
        println!("  • Click on yellow:");
        println!("    → Should hit YELLOW");
        println!();
        println!("Watch the terminal for mouse event logs!");
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
