//! Rich text label widget with limited markdown support
//!
//! Features:
//! - Limited markdown: bold, italic, strikethrough, links, bullets
//! - Optional text wrapping
//! - Line-based vertical scrolling (discrete)
//! - Pixel-based horizontal scrolling (smooth, when wrapping disabled)
//! - Clickable links with cursor changes
//! - Embedded scrollbars (overflow:auto behavior)

mod types;
mod markdown;
mod widget;

pub use types::{RichText, Span, SpanAttrs, LinkSpan, BulletItem};
pub use markdown::parse_markdown;
pub use widget::RichTextLabel;
