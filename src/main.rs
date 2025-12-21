use assorted_widgets::{GuiEventLoop, WindowOptions};
use assorted_widgets::types::{Point, Rect, Size};

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
        println!("Phase 3.2 Features Demonstrated:");
        println!("  ✓ Text shaping with kerning and ligatures");
        println!("  ✓ Bidirectional text (English + Arabic + Hebrew + Chinese)");
        println!("  ✓ Emoji rendering with color glyph support");
        println!("  ✓ Text truncation with ellipsis");
        println!("  ✓ Text wrapping (multi-line)");
        println!("  ✓ Font fallback for multi-language text");
        println!("  ✓ Glyph atlas with automatic page management");
        println!("  ✓ TextEngine with dual-mode caching (managed + manual)");
        println!();
        println!("The demo is integrated into the event loop.");
        println!("Press Cmd+Q to quit.");
        println!();

        // Run event loop (demo is built-in via render_test_text)
        event_loop.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
