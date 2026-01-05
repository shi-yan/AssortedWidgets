// Terminal minimap module
//
// Provides visual overview of terminal content for navigation.
// Phase 2: Synchronous rasterization.
// Phase 3: Incremental updates with dirty tracking.
// Phase 5: Multi-threaded background rasterization.

mod color;
mod page;
mod range_set;

pub use color::{encode_rgb565, decode_rgb565};
pub use page::{TerminalMinimapPage, PageStatus};
pub use range_set::RangeSet;

use std::collections::BTreeMap;
use std::sync::Arc;

use super::config::MINIMAP_PAGE_SIZE;
use crate::widgets::code_editor::CharSheet;

use alacritty_terminal::term::Term;
use alacritty_terminal::event::EventListener;
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Line, Column};
use alacritty_terminal::vte::ansi::Color as AnsiColor;

/// Terminal minimap manager
///
/// Manages minimap pages for the terminal.
/// Unlike code editor, pages may not align with newline boundaries.
pub struct TerminalMinimapManager {
    /// Page storage (keyed by page number)
    pages: BTreeMap<usize, TerminalMinimapPage>,

    /// Character sheet for glyph rendering (shared with code editor)
    char_sheet: Arc<CharSheet>,

    /// Minimap width in pixels
    width_pixels: usize,

    /// Grid generation counter (incremented on reflow)
    grid_generation: usize,

    /// Currently visible page (for prioritization)
    visible_page: Option<usize>,
}

impl TerminalMinimapManager {
    /// Create a new minimap manager
    ///
    /// # Arguments
    /// * `width_pixels` - Width of the minimap in pixels
    /// * `char_sheet` - Shared character sheet for micro-glyph rendering
    pub fn new(width_pixels: usize, char_sheet: Arc<CharSheet>) -> Self {
        Self {
            pages: BTreeMap::new(),
            char_sheet,
            width_pixels,
            grid_generation: 0,
            visible_page: None,
        }
    }

    /// Get the number of pages
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Get a page by number
    pub fn get_page(&self, page_num: usize) -> Option<&TerminalMinimapPage> {
        self.pages.get(&page_num)
    }

    /// Get a mutable page by number
    pub fn get_page_mut(&mut self, page_num: usize) -> Option<&mut TerminalMinimapPage> {
        self.pages.get_mut(&page_num)
    }

    /// Invalidate all pages (called on grid reflow)
    pub fn invalidate_all(&mut self) {
        self.grid_generation += 1;

        for page in self.pages.values_mut() {
            page.status = PageStatus::Stale;
            page.grid_generation = self.grid_generation;
        }
    }

    /// Clear all pages
    pub fn clear(&mut self) {
        self.pages.clear();
    }

    /// Get total memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        self.pages.values().map(|p| p.byte_size()).sum()
    }

    /// Update minimap for the current terminal state
    ///
    /// Creates pages as needed and rasterizes dirty pages.
    ///
    /// # Arguments
    /// * `term` - Terminal state
    /// * `visible_line` - Currently visible line (for prioritization)
    pub fn update<L: EventListener>(&mut self, term: &Term<L>, visible_line: usize) {
        // Determine which page is currently visible
        self.visible_page = Some(visible_line / MINIMAP_PAGE_SIZE);

        // Calculate total lines (scrollback + active screen)
        let total_lines = term.grid().history_size() + term.grid().screen_lines();
        let num_pages = (total_lines + MINIMAP_PAGE_SIZE - 1) / MINIMAP_PAGE_SIZE;

        // Ensure all needed pages exist
        for page_num in 0..num_pages {
            if !self.pages.contains_key(&page_num) {
                let start_line = page_num * MINIMAP_PAGE_SIZE;
                let line_count = (MINIMAP_PAGE_SIZE).min(total_lines - start_line);
                let page = TerminalMinimapPage::new(
                    start_line,
                    line_count,
                    self.width_pixels,
                    self.grid_generation,
                );
                self.pages.insert(page_num, page);
            }
        }

        // Rasterize dirty pages (synchronous for Phase 2)
        self.rasterize_dirty_pages(term);
    }

    /// Rasterize all dirty pages
    fn rasterize_dirty_pages<L: EventListener>(&mut self, term: &Term<L>) {
        let char_sheet = &self.char_sheet;
        for (page_num, page) in self.pages.iter_mut() {
            if page.status == PageStatus::Dirty || page.status == PageStatus::Stale {
                Self::rasterize_page(*page_num, page, term, char_sheet);
            }
        }
    }

    /// Rasterize a single page from the terminal grid
    ///
    /// # Arguments
    /// * `page_num` - Page number
    /// * `page` - Minimap page to rasterize into
    /// * `term` - Terminal state
    /// * `char_sheet` - Character sheet for micro-glyphs
    fn rasterize_page<L: EventListener>(
        page_num: usize,
        page: &mut TerminalMinimapPage,
        term: &Term<L>,
        char_sheet: &CharSheet,
    ) {
        page.clear();

        let grid = term.grid();
        let start_line = page_num * MINIMAP_PAGE_SIZE;
        let end_line = (start_line + page.line_count).min(grid.history_size() + grid.screen_lines());

        // Convert to grid line indices
        // Lines are indexed from top of scrollback (negative indices)
        let display_offset = grid.display_offset();

        for line_idx in start_line..end_line {
            // Calculate grid line index
            // Scrollback lines are negative, active screen lines are positive
            let grid_line_offset = line_idx as i32 - grid.history_size() as i32;
            let line = Line(grid_line_offset);

            // Calculate Y position in minimap page (2 pixels per line)
            let page_line = line_idx - start_line;
            let y_base = page_line * 2;

            // Track X position in minimap
            let mut x = 0;

            // Iterate through columns
            for col_idx in 0..grid.columns() {
                if x >= page.width_pixels {
                    break;
                }

                let column = Column(col_idx);

                // Try to get cell - skip if out of bounds
                let cell = match grid.get(line, column) {
                    Some(cell) => cell,
                    None => continue,
                };

                // Get character and render as micro-glyph
                let c = cell.c;
                if c == ' ' || c == '\0' {
                    // Skip empty cells
                    x += 1; // Still advance position
                    continue;
                }

                let (glyph_pixels, width) = char_sheet.render_char(c);

                // Convert ANSI color to RGB
                let (fg_r, fg_g, fg_b) = ansi_color_to_rgb(&cell.fg);

                if width == 1 {
                    // Narrow glyph (1x2 pixels)
                    if x < page.width_pixels {
                        page.set_pixel_rgb(x, y_base, glyph_pixels[0], fg_r, fg_g, fg_b);
                        page.set_pixel_rgb(x, y_base + 1, glyph_pixels[1], fg_r, fg_g, fg_b);
                        x += 1;
                    }
                } else {
                    // Wide glyph (2x2 pixels)
                    if x + 1 < page.width_pixels {
                        page.set_pixel_rgb(x, y_base, glyph_pixels[0], fg_r, fg_g, fg_b);
                        page.set_pixel_rgb(x + 1, y_base, glyph_pixels[1], fg_r, fg_g, fg_b);
                        page.set_pixel_rgb(x, y_base + 1, glyph_pixels[2], fg_r, fg_g, fg_b);
                        page.set_pixel_rgb(x + 1, y_base + 1, glyph_pixels[3], fg_r, fg_g, fg_b);
                        x += 2;
                    }
                }
            }
        }

        page.mark_clean();
    }
}

/// Convert ANSI color to RGB
fn ansi_color_to_rgb(ansi_color: &AnsiColor) -> (u8, u8, u8) {
    use alacritty_terminal::vte::ansi::NamedColor;

    match ansi_color {
        AnsiColor::Named(named) => match named {
            NamedColor::Black => (0, 0, 0),
            NamedColor::Red => (205, 49, 49),
            NamedColor::Green => (13, 188, 121),
            NamedColor::Yellow => (229, 229, 16),
            NamedColor::Blue => (36, 114, 200),
            NamedColor::Magenta => (188, 63, 188),
            NamedColor::Cyan => (17, 168, 205),
            NamedColor::White => (229, 229, 229),
            NamedColor::BrightBlack => (102, 102, 102),
            NamedColor::BrightRed => (241, 76, 76),
            NamedColor::BrightGreen => (35, 209, 139),
            NamedColor::BrightYellow => (245, 245, 67),
            NamedColor::BrightBlue => (59, 142, 234),
            NamedColor::BrightMagenta => (214, 112, 214),
            NamedColor::BrightCyan => (41, 184, 219),
            NamedColor::BrightWhite => (255, 255, 255),
            NamedColor::Foreground => (229, 229, 229),
            NamedColor::Background => (15, 15, 20),
            _ => (229, 229, 229),
        },
        AnsiColor::Spec(rgb) => (rgb.r, rgb.g, rgb.b),
        AnsiColor::Indexed(idx) => {
            // Simplified 256 color palette
            if *idx < 16 {
                let named = match idx {
                    0 => NamedColor::Black,
                    1 => NamedColor::Red,
                    2 => NamedColor::Green,
                    3 => NamedColor::Yellow,
                    4 => NamedColor::Blue,
                    5 => NamedColor::Magenta,
                    6 => NamedColor::Cyan,
                    7 => NamedColor::White,
                    8 => NamedColor::BrightBlack,
                    9 => NamedColor::BrightRed,
                    10 => NamedColor::BrightGreen,
                    11 => NamedColor::BrightYellow,
                    12 => NamedColor::BrightBlue,
                    13 => NamedColor::BrightMagenta,
                    14 => NamedColor::BrightCyan,
                    _ => NamedColor::BrightWhite,
                };
                ansi_color_to_rgb(&AnsiColor::Named(named))
            } else {
                // Grayscale or 256 color (placeholder)
                let gray = ((idx - 16) * 10).min(255) as u8;
                (gray, gray, gray)
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_manager() {
        let char_sheet = Arc::new(CharSheet::with_defaults());
        let manager = TerminalMinimapManager::new(100, char_sheet);
        assert_eq!(manager.page_count(), 0);
        assert_eq!(manager.width_pixels, 100);
    }

    #[test]
    fn test_invalidate_all() {
        let char_sheet = Arc::new(CharSheet::with_defaults());
        let mut manager = TerminalMinimapManager::new(100, char_sheet);

        // Create a page
        let page = TerminalMinimapPage::new(0, 512, 100, 0);
        manager.pages.insert(0, page);

        let initial_gen = manager.grid_generation;
        manager.invalidate_all();

        assert_eq!(manager.grid_generation, initial_gen + 1);
        assert_eq!(manager.pages[&0].status, PageStatus::Stale);
    }

    #[test]
    fn test_memory_usage() {
        let char_sheet = Arc::new(CharSheet::with_defaults());
        let mut manager = TerminalMinimapManager::new(100, char_sheet);

        let page = TerminalMinimapPage::new(0, 512, 100, 0);
        manager.pages.insert(0, page);

        let usage = manager.memory_usage();
        assert!(usage > 0);
    }
}
