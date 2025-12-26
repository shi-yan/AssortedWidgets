use assorted_widgets::Application;
use assorted_widgets::widgets::{Label, WrapMode, Padding};
use assorted_widgets::paint::Color;
use assorted_widgets::text::TextAlign;

fn main() {
    println!("AssortedWidgets - Label Widget Demo");
    println!("=====================================");
    println!();

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            println!("WebGPU initialized successfully!");
            println!();

            // Create main demo window with Qt-style implicit root container
            app.spawn_window("Label Widget Demo", 850.0, 1100.0, |window| {
                println!("Window created!");
                println!();

                // Configure the window's implicit root container layout (Qt-style)
                window.set_root_layout(taffy::Style {
                    display: taffy::Display::Flex,
                    flex_direction: taffy::FlexDirection::Column,
                    gap: taffy::Size {
                        width: taffy::LengthPercentage::length(15.0),
                        height: taffy::LengthPercentage::length(15.0),
                    },
                    padding: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0),
                        right: taffy::LengthPercentage::length(20.0),
                        top: taffy::LengthPercentage::length(20.0),
                        bottom: taffy::LengthPercentage::length(20.0),
                    },
                    ..Default::default()
                });

                // ================================================================
                // Example 1: Single Line with Ellipsis
                // ================================================================
                let label1 = Label::new("1. SINGLE LINE ELLIPSIS: This is a very long text that will be truncated with ellipses when it doesn't fit in the available space. You should see ... at the end.")
                    .wrap_mode(WrapMode::SingleLineEllipsis)
                    .bg_color(Color::rgb(0.25, 0.20, 0.30))  // Purple background
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(12.0));

                let label_style1 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(700.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label1), label_style1)
                    .expect("Failed to add label 1");

                // ================================================================
                // Example 2: Single Line without Ellipsis (Hard Clip)
                // ================================================================
                let label2 = Label::new("2. SINGLE LINE CLIP: This text will be clipped without ellipses. It just cuts off abruptly at the edge of the container. No ellipsis marker shown.")
                    .wrap_mode(WrapMode::SingleLine)
                    .bg_color(Color::rgb(0.30, 0.20, 0.20))  // Red background
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(12.0));

                let label_style2 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(700.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label2), label_style2)
                    .expect("Failed to add label 2");

                // ================================================================
                // Example 3: Word Wrapping (breaks between words only)
                // ================================================================
                let label3 = Label::new("3. WORD WRAPPING: This is a longer text that demonstrates word wrapping. When horizontal space is limited, the text will wrap to multiple lines, but only at word boundaries. This makes it easy to read and looks professional. Notice how words stay intact.")
                    .wrap_mode(WrapMode::WrapWord)
                    .bg_color(Color::rgb(0.20, 0.30, 0.20))  // Green background
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(12.0));

                let label_style3 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(500.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label3), label_style3)
                    .expect("Failed to add label 3");

                // ================================================================
                // Example 4: Wrap Anywhere (can break mid-word)
                // ================================================================
                let label4 = Label::new("4. WRAP ANYWHERE: Thisisaverylongwordthatcannotfitonasinglelineanddemonstrateswrappinganywheremodeforcaseswhereverylong­continuoustextexistswithoutanyspacesorbreaks")
                    .wrap_mode(WrapMode::WrapAnywhere)
                    .bg_color(Color::rgb(0.20, 0.25, 0.35))  // Blue background
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(12.0));

                let label_style4 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(400.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label4), label_style4)
                    .expect("Failed to add label 4");

                // ================================================================
                // Example 5: Different Alignments
                // ================================================================

                // Left aligned
                let label5a = Label::new("5a. LEFT ALIGNED: Text starts from the left edge")
                    .align(TextAlign::Left)
                    .bg_color(Color::rgb(0.30, 0.25, 0.20))  // Brown background
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(12.0));

                let label_style5a = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(600.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label5a), label_style5a)
                    .expect("Failed to add label 5a");

                // Center aligned
                let label5b = Label::new("5b. CENTER ALIGNED: Text is centered")
                    .align(TextAlign::Center)
                    .bg_color(Color::rgb(0.25, 0.30, 0.25))  // Teal background
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(12.0));

                let label_style5b = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(600.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label5b), label_style5b)
                    .expect("Failed to add label 5b");

                // Right aligned
                let label5c = Label::new("5c. RIGHT ALIGNED: Text ends at the right edge")
                    .align(TextAlign::Right)
                    .bg_color(Color::rgb(0.30, 0.20, 0.25))  // Magenta background
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(12.0));

                let label_style5c = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(600.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label5c), label_style5c)
                    .expect("Failed to add label 5c");

                // ================================================================
                // Example 6: Different Padding Values
                // ================================================================
                let label6a = Label::new("6a. SMALL PADDING (4px): Tight spacing around text")
                    .bg_color(Color::rgb(0.35, 0.25, 0.25))
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(4.0));

                window.add_to_root(Box::new(label6a), taffy::Style::default())
                    .expect("Failed to add label 6a");

                let label6b = Label::new("6b. MEDIUM PADDING (12px): Comfortable spacing")
                    .bg_color(Color::rgb(0.25, 0.35, 0.25))
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(12.0));

                window.add_to_root(Box::new(label6b), taffy::Style::default())
                    .expect("Failed to add label 6b");

                let label6c = Label::new("6c. LARGE PADDING (24px): Spacious layout")
                    .bg_color(Color::rgb(0.25, 0.25, 0.35))
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(24.0));

                window.add_to_root(Box::new(label6c), taffy::Style::default())
                    .expect("Failed to add label 6c");

                let label6d = Label::new("6d. ASYMMETRIC PADDING: Left=40, Right=8, Top=20, Bottom=6")
                    .bg_color(Color::rgb(0.35, 0.30, 0.20))
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::new(20.0, 8.0, 6.0, 40.0));

                window.add_to_root(Box::new(label6d), taffy::Style::default())
                    .expect("Failed to add label 6d");

                // ================================================================
                // Example 7: Different Font Sizes
                // ================================================================
                let label7a = Label::new("7a. SMALL TEXT (12pt)")
                    .font_size(12.0)
                    .bg_color(Color::rgb(0.28, 0.24, 0.22))
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(8.0));

                window.add_to_root(Box::new(label7a), taffy::Style::default())
                    .expect("Failed to add label 7a");

                let label7b = Label::new("7b. MEDIUM TEXT (16pt)")
                    .font_size(16.0)
                    .bg_color(Color::rgb(0.24, 0.28, 0.22))
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(10.0));

                window.add_to_root(Box::new(label7b), taffy::Style::default())
                    .expect("Failed to add label 7b");

                let label7c = Label::new("7c. LARGE TEXT (24pt)")
                    .font_size(24.0)
                    .bg_color(Color::rgb(0.22, 0.24, 0.28))
                    .text_color(Color::rgb(1.0, 1.0, 1.0))
                    .padding(Padding::uniform(16.0));

                window.add_to_root(Box::new(label7c), taffy::Style::default())
                    .expect("Failed to add label 7c");

                // ================================================================
                // Example 8: Transparent Background
                // ================================================================
                let label8 = Label::new("8. TRANSPARENT BACKGROUND: No background color, blends with parent")
                    .transparent()
                    .text_color(Color::rgb(1.0, 0.8, 0.0))  // Golden text
                    .font_size(18.0)
                    .padding(Padding::uniform(12.0));

                window.add_to_root(Box::new(label8), taffy::Style::default())
                    .expect("Failed to add label 8");

                println!("Label Widget Demo - Test Cases");
                println!("================================");
                println!();
                println!("Features demonstrated:");
                println!("  ✓ Single line truncation (with and without ellipses)");
                println!("  ✓ Word wrapping (word boundaries only)");
                println!("  ✓ Character wrapping (anywhere - for long continuous text)");
                println!("  ✓ Text alignment (left, center, right)");
                println!("  ✓ Different padding values (4px, 12px, 24px, asymmetric)");
                println!("  ✓ Different font sizes (12pt, 16pt, 24pt)");
                println!("  ✓ Background colors and transparency");
                println!();
                println!("Each label has a colored background to show its bounds.");
                println!("Press Cmd+Q to quit.");
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
