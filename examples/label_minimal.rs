use assorted_widgets::Application;
use assorted_widgets::widgets::{Label, WrapMode, Padding};
use assorted_widgets::paint::Color;

fn main() {
    #[cfg(target_os = "macos")]
    {
        Application::launch(|app| {
            app.spawn_window("Minimal Label Test", 500.0, 300.0, |window| {
                // Single label with explicit size
                let label = Label::new("Hello, this is a test label with some text")
                    .wrap_mode(WrapMode::WrapWord)
                    .bg_color(Color::rgb(0.3, 0.2, 0.3))
                    .text_color(Color::WHITE)
                    .padding(Padding::uniform(12.0));

                let style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(300.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                };

                window.add_to_root(Box::new(label), style)
                    .expect("Failed to add label");

                println!("Minimal label test");
            });
        });
    }
}
