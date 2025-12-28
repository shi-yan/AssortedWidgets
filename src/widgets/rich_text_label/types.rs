//! Data structures for rich text representation

use crate::paint::Color;

/// Styled span of text
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    /// Byte range in the full text string
    pub range: std::ops::Range<usize>,

    /// Text attributes (bold, italic, color, etc.)
    pub attrs: SpanAttrs,
}

/// Text styling attributes
#[derive(Clone, Debug, PartialEq)]
pub struct SpanAttrs {
    /// Bold text (weight: 700)
    pub bold: bool,

    /// Italic text
    pub italic: bool,

    /// Strikethrough text (rendered manually, not via cosmic-text)
    pub strikethrough: bool,

    /// Text color (if different from base color)
    pub color: Option<Color>,
}

impl Default for SpanAttrs {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            strikethrough: false,
            color: None,
        }
    }
}

/// Link span with URL
#[derive(Clone, Debug, PartialEq)]
pub struct LinkSpan {
    /// Character range (NOT byte range, for easier hit testing with cosmic-text)
    /// cosmic-text's buffer.hit() returns character indices, not byte indices
    pub char_range: std::ops::Range<usize>,

    /// Target URL
    pub url: String,
}

/// Bullet list item
#[derive(Clone, Debug, PartialEq)]
pub struct BulletItem {
    /// Line index where bullet starts (0-based)
    pub line: usize,

    /// Indentation level (0 = top level, 1 = nested, etc.)
    pub indent_level: u16,
}

/// Complete rich text document
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RichText {
    /// Plain text content (all markup stripped)
    pub text: String,

    /// Styled spans (sorted by range.start)
    pub spans: Vec<Span>,

    /// Link spans (subset of spans with URLs)
    pub links: Vec<LinkSpan>,

    /// Bullet list items
    pub bullets: Vec<BulletItem>,
}

impl RichText {
    /// Create a new empty rich text document
    pub fn new() -> Self {
        Self::default()
    }

    /// Create from plain text (no styling)
    pub fn from_plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Check if the document is empty
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Get the plain text length in bytes
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Get the plain text length in characters
    pub fn char_len(&self) -> usize {
        self.text.chars().count()
    }
}
