//! Markdown parser with filtering for limited rich text support

use super::types::{RichText, Span, SpanAttrs, LinkSpan, BulletItem};
use pulldown_cmark::{Event, Parser, Tag, Options};
use crate::paint::Color;

/// Parse filtered markdown into a RichText document
///
/// Supported markdown features:
/// - **Bold** (`**text**` or `__text__`)
/// - *Italic* (`*text*` or `_text_`)
/// - ~~Strikethrough~~ (`~~text~~`)
/// - [Links](url) (`[text](url)`)
/// - Bullet lists (`- item` or `* item`)
///
/// Unsupported features are ignored (headings, code blocks, images, etc.)
pub fn parse_markdown(input: &str) -> RichText {
    // Enable strikethrough extension
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);

    let mut text = String::new();
    let mut spans = Vec::new();
    let mut links = Vec::new();
    let mut bullets = Vec::new();

    // Stack to track active styles
    let mut attr_stack: Vec<SpanAttrs> = vec![];
    let mut current_attrs = SpanAttrs::default();

    // Stack to track links (start_char, url)
    let mut link_stack: Vec<(usize, String)> = vec![];

    // List depth tracking
    let mut list_depth: usize = 0;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Strong => {
                    attr_stack.push(current_attrs.clone());
                    current_attrs.bold = true;
                }
                Tag::Emphasis => {
                    attr_stack.push(current_attrs.clone());
                    current_attrs.italic = true;
                }
                Tag::Strikethrough => {
                    attr_stack.push(current_attrs.clone());
                    current_attrs.strikethrough = true;
                }
                Tag::Link(_, dest_url, _) => {
                    attr_stack.push(current_attrs.clone());
                    // Set link color (blue)
                    current_attrs.color = Some(Color::rgb(0.4, 0.6, 1.0));
                    let start_char = text.chars().count();
                    link_stack.push((start_char, dest_url.to_string()));
                }
                Tag::List(_) => {
                    list_depth += 1;
                }
                Tag::Item => {
                    // Add bullet symbol with indentation
                    // Use tab characters for indentation (cosmic-text preserves tabs)
                    let indent_level = list_depth.saturating_sub(1);
                    let indent = "\t".repeat(indent_level);  // 1 tab per level
                    text.push_str(&indent);
                    text.push_str("• ");

                    // Track bullet item
                    let line = text.lines().count();
                    bullets.push(BulletItem {
                        line,
                        indent_level: indent_level as u16,
                    });
                }
                _ => {} // Ignore other tags
            },

            Event::End(tag) => match tag {
                Tag::Strong | Tag::Emphasis | Tag::Strikethrough => {
                    // End styled span
                    if let Some(prev_attrs) = attr_stack.pop() {
                        // Only create span if attributes differ from default
                        if current_attrs != SpanAttrs::default() {
                            let end_byte = text.len();
                            // Find the start of this span by tracking text length
                            // We need to search backwards through spans
                            let start_byte = find_span_start(&text, &spans, &current_attrs);

                            if start_byte < end_byte {
                                spans.push(Span {
                                    range: start_byte..end_byte,
                                    attrs: current_attrs.clone(),
                                });
                            }
                        }
                        current_attrs = prev_attrs;
                    }
                }
                Tag::Link(_, _, _) => {
                    if let Some((start_char, url)) = link_stack.pop() {
                        let end_char = text.chars().count();
                        links.push(LinkSpan {
                            char_range: start_char..end_char,
                            url,
                        });

                        // Also create a styled span for the link
                        if let Some(prev_attrs) = attr_stack.pop() {
                            let end_byte = text.len();
                            let start_byte = find_span_start(&text, &spans, &current_attrs);

                            if start_byte < end_byte {
                                spans.push(Span {
                                    range: start_byte..end_byte,
                                    attrs: current_attrs.clone(),
                                });
                            }

                            current_attrs = prev_attrs;
                        }
                    }
                }
                Tag::List(_) => {
                    list_depth = list_depth.saturating_sub(1);
                }
                Tag::Item => {
                    // Add newline after item
                    if !text.ends_with('\n') {
                        text.push('\n');
                    }
                }
                _ => {}
            },

            Event::Text(t) => {
                let start_byte = text.len();
                text.push_str(&t);
                let end_byte = text.len();

                // Create span if we have active styling
                if current_attrs != SpanAttrs::default() && start_byte < end_byte {
                    spans.push(Span {
                        range: start_byte..end_byte,
                        attrs: current_attrs.clone(),
                    });
                }
            }

            Event::SoftBreak => {
                text.push(' ');
            }

            Event::HardBreak => {
                text.push('\n');
            }

            _ => {} // Ignore other events
        }
    }

    // Merge overlapping/adjacent spans with same attributes
    let spans = merge_spans(spans);

    // Debug: print parsed text with visible whitespace
    println!("\n[Markdown Parser] Parsed text:");
    println!("Text length: {} chars, {} bytes", text.chars().count(), text.len());
    println!("Text with visible whitespace:");
    for (i, line) in text.lines().enumerate() {
        let visible = line.replace('\t', "→").replace(' ', "·");
        println!("  Line {}: \"{}\"", i, visible);
    }
    println!("Spans: {} spans", spans.len());
    println!("Links: {} links", links.len());
    println!("Bullets: {} bullets", bullets.len());

    RichText {
        text,
        spans,
        links,
        bullets,
    }
}

/// Find the start byte position for the current styled span
///
/// This searches backwards through already-created spans to find where
/// the current style started being applied.
fn find_span_start(text: &str, spans: &[Span], current_attrs: &SpanAttrs) -> usize {
    // Look for the last span with different attributes
    // The new span starts after that
    for span in spans.iter().rev() {
        if span.attrs != *current_attrs {
            return span.range.end;
        }
    }

    // If no previous span, or all previous spans have the same attrs,
    // this span starts at the beginning
    0
}

/// Merge adjacent or overlapping spans with identical attributes
///
/// This optimization reduces the number of spans and simplifies rendering.
fn merge_spans(mut spans: Vec<Span>) -> Vec<Span> {
    if spans.is_empty() {
        return spans;
    }

    // Sort by start position
    spans.sort_by_key(|s| s.range.start);

    let mut merged = Vec::new();
    let mut current = spans[0].clone();

    for span in spans.into_iter().skip(1) {
        if span.attrs == current.attrs && span.range.start <= current.range.end {
            // Merge: extend current span to include this one
            current.range.end = current.range.end.max(span.range.end);
        } else {
            // Different attrs or non-adjacent: push current and start new
            merged.push(current);
            current = span;
        }
    }

    merged.push(current);
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bold() {
        let rt = parse_markdown("Hello **world**!");
        assert_eq!(rt.text, "Hello world!");
        assert_eq!(rt.spans.len(), 1);
        assert!(rt.spans[0].attrs.bold);
        // "world" starts at byte 6
        assert_eq!(rt.spans[0].range.start, 6);
        assert_eq!(rt.spans[0].range.end, 11);
    }

    #[test]
    fn test_parse_italic() {
        let rt = parse_markdown("This is *italic* text");
        assert_eq!(rt.text, "This is italic text");
        assert_eq!(rt.spans.len(), 1);
        assert!(rt.spans[0].attrs.italic);
    }

    #[test]
    fn test_parse_strikethrough() {
        let rt = parse_markdown("Some ~~deleted~~ text");
        assert_eq!(rt.text, "Some deleted text");
        assert_eq!(rt.spans.len(), 1);
        assert!(rt.spans[0].attrs.strikethrough);
    }

    #[test]
    fn test_parse_link() {
        let rt = parse_markdown("Click [here](https://example.com) to visit");
        assert_eq!(rt.text, "Click here to visit");
        assert_eq!(rt.links.len(), 1);
        assert_eq!(rt.links[0].url, "https://example.com");
        // "here" is characters 6-10
        assert_eq!(rt.links[0].char_range.start, 6);
        assert_eq!(rt.links[0].char_range.end, 10);
    }

    #[test]
    fn test_parse_nested_styles() {
        let rt = parse_markdown("***bold italic***");
        assert_eq!(rt.text, "bold italic");
        // Should have spans for both bold and italic
        // They may be merged or separate depending on implementation
        assert!(!rt.spans.is_empty());
        // At least one span should have both bold and italic
        let has_both = rt.spans.iter().any(|s| s.attrs.bold && s.attrs.italic);
        assert!(has_both);
    }

    #[test]
    fn test_parse_bullets() {
        let rt = parse_markdown("- Item 1\n- Item 2\n- Item 3");
        assert_eq!(rt.bullets.len(), 3);
        assert_eq!(rt.bullets[0].indent_level, 0);
        // Text should contain bullet symbols
        assert!(rt.text.contains('•'));
    }

    #[test]
    fn test_parse_nested_bullets() {
        let markdown = "- Top\n    - Nested\n    - Nested 2\n- Top 2";
        let rt = parse_markdown(markdown);
        // Should have 4 items total
        assert_eq!(rt.bullets.len(), 4);
        // Check indent levels
        assert_eq!(rt.bullets[0].indent_level, 0); // Top
        assert_eq!(rt.bullets[1].indent_level, 1); // Nested
        assert_eq!(rt.bullets[2].indent_level, 1); // Nested 2
        assert_eq!(rt.bullets[3].indent_level, 0); // Top 2
    }

    #[test]
    fn test_parse_plain_text() {
        let rt = parse_markdown("Just plain text");
        assert_eq!(rt.text, "Just plain text");
        assert!(rt.spans.is_empty());
        assert!(rt.links.is_empty());
        assert!(rt.bullets.is_empty());
    }

    #[test]
    fn test_parse_mixed() {
        let rt = parse_markdown("**Bold** and *italic* and [link](url) and ~~strike~~");
        assert!(!rt.spans.is_empty());
        assert_eq!(rt.links.len(), 1);
        let has_bold = rt.spans.iter().any(|s| s.attrs.bold);
        let has_italic = rt.spans.iter().any(|s| s.attrs.italic);
        let has_strike = rt.spans.iter().any(|s| s.attrs.strikethrough);
        assert!(has_bold);
        assert!(has_italic);
        assert!(has_strike);
    }
}
