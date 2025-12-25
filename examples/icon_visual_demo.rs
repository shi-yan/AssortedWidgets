///! Visual Demo: Icon and Image Rendering (Phase 5 Complete Demo)
///!
///! This example creates a window and displays:
///! - Material Icons using the Phase 5 icon system (rendered as glyphs through TextPipeline)
///! - Tzuyu2.png image using the Phase 5 image system (individual texture rendering)

use assorted_widgets::{Application, Widget};
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::paint::{Color, PaintContext};
use assorted_widgets::layout::Style;
use assorted_widgets::image::ImageId;
use std::path::PathBuf;

/// Widget that showcases Material Icons and Image Rendering
struct IconShowcase {
    id: WidgetId,
    bounds: Rect,
    is_dirty: bool,
    image_id: Option<ImageId>,
    image_size: Option<(f64, f64)>, // (width, height) of the actual image
}

impl IconShowcase {
    fn new(id: WidgetId) -> Self {
        // Try to load Tzuyu2.png if it exists
        let (image_id, image_size) = if PathBuf::from("Tzuyu2.png").exists() {
            let id = ImageId::from_file(PathBuf::from("Tzuyu2.png"));
            // Load the image to get its dimensions
            if let Ok(img) = image::open("Tzuyu2.png") {
                let (w, h) = (img.width() as f64, img.height() as f64);
                (Some(id), Some((w, h)))
            } else {
                (Some(id), None)
            }
        } else {
            (None, None)
        };

        Self {
            id,
            bounds: Rect::new(Point::new(0.0, 0.0), Size::new(1000.0, 800.0)),
            is_dirty: true,
            image_id,
            image_size,
        }
    }
}

impl Widget for IconShowcase {
    // Note: Can't use impl_widget_essentials!() because this widget has custom dirty tracking

    fn id(&self) -> WidgetId { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, bounds: Rect) { self.bounds = bounds; }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn paint(&self, ctx: &mut PaintContext) {
        // Background
        ctx.draw_rect(
            self.bounds,
            Color::rgb(0.15, 0.15, 0.15), // Dark gray
        );

        // Title text
        ctx.draw_text(
            "Phase 5 Complete: Icons + Images",
            &assorted_widgets::text::TextStyle::new()
                .size(32.0)
                .color(Color::rgb(1.0, 1.0, 1.0)),
            Point::new(50.0, 30.0),
            None,
        );

        // Subtitle
        ctx.draw_text(
            "Icons via TextPipeline • Images via ImagePipeline",
            &assorted_widgets::text::TextStyle::new()
                .size(16.0)
                .color(Color::rgb(0.7, 0.7, 0.7)),
            Point::new(50.0, 75.0),
            None,
        );

        // ===== IMAGE RENDERING DEMO (Phase 5) =====
        if let Some(ref image_id) = self.image_id {
            // Section title
            ctx.draw_text(
                "Image Rendering (Tzuyu2.png)",
                &assorted_widgets::text::TextStyle::new()
                    .size(18.0)
                    .color(Color::rgb(0.9, 0.9, 0.9)),
                Point::new(550.0, 130.0),
                None,
            );

            // Calculate scaled size maintaining aspect ratio
            let (display_width, display_height) = if let Some((orig_w, orig_h)) = self.image_size {
                // Scale to fit in 250px height while maintaining aspect ratio
                let max_height = 250.0;
                let scale = max_height / orig_h;
                (orig_w * scale, orig_h * scale)
            } else {
                // Fallback to square if we don't know the size
                (200.0, 200.0)
            };

            // Draw the image at actual aspect ratio
            let image_rect = Rect::new(
                Point::new(550.0, 170.0),
                Size::new(display_width, display_height),
            );
            ctx.draw_image(image_id.clone(), image_rect, None);

            // Label below image with actual dimensions
            let label = if let Some((w, h)) = self.image_size {
                format!("{}×{} (aspect ratio preserved)", w as u32, h as u32)
            } else {
                "GPU texture".to_string()
            };
            ctx.draw_text(
                &label,
                &assorted_widgets::text::TextStyle::new()
                    .size(14.0)
                    .color(Color::rgb(0.6, 0.6, 0.6)),
                Point::new(580.0, 170.0 + display_height + 10.0),
                None,
            );

            // Tinted version
            let tinted_y = 170.0 + display_height + 60.0;
            ctx.draw_text(
                "Tinted Version",
                &assorted_widgets::text::TextStyle::new()
                    .size(18.0)
                    .color(Color::rgb(0.9, 0.9, 0.9)),
                Point::new(550.0, tinted_y),
                None,
            );

            // Smaller tinted version (scale to 150px height)
            let (tinted_width, tinted_height) = if let Some((orig_w, orig_h)) = self.image_size {
                let max_height = 150.0;
                let scale = max_height / orig_h;
                (orig_w * scale, orig_h * scale)
            } else {
                (150.0, 150.0)
            };

            let tinted_rect = Rect::new(
                Point::new(550.0, tinted_y + 40.0),
                Size::new(tinted_width, tinted_height),
            );
            ctx.draw_image(
                image_id.clone(),
                tinted_rect,
                Some(Color::rgba(0.3, 0.6, 1.0, 0.7)),
            );

            ctx.draw_text(
                &format!("{:.0}×{:.0} (blue tint)", tinted_width, tinted_height),
                &assorted_widgets::text::TextStyle::new()
                    .size(14.0)
                    .color(Color::rgb(0.6, 0.6, 0.6)),
                Point::new(570.0, tinted_y + 40.0 + tinted_height + 10.0),
                None,
            );
        }

        // ===== ICON RENDERING DEMO (Phase 5) =====
        ctx.draw_text(
            "Material Icons (Font-Based Rendering)",
            &assorted_widgets::text::TextStyle::new()
                .size(18.0)
                .color(Color::rgb(0.9, 0.9, 0.9)),
            Point::new(50.0, 130.0),
            None,
        );

        // Icon grid - 4x3 grid of common icons
        let icons = vec![
            ("search", "search"),
            ("home", "home"),
            ("favorite", "favorite"),
            ("star", "star"),
            ("settings", "settings"),
            ("person", "person"),
            ("menu", "menu"),
            ("close", "close"),
            ("check", "check"),
            ("add", "add"),
            ("remove", "remove"),
            ("info", "info"),
        ];

        let start_x = 50.0;
        let start_y = 170.0;
        let icon_size = 48.0;
        let grid_spacing_x = 110.0;
        let grid_spacing_y = 120.0;

        for (i, (icon_id, label)) in icons.iter().enumerate() {
            let col = i % 4;
            let row = i / 4;

            let x = start_x + (col as f64 * grid_spacing_x);
            let y = start_y + (row as f64 * grid_spacing_y);

            // Draw the icon
            ctx.draw_icon(
                icon_id,
                Point::new(x + 10.0, y),
                icon_size,
                Color::rgb(1.0, 1.0, 1.0),
            );

            // Label below icon
            ctx.draw_text(
                label,
                &assorted_widgets::text::TextStyle::new()
                    .size(12.0)
                    .color(Color::rgb(0.7, 0.7, 0.7)),
                Point::new(x, y + icon_size as f64 + 10.0),
                None,
            );
        }

        // Colored icons demo
        ctx.draw_text(
            "Colored Icons",
            &assorted_widgets::text::TextStyle::new()
                .size(18.0)
                .color(Color::rgb(0.9, 0.9, 0.9)),
            Point::new(50.0, 570.0),
            None,
        );

        let colored_icons = vec![
            ("favorite", Color::rgb(1.0, 0.3, 0.3)), // Red
            ("star", Color::rgb(1.0, 0.8, 0.2)),     // Yellow
            ("check", Color::rgb(0.3, 1.0, 0.3)),    // Green
            ("info", Color::rgb(0.3, 0.6, 1.0)),     // Blue
        ];

        for (i, (icon_id, color)) in colored_icons.iter().enumerate() {
            let x = 50.0 + (i as f64 * 80.0);
            let y = 610.0;

            ctx.draw_icon(
                icon_id,
                Point::new(x + 5.0, y),
                64.0,
                *color,
            );
        }

        // Size variations
        ctx.draw_text(
            "Size Variations (24px, 48px, 72px)",
            &assorted_widgets::text::TextStyle::new()
                .size(14.0)
                .color(Color::rgb(0.7, 0.7, 0.7)),
            Point::new(50.0, 710.0),
            None,
        );

        ctx.draw_icon("settings", Point::new(60.0, 735.0), 24.0, Color::rgb(0.8, 0.8, 0.8));
        ctx.draw_icon("settings", Point::new(120.0, 723.0), 48.0, Color::rgb(0.8, 0.8, 0.8));
        ctx.draw_icon("settings", Point::new(210.0, 705.0), 72.0, Color::rgb(0.8, 0.8, 0.8));
    }

    // Custom dirty tracking (overrides defaults)
    fn set_dirty(&mut self, dirty: bool) { self.is_dirty = dirty; }
    fn is_dirty(&self) -> bool { self.is_dirty }
}

fn main() {
    println!("=== Phase 5 Complete Visual Demo ===");
    println!("Icons + Images Rendering Demonstration\n");

    println!("✅ Displaying Material Icons via TextPipeline");

    // Check if image exists
    if PathBuf::from("Tzuyu2.png").exists() {
        println!("✅ Displaying Tzuyu2.png via ImagePipeline");
    } else {
        println!("⚠️  Tzuyu2.png not found (icons will still render)");
    }
    println!();

    Application::launch(|app| {
        app.spawn_window("Phase 5: Icons + Images - AssortedWidgets", 1000.0, 800.0, |window| {
            let showcase = IconShowcase::new(WidgetId::new(1));

            use assorted_widgets::layout::Display;
            window
                .add_root(
                    Box::new(showcase),
                    Style {
                        display: Display::Block,
                        size: taffy::Size {
                            width: taffy::Dimension::length(1000.0),
                            height: taffy::Dimension::length(800.0),
                        },
                        ..Default::default()
                    },
                )
                .expect("Failed to add root widget");

            println!("✅ Window created!");
            println!("Phase 5 showcase ready!");
            println!("Press Cmd+Q to quit.\n");
        });
    });
}
