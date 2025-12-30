use assorted_widgets::Application;
use assorted_widgets::widgets::TextInput;
use assorted_widgets::types::WidgetId;

fn main() {
    println!("AssortedWidgets - TextInput Widget Demo");
    println!("========================================");
    println!();
    println!("This demo showcases the TextInput widget features:");
    println!("  ✓ Basic text input with placeholder");
    println!("  ✓ Search box with left icon");
    println!("  ✓ Email input with validation");
    println!("  ✓ Password input with reveal button");
    println!("  ✓ Keyboard shortcuts (Cmd+C/V/X/A/Z/Y)");
    println!("  ✓ Text selection and cursor movement");
    println!("  ✓ Automatic horizontal scrolling");
    println!("  ✓ Undo/redo system");
    println!();
    println!("Instructions:");
    println!("  • Click any input to focus it");
    println!("  • Type text and use arrow keys to navigate");
    println!("  • Press Tab to switch between inputs");
    println!("  • Try keyboard shortcuts (Cmd+C, Cmd+V, etc.)");
    println!("  • For email: must contain @ and .");
    println!("  • For password: click eye button to reveal");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        Application::launch(|app| {
            app.spawn_window("TextInput Widget Demo", 600.0, 700.0, |window| {
                println!("WebGPU initialized successfully!");
                println!();
                println!("Window created!");
                println!();

                // ================================================================
                // Create test inputs
                // ================================================================

                // 1. Basic text input
                let basic_input = TextInput::new()
                    .placeholder("Enter your name...")
                    .font_size(16.0)
                    .on_change(|text| {
                        println!("[Basic Input] Text changed: '{}'", text);
                    });
 
                // 2. Search box with icon
                 let search_input = TextInput::new()
                    .placeholder("Search...")
                    .icon("search")
                    .font_size(16.0)
                    .on_change(|text| {
                        println!("[Search] Text changed: '{}'", text);
                    });

                // 3. Email input with validation
                let email_validator = |text: &str| -> Result<(), String> {
                    if text.is_empty() {
                        return Ok(());
                    }
                    if text.contains('@') && text.contains('.') {
                        Ok(())
                    } else {
                        Err("Invalid email format".to_string())
                    }
                };

                let email_input = TextInput::new()
                    .placeholder("Enter email address...")
                    .icon("mail")
                    .validator(Box::new(email_validator))
                    .font_size(16.0)
                    .on_change(|text| {
                        println!("[Email] Text changed: '{}'", text);
                    })
                    .on_submit(|text| {
                        println!("[Email] Submitted: '{}'", text);
                    });

                // 4. Password input with reveal button
                let password_input = TextInput::new()
                    .placeholder("Password")
                    .password(true)
                    .show_password_toggle(true)
                    .font_size(16.0)
                    .on_change(|text| {
                        println!("[Password] Text changed (length: {})", text.len());
                    })
                    .on_submit(|text| {
                        println!("[Password] Submitted: '{}' (redacted in actual use)", text);
                    });

                // Configure root container with proper size (fills window)
                let container_style = taffy::Style {
                    display: taffy::Display::Flex,
                    flex_direction: taffy::FlexDirection::Column,
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),   // CRITICAL: Fill window width
                        height: taffy::Dimension::percent(1.0),  // CRITICAL: Fill window height
                    },
                    gap: taffy::Size {
                        width: taffy::LengthPercentage::length(20.0),
                        height: taffy::LengthPercentage::length(20.0),
                    },
                    padding: taffy::Rect {
                        left: taffy::LengthPercentage::length(40.0),
                        right: taffy::LengthPercentage::length(40.0),
                        top: taffy::LengthPercentage::length(40.0),
                        bottom: taffy::LengthPercentage::length(40.0),
                    },
                    ..Default::default()
                };

                window.set_root_layout(container_style);

                // Input style uses percentage width (now works because root has size!)
                let input_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),  // 100% of parent (minus padding)
                        height: taffy::Dimension::length(44.0),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(basic_input), input_style.clone())
                    .expect("Failed to add basic input");
 
                window.add_to_root(Box::new(search_input), input_style.clone())
                    .expect("Failed to add search input");

                window.add_to_root(Box::new(email_input), input_style.clone())
                    .expect("Failed to add email input");

                window.add_to_root(Box::new(password_input), input_style)
                    .expect("Failed to add password input");

                println!("✅ Demo setup complete!");
                println!();
                println!("═══════════════════════════════════════════════════════");
                println!("  FEATURES TO TEST");
                println!("═══════════════════════════════════════════════════════");
                println!();
                println!("Basic Input:");
                println!("  • Simple text entry with placeholder");
                println!("  • Text change events logged to console");
                println!();
                println!("Search Box:");
                println!("  • Left-side search icon");
                println!("  • Text updates logged");
                println!();
                println!("Email Input:");
                println!("  • Left-side mail icon");
                println!("  • Real-time validation (must have @ and .)");
                println!("  • Red error label appears if invalid");
                println!("  • Press Enter to submit");
                println!();
                println!("Password Input:");
                println!("  • Text displayed as bullets (••••)");
                println!("  • Eye button on right to toggle visibility");
                println!("  • Press Enter to submit");
                println!();
                println!("Keyboard Shortcuts (all inputs):");
                println!("  • Cmd+C: Copy selected text (signal logged)");
                println!("  • Cmd+V: Paste (signal logged)");
                println!("  • Cmd+X: Cut selected text (signal logged)");
                println!("  • Cmd+A: Select all");
                println!("  • Cmd+Z: Undo");
                println!("  • Cmd+Shift+Z or Cmd+Y: Redo");
                println!();
                println!("Navigation:");
                println!("  • Arrow keys: Move cursor left/right");
                println!("  • Home/End: Jump to start/end");
                println!("  • Shift+Arrow: Select text");
                println!("  • Double-click: Select all");
                println!();
                println!("Scrolling:");
                println!("  • Type long text to see auto-scroll");
                println!("  • Cursor stays visible while typing");
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
