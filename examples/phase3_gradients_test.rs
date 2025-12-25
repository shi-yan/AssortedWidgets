//! Phase 3 Visual Test - Gradient Support
//!
//! This demonstrates Phase 3 implementation with gradient rendering:
//! 1. Linear gradients (vertical, horizontal, diagonal)
//! 2. Radial gradients (centered and custom)
//! 3. Multi-stop gradients (2-8 color stops)
//! 4. Gradient with rounded corners and borders

use assorted_widgets::paint::{
    Border, Brush, Color, ColorStop, CornerRadius, LinearGradient, RadialGradient, ShapeStyle,
};
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::{Application, Widget};

/// Phase 3 visual test widget
struct Phase3GradientsTest {
    id: WidgetId,
    bounds: Rect,
}

impl Phase3GradientsTest {
    fn new(id: WidgetId, bounds: Rect) -> Self {
        Self { id, bounds }
    }
}

impl Widget for Phase3GradientsTest {
    assorted_widgets::impl_widget_essentials!();

    fn paint(&self, ctx: &mut assorted_widgets::paint::PaintContext) {
        // Background
        ctx.draw_styled_rect(
            self.bounds,
            ShapeStyle::solid(Color::rgb(0.12, 0.12, 0.15)),
        );

        // Helper to create Rect with Point + Size
        let rect = |x: f64, y: f64, w: f64, h: f64| Rect::new(Point::new(x, y), Size::new(w, h));

        // ========================================
        // Section 1: Linear Gradients
        // ========================================

        // Title
        ctx.draw_styled_rect(
            rect(30.0, 20.0, 360.0, 40.0),
            ShapeStyle::rounded(Color::rgb(0.25, 0.25, 0.30), 6.0),
        );

        // Vertical gradient (top to bottom)
        ctx.draw_styled_rect(
            rect(30.0, 80.0, 110.0, 160.0),
            ShapeStyle {
                fill: Brush::LinearGradient(LinearGradient::vertical(
                    Color::rgb(0.9, 0.3, 0.3), // Red top
                    Color::rgb(0.3, 0.3, 0.9), // Blue bottom
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.25), 2.0)),
                shadow: None,
            },
        );

        // Horizontal gradient (left to right)
        ctx.draw_styled_rect(
            rect(150.0, 80.0, 110.0, 160.0),
            ShapeStyle {
                fill: Brush::LinearGradient(LinearGradient::horizontal(
                    Color::rgb(0.3, 0.9, 0.3), // Green left
                    Color::rgb(0.9, 0.9, 0.3), // Yellow right
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.25), 2.0)),
                shadow: None,
            },
        );

        // Diagonal gradient (top-left to bottom-right)
        ctx.draw_styled_rect(
            rect(270.0, 80.0, 110.0, 160.0),
            ShapeStyle {
                fill: Brush::LinearGradient(LinearGradient::diagonal(
                    Color::rgb(0.9, 0.3, 0.9), // Magenta top-left
                    Color::rgb(0.3, 0.9, 0.9), // Cyan bottom-right
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.25), 2.0)),
                shadow: None,
            },
        );

        // ========================================
        // Section 2: Radial Gradients
        // ========================================

        // Title
        ctx.draw_styled_rect(
            rect(410.0, 20.0, 360.0, 40.0),
            ShapeStyle::rounded(Color::rgb(0.25, 0.25, 0.30), 6.0),
        );

        // Centered radial gradient
        ctx.draw_styled_rect(
            rect(410.0, 80.0, 110.0, 160.0),
            ShapeStyle {
                fill: Brush::RadialGradient(RadialGradient::centered(
                    Color::rgb(1.0, 1.0, 0.3), // Yellow center
                    Color::rgb(0.9, 0.3, 0.3), // Red outer
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.25), 2.0)),
                shadow: None,
            },
        );

        // Off-center radial gradient
        ctx.draw_styled_rect(
            rect(530.0, 80.0, 110.0, 160.0),
            ShapeStyle {
                fill: Brush::RadialGradient(RadialGradient::new(
                    Point::new(0.3, 0.3), // Top-left-ish center
                    0.7,                  // Large radius
                    vec![
                        ColorStop::new(0.0, Color::rgb(1.0, 1.0, 1.0)), // White center
                        ColorStop::new(1.0, Color::rgb(0.2, 0.4, 0.8)), // Blue outer
                    ],
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.25), 2.0)),
                shadow: None,
            },
        );

        // Small radius radial gradient (spotlight effect)
        ctx.draw_styled_rect(
            rect(650.0, 80.0, 110.0, 160.0),
            ShapeStyle {
                fill: Brush::RadialGradient(RadialGradient::new(
                    Point::new(0.5, 0.5),
                    0.3, // Small radius
                    vec![
                        ColorStop::new(0.0, Color::rgb(1.0, 0.8, 0.3)), // Warm center
                        ColorStop::new(1.0, Color::rgb(0.1, 0.1, 0.15)), // Dark outer
                    ],
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.25), 2.0)),
                shadow: None,
            },
        );

        // ========================================
        // Section 3: Multi-Stop Gradients
        // ========================================

        // Title
        ctx.draw_styled_rect(
            rect(30.0, 260.0, 740.0, 40.0),
            ShapeStyle::rounded(Color::rgb(0.25, 0.25, 0.30), 6.0),
        );

        // Rainbow gradient (7 stops)
        ctx.draw_styled_rect(
            rect(30.0, 320.0, 350.0, 140.0),
            ShapeStyle {
                fill: Brush::LinearGradient(LinearGradient::new(
                    Point::new(0.0, 0.5),
                    Point::new(1.0, 0.5),
                    vec![
                        ColorStop::new(0.0, Color::rgb(0.9, 0.2, 0.2)),      // Red
                        ColorStop::new(0.16, Color::rgb(0.9, 0.6, 0.2)),     // Orange
                        ColorStop::new(0.33, Color::rgb(0.9, 0.9, 0.2)),     // Yellow
                        ColorStop::new(0.5, Color::rgb(0.2, 0.9, 0.2)),      // Green
                        ColorStop::new(0.66, Color::rgb(0.2, 0.6, 0.9)),     // Blue
                        ColorStop::new(0.83, Color::rgb(0.5, 0.2, 0.9)),     // Indigo
                        ColorStop::new(1.0, Color::rgb(0.9, 0.2, 0.9)),      // Violet
                    ],
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.25), 3.0)),
                shadow: None,
            },
        );

        // Sunset gradient (4 stops with warm colors)
        ctx.draw_styled_rect(
            rect(410.0, 320.0, 350.0, 140.0),
            ShapeStyle {
                fill: Brush::LinearGradient(LinearGradient::new(
                    Point::new(0.5, 0.0),
                    Point::new(0.5, 1.0),
                    vec![
                        ColorStop::new(0.0, Color::rgb(0.1, 0.1, 0.2)),   // Dark blue top
                        ColorStop::new(0.3, Color::rgb(0.9, 0.4, 0.2)),   // Orange
                        ColorStop::new(0.6, Color::rgb(0.95, 0.7, 0.3)),  // Golden
                        ColorStop::new(1.0, Color::rgb(1.0, 0.95, 0.8)),  // Pale yellow
                    ],
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.25), 3.0)),
                shadow: None,
            },
        );

        // Footer info
        ctx.draw_styled_rect(
            rect(30.0, 480.0, 740.0, 40.0),
            ShapeStyle::rounded(Color::rgb(0.18, 0.18, 0.22), 8.0)
                .with_border(Border::new(Color::rgb(0.3, 0.3, 0.35), 1.0)),
        );
    }
}

fn main() {
    println!("Phase 3 Visual Test - Gradient Support");
    println!("=======================================");
    println!();
    println!("This test showcases:");
    println!("  • Linear gradients (vertical, horizontal, diagonal)");
    println!("  • Radial gradients (centered, off-center, spotlight)");
    println!("  • Multi-stop gradients (rainbow, sunset)");
    println!("  • Gradients with rounded corners and borders");
    println!();

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Phase 3 - Gradient Support", 800.0, 550.0, |window| {
                // Create the test widget
                let demo = Phase3GradientsTest::new(
                    WidgetId::new(1),
                    Rect::new(Point::new(0.0, 0.0), Size::new(800.0, 550.0)),
                );

                // Use add_root to properly register in all internal systems
                use assorted_widgets::layout::{Display, Style};
                window
                    .add_root(
                        Box::new(demo),
                        Style {
                            display: Display::Block,
                            size: taffy::Size {
                                width: taffy::Dimension::length(800.0),
                                height: taffy::Dimension::length(550.0),
                            },
                            ..Default::default()
                        },
                    )
                    .expect("Failed to add root widget");

                println!("✓ Window created");
                println!("✓ Demo element added");
                println!();
                println!("Close the window to exit.");
            });
        });
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This example currently only runs on macOS");
    }
}
