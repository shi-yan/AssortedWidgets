//! Controls demo - Checkbox, RadioGroup, and Switch widgets
//!
//! This example demonstrates the three new UI controls:
//! - Checkbox: Boolean toggle with optional label
//! - RadioGroup: Single selection from multiple options
//! - Switch: Visual on/off toggle

use assorted_widgets::Application;
use assorted_widgets::widgets::{Checkbox, RadioGroup, Switch, Label};
use assorted_widgets::paint::primitives::Color;
use assorted_widgets::text::TextAlign;

fn main() {
    println!("🎛️  Controls Demo");
    println!("================");
    println!();
    println!("Testing three new widgets:");
    println!("  ☑️  Checkbox - Boolean toggle");
    println!("  ⭕ RadioGroup - Single selection");
    println!("  🔘 Switch - On/off toggle");
    println!();

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Controls Demo", 800.0, 600.0, |window| {
                // Configure implicit root container with absolute positioning
                window.set_root_layout(taffy::Style::default());

                // ============================================================
                // SECTION 1: Checkboxes
                // ============================================================

                // Section label
                let label1 = Label::new("Checkboxes:")
                    .font_size(20.0)
                    .text_color(Color::rgb(0.9, 0.9, 0.9))
                    .align(TextAlign::Left);

                window.add_to_root(Box::new(label1), taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(20.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add label");

                // Test 1: Basic checkbox
                let checkbox1 = Checkbox::new("Enable notifications")
                    .on_changed(|checked| {
                        println!("✓ Notifications: {}", if checked { "ON" } else { "OFF" });
                    });

                window.add_to_root(Box::new(checkbox1), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(200.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(55.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add checkbox 1");

                // Test 2: Pre-checked checkbox
                let checkbox2 = Checkbox::new("Auto-save")
                    .checked()
                    .on_changed(|checked| {
                        println!("💾 Auto-save: {}", if checked { "ON" } else { "OFF" });
                    });

                window.add_to_root(Box::new(checkbox2), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(200.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(95.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add checkbox 2");

                // Test 3: Disabled checkbox
                let checkbox3 = Checkbox::new("Premium feature (disabled)")
                    .disabled();

                window.add_to_root(Box::new(checkbox3), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(250.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(135.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add checkbox 3");

                // Test 4: Icon-only checkbox
                let checkbox4 = Checkbox::icon_only()
                    .on_changed(|checked| {
                        println!("📌 Icon checkbox: {}", if checked { "ON" } else { "OFF" });
                    });

                window.add_to_root(Box::new(checkbox4), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(30.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(175.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add checkbox 4");

                // ============================================================
                // SECTION 2: Radio Groups
                // ============================================================

                // Section label
                let label2 = Label::new("Radio Groups:")
                    .font_size(20.0)
                    .text_color(Color::rgb(0.9, 0.9, 0.9))
                    .align(TextAlign::Left);

                window.add_to_root(Box::new(label2), taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(300.0).into(),
                        top: taffy::LengthPercentage::length(20.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add label 2");

                // Test 5: Basic radio group
                let radio1 = RadioGroup::new()
                    .add_item("Small")
                    .add_item("Medium")
                    .add_item("Large")
                    .selected(1)  // Select "Medium" by default
                    .on_selection_change(|index| {
                        let sizes = ["Small", "Medium", "Large"];
                        println!("📏 Size selected: {}", sizes[index]);
                    });

                window.add_to_root(Box::new(radio1), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(200.0),
                        height: taffy::Dimension::length(140.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(300.0).into(),
                        top: taffy::LengthPercentage::length(55.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add radio group 1");

                // Test 6: Radio group with colors
                let radio2 = RadioGroup::new()
                    .add_item("Red")
                    .add_item("Green")
                    .add_item("Blue")
                    .add_disabled_item("Gold (Premium)")
                    .on_selection_change(|index| {
                        let colors = ["Red", "Green", "Blue", "Gold"];
                        println!("🎨 Color selected: {}", colors[index]);
                    });

                window.add_to_root(Box::new(radio2), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(200.0),
                        height: taffy::Dimension::length(180.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(300.0).into(),
                        top: taffy::LengthPercentage::length(220.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add radio group 2");

                // ============================================================
                // SECTION 3: Switches
                // ============================================================

                // Section label
                let label3 = Label::new("Switches:")
                    .font_size(20.0)
                    .text_color(Color::rgb(0.9, 0.9, 0.9))
                    .align(TextAlign::Left);

                window.add_to_root(Box::new(label3), taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(550.0).into(),
                        top: taffy::LengthPercentage::length(20.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add label 3");

                // Test 7: Basic switch (off)
                let switch1 = Switch::new()
                    .on_changed(|is_on| {
                        println!("🔆 Brightness: {}", if is_on { "ON" } else { "OFF" });
                    });

                window.add_to_root(Box::new(switch1), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(50.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(550.0).into(),
                        top: taffy::LengthPercentage::length(55.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add switch 1");

                // Switch label
                let switch1_label = Label::new("Brightness")
                    .font_size(16.0)
                    .text_color(Color::rgb(0.9, 0.9, 0.9))
                    .align(TextAlign::Left);

                window.add_to_root(Box::new(switch1_label), taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(610.0).into(),
                        top: taffy::LengthPercentage::length(60.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add switch label");

                // Test 8: Switch (pre-enabled)
                let switch2 = Switch::new()
                    .on()
                    .on_changed(|is_on| {
                        println!("🔊 Sound: {}", if is_on { "ON" } else { "OFF" });
                    });

                window.add_to_root(Box::new(switch2), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(50.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(550.0).into(),
                        top: taffy::LengthPercentage::length(105.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add switch 2");

                // Switch 2 label
                let switch2_label = Label::new("Sound (ON)")
                    .font_size(16.0)
                    .text_color(Color::rgb(0.9, 0.9, 0.9))
                    .align(TextAlign::Left);

                window.add_to_root(Box::new(switch2_label), taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(610.0).into(),
                        top: taffy::LengthPercentage::length(110.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add switch 2 label");

                // Test 9: Disabled switch
                let switch3 = Switch::new()
                    .disabled();

                window.add_to_root(Box::new(switch3), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(50.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(550.0).into(),
                        top: taffy::LengthPercentage::length(155.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add switch 3");

                // Switch 3 label
                let switch3_label = Label::new("VPN (disabled)")
                    .font_size(16.0)
                    .text_color(Color::rgb(0.5, 0.5, 0.5))
                    .align(TextAlign::Left);

                window.add_to_root(Box::new(switch3_label), taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(610.0).into(),
                        top: taffy::LengthPercentage::length(160.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add switch 3 label");

                // Test 10: Custom colored switch
                let switch4 = Switch::new()
                    .on_track_color(Color::rgb(0.2, 0.7, 0.3))  // Green
                    .on_thumb_color(Color::rgb(1.0, 1.0, 1.0))
                    .off_track_color(Color::rgb(0.7, 0.2, 0.2)) // Red
                    .off_thumb_color(Color::rgb(0.9, 0.9, 0.9))
                    .on_changed(|is_on| {
                        println!("🌐 WiFi: {}", if is_on { "CONNECTED" } else { "DISCONNECTED" });
                    });

                window.add_to_root(Box::new(switch4), taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(50.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(550.0).into(),
                        top: taffy::LengthPercentage::length(205.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add switch 4");

                // Switch 4 label
                let switch4_label = Label::new("WiFi (custom colors)")
                    .font_size(16.0)
                    .text_color(Color::rgb(0.9, 0.9, 0.9))
                    .align(TextAlign::Left);

                window.add_to_root(Box::new(switch4_label), taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(610.0).into(),
                        top: taffy::LengthPercentage::length(210.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add switch 4 label");

                // ============================================================
                // Instructions
                // ============================================================

                let instructions = Label::new("💡 Try these interactions:\n\
                    • Click checkboxes to toggle\n\
                    • Press Space/Enter when focused\n\
                    • Click radio buttons to select\n\
                    • Toggle switches on/off\n\
                    • Hover to see hover states")
                    .font_size(14.0)
                    .text_color(Color::rgb(0.7, 0.7, 0.7))
                    .align(TextAlign::Left);

                window.add_to_root(Box::new(instructions), taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(450.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                }).expect("Failed to add instructions");

                println!("✓ All controls added to window");
                println!();
                println!("Controls in window:");
                println!("  • 4 checkboxes (normal, checked, disabled, icon-only)");
                println!("  • 2 radio groups (sizes, colors)");
                println!("  • 4 switches (normal, on, disabled, custom colors)");
                println!();
                println!("Watch console for interaction feedback!");
            });
        });
    }
}
