///! Visual Demo: Icon and Image Rendering (Phase 5 Complete Demo)
///!
///! This example creates a window and displays:
///! - Material Icons using the Phase 5 icon system (rendered as glyphs through TextPipeline)
///! - Tzuyu2.png image using the Phase 5 image system (individual texture rendering)

use assorted_widgets::{Application, Widget, WindowOptions, GuiMessage, DeferredCommand};
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::paint::{Color, PaintContext};
use assorted_widgets::layout::Style;
use assorted_widgets::event::OsEvent;
use assorted_widgets::image::ImageId;
use std::path::PathBuf;

/// Widget that showcases Material Icons and Image Rendering
struct IconShowcase {
    id: WidgetId,
    bounds: Rect,
    is_dirty: bool,
    image_id: Option<ImageId>,
}

impl IconShowcase {
    fn new(id: WidgetId) -> Self {
        // Try to load Tzuyu2.png if it exists
        let image_id = if PathBuf::from("Tzuyu2.png").exists() {
            Some(ImageId::from_file(PathBuf::from("Tzuyu2.png")))
        } else {
            None
        };

        Self {
            id,
            bounds: Rect::new(Point::new(0.0, 0.0), Size::new(1000.0, 800.0)),
            is_dirty: true,
            image_id,
        }
    }
}

impl Widget for IconShowcase {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

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

            // Draw the image at 200x200
            let image_rect = Rect::new(
                Point::new(550.0, 170.0),
                Size::new(200.0, 200.0),
            );
            ctx.draw_image(image_id.clone(), image_rect, None);

            // Label below image
            ctx.draw_text(
                "Individual texture (ImageCache + ImagePipeline)",
                &assorted_widgets::text::TextStyle::new()
                    .size(11.0)
                    .color(Color::rgb(0.6, 0.6, 0.6)),
                Point::new(550.0, 380.0),
                None,
            );
        }

        // Section 1: Large Icons
        ctx.draw_text(
            "Large Icons (48pt)",
            &assorted_widgets::text::TextStyle::new()
                .size(18.0)
                .color(Color::rgb(0.9, 0.9, 0.9)),
            Point::new(50.0, 130.0),
            None,
        );

        let large_icons = vec![
            ("search", 50.0, 170.0, Color::rgb(0.3, 0.6, 0.9)),     // Blue
            ("home", 120.0, 170.0, Color::rgb(0.3, 0.8, 0.4)),      // Green
            ("settings", 190.0, 170.0, Color::rgb(0.9, 0.5, 0.2)),  // Orange
            ("favorite", 260.0, 170.0, Color::rgb(0.9, 0.2, 0.3)),  // Red
            ("star", 330.0, 170.0, Color::rgb(1.0, 0.9, 0.2)),      // Yellow
            ("person", 400.0, 170.0, Color::rgb(0.6, 0.3, 0.9)),    // Purple
        ];

        for (icon_id, x, y, color) in large_icons {
            ctx.draw_icon(icon_id, Point::new(x, y), 48.0, color);

            // Label below icon
            ctx.draw_text(
                icon_id,
                &assorted_widgets::text::TextStyle::new()
                    .size(11.0)
                    .color(Color::rgb(0.6, 0.6, 0.6)),
                Point::new(x - 5.0, y + 55.0),
                None,
            );
        }

        // Section 2: Medium Icons
        ctx.draw_text(
            "Medium Icons (32pt)",
            &assorted_widgets::text::TextStyle::new()
                .size(18.0)
                .color(Color::rgb(0.9, 0.9, 0.9)),
            Point::new(50.0, 280.0),
            None,
        );

        let medium_icons = vec![
            ("notifications", 50.0, 320.0),
            ("email", 110.0, 320.0),
            ("calendar", 170.0, 320.0),
            ("location", 230.0, 320.0),
            ("phone", 290.0, 320.0),
            ("camera", 350.0, 320.0),
            ("music", 410.0, 320.0),
            ("video", 470.0, 320.0),
        ];

        for (icon_id, x, y) in medium_icons {
            ctx.draw_icon(icon_id, Point::new(x, y), 32.0, Color::rgb(0.7, 0.7, 0.7));
        }

        // Section 3: Small Icons
        ctx.draw_text(
            "Small Icons (24pt) - UI Controls",
            &assorted_widgets::text::TextStyle::new()
                .size(18.0)
                .color(Color::rgb(0.9, 0.9, 0.9)),
            Point::new(50.0, 400.0),
            None,
        );

        let small_icons = vec![
            ("check", 50.0, 440.0, Color::rgb(0.3, 0.8, 0.4)),       // Green checkmark
            ("close", 90.0, 440.0, Color::rgb(0.9, 0.2, 0.3)),       // Red X
            ("add", 130.0, 440.0, Color::rgb(0.3, 0.6, 0.9)),        // Blue plus
            ("remove", 170.0, 440.0, Color::rgb(0.9, 0.5, 0.2)),     // Orange minus
            ("edit", 210.0, 440.0, Color::rgb(0.6, 0.3, 0.9)),       // Purple edit
            ("delete", 250.0, 440.0, Color::rgb(0.9, 0.2, 0.3)),     // Red delete
            ("save", 290.0, 440.0, Color::rgb(0.3, 0.8, 0.4)),       // Green save
            ("refresh", 330.0, 440.0, Color::rgb(0.3, 0.6, 0.9)),    // Blue refresh
        ];

        for (icon_id, x, y, color) in small_icons {
            ctx.draw_icon(icon_id, Point::new(x, y), 24.0, color);
        }

        // Section 4: Icon Grid
        ctx.draw_text(
            "Icon Grid - All Available Icons",
            &assorted_widgets::text::TextStyle::new()
                .size(18.0)
                .color(Color::rgb(0.9, 0.9, 0.9)),
            Point::new(50.0, 510.0),
            None,
        );

        // Draw a grid of all available icons
        let grid_start_x = 50.0;
        let grid_start_y = 550.0;
        let icon_spacing = 50.0;
        let icons_per_row = 15;

        let all_icons = vec![
            "search", "home", "settings", "favorite", "star", "person",
            "notifications", "email", "calendar", "location", "phone",
            "camera", "music", "video", "check", "close", "add", "remove",
            "edit", "delete", "save", "refresh", "download", "upload",
            "share", "link", "lock", "unlock", "visibility", "visibility_off",
            "help", "info", "warning", "error", "done", "arrow_back",
            "arrow_forward", "arrow_up", "arrow_down", "arrow_drop_down",
            "arrow_drop_up", "menu", "more", "more_vert", "expand_more",
            "expand_less", "folder", "insert_drive_file", "image", "audio",
            "play", "pause",
        ];

        for (i, icon_id) in all_icons.iter().enumerate() {
            let row = i / icons_per_row;
            let col = i % icons_per_row;
            let x = grid_start_x + (col as f64 * icon_spacing);
            let y = grid_start_y + (row as f64 * icon_spacing);

            ctx.draw_icon(
                icon_id,
                Point::new(x, y),
                20.0,
                Color::rgb(0.5, 0.5, 0.5),
            );
        }

        // Footer
        ctx.draw_text(
            "✅ Phase 5 Complete: Icons via TextPipeline • Images via ImagePipeline",
            &assorted_widgets::text::TextStyle::new()
                .size(12.0)
                .color(Color::rgb(0.5, 0.7, 0.3)),
            Point::new(50.0, 750.0),
            None,
        );
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn layout(&self) -> Style {
        Style {
            size: taffy::Size {
                width: taffy::Dimension::length(self.bounds.size.width as f32),
                height: taffy::Dimension::length(self.bounds.size.height as f32),
            },
            ..Default::default()
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    println!("=== Phase 5 Complete Visual Demo ===");
    println!("Icons + Images Rendering Demonstration\n");

    // Initialize application
    let mut app = pollster::block_on(async {
        Application::new().await
    })
    .expect("Failed to initialize rendering");

    // Create window
    let window_id = app.create_window(WindowOptions {
        bounds: Rect::new(Point::new(100.0, 100.0), Size::new(1000.0, 800.0)),
        title: "Phase 5: Icons + Images - AssortedWidgets".to_string(),
        titlebar: None,
        borderless: false,
        transparent: false,
        always_on_top: false,
        utility: false,
    })
    .expect("Failed to create window");

    println!("✅ Window created!");
    println!("✅ Displaying Material Icons via TextPipeline");

    // Check if image exists
    if PathBuf::from("Tzuyu2.png").exists() {
        println!("✅ Displaying Tzuyu2.png via ImagePipeline");
    } else {
        println!("⚠️  Tzuyu2.png not found (icons will still render)");
    }
    println!();

    // Create showcase widget
    let showcase = IconShowcase::new(WidgetId::new(1));

    // Add to window using clean API
    let window = app.window_mut(window_id).expect("Window not found");
    window.add_root(Box::new(showcase), Style::default())
        .expect("Failed to add root widget");

    println!("Phase 5 showcase ready!");
    println!("Press Cmd+Q to quit.\n");

    // Run application event loop
    app.run();
}
