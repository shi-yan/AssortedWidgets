//! Multi-Window Demo
//!
//! Demonstrates the ergonomic multi-window API:
//! - Application::launch() - hides async/pollster details
//! - spawn_window() - creates windows with clean callbacks
//! - set_main_widget() - simple widget setup

use assorted_widgets::paint::{Color, ShapeStyle};
use assorted_widgets::text::TextStyle;
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::{Application, Widget};

/// Simple colored panel widget
struct ColorPanel {
    id: WidgetId,
    bounds: Rect,
    color: Color,
    label: String,
}

impl ColorPanel {
    fn new(id: WidgetId, color: Color, label: &str) -> Self {
        Self {
            id,
            bounds: Rect::new(Point::new(0.0, 0.0), Size::new(400.0, 300.0)),
            color,
            label: label.to_string(),
        }
    }
}

impl Widget for ColorPanel {
    assorted_widgets::impl_widget_essentials!();

    fn paint(&self, ctx: &mut assorted_widgets::paint::PaintContext) {
        // Background
        ctx.draw_styled_rect(self.bounds, ShapeStyle::solid(self.color));

        // Label
        let text_style = TextStyle::new()
            .size(24.0)
            .color(Color::rgba(1.0, 1.0, 1.0, 0.9));
        ctx.draw_text(&self.label, &text_style, Point::new(20.0, 20.0), None);
    }
}

fn main() {
    println!("Multi-Window Demo");
    println!("=================");
    println!();
    println!("This demo creates three windows to showcase the new API:");
    println!("  • Red window (Main)");
    println!("  • Green window (Secondary)");
    println!("  • Blue window (Tertiary)");
    println!();

    #[cfg(target_os = "macos")]
    {
        // ✨ Clean ergonomic API - all window setup in one place!
        Application::launch(|app| {
            // Main window - Red
            app.spawn_window("Main Window", 400.0, 300.0, |window| {
                let panel = ColorPanel::new(
                    WidgetId::new(1),
                    Color::rgb(0.8, 0.2, 0.2),
                    "Main Window (Red)",
                );
                window.set_main_widget(panel);
                println!("✓ Main window created");
            });

            // Secondary window - Green
            app.spawn_window("Secondary Window", 400.0, 300.0, |window| {
                let panel = ColorPanel::new(
                    WidgetId::new(2),
                    Color::rgb(0.2, 0.8, 0.2),
                    "Secondary Window (Green)",
                );
                window.set_main_widget(panel);
                println!("✓ Secondary window created");
            });

            // Tertiary window - Blue
            app.spawn_window("Tertiary Window", 400.0, 300.0, |window| {
                let panel = ColorPanel::new(
                    WidgetId::new(3),
                    Color::rgb(0.2, 0.2, 0.8),
                    "Tertiary Window (Blue)",
                );
                window.set_main_widget(panel);
                println!("✓ Tertiary window created");
            });

            println!();
            println!("Close all windows to exit.");
        });
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This example currently only runs on macOS");
    }
}
