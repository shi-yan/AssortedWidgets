//! Simple rich text test to debug link offset issues

use assorted_widgets::{
    Application,
    widgets::{RichTextLabel, Padding},
    layout::Style,
    paint::Color,
};

fn main() {
    Application::launch(|app| {
        app.spawn_window("Rich Text Link Test", 800.0, 600.0, |window| {
            let markdown = r#"
Paragraph 1: This is some text before [first link](https://example.com/first) and some text after.

Paragraph 2: This is some text before [second link](https://example.com/second) and some text after.

Paragraph 3: This is some text before [third link](https://example.com/third) and some text after.
"#;

            let rich_text = RichTextLabel::new(markdown)
                .wrapping(true)
                .padding(Padding::uniform(16.0))
                .background(Color::rgb(0.15, 0.15, 0.17))
                .text_color(Color::rgb(0.9, 0.9, 0.9))
                .link_color(Color::rgb(0.4, 0.7, 1.0))
                .font_size(16.0)
                .on_link_clicked(|url| {
                    println!("Link clicked: {}", url);
                });

            window.set_root_layout(taffy::Style {
                display: taffy::Display::Flex,
                size: taffy::Size {
                    width: taffy::Dimension::percent(1.0),
                    height: taffy::Dimension::percent(1.0),
                },
                ..Default::default()
            });

            window.add_to_root(Box::new(rich_text), taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::percent(1.0),
                    height: taffy::Dimension::percent(1.0),
                },
                ..Default::default()
            }).expect("Failed to add widget");
        });
    });
}
