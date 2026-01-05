// Gutter (left sidebar with line numbers)

use crate::paint::PaintContext;
use crate::paint::types::ShapeStyle;
use crate::text::TextStyle;
use crate::types::{Point, Rect, Size};
use super::theme::EditorTheme;

/// Gutter for displaying line numbers
///
/// The gutter appears on the left side of the editor and shows:
/// - Line numbers
/// - Current line indicator
/// - Fold indicators (Phase 3.5)
/// - Breakpoints (future)
pub struct Gutter {
    /// Gutter width in pixels (auto-sized based on max line number)
    pub width: f32,

    /// Padding between gutter edge and line numbers
    padding: f32,

    /// Font size for line numbers
    font_size: f32,
}

impl Gutter {
    /// Create a new gutter
    pub fn new() -> Self {
        Self {
            width: 50.0,  // Initial width, will be updated
            padding: 10.0,
            font_size: 12.0,
        }
    }

    /// Update gutter width based on maximum line number
    ///
    /// This should be called when the document length changes.
    pub fn update_width(&mut self, max_line_number: usize) {
        let digits = if max_line_number == 0 {
            1
        } else {
            max_line_number.to_string().len()
        };

        // Width = digits * 8px + padding on both sides
        self.width = (digits as f32 * 8.0) + (self.padding * 2.0);
    }

    /// Paint the gutter
    ///
    /// # Arguments
    /// * `ctx` - Paint context for drawing
    /// * `editor_bounds` - Full editor bounds
    /// * `theme` - Editor theme for colors
    /// * `visible_lines` - Range of visible line indices (0-indexed)
    /// * `scroll_offset_y` - Vertical scroll offset in pixels
    /// * `line_height` - Height of each line in pixels
    /// * `current_line` - Currently active line (for highlighting)
    pub fn paint(
        &self,
        ctx: &mut PaintContext,
        editor_bounds: Rect,
        theme: &EditorTheme,
        visible_lines: std::ops::Range<usize>,
        scroll_offset_y: f32,
        line_height: f32,
        current_line: Option<usize>,
    ) {
        // Draw gutter background
        let gutter_rect = Rect::new(
            editor_bounds.origin,
            Size::new(self.width as f64, editor_bounds.size.height),
        );

        ctx.draw_styled_rect(
            gutter_rect,
            ShapeStyle::solid(theme.gutter_background),
        );

        // Draw line numbers
        // Start at the top of the viewport and increment for each line
        let mut y = editor_bounds.origin.y as f32 - scroll_offset_y;

        for line_idx in visible_lines {
            // Skip if line is outside viewport
            if y + line_height < editor_bounds.origin.y as f32 {
                y += line_height;
                continue;
            }
            if y > (editor_bounds.origin.y + editor_bounds.size.height) as f32 {
                break;
            }

            // Highlight current line
            if Some(line_idx) == current_line {
                let highlight_rect = Rect::new(
                    Point::new(editor_bounds.origin.x, y as f64),
                    Size::new(self.width as f64, line_height as f64),
                );
                ctx.draw_styled_rect(
                    highlight_rect,
                    ShapeStyle::solid(theme.current_line_background),
                );
            }

            // Draw line number
            let line_number = format!("{}", line_idx + 1); // 1-indexed for display

            let text_color = if Some(line_idx) == current_line {
                theme.line_number_active
            } else {
                theme.line_number
            };

            let text_style = TextStyle::new()
                .size(self.font_size)
                .color(text_color);

            // Right-align line numbers
            let number_width = line_number.len() as f32 * 8.0;
            let x = editor_bounds.origin.x as f32 + self.width - self.padding - number_width;

            ctx.draw_text(
                &line_number,
                &text_style,
                Point::new(x as f64, y as f64 + 4.0),
                None,
            );

            y += line_height;
        }
    }
}

impl Default for Gutter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_gutter() {
        let gutter = Gutter::new();
        assert!(gutter.width > 0.0);
        assert!(gutter.padding > 0.0);
    }

    #[test]
    fn test_update_width() {
        let mut gutter = Gutter::new();

        // Test with 2-digit line numbers (99 lines)
        gutter.update_width(99);
        let width_2_digits = gutter.width;

        // Test with 4-digit line numbers (1000 lines)
        gutter.update_width(1000);
        let width_4_digits = gutter.width;

        // Width should increase with more digits
        assert!(width_4_digits > width_2_digits);
    }

    #[test]
    fn test_width_calculation() {
        let mut gutter = Gutter::new();

        // 1 digit: 1 * 8 + 2 * 10 = 28
        gutter.update_width(9);
        assert_eq!(gutter.width, 28.0);

        // 2 digits: 2 * 8 + 2 * 10 = 36
        gutter.update_width(99);
        assert_eq!(gutter.width, 36.0);

        // 3 digits: 3 * 8 + 2 * 10 = 44
        gutter.update_width(999);
        assert_eq!(gutter.width, 44.0);
    }

    #[test]
    fn test_edge_case_zero_lines() {
        let mut gutter = Gutter::new();
        gutter.update_width(0);

        // Should still have at least 1 digit width
        assert_eq!(gutter.width, 28.0); // 1 * 8 + 20
    }
}
