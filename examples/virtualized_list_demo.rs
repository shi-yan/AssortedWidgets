//! Virtualized List Demo
//!
//! Demonstrates the VirtualizedList widget with:
//! - 10,000 items (alternating Label and Button)
//! - Widget recycling for efficient scrolling
//! - Even indices = Label widgets
//! - Odd indices = Button widgets

use assorted_widgets::application::Application;
use assorted_widgets::layout::Style;
use assorted_widgets::paint::Color;
use assorted_widgets::widget::Widget;
use assorted_widgets::widgets::{Button, Label, ListDataSource, VirtualizedList};

/// Data source that alternates between Label (even) and Button (odd)
struct AlternatingDataSource {
    item_count: usize,
}

impl AlternatingDataSource {
    fn new(item_count: usize) -> Self {
        Self { item_count }
    }
}

impl ListDataSource for AlternatingDataSource {
    fn item_count(&self) -> usize {
        self.item_count
    }

    fn item_height(&self, _index: usize) -> Option<f64> {
        // Fixed height for all items
        Some(50.0)
    }

    fn widget_for_item(
        &mut self,
        index: usize,
        reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget> {
        if index % 2 == 0 {
            // Even index - use Label
            if let Some(mut widget) = reused_widget {
                // Check if it's already a Label
                if widget.as_any().downcast_ref::<Label>().is_some() {
                    // Reuse it
                    if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                        label.set_text(&format!("📄 Label Item {}", index));
                    }
                    return widget;
                }
            }

            // Create new Label
            Box::new(
                Label::new(&format!("📄 Label Item {}", index))
                    .font_size(16.0)
                    .text_color(Color::rgb(0.2, 0.2, 0.2))
            )
        } else {
            // Odd index - use Button
            if let Some(mut widget) = reused_widget {
                // Check if it's already a Button
                if widget.as_any().downcast_ref::<Button>().is_some() {
                    // Reuse it
                    if let Some(button) = widget.as_any_mut().downcast_mut::<Button>() {
                        button.set_text(&format!("🔘 Button {}", index));
                    }
                    return widget;
                }
            }

            // Create new Button
            let button_id = index;  // Capture for callback
            Box::new(
                Button::text(&format!("🔘 Button {}", index))
                    .on_click(move || {
                        eprintln!("✅ Button {} clicked!", button_id);
                    })
            )
        }
    }
}

fn main() {
    eprintln!("Starting Virtualized List Demo...");

    Application::launch(|app| {
        eprintln!("Application launched, spawning window...");

        app.spawn_window("Virtualized List Demo - 10,000 Items", 600.0, 800.0, |window| {
            eprintln!("Window created, setting up list...");

            // Create a virtualized list with 10,000 items
            let list = VirtualizedList::new()
                .fixed_height(50.0)  // All items are 50px tall (optimized mode)
                .data_source(Box::new(AlternatingDataSource::new(10_000)))
                .background(Color::rgb(0.95, 0.95, 0.95))
                .layout_style(Style {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    ..Style::default()
                });

            window.set_main_widget(list);

            eprintln!("List configured with 10,000 items!");
            eprintln!("  - Even indices: 📄 Label widgets");
            eprintln!("  - Odd indices:  🔘 Button widgets");
            eprintln!("  - Try scrolling and clicking buttons!");
        });

        eprintln!("Window spawned successfully!");
    });
}
