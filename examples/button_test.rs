//! Button widget test - demonstrates button functionality
//!
//! This example tests:
//! - Text-only buttons
//! - Icon-only buttons
//! - Icon+text buttons
//! - Button states (normal, hovered, pressed, disabled, focused)
//! - Toggleable buttons
//! - Click callbacks

use assorted_widgets::Application;
use assorted_widgets::widgets::{Button, Padding};
use assorted_widgets::paint::primitives::Color;

fn main() {
    println!("🔘 Button Widget Test");
    println!("====================");
    println!("This example demonstrates button functionality:");
    println!("- Text-only buttons");
    println!("- Icon-only buttons");
    println!("- Icon+text buttons");
    println!("- Click callbacks");
    println!("- Toggleable buttons");
    println!("- Hover states");
    println!("- Disabled buttons");
    println!();

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Button Test", 700.0, 550.0, |window| {
                // Configure implicit root container with absolute positioning
                window.set_root_layout(taffy::Style::default());

                let mut y = 20.0;
                let x = 20.0;
                let spacing = 20.0;

                // Row 1: Text buttons
                let button1 = Button::text("Click Me!")
                    .padding(Padding::symmetric(20.0, 10.0))
                    .on_click(|| {
                        println!("✓ Text button clicked!");
                    });

                let style1 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button1), style1)
                    .expect("Failed to add button 1");

                // Disabled button
                let button2 = Button::text("Disabled")
                    .disabled();

                let style2 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x + 180.0).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button2), style2)
                    .expect("Failed to add button 2");

                y += 70.0;

                // Row 2: Icon buttons
                let button3 = Button::icon("search")
                    .font_size(24.0)
                    .padding(Padding::uniform(12.0))
                    .on_click(|| {
                        println!("🔍 Search button clicked!");
                    });

                let style3 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button3), style3)
                    .expect("Failed to add button 3");

                let button4 = Button::icon("settings")
                    .font_size(24.0)
                    .padding(Padding::uniform(12.0))
                    .on_click(|| {
                        println!("⚙️  Settings button clicked!");
                    });

                let style4 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x + 80.0).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button4), style4)
                    .expect("Failed to add button 4");

                let button5 = Button::icon("favorite")
                    .font_size(24.0)
                    .padding(Padding::uniform(12.0))
                    .on_click(|| {
                        println!("❤️  Favorite button clicked!");
                    });

                let style5 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x + 160.0).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button5), style5)
                    .expect("Failed to add button 5");

                y += 90.0;

                // Row 3: Icon+text buttons
                let button6 = Button::icon_text("search", "Search")
                    .on_click(|| {
                        println!("🔍 Search (icon+text) clicked!");
                    });

                let style6 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button6), style6)
                    .expect("Failed to add button 6");

                let button7 = Button::icon_text("settings", "Settings")
                    .on_click(|| {
                        println!("⚙️  Settings (icon+text) clicked!");
                    });

                let style7 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x + 180.0).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button7), style7)
                    .expect("Failed to add button 7");

                y += 70.0;

                // Row 4: Toggle button
                let button8 = Button::text("Toggle Me")
                    .togglable()
                    .on_click(|| {
                        println!("🔄 Toggle button clicked!");
                    });

                let style8 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button8), style8)
                    .expect("Failed to add button 8");

                let button9 = Button::icon("favorite")
                    .togglable()
                    .on_click(|| {
                        println!("❤️  Toggle favorite clicked!");
                    });

                let style9 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x + 180.0).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button9), style9)
                    .expect("Failed to add button 9");

                y += 90.0;

                // Row 5: Custom styled button
                let button10 = Button::text("Custom Style")
                    .padding(Padding::symmetric(24.0, 12.0))
                    .font_size(18.0)
                    .on_click(|| {
                        println!("🎨 Custom styled button clicked!");
                    });

                let style10 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::auto(),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(x).into(),
                        top: taffy::LengthPercentage::length(y).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button10), style10)
                    .expect("Failed to add button 10");

                println!("✓ Buttons added to window");
                println!("• Hover over buttons to see hover state");
                println!("• Click buttons to trigger callbacks (see console)");
                println!("• Tab to navigate between buttons");
                println!("• Space or Enter to activate focused button");
            });
        });
    }
}
