//! Simple button test - minimal example to debug layout

use assorted_widgets::Application;
use assorted_widgets::widgets::{Button, Padding};
use assorted_widgets::paint::primitives::Color;

fn main() {
    println!("🔘 Simple Button Test");
    println!("====================");

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Simple Button Test", 600.0, 400.0, |window| {
                // Configure implicit root container with absolute positioning
                window.set_root_layout(taffy::Style::default());

                // Test 1: Simple text button
                let button1 = Button::text("Click Me!")
                    .padding(Padding::symmetric(20.0, 10.0))
                    .on_click(|| {
                        println!("✓ Button 1 clicked!");
                    });

                let style1 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(150.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(20.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button1), style1)
                    .expect("Failed to add button 1");

                // Test 2: Icon button
                let button2 = Button::icon("search")
                    .font_size(24.0)
                    .on_click(|| {
                        println!("🔍 Button 2 clicked!");
                    });

                let style2 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(60.0),
                        height: taffy::Dimension::length(60.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(90.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button2), style2)
                    .expect("Failed to add button 2");

                // Test 3: Icon+text button
                let button3 = Button::icon_text("settings", "Settings")
                    .on_click(|| {
                        println!("⚙️  Button 3 clicked!");
                    });

                let style3 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(180.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(170.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button3), style3)
                    .expect("Failed to add button 3");

                // Test 4: Disabled button
                let button4 = Button::text("Disabled")
                    .disabled();

                let style4 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(150.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(220.0).into(),
                        top: taffy::LengthPercentage::length(20.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button4), style4)
                    .expect("Failed to add button 4");

                // Test 5: Toggle button
                let button5 = Button::text("Toggle")
                    .togglable()
                    .on_click(|| {
                        println!("🔄 Button 5 toggled!");
                    });

                let style5 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(150.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(220.0).into(),
                        top: taffy::LengthPercentage::length(90.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button5), style5)
                    .expect("Failed to add button 5");

                println!("✓ 5 buttons added to window");
                println!("• Hover over buttons to see hover state");
                println!("• Click buttons to trigger callbacks");
            });
        });
    }
}
