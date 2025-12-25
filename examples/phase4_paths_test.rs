//! Phase 4 Visual Test - Lines and Paths
//!
//! This demonstrates Phase 4 implementation with vector graphics:
//! 1. Line segments with different cap and join styles
//! 2. Custom paths with bezier curves
//! 3. Filled and stroked paths
//! 4. Vector icons (star, heart)

use assorted_widgets::paint::{
    Color, LineCap, LineJoin, Path, Stroke,
};
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::{Application, Widget};

/// Phase 4 visual test widget
struct Phase4PathsTest {
    id: WidgetId,
    bounds: Rect,
}

impl Phase4PathsTest {
    fn new(id: WidgetId, bounds: Rect) -> Self {
        Self { id, bounds }
    }
}

impl Widget for Phase4PathsTest {
    // ✅ This single line replaces ~40 lines of boilerplate!
    assorted_widgets::impl_widget_essentials!();

    fn paint(&self, ctx: &mut assorted_widgets::paint::PaintContext) {
        use assorted_widgets::paint::ShapeStyle;

        // Background
        ctx.draw_styled_rect(
            self.bounds,
            ShapeStyle::solid(Color::rgb(0.12, 0.12, 0.15)),
        );

        // ========================================
        // Section 1: Line Caps and Joins
        // ========================================

        // Butt cap
        ctx.draw_line(
            Point::new(50.0, 80.0),
            Point::new(200.0, 80.0),
            Stroke::new(Color::rgb(0.9, 0.4, 0.4), 8.0).with_cap(LineCap::Butt),
        );

        // Round cap
        ctx.draw_line(
            Point::new(50.0, 120.0),
            Point::new(200.0, 120.0),
            Stroke::new(Color::rgb(0.4, 0.9, 0.4), 8.0).with_cap(LineCap::Round),
        );

        // Square cap
        ctx.draw_line(
            Point::new(50.0, 160.0),
            Point::new(200.0, 160.0),
            Stroke::new(Color::rgb(0.4, 0.4, 0.9), 8.0).with_cap(LineCap::Square),
        );

        // Line joins demonstration (zigzag)
        let mut zigzag = Path::new();
        zigzag
            .move_to(Point::new(250.0, 60.0))
            .line_to(Point::new(300.0, 100.0))
            .line_to(Point::new(250.0, 140.0))
            .line_to(Point::new(300.0, 180.0));

        // Miter join
        ctx.stroke_path(
            zigzag.clone(),
            Stroke::new(Color::rgb(0.9, 0.4, 0.9), 6.0).with_join(LineJoin::Miter),
        );

        // Round join
        let mut zigzag2 = Path::new();
        zigzag2
            .move_to(Point::new(320.0, 60.0))
            .line_to(Point::new(370.0, 100.0))
            .line_to(Point::new(320.0, 140.0))
            .line_to(Point::new(370.0, 180.0));

        ctx.stroke_path(
            zigzag2,
            Stroke::new(Color::rgb(0.4, 0.9, 0.9), 6.0).with_join(LineJoin::Round),
        );

        // ========================================
        // Section 2: Bezier Curves
        // ========================================

        // Quadratic bezier curve
        let mut quad_curve = Path::new();
        quad_curve
            .move_to(Point::new(50.0, 250.0))
            .quadratic_to(Point::new(125.0, 200.0), Point::new(200.0, 250.0));

        ctx.stroke_path(
            quad_curve,
            Stroke::new(Color::rgb(0.9, 0.6, 0.3), 4.0)
                .with_cap(LineCap::Round),
        );

        // Cubic bezier curve (S-shape)
        let mut cubic_curve = Path::new();
        cubic_curve
            .move_to(Point::new(50.0, 300.0))
            .cubic_to(
                Point::new(100.0, 250.0),
                Point::new(150.0, 350.0),
                Point::new(200.0, 300.0),
            );

        ctx.stroke_path(
            cubic_curve,
            Stroke::new(Color::rgb(0.6, 0.3, 0.9), 4.0)
                .with_cap(LineCap::Round),
        );

        // ========================================
        // Section 3: Filled Paths
        // ========================================

        // Triangle (filled)
        let mut triangle = Path::new();
        triangle
            .move_to(Point::new(450.0, 80.0))
            .line_to(Point::new(550.0, 80.0))
            .line_to(Point::new(500.0, 160.0))
            .close();

        ctx.fill_path(triangle, Color::rgb(0.9, 0.4, 0.4));

        // Pentagon (filled with stroke)
        let mut pentagon = Path::new();
        let center = Point::new(630.0, 120.0);
        let radius = 50.0;
        for i in 0..5 {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / 5.0 - std::f64::consts::PI / 2.0;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            if i == 0 {
                pentagon.move_to(Point::new(x, y));
            } else {
                pentagon.line_to(Point::new(x, y));
            }
        }
        pentagon.close();

        ctx.draw_path(
            pentagon,
            Some(Color::rgb(0.3, 0.7, 0.9)),
            Some(Stroke::new(Color::rgb(0.1, 0.3, 0.5), 3.0)),
        );

        // ========================================
        // Section 4: Vector Icons
        // ========================================

        // Star icon
        let mut star = Path::new();
        let star_center = Point::new(480.0, 280.0);
        let outer_radius = 40.0;
        let inner_radius = 18.0;
        for i in 0..10 {
            let angle = (i as f64) * std::f64::consts::PI / 5.0 - std::f64::consts::PI / 2.0;
            let radius = if i % 2 == 0 { outer_radius } else { inner_radius };
            let x = star_center.x + radius * angle.cos();
            let y = star_center.y + radius * angle.sin();
            if i == 0 {
                star.move_to(Point::new(x, y));
            } else {
                star.line_to(Point::new(x, y));
            }
        }
        star.close();

        ctx.draw_path(
            star,
            Some(Color::rgb(0.95, 0.8, 0.2)),
            Some(Stroke::new(Color::rgb(0.7, 0.5, 0.0), 2.0)),
        );

        // Heart icon (using cubic beziers)
        let heart_center = Point::new(620.0, 280.0);
        let size = 35.0;

        let mut heart = Path::new();
        heart.move_to(Point::new(heart_center.x, heart_center.y - size * 0.3));

        // Left arc
        heart.cubic_to(
            Point::new(heart_center.x - size * 0.5, heart_center.y - size * 0.8),
            Point::new(heart_center.x - size * 0.8, heart_center.y - size * 0.3),
            Point::new(heart_center.x - size * 0.5, heart_center.y),
        );

        // Bottom point
        heart.line_to(Point::new(heart_center.x, heart_center.y + size * 0.5));

        // Right arc (reverse)
        heart.line_to(Point::new(heart_center.x + size * 0.5, heart_center.y));
        heart.cubic_to(
            Point::new(heart_center.x + size * 0.8, heart_center.y - size * 0.3),
            Point::new(heart_center.x + size * 0.5, heart_center.y - size * 0.8),
            Point::new(heart_center.x, heart_center.y - size * 0.3),
        );

        heart.close();

        ctx.draw_path(
            heart,
            Some(Color::rgb(0.95, 0.3, 0.4)),
            Some(Stroke::new(Color::rgb(0.7, 0.1, 0.2), 2.0)),
        );

        // ========================================
        // Section 5: Complex Path
        // ========================================

        // Sine wave
        let mut sine_wave = Path::new();
        let wave_start_x = 50.0;
        let wave_y = 420.0;
        let wave_length = 650.0;
        let amplitude = 30.0;
        let frequency = 4.0;

        sine_wave.move_to(Point::new(wave_start_x, wave_y));

        for i in 1..=100 {
            let t = (i as f64) / 100.0;
            let x = wave_start_x + wave_length * t;
            let y = wave_y + amplitude * (frequency * std::f64::consts::PI * t).sin();
            sine_wave.line_to(Point::new(x, y));
        }

        ctx.stroke_path(
            sine_wave,
            Stroke::new(Color::rgb(0.4, 0.8, 0.6), 3.0)
                .with_cap(LineCap::Round)
                .with_join(LineJoin::Round),
        );
    }
}

fn main() {
    println!("Phase 4 Visual Test - Lines and Paths");
    println!("======================================");
    println!();
    println!("This test showcases:");
    println!("  • Line segments with different cap styles (Butt, Round, Square)");
    println!("  • Line joins (Miter, Round, Bevel)");
    println!("  • Bezier curves (Quadratic and Cubic)");
    println!("  • Filled paths (Triangle, Pentagon)");
    println!("  • Vector icons (Star, Heart)");
    println!("  • Complex paths (Sine wave)");
    println!();

    #[cfg(target_os = "macos")]
    {
        // ✨ New ergonomic API - no pollster::block_on, no window ID juggling!
        Application::launch(|app| {
            app.spawn_window("Phase 4 - Lines and Paths", 750.0, 480.0, |window| {
                // Create and set the main widget
                let demo = Phase4PathsTest::new(
                    WidgetId::new(1),
                    Rect::new(Point::new(0.0, 0.0), Size::new(750.0, 480.0)),
                );

                window.set_main_widget(demo);

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
