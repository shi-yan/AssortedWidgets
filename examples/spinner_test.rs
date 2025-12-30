//! Spinner Widget Example
//!
//! This example demonstrates the Spinner widget with both modes:
//! - Indeterminate mode: Spinning animation for loading states
//! - Determinate mode: Progress circle showing completion percentage
//!
//! Features:
//! - Multiple spinner sizes and colors
//! - Animated spinning in indeterminate mode
//! - Interactive buttons to control progress
//! - Tests the animation system with continuous updates

use assorted_widgets::Application;
use assorted_widgets::paint::primitives::Color;
use assorted_widgets::widgets::{Label, Spinner};

fn main() {
    println!("🔄 Spinner Widget Demo");
    println!("=====================");
    println!();
    println!("This example demonstrates:");
    println!("  • Indeterminate spinners (continuous rotation animation)");
    println!("  • Determinate spinners (static progress display)");
    println!("  • Different sizes, colors, and styling options");

    Application::launch(|app| {
        app.spawn_window("Spinner Demo", 700.0, 800.0, |window| {
            // Use absolute positioning for layout
            window.set_root_layout(taffy::Style::default());

            // Title
            let title = Label::new("Spinner Widget Demo")
                .font_size(24.0)
                .text_color(Color::rgba(0.95, 0.95, 0.95, 1.0));

            let title_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(30.0).into(),
                    top: taffy::LengthPercentage::length(20.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(title), title_style).unwrap();

            // ================================================================
            // Section 1: Indeterminate Spinners (Animated)
            // ================================================================

            let section1 = Label::new("Indeterminate Mode (Animated Loading)")
                .font_size(18.0)
                .text_color(Color::rgba(0.8, 0.9, 1.0, 1.0));

            let section1_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(30.0).into(),
                    top: taffy::LengthPercentage::length(70.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(section1), section1_style).unwrap();

            // Small indeterminate spinner
            let small_spinner = Spinner::indeterminate()
                .size(24.0)
                .color(Color::rgb(0.3, 0.6, 1.0));

            let small_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(50.0).into(),
                    top: taffy::LengthPercentage::length(110.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(small_spinner), small_style).unwrap();

            let small_label = Label::new("Small (24px)")
                .font_size(12.0)
                .text_color(Color::rgba(0.7, 0.7, 0.7, 1.0));

            let small_label_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(85.0).into(),
                    top: taffy::LengthPercentage::length(115.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(small_label), small_label_style).unwrap();

            // Medium indeterminate spinner
            let medium_spinner = Spinner::indeterminate()
                .size(40.0)
                .color(Color::rgb(0.8, 0.3, 0.6));

            let medium_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(250.0).into(),
                    top: taffy::LengthPercentage::length(110.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(medium_spinner), medium_style).unwrap();

            let medium_label = Label::new("Medium (40px)")
                .font_size(12.0)
                .text_color(Color::rgba(0.7, 0.7, 0.7, 1.0));

            let medium_label_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(305.0).into(),
                    top: taffy::LengthPercentage::length(120.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(medium_label), medium_label_style).unwrap();

            // Large indeterminate spinner
            let large_spinner = Spinner::indeterminate()
                .size(64.0)
                .color(Color::rgb(0.3, 0.8, 0.4))
                .speed(4.0 * std::f32::consts::PI); // Faster rotation

            let large_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(480.0).into(),
                    top: taffy::LengthPercentage::length(100.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(large_spinner), large_style).unwrap();

            let large_label = Label::new("Large (64px, faster)")
                .font_size(12.0)
                .text_color(Color::rgba(0.7, 0.7, 0.7, 1.0));

            let large_label_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(560.0).into(),
                    top: taffy::LengthPercentage::length(120.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(large_label), large_label_style).unwrap();

            // ================================================================
            // Section 2: Determinate Spinners (Static Progress)
            // ================================================================

            let section2 = Label::new("Determinate Mode (Static Progress Display)")
                .font_size(18.0)
                .text_color(Color::rgba(0.8, 0.9, 1.0, 1.0));

            let section2_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(30.0).into(),
                    top: taffy::LengthPercentage::length(210.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(section2), section2_style).unwrap();

            // Progress spinner at 25%
            let progress_25 = Spinner::determinate(0.25)
                .size(60.0)
                .color(Color::rgb(1.0, 0.5, 0.2));

            let p25_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(80.0).into(),
                    top: taffy::LengthPercentage::length(260.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(progress_25), p25_style).unwrap();

            // Progress spinner at 50%
            let progress_50 = Spinner::determinate(0.50)
                .size(60.0)
                .color(Color::rgb(0.3, 0.7, 1.0));

            let p50_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(200.0).into(),
                    top: taffy::LengthPercentage::length(260.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(progress_50), p50_style).unwrap();

            // Progress spinner at 75%
            let progress_75 = Spinner::determinate(0.75)
                .size(60.0)
                .color(Color::rgb(0.7, 0.3, 0.9));

            let p75_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(320.0).into(),
                    top: taffy::LengthPercentage::length(260.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(progress_75), p75_style).unwrap();

            // Progress spinner at 100%
            let progress_100 = Spinner::determinate(1.0)
                .size(60.0)
                .color(Color::rgb(0.3, 0.9, 0.3));

            let p100_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(440.0).into(),
                    top: taffy::LengthPercentage::length(260.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(progress_100), p100_style).unwrap();

            // ================================================================
            // Section 3: Different Progress Values
            // ================================================================

            let section3 = Label::new("Various Progress Values")
                .font_size(18.0)
                .text_color(Color::rgba(0.8, 0.9, 1.0, 1.0));

            let section3_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(30.0).into(),
                    top: taffy::LengthPercentage::length(370.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(section3), section3_style).unwrap();

            // 0% progress
            let prog_0 = Spinner::determinate(0.0)
                .size(70.0)
                .color(Color::rgb(0.6, 0.6, 0.6))
                .stroke_width(5.0)
                .show_percentage(true)
                .font_size(18.0);

            let prog_0_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(80.0).into(),
                    top: taffy::LengthPercentage::length(420.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(prog_0), prog_0_style).unwrap();

            // 33% progress
            let prog_33 = Spinner::determinate(0.33)
                .size(70.0)
                .color(Color::rgb(1.0, 0.6, 0.0))
                .stroke_width(5.0)
                .show_percentage(true)
                .font_size(18.0);

            let prog_33_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(220.0).into(),
                    top: taffy::LengthPercentage::length(420.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(prog_33), prog_33_style).unwrap();

            // 67% progress
            let prog_67 = Spinner::determinate(0.67)
                .size(70.0)
                .color(Color::rgb(0.3, 0.7, 1.0))
                .stroke_width(5.0)
                .show_percentage(true)
                .font_size(18.0);

            let prog_67_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(360.0).into(),
                    top: taffy::LengthPercentage::length(420.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(prog_67), prog_67_style).unwrap();

            // 100% progress
            let prog_100_large = Spinner::determinate(1.0)
                .size(70.0)
                .color(Color::rgb(0.2, 0.9, 0.2))
                .stroke_width(5.0)
                .show_percentage(true)
                .font_size(18.0);

            let prog_100_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(500.0).into(),
                    top: taffy::LengthPercentage::length(420.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(prog_100_large), prog_100_style).unwrap();

            // ================================================================
            // Section 4: Custom Styling
            // ================================================================

            let section4 = Label::new("Custom Styling")
                .font_size(18.0)
                .text_color(Color::rgba(0.8, 0.9, 1.0, 1.0));

            let section4_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(30.0).into(),
                    top: taffy::LengthPercentage::length(610.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(section4), section4_style).unwrap();

            // Thin stroke spinner
            let thin_spinner = Spinner::determinate(0.6)
                .size(50.0)
                .stroke_width(2.0)
                .color(Color::rgb(1.0, 0.8, 0.2))
                .background_color(None); // No background track

            let thin_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(100.0).into(),
                    top: taffy::LengthPercentage::length(660.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(thin_spinner), thin_style).unwrap();

            let thin_label = Label::new("Thin stroke, no BG")
                .font_size(11.0)
                .text_color(Color::rgba(0.6, 0.6, 0.6, 1.0));

            let thin_label_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(80.0).into(),
                    top: taffy::LengthPercentage::length(720.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(thin_label), thin_label_style).unwrap();

            // Thick stroke spinner
            let thick_spinner = Spinner::determinate(0.85)
                .size(50.0)
                .stroke_width(8.0)
                .color(Color::rgb(0.9, 0.3, 0.5))
                .show_percentage(false);

            let thick_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(250.0).into(),
                    top: taffy::LengthPercentage::length(660.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(thick_spinner), thick_style).unwrap();

            let thick_label = Label::new("Thick stroke, no %")
                .font_size(11.0)
                .text_color(Color::rgba(0.6, 0.6, 0.6, 1.0));

            let thick_label_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(230.0).into(),
                    top: taffy::LengthPercentage::length(720.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(thick_label), thick_label_style).unwrap();

            // Custom colors spinner
            let custom_spinner = Spinner::indeterminate()
                .size(50.0)
                .color(Color::rgb(1.0, 0.4, 0.0))
                .background_color(Some(Color::rgba(1.0, 0.4, 0.0, 0.15)))
                .speed(6.0 * std::f32::consts::PI); // Very fast

            let custom_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(400.0).into(),
                    top: taffy::LengthPercentage::length(660.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(custom_spinner), custom_style).unwrap();

            let custom_label = Label::new("Fast + custom colors")
                .font_size(11.0)
                .text_color(Color::rgba(0.6, 0.6, 0.6, 1.0));

            let custom_label_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(370.0).into(),
                    top: taffy::LengthPercentage::length(720.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(custom_label), custom_label_style).unwrap();

            // Info message
            let info = Label::new("✨ The indeterminate spinners (top row) use continuous animation via update()")
                .font_size(11.0)
                .text_color(Color::rgba(0.7, 0.8, 0.9, 1.0));

            let info_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(60.0).into(),
                    top: taffy::LengthPercentage::length(760.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(info), info_style).unwrap();
        });
    });
}
