//! Button Group demo - connected buttons with single selection

use assorted_widgets::Application;
use assorted_widgets::widgets::{ButtonGroup, Padding};

fn main() {
    println!("🔘 Button Group Demo");
    println!("===================");

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Button Group Demo", 600.0, 400.0, |window| {
                // Configure implicit root container with absolute positioning
                window.set_root_layout(taffy::Style::default());

                // Test 1: Basic button group (like the image)
                let button_group1 = ButtonGroup::new()
                    .add_button("Left")
                    .add_button("Middle")
                    .add_button("Right")
                    .selected(1)  // Select "Middle" by default
                    .padding(Padding::symmetric(16.0, 8.0))
                    .on_selection_change(|index| {
                        let labels = ["Left", "Middle", "Right"];
                        println!("✓ Selected: {}", labels[index]);
                    });

                let style1 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(400.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(100.0).into(),
                        top: taffy::LengthPercentage::length(40.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button_group1), style1)
                    .expect("Failed to add button group 1");

                // Test 2: More buttons
                let button_group2 = ButtonGroup::new()
                    .add_button("Option 1")
                    .add_button("Option 2")
                    .add_button("Option 3")
                    .add_button("Option 4")
                    .selected(0)
                    .on_selection_change(|index| {
                        println!("✓ Group 2 selected: Option {}", index + 1);
                    });

                let style2 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(500.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(50.0).into(),
                        top: taffy::LengthPercentage::length(120.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button_group2), style2)
                    .expect("Failed to add button group 2");

                // Test 3: Two buttons (alignment toggle)
                let button_group3 = ButtonGroup::new()
                    .add_button("Horizontal")
                    .add_button("Vertical")
                    .selected(0)
                    .on_selection_change(|index| {
                        let labels = ["Horizontal", "Vertical"];
                        println!("✓ Alignment: {}", labels[index]);
                    });

                let style3 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(300.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(150.0).into(),
                        top: taffy::LengthPercentage::length(200.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button_group3), style3)
                    .expect("Failed to add button group 3");

                // Test 4: Small button group (compact)
                let button_group4 = ButtonGroup::new()
                    .add_button("S")
                    .add_button("M")
                    .add_button("L")
                    .add_button("XL")
                    .selected(2)
                    .padding(Padding::symmetric(12.0, 6.0))
                    .font_size(14.0)
                    .on_selection_change(|index| {
                        let sizes = ["S", "M", "L", "XL"];
                        println!("✓ Size: {}", sizes[index]);
                    });

                let style4 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(250.0),
                        height: taffy::Dimension::length(40.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(175.0).into(),
                        top: taffy::LengthPercentage::length(280.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(button_group4), style4)
                    .expect("Failed to add button group 4");

                println!("✓ 4 button groups added to window");
                println!("• Click buttons to change selection");
                println!("• Only one button per group can be selected");
            });
        });
    }
}
