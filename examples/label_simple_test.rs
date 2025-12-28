use assorted_widgets::Application;
use assorted_widgets::widgets::{Label, WrapMode, Padding};
use assorted_widgets::paint::Color;

fn main() {
    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Simple Label Test", 600.0, 400.0, |window| {
                // Test 1: Simple single-line label with explicit size
                let label1 = Label::new("Test 1: Single line with ellipsis")
                    .wrap_mode(WrapMode::SingleLineEllipsis)
                    .bg_color(Color::rgb(0.3, 0.2, 0.3))
                    .text_color(Color::WHITE)
                    .padding(Padding::uniform(12.0));

                let style1 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(400.0),
                        height: taffy::Dimension::auto(),
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

                window.add_to_root(Box::new(label1), style1)
                    .expect("Failed to add label 1");

                // Test 2: Multi-line label with word wrap
                let label2 = Label::new("Test 2: This is a longer text with word wrapping enabled to test multi-line layout")
                    .wrap_mode(WrapMode::WrapWord)
                    .bg_color(Color::rgb(0.2, 0.3, 0.2))
                    .text_color(Color::WHITE)
                    .padding(Padding::uniform(12.0));

                let style2 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(300.0),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(80.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label2), style2)
                    .expect("Failed to add label 2");

                // Test 3: Glyph wrapping
                let label3 = Label::new("Test3:Verylongwordwithnospacestotestwrapanywhere")
                    .wrap_mode(WrapMode::WrapAnywhere)
                    .bg_color(Color::rgb(0.2, 0.2, 0.3))
                    .text_color(Color::WHITE)
                    .padding(Padding::uniform(12.0));

                let style3 = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(250.0),
                        height: taffy::Dimension::auto(),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(20.0).into(),
                        top: taffy::LengthPercentage::length(200.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label3), style3)
                    .expect("Failed to add label 3");

                println!("Simple label test - using absolute positioning");
                println!("This tests if labels work correctly without Container layout");
            });
        });
    }
}
