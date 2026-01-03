//! Simple Scrollable Container Demo
//!
//! A minimal test case with just one long label to debug scrolling issues

use assorted_widgets::Application;
use assorted_widgets::widgets::{Label, WrapMode, ScrollableContainer, ScrollMode};
use assorted_widgets::paint::Color;

fn main() {
    println!("Simple Scrollable Container Demo");
    println!("=================================\n");

    // Launch the application
    Application::launch(|app| {
        // Create a window
        app.spawn_window("Simple Scroll Demo", 600.0, 400.0, |window| {
            println!("Setting up window...");

            // Create a VERY long text string that will definitely require scrolling
            let long_text = "SCROLLABLE TEXT DEMO - This text should wrap and scroll\n\n\
                           Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n\n\
                           Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n\n\
                           Paragraph 3: Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.\n\n\
                           Paragraph 4: Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.\n\n\
                           Paragraph 5: Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem.\n\n\
                           Paragraph 6: Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur.\n\n\
                           Paragraph 7: Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur.\n\n\
                           Paragraph 8: At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident.\n\n\
                           Paragraph 9: Similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio.\n\n\
                           Paragraph 10: Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus.\n\n\
                           Paragraph 11: Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae.\n\n\
                           Paragraph 12: Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.\n\n\
                           FINAL PARAGRAPH: This is the end of the very long scrollable text. If you can see this, scrolling worked!";

            // Create a label with the long text
            let label = Label::new(long_text)
                .font_size(18.0)
                .text_color(Color::rgb(1.0, 1.0, 1.0))
                .wrap_mode(WrapMode::WrapAnywhere);

            let label_style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::percent(1.0),
                    height: taffy::Dimension::auto(),
                },
                padding: taffy::Rect::length(20.0),
                ..Default::default()
            };

            // Create scrollable container
            let gui_handle = window.get_handle();
            let (scroll_container, scroll_children) = ScrollableContainer::new(
                ScrollMode::Vertical,
                &gui_handle,
            );

            let scroll_style = taffy::Style {
                display: taffy::Display::Flex,
                size: taffy::Size {
                    width: taffy::Dimension::percent(1.0),
                    height: taffy::Dimension::percent(1.0),
                },
                ..Default::default()
            };

            // Add the scrollable container
            let (scroll_id, _child_ids) = window.add_composite(
                Box::new(scroll_container),
                scroll_style,
                None,
                scroll_children,
            ).expect("Failed to add ScrollableContainer");

            println!("ScrollableContainer added with ID: {:?}", scroll_id);

            // Get the content container ID and add the label to it
            let content_id = window.scrollable_container_content_id(scroll_id)
                .expect("Failed to get content container ID");

            println!("Content container ID: {:?}", content_id);

            // Add the label to the content container
            let _label_id = window.add_child(
                Box::new(label),
                label_style,
                content_id,
            ).expect("Failed to add label");

            println!("Label added successfully!");
            println!("\nControls:");
            println!("  - Mouse wheel: Scroll content");
            println!("  - Drag scrollbar: Scroll content");
            println!("  - Watch console for debug output");
        });
    });
}
