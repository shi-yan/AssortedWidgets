use assorted_widgets::{Application, Element, WindowOptions};
use assorted_widgets::elements::TextDemoElement;
use assorted_widgets::scene_graph::SceneNode;
use assorted_widgets::types::{Point, Rect, Size, WidgetId};

fn main() {
    println!("AssortedWidgets - Phase 3.2 Text Rendering Demo");
    println!("===============================================");
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
            bounds: Rect::new(Point::new(100.0, 100.0), Size::new(1200.0, 900.0)),
            title: "AssortedWidgets - Phase 3.2 Demo".to_string(),
            titlebar: None,
        })
        .expect("Failed to create window");

        println!("Window created!");
        println!();

        // ================================================================
        // Create demo element using the Element trait (proper way!)
        // ================================================================
        let demo = TextDemoElement::new(WidgetId::new(1));
        let demo_id = demo.id();

        // Get mutable reference to the window to set up UI
        let window = app.window_mut(window_id).expect("Window not found");

        // Add demo element to element manager
        window.element_manager_mut().add_element(Box::new(demo));

        // Create layout node for the element (LayoutManager needs to be synced)
        window.layout_manager_mut().create_node(demo_id, taffy::Style::default())
            .expect("Failed to create layout node");

        // Set as root of both scene graph and layout
        window.scene_graph_mut().set_root(SceneNode::new(demo_id));
        window.layout_manager_mut().set_root(demo_id)
            .expect("Failed to set layout root");

        println!("Demo element created using Element trait (clean architecture!)");
        println!();
        println!("Phase 3.2 Features Demonstrated:");
        println!("  ✓ Text shaping with kerning and ligatures");
        println!("  ✓ Bidirectional text (English + Arabic + Hebrew + Chinese)");
        println!("  ✓ Emoji rendering with color glyph support");
        println!("  ✓ Text wrapping (multi-line)");
        println!("  ✓ Font fallback for multi-language text");
        println!("  ✓ Glyph atlas with automatic page management");
        println!("  ✓ TextEngine with dual-mode caching (managed + manual)");
        println!("  ✓ Clean two-tier API (high-level ctx.draw_text())");
        println!();
        println!("Phase 3.3 Architecture:");
        println!("  ✓ Clean Application + Window separation");
        println!("  ✓ Multi-window ready (single event loop)");
        println!("  ✓ Shared GPU resources across windows");
        println!();
        println!("Press Cmd+Q to quit.");
        println!();

        // Run application event loop - demo renders via Element::paint()
        app.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
