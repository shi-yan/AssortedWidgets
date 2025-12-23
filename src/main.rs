use assorted_widgets::{Application, Element, WindowOptions};
use assorted_widgets::elements::{TextDemoElement, AnimatedTextLabel};
use assorted_widgets::scene_graph::SceneNode;
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::paint::Color;

fn main() {
    println!("AssortedWidgets - Phase 3.3 Complete");
    println!("====================================");
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
            title: "AssortedWidgets - Phase 3.3 Demo".to_string(),
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
        println!("Phase 3 Text Rendering - COMPLETE ✅");
        println!();
        println!("Phase 3.2 Features:");
        println!("  ✓ Text shaping with kerning and ligatures");
        println!("  ✓ Bidirectional text (English + Arabic + Hebrew + Chinese)");
        println!("  ✓ Emoji rendering with color glyph support");
        println!("  ✓ Text wrapping (multi-line)");
        println!("  ✓ Font fallback for multi-language text");
        println!("  ✓ Glyph atlas with automatic page management");
        println!("  ✓ TextEngine with dual-mode caching (managed + manual)");
        println!("  ✓ Clean two-tier API (high-level + low-level)");
        println!();
        println!("Phase 3.3 Features (NEW):");
        println!("  ✓ Text alignment (left, center, right)");
        println!("  ✓ Ellipsis truncation with binary search");
        println!("  ✓ TextLabel element with measure() integration");
        println!("  ✓ Performance benchmarking and cache stats");
        println!("  ✓ Clean architecture (Element trait demo)");
        println!("  ✓ Multi-window ready with shared GPU resources");
        println!();
        println!("Press Cmd+Q to quit.");
        println!();
 
        // ================================================================
        // Create second window for animated text truncation demo
        // ================================================================
        let animated_window_id = app.create_window(WindowOptions {
            bounds: Rect::new(Point::new(400.0, 200.0), Size::new(800.0, 400.0)),
            title: "Animated Text Truncation Demo".to_string(),
            titlebar: None,
        })
        .expect("Failed to create animated demo window");

        // Create animated text label
        let animated_label = AnimatedTextLabel::new(
            WidgetId::new(100),
            "This is a long text that will demonstrate dynamic truncation with ellipsis (...) as the container width changes. Watch how the text truncates smoothly!",
            100.0,   // min_width: text heavily truncated
            600.0,   // max_width: text fully visible
        )
        .with_bg_color(Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 });

        let animated_id = animated_label.id();

        // Add to second window
        let animated_window = app.window_mut(animated_window_id).expect("Window not found");
        animated_window.element_manager_mut().add_element(Box::new(animated_label));

        // Create layout node with custom measurement (for animated width)
        animated_window.layout_manager_mut()
            .create_measurable_node(animated_id, taffy::Style::default())
            .expect("Failed to create layout node");

        // Set as root
        animated_window.scene_graph_mut().set_root(SceneNode::new(animated_id));
        animated_window.layout_manager_mut().set_root(animated_id)
            .expect("Failed to set layout root");

        println!("Animated text truncation demo window created!");
        println!("  → Watch the text truncate with '...' as width oscillates");
        println!("  → min width: 100px, max width: 600px");
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
