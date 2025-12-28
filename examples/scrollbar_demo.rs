//! ScrollBar demo - demonstrates horizontal and vertical scrollbars
//!
//! This example shows:
//! - Vertical scrollbar (right side) for scrolling 0-100 lines
//! - Horizontal scrollbar (bottom) for scrolling 0-200 columns
//! - Value change callbacks that print to console
//! - Visual feedback boxes showing scroll position

use assorted_widgets::Application;
use assorted_widgets::widgets::{ScrollBar, Label, Padding};
use assorted_widgets::paint::primitives::Color;

fn main() {
    println!("📜 ScrollBar Demo");
    println!("=================");
    println!("• Vertical scrollbar (right): 0-100 lines, page size 10");
    println!("• Horizontal scrollbar (bottom): 0-200 columns, page size 20");
    println!("• Click on slider to drag");
    println!("• Click on track to jump");
    println!("");

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("ScrollBar Demo", 700.0, 600.0, |window| {
                // The root container is already configured with 100% size by default
                // No need to call set_root_layout() unless you want to customize it

                // Title label
                 let title = Label::new("ScrollBar Demo - Drag the sliders!")
                    .font_size(24.0)
                    .text_color(Color::WHITE)
                    .padding(Padding::uniform(16.0));

                let title_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(700.0),
                        height: taffy::Dimension::length(60.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(0.0).into(),
                        top: taffy::LengthPercentage::length(0.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(title), title_style)
                    .expect("Failed to add title");

                // Content area background (to show scrollbars better)
                let content_bg = Label::new("")
                    .bg_color(Color::rgb(0.15, 0.15, 0.18))
                    .padding(Padding::uniform(0.0));

                let content_bg_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(640.0),
                        height: taffy::Dimension::length(440.0),
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

                window.add_to_root(Box::new(content_bg), content_bg_style)
                    .expect("Failed to add content background");

                // Vertical scrollbar info
                 let v_info = Label::new("Vertical Scrollbar →\n(0-100, page: 10)")
                    .font_size(14.0)
                    .text_color(Color::rgb(0.8, 0.9, 1.0))
                    .padding(Padding::uniform(12.0));

                let v_info_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(200.0),
                        height: taffy::Dimension::length(60.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(440.0).into(),
                        top: taffy::LengthPercentage::length(100.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(v_info), v_info_style)
                    .expect("Failed to add v_info");

                // Vertical value label (will update dynamically via signal/slot)
                 let v_value_label = Label::new("Vertical: 0/100 (0%)")
                    .font_size(20.0)
                    .text_color(Color::rgba(0.3, 0.7, 1.0, 1.0))  // Match scrollbar color
                    .padding(Padding::uniform(12.0));

                let v_value_label_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(250.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(200.0).into(),
                        top: taffy::LengthPercentage::length(180.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                let v_label_id = window.add_to_root(Box::new(v_value_label), v_value_label_style)
                    .expect("Failed to add v_value_label");

                // Vertical scrollbar (right side) - MUCH MORE VISIBLE
                let vscrollbar = ScrollBar::vertical(0, 100, 10)
                    .width(20.0)  // Wider for visibility
                    .slider_color(Color::rgba(0.3, 0.7, 1.0, 0.9))  // Bright blue
                    .slider_hover_color(Color::rgba(0.4, 0.8, 1.0, 1.0))  // Brighter on hover
                    .slider_drag_color(Color::rgba(0.2, 0.6, 0.9, 1.0))  // Darker when dragging
                    .track_color(Color::rgba(0.1, 0.1, 0.12, 0.8))  // Dark track
                    .on_value_changed(|value| {
                        println!("📐 Vertical: {}/100 ({}%)", value, value);
                    });

                let vscrollbar_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(20.0),
                        height: taffy::Dimension::length(420.0),  // Tall
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentageAuto::auto(),
                        top: taffy::LengthPercentage::length(90.0).into(),
                        right: taffy::LengthPercentage::length(30.0).into(),  // 30px from right
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                let v_scrollbar_id = window.add_to_root(Box::new(vscrollbar), vscrollbar_style)
                    .expect("Failed to add vertical scrollbar");

                // Connect vertical scrollbar to its label
                window.connect(v_scrollbar_id, "value_changed".to_string(), v_label_id);
 
                // Horizontal scrollbar info
                let h_info = Label::new("↓ Horizontal Scrollbar (0-200, page: 20)")
                    .font_size(14.0)
                    .text_color(Color::rgb(1.0, 0.9, 0.8))
                    .padding(Padding::uniform(12.0));

                let h_info_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(400.0),
                        height: taffy::Dimension::length(40.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(150.0).into(),
                        top: taffy::LengthPercentage::length(480.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(h_info), h_info_style)
                    .expect("Failed to add h_info");

                // Horizontal value label (will update dynamically via signal/slot)
                let h_value_label = Label::new("Horizontal: 0/200 (0%)")
                    .font_size(20.0)
                    .text_color(Color::rgba(1.0, 0.6, 0.3, 1.0))  // Match scrollbar color
                    .padding(Padding::uniform(12.0));

                let h_value_label_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(300.0),
                        height: taffy::Dimension::length(50.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(200.0).into(),
                        top: taffy::LengthPercentage::length(400.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                let h_label_id = window.add_to_root(Box::new(h_value_label), h_value_label_style)
                    .expect("Failed to add h_value_label");

                // Horizontal scrollbar (bottom) - BRIGHT AND VISIBLE
                let hscrollbar = ScrollBar::horizontal(0, 200, 20)
                    .width(20.0)  // Thicker
                    .slider_color(Color::rgba(1.0, 0.6, 0.3, 0.9))  // Bright orange
                    .slider_hover_color(Color::rgba(1.0, 0.7, 0.4, 1.0))  // Brighter
                    .slider_drag_color(Color::rgba(0.9, 0.5, 0.2, 1.0))  // Darker
                    .track_color(Color::rgba(0.1, 0.1, 0.12, 0.8))  // Dark track
                    .on_value_changed(|value| {
                        println!("📏 Horizontal: {}/200 ({}%)", value, (value * 100) / 200);
                    });

                let hscrollbar_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(600.0),  // Wide
                        height: taffy::Dimension::length(20.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(50.0).into(),
                        top: taffy::LengthPercentage::length(530.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                let h_scrollbar_id = window.add_to_root(Box::new(hscrollbar), hscrollbar_style)
                    .expect("Failed to add horizontal scrollbar");

                // Connect horizontal scrollbar to its label
                window.connect(h_scrollbar_id, "value_changed".to_string(), h_label_id);

                // Instructions
                let instructions = Label::new("✨ Labels update dynamically via signal/slot connections!\nWatch values change as you drag the scrollbars.")
                    .font_size(14.0)
                    .text_color(Color::rgb(0.7, 1.0, 0.7))
                    .padding(Padding::uniform(8.0));

                let instructions_style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(500.0),
                        height: taffy::Dimension::length(30.0),
                    },
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        left: taffy::LengthPercentage::length(100.0).into(),
                        top: taffy::LengthPercentage::length(560.0).into(),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(instructions), instructions_style)
                    .expect("Failed to add instructions");

                println!("✓ ScrollBar demo ready!");
                println!("• BLUE scrollbar on the right (vertical)");
                println!("• ORANGE scrollbar at the bottom (horizontal)");
                println!("• Labels update dynamically via signal/slot connections!");
                println!("• Drag the sliders to see value changes in real-time");
                println!("• Click on the track to jump to a position");
            });
        });
    }
}
