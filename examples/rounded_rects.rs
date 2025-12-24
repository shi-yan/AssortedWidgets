//! Rounded Rectangles Example - Demonstrating SDF rendering via PaintContext

use assorted_widgets::paint::{Border, Brush, Color, CornerRadius, ShapeStyle};
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::{Application, Element, WindowOptions};

/// Demo element that shows various rounded rectangle styles
struct RoundedRectsDemo {
    id: WidgetId,
    bounds: Rect,
}

impl RoundedRectsDemo {
    fn new(id: WidgetId, bounds: Rect) -> Self {
        Self { id, bounds }
    }
}

impl Element for RoundedRectsDemo {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn on_message(&mut self, _msg: &assorted_widgets::GuiMessage) -> Vec<assorted_widgets::types::DeferredCommand> {
        vec![]
    }

    fn on_event(&mut self, _event: &assorted_widgets::OsEvent) -> Vec<assorted_widgets::types::DeferredCommand> {
        vec![]
    }

    fn set_dirty(&mut self, _dirty: bool) {}

    fn is_dirty(&self) -> bool {
        false
    }

    fn layout(&self) -> assorted_widgets::layout::Style {
        Default::default()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn paint(&self, ctx: &mut assorted_widgets::paint::PaintContext) {
        // Background
        ctx.draw_rect(self.bounds, Color::rgb(0.15, 0.15, 0.18));

        // Title card
        ctx.draw_styled_rect(
            Rect::new(Point::new(50.0, 30.0), Size::new(500.0, 70.0)),
            ShapeStyle::rounded(Color::rgb(0.2, 0.4, 0.8), 12.0)
                .with_border(Border::new(Color::rgb(0.1, 0.2, 0.6), 2.0)),
        );

        // Row 1: Different corner radii
        let y1 = 130.0;
        let x = 50.0;
        let w = 130.0;
        let h = 90.0;
        let spacing = 160.0;

        // Sharp corners
        ctx.draw_styled_rect(
            Rect::new(Point::new(x, y1), Size::new(w, h)),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.9, 0.3, 0.3)),
                corner_radius: CornerRadius::uniform(0.0),
                border: Some(Border::new(Color::rgb(0.7, 0.1, 0.1), 3.0)),
            },
        );

        // Small radius (8px)
        ctx.draw_styled_rect(
            Rect::new(Point::new(x + spacing, y1), Size::new(w, h)),
            ShapeStyle::rounded(Color::rgb(0.3, 0.9, 0.3), 8.0)
                .with_border(Border::new(Color::rgb(0.1, 0.7, 0.1), 3.0)),
        );

        // Medium radius (16px)
        ctx.draw_styled_rect(
            Rect::new(Point::new(x + spacing * 2.0, y1), Size::new(w, h)),
            ShapeStyle::rounded(Color::rgb(0.3, 0.3, 0.9), 16.0)
                .with_border(Border::new(Color::rgb(0.1, 0.1, 0.7), 3.0)),
        );

        // Large radius (40px - pill shape)
        ctx.draw_styled_rect(
            Rect::new(Point::new(x + spacing * 3.0, y1), Size::new(w, h)),
            ShapeStyle::rounded(Color::rgb(0.9, 0.6, 0.2), 40.0)
                .with_border(Border::new(Color::rgb(0.7, 0.4, 0.0), 3.0)),
        );

        // Row 2: Per-corner radius control
        let y2 = 250.0;

        // Top-left only
        ctx.draw_styled_rect(
            Rect::new(Point::new(x, y2), Size::new(w, h)),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.8, 0.4, 0.8)),
                corner_radius: CornerRadius {
                    top_left: 30.0,
                    top_right: 0.0,
                    bottom_right: 0.0,
                    bottom_left: 0.0,
                },
                border: Some(Border::new(Color::rgb(0.6, 0.2, 0.6), 2.0)),
            },
        );

        // Top corners
        ctx.draw_styled_rect(
            Rect::new(Point::new(x + spacing, y2), Size::new(w, h)),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.4, 0.8, 0.8)),
                corner_radius: CornerRadius {
                    top_left: 20.0,
                    top_right: 20.0,
                    bottom_right: 0.0,
                    bottom_left: 0.0,
                },
                border: Some(Border::new(Color::rgb(0.2, 0.6, 0.6), 2.0)),
            },
        );

        // Alternating corners
        ctx.draw_styled_rect(
            Rect::new(Point::new(x + spacing * 2.0, y2), Size::new(w, h)),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.9, 0.9, 0.3)),
                corner_radius: CornerRadius {
                    top_left: 25.0,
                    top_right: 5.0,
                    bottom_right: 25.0,
                    bottom_left: 5.0,
                },
                border: Some(Border::new(Color::rgb(0.7, 0.7, 0.1), 2.0)),
            },
        );

        // All different radii
        ctx.draw_styled_rect(
            Rect::new(Point::new(x + spacing * 3.0, y2), Size::new(w, h)),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.5, 0.9, 0.5)),
                corner_radius: CornerRadius {
                    top_left: 5.0,
                    top_right: 15.0,
                    bottom_right: 25.0,
                    bottom_left: 35.0,
                },
                border: Some(Border::new(Color::rgb(0.3, 0.7, 0.3), 2.0)),
            },
        );

        // Row 3: Different border widths
        let y3 = 370.0;
        for i in 0..6 {
            let border_width = (i + 1) as f32;
            ctx.draw_styled_rect(
                Rect::new(Point::new(x + i as f64 * 90.0, y3), Size::new(70.0, 70.0)),
                ShapeStyle::rounded(Color::rgb(0.95, 0.95, 0.95), 10.0)
                    .with_border(Border::new(Color::rgb(0.2, 0.5, 0.9), border_width)),
            );
        }

        // Grid of colorful rounded rectangles (performance test)
        let grid_y = 470.0;
        for row in 0..5 {
            for col in 0..16 {
                let hue = (col as f32 / 16.0) * 360.0;
                let color = hsl_to_rgb(hue, 0.7, 0.6);
                ctx.draw_styled_rect(
                    Rect::new(
                        Point::new(x + col as f64 * 38.0, grid_y + row as f64 * 28.0),
                        Size::new(33.0, 23.0),
                    ),
                    ShapeStyle::rounded(color, 5.0),
                );
            }
        }
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::rgb(r + m, g + m, b + m)
}

fn main() {
    println!("Rounded Rectangles Demo - SDF Rendering");
    println!("========================================");
    println!();
    println!("This demo showcases:");
    println!("  • Rounded rectangles with SDF anti-aliasing");
    println!("  • Per-corner radius control");
    println!("  • Configurable borders");
    println!("  • Batched rendering (100+ rectangles)");
    println!();

    #[cfg(target_os = "macos")]
    {
        let mut app = pollster::block_on(async { Application::new().await })
            .expect("Failed to initialize application");

        let window_id = app
            .create_window(WindowOptions {
                bounds: Rect::new(Point::new(100.0, 100.0), Size::new(750.0, 650.0)),
                title: "Rounded Rectangles - SDF Demo".to_string(),
                titlebar: None,
                borderless: false,
                transparent: false,
                always_on_top: false,
                utility: false,
            })
            .expect("Failed to create window");

        // Add demo element using proper API (adds to ElementManager, LayoutManager, and SceneGraph)
        {
            let window = app.window_mut(window_id).expect("Window not found");
            let demo = RoundedRectsDemo::new(
                WidgetId::new(1),
                Rect::new(Point::new(0.0, 0.0), Size::new(750.0, 650.0)),
            );

            // Use add_root_widget to properly register in all three systems
            use assorted_widgets::layout::{Style, Display};
            window.add_root_widget(
                Box::new(demo),
                Style {
                    display: Display::Block,
                    size: taffy::Size {
                        width: taffy::Dimension::length(750.0),
                        height: taffy::Dimension::length(650.0),
                    },
                    ..Default::default()
                },
            ).expect("Failed to add root widget");
        }

        println!("✓ Window created");
        println!("✓ Demo element added");
        println!();
        println!("Close the window to exit.");

        app.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This example currently only works on macOS.");
        println!("Other platforms coming soon!");
    }
}
