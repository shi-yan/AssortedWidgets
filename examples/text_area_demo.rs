use assorted_widgets::Application;
use assorted_widgets::widgets::{TextArea, Padding};

fn main() {
    println!("AssortedWidgets - TextArea Widget Demo");
    println!("========================================");
    println!();
    println!("This demo showcases the TextArea widget features:");
    println!("  ✓ Multi-line text editing");
    println!("  ✓ Word wrap and no-wrap modes");
    println!("  ✓ Vertical and horizontal scrolling");
    println!("  ✓ Up/down arrow navigation with preferred column");
    println!("  ✓ Multi-line text selection");
    println!("  ✓ Keyboard shortcuts (Cmd+C/V/X/A/Z/Y)");
    println!("  ✓ Home/End for line navigation");
    println!("  ✓ Enter for newlines, Tab for indentation");
    println!("  ✓ Undo/redo system");
    println!();
    println!("Instructions:");
    println!("  • Click any text area to focus it");
    println!("  • Type text and use arrow keys (including ⬆️⬇️) to navigate");
    println!("  • Press Enter to create new lines");
    println!("  • Press Tab to insert 4 spaces");
    println!("  • Try keyboard shortcuts (Cmd+C, Cmd+V, etc.)");
    println!("  • Use Shift+Arrow keys for multi-line selection");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        Application::launch(|app| {
            app.spawn_window("TextArea Widget Demo", 800.0, 900.0, |window| {
                println!("WebGPU initialized successfully!");
                println!();
                println!("Window created!");
                println!();

                // ================================================================
                // Create test text areas
                // ================================================================

                // 1. Basic multi-line text area with wrapping
                let basic_textarea = TextArea::new()
                    .placeholder("Enter your notes here...\n\nThis text area supports:\n  • Multiple lines\n  • Word wrapping\n  • Vertical scrolling\n  • Up/down arrow navigation")
                    .font_size(14.0)
                    .wrapping(true)
                    .padding(Padding::uniform(12.0))
                    .on_change(|text| {
                        let line_count = text.lines().count();
                        let char_count = text.chars().count();
                        println!("[Basic TextArea] Lines: {}, Chars: {}", line_count, char_count);
                    })
                    .text("Welcome to TextArea!\n\nThis is a multi-line text input widget.\n\nTry using the arrow keys (including up/down) to navigate.\nPress Enter to create new lines.\nPress Tab to indent.\n\nWord wrapping is enabled, so long lines will automatically wrap to the next line when they exceed the available width.");

                // 2. Code editor-style (no wrapping, horizontal scroll)
                let code_textarea = TextArea::new()
                    .placeholder("// Enter code here...")
                    .font_size(13.0)
                    .wrapping(false)  // No wrapping - horizontal scroll instead
                    .padding(Padding::uniform(12.0))
                    .text("fn main() {\n    println!(\"Hello, world!\");\n    \n    // This text area has wrapping disabled\n    // Long lines will scroll horizontally instead of wrapping\n    let long_line = \"This is a very long line that would normally wrap, but in no-wrap mode it extends horizontally and you can scroll to see it all.\";\n}")
                    .on_change(|text| {
                        println!("[Code TextArea] Updated: {} chars", text.chars().count());
                    });

                // 3. Validated text area (example: must not be empty)
                let validator = |text: &str| -> Result<(), String> {
                    if text.trim().is_empty() {
                        Err("Text cannot be empty".to_string())
                    } else if text.lines().count() < 2 {
                        Err("Please enter at least 2 lines".to_string())
                    } else {
                        Ok(())
                    }
                };

                let validated_textarea = TextArea::new()
                    .placeholder("Enter feedback (at least 2 lines)...")
                    .font_size(14.0)
                    .wrapping(true)
                    .padding(Padding::uniform(12.0))
                    .validator(validator)
                    .text("This text area has validation.\n\nTry deleting all text to see the error state.")
                    .on_change(|_text| {
                        println!("[Validated TextArea] Updated");
                    });

                // Configure root container
                let container_style = taffy::Style {
                    display: taffy::Display::Flex,
                    flex_direction: taffy::FlexDirection::Column,
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::percent(1.0),
                    },
                    gap: taffy::Size {
                        width: taffy::LengthPercentage::length(20.0),
                        height: taffy::LengthPercentage::length(20.0),
                    },
                    padding: taffy::Rect {
                        left: taffy::LengthPercentage::length(30.0),
                        right: taffy::LengthPercentage::length(30.0),
                        top: taffy::LengthPercentage::length(30.0),
                        bottom: taffy::LengthPercentage::length(30.0),
                    },
                    ..Default::default()
                };

                window.set_root_layout(container_style);

                // TextArea styles
                let textarea_style_large = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::length(250.0),
                    },
                    ..Default::default()
                };

                let textarea_style_medium = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::length(200.0),
                    },
                    ..Default::default()
                };

                let textarea_style_small = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::length(150.0),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(basic_textarea), textarea_style_large)
                    .expect("Failed to add basic textarea");

                window.add_to_root(Box::new(code_textarea), textarea_style_medium)
                    .expect("Failed to add code textarea");

                window.add_to_root(Box::new(validated_textarea), textarea_style_small)
                    .expect("Failed to add validated textarea");

                println!("✅ Demo setup complete!");
                println!();
                println!("═══════════════════════════════════════════════════════");
                println!("  FEATURES TO TEST");
                println!("═══════════════════════════════════════════════════════");
                println!();
                println!("Basic TextArea (top, with wrapping):");
                println!("  • Multi-line text editing");
                println!("  • Word wrapping enabled");
                println!("  • Vertical scrolling when content overflows");
                println!("  • Arrow keys for navigation (⬆️⬇️⬅️➡️)");
                println!("  • Home/End: Move to start/end of current line");
                println!("  • Enter: Insert newline");
                println!("  • Tab: Insert 4 spaces");
                println!();
                println!("Code TextArea (middle, no wrapping):");
                println!("  • Word wrapping disabled");
                println!("  • Horizontal scrolling for long lines");
                println!("  • Vertical scrolling when many lines");
                println!("  • Try typing a very long line to see horizontal scroll");
                println!();
                println!("Validated TextArea (bottom):");
                println!("  • Real-time validation (needs 2+ lines)");
                println!("  • Red border and error message when invalid");
                println!("  • Try deleting text to trigger validation");
                println!();
                println!("Multi-line Selection:");
                println!("  • Click and drag to select across lines");
                println!("  • Shift+Arrow keys to extend selection");
                println!("  • Shift+Home/End to select to line boundaries");
                println!("  • Double-click: Select all");
                println!();
                println!("Keyboard Shortcuts (all text areas):");
                println!("  • Cmd+C: Copy selected text");
                println!("  • Cmd+V: Paste");
                println!("  • Cmd+X: Cut selected text");
                println!("  • Cmd+A: Select all");
                println!("  • Cmd+Z: Undo");
                println!("  • Cmd+Shift+Z or Cmd+Y: Redo");
                println!();
                println!("Up/Down Navigation:");
                println!("  • Arrow Up/Down: Move cursor between lines");
                println!("  • Maintains horizontal position (preferred column)");
                println!("  • Try moving up/down through lines of different lengths");
                println!();
                println!("Scrolling:");
                println!("  • Mouse wheel: Scroll vertically");
                println!("  • Cursor automatically scrolls into view");
                println!("  • Horizontal scroll (code area): Shift+wheel or trackpad");
                println!("  • Scrollbars appear when content overflows");
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
