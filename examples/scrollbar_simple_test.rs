//! Minimal scrollbar test - single scrollbar to debug rendering

use assorted_widgets::Application;
use assorted_widgets::widgets::ScrollBar;
use assorted_widgets::paint::primitives::Color;

fn main() {
    println!("🔍 Minimal ScrollBar Test");
    println!("=========================");

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Scrollbar Test", 400.0, 300.0, |window| {
                window.set_root_layout(taffy::Style::default());

                // Single bright red scrollbar in the middle - IMPOSSIBLE TO MISS
                let scrollbar = ScrollBar::horizontal(0, 100, 10)
                    .width(30.0)  // VERY thick
                    .slider_color(Color::rgb(1.0, 0.0, 0.0))  // BRIGHT RED
                    .track_color(Color::rgb(0.0, 0.0, 1.0))   // BRIGHT BLUE track
                    .on_value_changed(|value| {
                        println!("Value: {}", value);
                    });

                let style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(300.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(50.0).into(),
                        top: taffy::LengthPercentage::length(135.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                let id = window.add_to_root(Box::new(scrollbar), style)
                    .expect("Failed to add scrollbar");

                println!("✓ Scrollbar added with ID: {:?}", id);
                println!("✓ Position: 50x135, Size: 300x30");
                println!("✓ Should be BRIGHT RED slider on BLUE track");
            });
        });
    }
}
