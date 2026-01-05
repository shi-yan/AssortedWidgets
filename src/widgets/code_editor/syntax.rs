// Syntax highlighting for code editor
//
// Phase 3 uses regex-based highlighting for simplicity.
// Phase 5+ will add tree-sitter support for more accurate highlighting.

use std::ops::Range;
use regex::Regex;
use super::theme::{
    COLOR_DEFAULT, COLOR_KEYWORD, COLOR_STRING, COLOR_COMMENT,
    COLOR_NUMBER, COLOR_FUNCTION, COLOR_OPERATOR, COLOR_BRACKET,
};

/// Syntax highlighting span (byte range + color index)
pub type SyntaxSpan = (Range<usize>, u8);

/// Trait for syntax highlighters
pub trait SyntaxHighlighter: Send + Sync {
    /// Highlight text and return color spans
    ///
    /// Returns a list of (byte_range, color_index) pairs.
    /// Color indices match the minimap palette constants.
    fn highlight(&self, text: &str) -> Vec<SyntaxSpan>;
}

/// Simple regex-based syntax highlighter for Rust code
pub struct RegexHighlighter {
    rules: Vec<(Regex, u8)>,
}

impl RegexHighlighter {
    /// Create a new regex highlighter with Rust syntax rules
    pub fn rust() -> Self {
        let rules = vec![
            // Comments (highest priority to avoid highlighting inside comments)
            (Regex::new(r"//.*").unwrap(), COLOR_COMMENT),
            (Regex::new(r"/\*[\s\S]*?\*/").unwrap(), COLOR_COMMENT),

            // Strings
            (Regex::new(r#""(?:[^"\\]|\\.)*""#).unwrap(), COLOR_STRING),
            (Regex::new(r#"'(?:[^'\\]|\\.)'"#).unwrap(), COLOR_STRING),

            // Numbers
            (Regex::new(r"\b\d+\.?\d*\b").unwrap(), COLOR_NUMBER),
            (Regex::new(r"\b0x[0-9a-fA-F]+\b").unwrap(), COLOR_NUMBER),

            // Keywords
            (Regex::new(r"\b(fn|let|mut|pub|impl|struct|enum|trait|use|mod|crate|super|self|const|static|match|if|else|while|for|loop|break|continue|return|as|in|where|unsafe|async|await|move|ref)\b").unwrap(), COLOR_KEYWORD),

            // Function calls (word followed by ()
            (Regex::new(r"\b([a-z_][a-zA-Z0-9_]*)\s*\(").unwrap(), COLOR_FUNCTION),

            // Type names (capitalized words)
            (Regex::new(r"\b[A-Z][a-zA-Z0-9_]*\b").unwrap(), COLOR_FUNCTION), // Reuse function color

            // Brackets (for folding - highlighted in gold)
            (Regex::new(r"[{}\[\]()]").unwrap(), COLOR_BRACKET),

            // Operators (kept for completeness, uses default color)
            (Regex::new(r"[+\-*/=<>!&|^~%]+").unwrap(), COLOR_OPERATOR),
        ];

        Self { rules }
    }

    /// Create a plain highlighter (no syntax highlighting)
    pub fn plain() -> Self {
        Self { rules: vec![] }
    }
}

impl SyntaxHighlighter for RegexHighlighter {
    fn highlight(&self, text: &str) -> Vec<SyntaxSpan> {
        let mut spans = Vec::new();

        for (regex, color_index) in &self.rules {
            for mat in regex.find_iter(text) {
                spans.push((mat.range(), *color_index));
            }
        }

        // Sort by start position for consistent ordering
        spans.sort_by_key(|(range, _)| range.start);

        // Remove overlapping spans (keep first match)
        let mut non_overlapping = Vec::new();
        let mut last_end = 0;

        for (range, color) in spans {
            if range.start >= last_end {
                last_end = range.end;
                non_overlapping.push((range, color));
            }
        }

        non_overlapping
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_highlighting() {
        let highlighter = RegexHighlighter::rust();
        let code = "fn main() { let x = 42; }";

        let spans = highlighter.highlight(code);

        // Should highlight "fn" and "let"
        assert!(spans.iter().any(|(range, color)| {
            &code[range.clone()] == "fn" && *color == COLOR_KEYWORD
        }));

        assert!(spans.iter().any(|(range, color)| {
            &code[range.clone()] == "let" && *color == COLOR_KEYWORD
        }));
    }

    #[test]
    fn test_string_highlighting() {
        let highlighter = RegexHighlighter::rust();
        let code = r#"let s = "hello";"#;

        let spans = highlighter.highlight(code);

        // Should highlight the string
        assert!(spans.iter().any(|(range, color)| {
            &code[range.clone()] == r#""hello""# && *color == COLOR_STRING
        }));
    }

    #[test]
    fn test_comment_highlighting() {
        let highlighter = RegexHighlighter::rust();
        let code = "// This is a comment\nlet x = 1;";

        let spans = highlighter.highlight(code);

        // Should highlight the comment
        assert!(spans.iter().any(|(range, color)| {
            code[range.clone()].starts_with("//") && *color == COLOR_COMMENT
        }));
    }

    #[test]
    fn test_number_highlighting() {
        let highlighter = RegexHighlighter::rust();
        let code = "let x = 42;";

        let spans = highlighter.highlight(code);

        // Should highlight the number
        assert!(spans.iter().any(|(range, color)| {
            &code[range.clone()] == "42" && *color == COLOR_NUMBER
        }));
    }

    #[test]
    fn test_function_highlighting() {
        let highlighter = RegexHighlighter::rust();
        let code = "println!(\"test\");";

        let spans = highlighter.highlight(code);

        // Should highlight "println"
        assert!(spans.iter().any(|(range, color)| {
            code[range.clone()].contains("println") && *color == COLOR_FUNCTION
        }));
    }

    #[test]
    fn test_plain_highlighter() {
        let highlighter = RegexHighlighter::plain();
        let code = "fn main() {}";

        let spans = highlighter.highlight(code);

        // Should return no spans
        assert_eq!(spans.len(), 0);
    }

    #[test]
    fn test_no_overlapping_spans() {
        let highlighter = RegexHighlighter::rust();
        let code = "let let_variable = 42;";

        let spans = highlighter.highlight(code);

        // Verify no overlapping spans
        for i in 0..spans.len().saturating_sub(1) {
            assert!(spans[i].0.end <= spans[i + 1].0.start);
        }
    }
}
