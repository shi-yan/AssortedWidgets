//! Phase 1, 2 & 3 Visual Test - Z-Ordering, Clipping, Shadows & Gradients
//!
//! This demonstrates Phase 1, 2, and 3 implementation with actual rendering:
//! 1. Z-ordering with explicit layers (SHADOW, NORMAL, OVERLAY)
//! 2. Rounded rectangle clipping (shader-based SDF)
//! 3. Analytical soft drop shadows
//! 4. Linear and radial gradients (Phase 3)
//! 5. Batched rendering with anti-aliasing

use assorted_widgets::paint::{
    Border, Brush, Color, ColorStop, CornerRadius, LinearGradient, RadialGradient, ShapeStyle,
    Shadow, PrimitiveBatcher, layers,
};
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::{Application, Widget};

/// Phase 1 visual test widget
struct Phase1VisualTest {
    id: WidgetId,
    bounds: Rect,
}

impl Phase1VisualTest {
    fn new(id: WidgetId, bounds: Rect) -> Self {
        Self { id, bounds }
    }
}

impl Widget for Phase1VisualTest {
    assorted_widgets::impl_widget_essentials!();

    fn paint(&self, ctx: &mut assorted_widgets::paint::PaintContext) {
        // Background
        ctx.draw_styled_rect(
            self.bounds,
            ShapeStyle::solid(Color::rgb(0.12, 0.12, 0.15)),
        );

        // Helper to create Rect with Point + Size
        let rect = |x: f64, y: f64, w: f64, h: f64| {
            Rect::new(Point::new(x, y), Size::new(w, h))
        };

        // ========================================
        // Test 1: Z-Ordering (Left Section)
        // ========================================

        // Title
        ctx.draw_styled_rect(
            rect(30.0, 20.0, 280.0, 50.0),
            ShapeStyle::rounded(Color::rgb(0.25, 0.25, 0.30), 8.0),
        );

        // Red rectangle at SHADOW layer (z=-100) - renders BEHIND
        // Has a soft drop shadow to demonstrate Phase 2
        let mut batcher = PrimitiveBatcher::new();
        batcher.draw_rect_z(
            rect(50.0, 100.0, 180.0, 180.0),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.9, 0.2, 0.2)),
                corner_radius: CornerRadius::uniform(20.0),
                border: Some(Border::new(Color::rgb(0.7, 0.1, 0.1), 3.0)),
                shadow: Some(Shadow::new(
                    Color::rgba(0.0, 0.0, 0.0, 0.4),
                    (4.0, 6.0),
                    15.0,
                )),
            },
            layers::SHADOW,
        );

        // Blue rectangle at NORMAL layer (z=0) - renders MIDDLE
        // Larger shadow offset to show it's "elevated"
        batcher.draw_rect_z(
            rect(100.0, 150.0, 180.0, 180.0),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.2, 0.4, 0.9)),
                corner_radius: CornerRadius::uniform(20.0),
                border: Some(Border::new(Color::rgb(0.1, 0.2, 0.7), 3.0)),
                shadow: Some(Shadow::new(
                    Color::rgba(0.0, 0.0, 0.0, 0.5),
                    (6.0, 8.0),
                    20.0,
                )),
            },
            layers::NORMAL,
        );

        // Green rectangle at OVERLAY layer (z=1000) - renders ON TOP
        // Even larger shadow to show highest elevation
        batcher.draw_rect_z(
            rect(150.0, 200.0, 180.0, 180.0),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.2, 0.9, 0.4)),
                corner_radius: CornerRadius::uniform(20.0),
                border: Some(Border::new(Color::rgb(0.1, 0.7, 0.2), 3.0)),
                shadow: Some(Shadow::new(
                    Color::rgba(0.0, 0.0, 0.0, 0.6),
                    (8.0, 12.0),
                    25.0,
                )),
            },
            layers::OVERLAY,
        );

        // Sort and render
        batcher.sort_commands();

        // Transfer sorted commands to PaintContext
        for cmd in batcher.commands() {
            if let assorted_widgets::paint::DrawCommand::Rect { rect, style, .. } = cmd {
                ctx.draw_styled_rect(*rect, style.clone());
            }
        }

        // ========================================
        // Test 2: Rounded Clipping (Right Section)
        // ========================================

        // Title
        ctx.draw_styled_rect(
            rect(370.0, 20.0, 380.0, 50.0),
            ShapeStyle::rounded(Color::rgb(0.25, 0.25, 0.30), 8.0),
        );

        // White background (unclipped)
        ctx.draw_styled_rect(
            rect(400.0, 100.0, 320.0, 280.0),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.95, 0.95, 0.98)),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.7, 0.7, 0.8), 2.0)),
                shadow: None,
            },
        );

        // Magenta pattern (will be clipped by shader)
        // NOTE: In Phase 1, clipping is integrated into the shader
        // This demonstrates the rendering, actual clipping will work when
        // we integrate ClipStack into the render pipeline

        // For now, we'll draw a magenta rect that SHOULD be clipped
        let magenta = Color::rgb(0.9, 0.3, 0.9);
        ctx.draw_styled_rect(
            rect(420.0, 120.0, 280.0, 240.0),
            ShapeStyle::rounded(magenta, 30.0)
                .with_border(Border::new(Color::rgb(0.7, 0.1, 0.7), 2.0)),
        );

        // Cyan overlay (also should be clipped to inner region)
        let cyan = Color::rgba(0.2, 0.9, 0.9, 0.6);
        ctx.draw_styled_rect(
            rect(460.0, 180.0, 200.0, 120.0),
            ShapeStyle::rounded(cyan, 20.0),
        );

        // ========================================
        // Test 3: Label Overlays (uses z-ordering)
        // ========================================

        // Labels rendered on top using OVERLAY layer
        let label_style = ShapeStyle {
            fill: Brush::Solid(Color::rgba(0.1, 0.1, 0.1, 0.85)),
            corner_radius: CornerRadius::uniform(6.0),
            border: Some(Border::new(Color::rgb(0.3, 0.3, 0.35), 1.0)),
            shadow: None,
        };

        ctx.draw_styled_rect(rect(55.0, 105.0, 60.0, 24.0), label_style.clone());
        ctx.draw_styled_rect(rect(105.0, 155.0, 60.0, 24.0), label_style.clone());
        ctx.draw_styled_rect(rect(155.0, 205.0, 60.0, 24.0), label_style);

        // ========================================
        // Test 4: Gradients (Phase 3)
        // ========================================

        // Gradient panel with linear gradient
        ctx.draw_styled_rect(
            rect(30.0, 390.0, 240.0, 70.0),
            ShapeStyle {
                fill: Brush::LinearGradient(LinearGradient::new(
                    Point::new(0.0, 0.5),
                    Point::new(1.0, 0.5),
                    vec![
                        ColorStop::new(0.0, Color::rgb(0.3, 0.5, 0.9)),
                        ColorStop::new(0.5, Color::rgb(0.6, 0.3, 0.9)),
                        ColorStop::new(1.0, Color::rgb(0.9, 0.3, 0.6)),
                    ],
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.2, 0.2, 0.3), 2.0)),
                shadow: Some(Shadow::new(
                    Color::rgba(0.0, 0.0, 0.0, 0.3),
                    (3.0, 4.0),
                    10.0,
                )),
            },
        );

        // Gradient panel with radial gradient
        ctx.draw_styled_rect(
            rect(290.0, 390.0, 240.0, 70.0),
            ShapeStyle {
                fill: Brush::RadialGradient(RadialGradient::new(
                    Point::new(0.5, 0.5),
                    0.5,
                    vec![
                        ColorStop::new(0.0, Color::rgb(1.0, 0.9, 0.3)),
                        ColorStop::new(0.7, Color::rgb(0.9, 0.5, 0.2)),
                        ColorStop::new(1.0, Color::rgb(0.6, 0.2, 0.2)),
                    ],
                )),
                corner_radius: CornerRadius::uniform(12.0),
                border: Some(Border::new(Color::rgb(0.3, 0.2, 0.2), 2.0)),
                shadow: Some(Shadow::new(
                    Color::rgba(0.0, 0.0, 0.0, 0.3),
                    (3.0, 4.0),
                    10.0,
                )),
            },
        );

        // Footer info
        ctx.draw_styled_rect(
            rect(30.0, 480.0, 720.0, 40.0),
            ShapeStyle::rounded(Color::rgb(0.18, 0.18, 0.22), 8.0)
                .with_border(Border::new(Color::rgb(0.3, 0.3, 0.35), 1.0)),
        );
    }
}

fn main() {
    println!("Phase 1, 2 & 3 Visual Test - Z-Ordering, Clipping, Shadows & Gradients");
    println!("========================================================================");
    println!();
    println!("This test showcases:");
    println!("  • Phase 1: Z-ordering with explicit layers (SHADOW, NORMAL, OVERLAY)");
    println!("  • Phase 1: Rounded rectangles with SDF anti-aliasing");
    println!("  • Phase 1: Shader-based clipping (foundation)");
    println!("  • Phase 2: Analytical soft drop shadows");
    println!("  • Phase 3: Linear and radial gradients with multi-stop support");
    println!();

    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Phase 1, 2 & 3 - Z-Ordering, Clipping, Shadows & Gradients", 800.0, 540.0, |window| {
                // Create the test widget
                let demo = Phase1VisualTest::new(
                    WidgetId::new(1),
                    Rect::new(Point::new(0.0, 0.0), Size::new(800.0, 540.0)),
                );

                // Use add_to_root to properly register in all internal systems
                use assorted_widgets::layout::{Style, Display};
                window.add_to_root(
                    Box::new(demo),
                    Style {
                        display: Display::Block,
                        size: taffy::Size {
                            width: taffy::Dimension::length(800.0),
                            height: taffy::Dimension::length(540.0),
                        },
                        ..Default::default()
                    },
                ).expect("Failed to add root widget");

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
