use assorted_widgets::Application;
use assorted_widgets::elements::SimpleInputBox;
use assorted_widgets::types::WidgetId;

fn main() {
    println!("AssortedWidgets - Phase 2.2: Focus & IME Demo");
    println!("==============================================");
    println!();
    println!("This demo tests:");
    println!("  ✓ Focus management (click input box to focus)");
    println!("  ✓ Tab navigation between focusable elements");
    println!("  ✓ Keyboard input to focused element");
    println!("  ✓ IME cursor positioning");
    println!("  ✓ Mouse capture (for future drag operations)");
    println!();
    println!("Instructions:");
    println!("  1. Click the input box to focus it");
    println!("  2. Type characters to see them appear");
    println!("  3. Press Backspace to delete");
    println!("  4. Use Tab to cycle focus (when multiple focusable elements)");
    println!();
    println!("Note: Full IME support (Chinese/Japanese/Korean) requires");
    println!("      NSTextInputClient implementation (future enhancement)");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        Application::launch(|app| {
            app.spawn_window("Phase 2: Hit Testing Demo", 800.0, 600.0, |window| {
                println!("WebGPU initialized successfully!");
                println!();
                println!("Window created!");
                println!();

                // ================================================================
                // Create test widgets for focus and IME
                // ================================================================

                // Input box IDs (capture before moving widgets)
                let input1_id = WidgetId::new(1);
                let input2_id = WidgetId::new(2);

                // Input box 1: Primary input
                let input1 = SimpleInputBox::new(input1_id);

                // Input box 2: Secondary input (for Tab testing)
                let input2 = SimpleInputBox::new(input2_id);

                // Create layout style for vertical stacking
                let layout_style = taffy::Style {
                    display: taffy::Display::Flex,
                    flex_direction: taffy::FlexDirection::Column,
                    gap: taffy::Size {
                        width: taffy::LengthPercentage::length(20.0),
                        height: taffy::LengthPercentage::length(20.0),
                    },
                    padding: taffy::Rect {
                        left: taffy::LengthPercentage::length(50.0),
                        right: taffy::LengthPercentage::length(50.0),
                        top: taffy::LengthPercentage::length(50.0),
                        bottom: taffy::LengthPercentage::length(50.0),
                    },
                    ..Default::default()
                };

                // Add input box 1 as root using clean Window API
                window.add_root(Box::new(input1), layout_style.clone())
                    .expect("Failed to add input box 1");

                // Add input box 2 as child of input box 1 using clean Window API
                window.add_child(Box::new(input2), layout_style, input1_id)
                    .expect("Failed to add input box 2");

                println!("✅ Demo setup complete!");
                println!();
                println!("═══════════════════════════════════════════════════════");
                println!("  TEST INSTRUCTIONS");
                println!("═══════════════════════════════════════════════════════");
                println!();
                println!("✓ Two input boxes are displayed vertically");
                println!("✓ Click an input box to focus it (watch terminal log)");
                println!("✓ Type characters to see them appear in white");
                println!("✓ Press Backspace to delete characters");
                println!("✓ Press Tab to switch focus between input boxes");
                println!();
                println!("Future enhancements:");
                println!("  - Full IME support requires NSTextInputClient");
                println!("  - Preedit text shown in yellow with underline");
                println!("  - Committed text shown in white");
                println!("═══════════════════════════════════════════════════════");
                println!();
            });
        });
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
