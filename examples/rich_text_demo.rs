//! Rich text label demonstration
//!
//! This example demonstrates the RichTextLabel widget with:
//! - Bold, italic, strikethrough text
//! - Clickable links with cursor changes
//! - Bullet lists with nesting
//! - Embedded scrollbars (overflow:auto)
//! - Both wrapping and non-wrapping modes

use assorted_widgets::{
    Application,
    widgets::{RichTextLabel, Padding},
    layout::Style,
    paint::Color,
};

fn main() {
    Application::launch(|app| {
        app.spawn_window("Rich Text Demo", 1200.0, 800.0, |window| {
            let markdown = r#"
# Rich Text Label Demo

This is a demonstration of the **RichTextLabel** widget with *limited markdown support*.

## Supported Features

### Text Styling

You can use **bold text** for emphasis, *italic text* for style, and ~~strikethrough~~ for corrections.

You can also combine them: ***bold and italic*** together!

### Links

Click on this link: [AssortedWidgets on GitHub](https://github.com/example/assorted-widgets)

Or visit the [Rust Language Website](https://www.rust-lang.org/) to learn more about Rust.

### Bullet Lists

Here are the key features:

- **Rich Text Support**: Bold, italic, strikethrough
- **Clickable Links**: Links change cursor to pointer on hover
- **Bullet Lists**: Including nested items
  - Nested item 1
  - Nested item 2 with **bold** and *italic*
  - Nested item 3
- **Scrollbars**: Automatically appear when content overflows (overflow:auto)
- **Line-based scrolling**: Vertical scrolling is discrete (by lines)
- **Pixel-based horizontal scrolling**: When wrapping is disabled

### Implementation Details

The widget architecture includes:

- Markdown parsing with pulldown-cmark
- cosmic-text for text shaping and rendering
- Span-based styling (flat string + style ranges)
- Embedded ScrollBar widgets that appear/disappear automatically

### Try These Features

1. **Hover over links** - cursor changes to pointer
2. **Click links** - see console output with URL
3. **Scroll vertically** - line-based discrete scrolling
4. **Resize window** - scrollbars appear/disappear automatically

### More Content for Scrolling

This is additional content to demonstrate vertical scrolling. The scrollbar should appear automatically when the content exceeds the viewport height.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

**Bold statement**: This widget is production-ready!

*Italic thought*: The architecture is clean and extensible.

~~Mistake~~: This is not a mistake, just demonstrating strikethrough.

Final link: [Learn More](https://example.com/learn-more)

---

## Additional Content to Test Scrolling

This section contains extra paragraphs to make the content long enough to trigger vertical scrolling.

**Paragraph 1**: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.

**Paragraph 2**: Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

**Paragraph 3**: Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.

**Paragraph 4**: Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet.

**Paragraph 5**: At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident.

**Paragraph 6**: Similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque.

**Paragraph 7**: Nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet.

**Paragraph 8**: Ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.

**Paragraph 9**: More content to ensure we have enough lines to trigger scrolling. The scrollbar should appear when the content height exceeds the viewport height. This demonstrates the overflow:auto behavior.

**Paragraph 10**: Testing scrollbar functionality with additional content. When you resize the window to make it smaller, the scrollbar should appear automatically. When you make it larger, it should disappear if all content fits.

---

**End of Demo Content**
"#;

            // Create wrapped version (default)
            let rich_text_wrapped = RichTextLabel::new(markdown)
                .wrapping(true)
                .padding(Padding::uniform(16.0))
                .background(Color::rgb(0.15, 0.15, 0.17))
                .text_color(Color::rgb(0.9, 0.9, 0.9))
                .link_color(Color::rgb(0.4, 0.7, 1.0))
                .font_size(14.0)
                .on_link_clicked(|url| {
                    println!("Link clicked: {}", url);
                });

            // Configure root container layout
            window.set_root_layout(taffy::Style {
                display: taffy::Display::Flex,
                flex_direction: taffy::FlexDirection::Column,
                size: taffy::Size {
                    width: taffy::Dimension::percent(1.0),
                    height: taffy::Dimension::percent(1.0),
                },
                ..Default::default()
            });

            // Add widget with size constraints
            // IMPORTANT: height must be constrained for scrollbars to appear!
            // Using auto() means the widget grows to fit all content (no overflow)
            window.add_to_root(Box::new(rich_text_wrapped), taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::percent(1.0),   // Fill parent width
                    height: taffy::Dimension::percent(1.0),  // Fill parent height (enables overflow:auto)
                },
                ..Default::default()
            }).expect("Failed to add widget");
        });
    });
}
