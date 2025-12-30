use assorted_widgets::application::Application;
use assorted_widgets::event::OsEvent;
use assorted_widgets::layout::Style;
use assorted_widgets::paint::{Color, ShapeStyle, Brush, CornerRadius, Border, Shadow, PaintContext};
use assorted_widgets::text::TextStyle;
use assorted_widgets::types::{Point, Rect, Size, WidgetId, DeferredCommand, GuiMessage};
use assorted_widgets::widget::Widget;

fn main() {
    Application::launch(|app| {
        app.spawn_window("Depth Test - All Pipelines", 800.0, 600.0, |window| {
            // Create a test widget that draws primitives from all pipelines
            window.set_main_widget(DepthTestWidget {
                id: WidgetId::default(),
                bounds: Rect::new(Point::new(0.0, 0.0), Size::new(800.0, 600.0)),
            });
        });
    });
}

struct DepthTestWidget {
    id: WidgetId,
    bounds: Rect,
}

impl Widget for DepthTestWidget {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn set_dirty(&mut self, _dirty: bool) {}

    fn is_dirty(&self) -> bool {
        false
    }

    fn layout(&self) -> Style {
        Style::default()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bounds = self.bounds();

        println!("\n=== Depth Test Widget Paint ===");
        println!("Bounds: {:?}", bounds);

        // 1. Draw SDF Rect (rounded rectangle with border)
        println!("Drawing SDF rect at (50, 50)");
        ctx.draw_styled_rect(
            Rect::new(Point::new(50.0, 50.0), Size::new(200.0, 100.0)),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.2, 0.4, 0.8)),
                corner_radius: CornerRadius::uniform(10.0),
                border: Some(Border {
                    width: 2.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                }),
                shadow: None,
            },
        );

        // 2. Draw basic rect (no rounded corners)
        println!("Drawing basic rect at (50, 200)");
        ctx.draw_rect(
            Rect::new(Point::new(50.0, 200.0), Size::new(200.0, 100.0)),
            Color::rgb(0.8, 0.2, 0.4),
        );

        // 3. Draw rect with shadow (uses shadow pipeline)
        println!("Drawing shadow rect at (50, 350)");
        ctx.draw_styled_rect(
            Rect::new(Point::new(50.0, 350.0), Size::new(200.0, 100.0)),
            ShapeStyle {
                fill: Brush::Solid(Color::rgb(0.4, 0.8, 0.2)),
                corner_radius: CornerRadius::uniform(10.0),
                border: None,
                shadow: Some(Shadow {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.5),
                    offset: (4.0, 4.0),
                    blur_radius: 8.0,
                    spread_radius: 0.0,
                }),
            },
        );

        // 4. Draw text
        let text_style = TextStyle::new()
            .size(16.0)
            .color(Color::WHITE);

        println!("Drawing text at (350, 50)");
        ctx.draw_text(
            "SDF Rect\n(rounded corners)",
            &text_style,
            Point::new(350.0, 50.0),
            None,
        );

        ctx.draw_text(
            "Basic Rect\n(no corners)",
            &text_style,
            Point::new(350.0, 200.0),
            None,
        );

        ctx.draw_text(
            "Shadow Rect\n(with shadow)",
            &text_style,
            Point::new(350.0, 350.0),
            None,
        );

        println!("=== Paint Complete ===\n");
    }
}
