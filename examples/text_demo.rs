use assorted_widgets::Application;
use assorted_widgets::elements::{TextDemoElement, AnimatedTextLabel};
use assorted_widgets::types::WidgetId;
use assorted_widgets::paint::Color;
use assorted_widgets::layout::Style;

fn main() {
    println!("AssortedWidgets - Phase 3.3 Complete");
    println!("====================================");
    println!();

    #[cfg(target_os = "macos")]
    {
        // ✨ New ergonomic API
        Application::launch(|app| {
            println!("WebGPU initialized successfully!");
            println!();

            // ================================================================
            // Create main demo window
            // ================================================================
            app.spawn_window("AssortedWidgets - Phase 3.3 Demo", 1200.0, 900.0, |window| {
                println!("Window created!");
                println!();

                // Create demo widget using clean Window API
                let demo = TextDemoElement::new(WidgetId::new(1));

                // Add demo widget using clean API - automatically coordinates all three systems
                window.add_to_root(Box::new(demo), Style::default())
                    .expect("Failed to add widget");

                println!("Demo widget created using Widget trait (clean architecture!)");
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
            });

            // ================================================================
            // Create second window for animated text truncation demo
            // ================================================================
            app.spawn_window("Animated Text Truncation Demo", 800.0, 400.0, |window| {
                // Create animated text label
                let animated_label = AnimatedTextLabel::new(
                    WidgetId::new(100),
                    "This is a long text that will demonstrate dynamic truncation with ellipsis (...) as the container width changes. Watch how the text truncates smoothly!",
                    100.0,   // min_width: text heavily truncated
                    600.0,   // max_width: text fully visible
                )
                .with_bg_color(Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 });

                // Add to second window using clean API
                window.add_to_root(Box::new(animated_label), Style::default())
                    .expect("Failed to add widget");

                println!("Animated text truncation demo window created!");
                println!("  → Watch the text truncate with '...' as width oscillates");
                println!("  → min width: 100px, max width: 600px");
                println!();
            });

            // Event loop runs automatically after Application::launch
        });
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
