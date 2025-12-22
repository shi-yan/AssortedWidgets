use assorted_widgets::{Element, GuiEventLoop, WindowOptions};
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

        let mut event_loop = pollster::block_on(async {
            GuiEventLoop::new_with_window(WindowOptions {
                bounds: Rect::new(Point::new(100.0, 100.0), Size::new(1200.0, 900.0)),
                title: "AssortedWidgets - Phase 3.2 Demo".to_string(),
                titlebar: None,
            })
            .await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!");
        println!();

        // ================================================================
        // Create demo element using the Element trait (proper way!)
        // ================================================================
        let demo = TextDemoElement::new(WidgetId::new(1));
        let demo_id = demo.id();

        // Add demo element to element manager
        event_loop.element_manager_mut().add_element(Box::new(demo));

        // Create layout node for the element (LayoutManager needs to be synced)
        event_loop.layout_manager_mut().create_node(demo_id, taffy::Style::default())
            .expect("Failed to create layout node");

        // Set as root of both scene graph and layout
        event_loop.scene_graph_mut().set_root(SceneNode::new(demo_id));
        event_loop.layout_manager_mut().set_root(demo_id)
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
        println!("Press Cmd+Q to quit.");
        println!();

        // Run event loop - demo renders via Element::paint()
        event_loop.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
