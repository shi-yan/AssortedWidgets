//! Slider and ProgressBar Example
//!
//! This example demonstrates the Slider and ProgressBar widgets:
//! - Horizontal sliders with labels and value display
//! - Custom value formatting
//! - Progress bars with different styles
//! - Different configurations

use assorted_widgets::Application;
use assorted_widgets::paint::primitives::Color;
use assorted_widgets::widgets::{Label, Slider, ProgressBar};

fn main() {
    println!("🎚️  Slider & ProgressBar Demo");
    println!("=============================");

    Application::launch(|app| {
        app.spawn_window("Slider & ProgressBar Demo", 500.0, 700.0, |window| {
            // Use absolute positioning for simplicity
            window.set_root_layout(taffy::Style::default());

            // Title
            let title = Label::new("Slider & ProgressBar Widgets")
                .font_size(20.0)
                .text_color(Color::rgba(0.9, 0.9, 0.9, 1.0));

            let title_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(20.0).into(),
                    top: taffy::LengthPercentage::length(20.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(title), title_style).unwrap();

            // ================================================================
            // Section 1: Basic Sliders
            // ================================================================

            let section1 = Label::new("Basic Sliders")
                .font_size(16.0)
                .text_color(Color::rgba(0.8, 0.8, 0.8, 1.0));

            let section1_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(20.0).into(),
                    top: taffy::LengthPercentage::length(60.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(section1), section1_style).unwrap();

            // Simple horizontal slider
            let simple_slider = Slider::horizontal(0.0, 100.0)
                .value(50.0)
                .track_height(6.0)
                .on_value_changed(|value| {
                    println!("Simple slider value: {:.1}", value);
                });

            let simple_style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(400.0),
                    height: taffy::Dimension::length(40.0),
                },
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(50.0).into(),
                    top: taffy::LengthPercentage::length(90.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(simple_slider), simple_style).unwrap();

            // Slider with label and value display
            let volume_slider = Slider::horizontal(0.0, 100.0)
                .label("Volume")
                .value(75.0)
                .show_value(true)
                .value_formatter(|v| format!("{:.0}%", v as i32))
                .fill_color(Color::rgb(0.3, 0.7, 0.4))
                .on_value_changed(|value| {
                    println!("Volume: {:.0}%", value);
                });

            let volume_style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(400.0),
                    height: taffy::Dimension::length(60.0),
                },
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(50.0).into(),
                    top: taffy::LengthPercentage::length(140.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(volume_slider), volume_style).unwrap();

            // Slider with step increments
            let rating_slider = Slider::horizontal(0.0, 10.0)
                .label("Rating")
                .value(7.0)
                .step(1.0)
                .show_value(true)
                .value_formatter(|v| format!("{:.0} / 10", v))
                .fill_color(Color::rgb(0.8, 0.6, 0.2))
                .on_value_changed(|value| {
                    println!("Rating: {:.0} / 10", value);
                });

            let rating_style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(400.0),
                    height: taffy::Dimension::length(60.0),
                },
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(50.0).into(),
                    top: taffy::LengthPercentage::length(215.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(rating_slider), rating_style).unwrap();

            // ================================================================
            // Section 2: Progress Bars
            // ================================================================

            let section2 = Label::new("Progress Bars")
                .font_size(16.0)
                .text_color(Color::rgba(0.8, 0.8, 0.8, 1.0));

            let section2_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(20.0).into(),
                    top: taffy::LengthPercentage::length(300.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(section2), section2_style).unwrap();

            // Simple progress bar
            let simple_progress = ProgressBar::horizontal()
                .progress(0.65)
                .bar_height(8.0);

            let progress1_style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(400.0),
                    height: taffy::Dimension::length(20.0),
                },
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(50.0).into(),
                    top: taffy::LengthPercentage::length(330.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(simple_progress), progress1_style).unwrap();

            // Progress bar with label and percentage
            let download_progress = ProgressBar::horizontal()
                .label("Download Progress")
                .progress(0.45)
                .show_percentage(true)
                .fill_color(Color::rgb(0.3, 0.6, 0.8))
                .bar_height(10.0);

            let progress2_style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(400.0),
                    height: taffy::Dimension::length(50.0),
                },
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(50.0).into(),
                    top: taffy::LengthPercentage::length(365.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(download_progress), progress2_style).unwrap();

            // Progress bar with custom text
            let install_progress = ProgressBar::horizontal()
                .label("Installing...")
                .progress(0.82)
                .progress_text(|p| format!("{:.0} MB / 100 MB", p * 100.0))
                .fill_color(Color::rgb(0.5, 0.7, 0.3))
                .bar_height(12.0)
                .corner_radius(6.0);

            let progress3_style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(400.0),
                    height: taffy::Dimension::length(50.0),
                },
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(50.0).into(),
                    top: taffy::LengthPercentage::length(430.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(install_progress), progress3_style).unwrap();

            // ================================================================
            // Info
            // ================================================================

            let info = Label::new("Move the sliders to see value changes in the console!")
                .font_size(12.0)
                .text_color(Color::rgba(0.6, 0.6, 0.6, 1.0));

            let info_style = taffy::Style {
                position: taffy::Position::Absolute,
                inset: taffy::Rect {
                    left: taffy::LengthPercentage::length(50.0).into(),
                    top: taffy::LengthPercentage::length(500.0).into(),
                    right: taffy::LengthPercentageAuto::auto(),
                    bottom: taffy::LengthPercentageAuto::auto(),
                },
                ..Default::default()
            };

            window.add_to_root(Box::new(info), info_style).unwrap();
        });
    });
}
