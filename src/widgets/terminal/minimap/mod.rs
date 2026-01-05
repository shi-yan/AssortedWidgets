// Terminal minimap module
//
// Provides visual overview of terminal content for navigation.
// Phase 2: Synchronous rasterization.
// Phase 3: Incremental updates with dirty tracking.
// Phase 5: Multi-threaded background rasterization.

mod color;
mod page;
mod range_set;
mod raster_job;

pub use color::{encode_rgb565, decode_rgb565};
pub use page::{TerminalMinimapPage, PageStatus};
pub use range_set::RangeSet;
pub use raster_job::{CellSnapshot, GridLine, RasterJob, RasterResult};

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

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
/// Phase 3: Supports incremental updates with dirty line tracking.
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

    /// Dirty line tracking (Phase 3)
    /// Tracks which lines have changed and need rasterization
    dirty_lines: RangeSet,

    /// Last update total lines count (for detecting scrollback changes)
    last_total_lines: usize,

    /// Debouncing: Last rasterization time
    last_rasterize_time: Option<Instant>,

    /// Debouncing: Minimum time between rasterizations (milliseconds)
    debounce_interval_ms: u64,

    /// Flag indicating pending updates (for deferred rasterization)
    has_pending_updates: bool,
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
            dirty_lines: RangeSet::new(),
            last_total_lines: 0,
            last_rasterize_time: None,
            debounce_interval_ms: 100,  // 100ms debounce (adjustable)
            has_pending_updates: false,
        }
    }

    /// Set debounce interval
    ///
    /// # Arguments
    /// * `interval_ms` - Minimum milliseconds between rasterizations
    pub fn set_debounce_interval(&mut self, interval_ms: u64) {
        self.debounce_interval_ms = interval_ms;
    }

    /// Check if there are pending updates that need rasterization
    pub fn has_pending_updates(&self) -> bool {
        self.has_pending_updates
    }

    /// Force immediate rasterization of all pending updates
    ///
    /// Bypasses debouncing to ensure minimap is fully up-to-date.
    /// Useful for idle periods or when user requests manual refresh.
    pub fn force_update<L: EventListener>(&mut self, term: &Term<L>) {
        if self.has_pending_updates || !self.dirty_lines.is_empty() {
            self.rasterize_dirty_pages(term);
            self.last_rasterize_time = Some(Instant::now());
            self.has_pending_updates = false;
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

        // Mark all lines as dirty (Phase 3)
        if self.last_total_lines > 0 {
            self.dirty_lines.add_range(0..self.last_total_lines);
        }
    }

    /// Mark a single line as dirty
    ///
    /// # Arguments
    /// * `line` - Line number to mark dirty
    pub fn mark_dirty_line(&mut self, line: usize) {
        self.dirty_lines.add_line(line);
    }

    /// Mark a range of lines as dirty
    ///
    /// # Arguments
    /// * `start_line` - First line to mark dirty
    /// * `end_line` - One past the last line to mark dirty
    pub fn mark_dirty_range(&mut self, start_line: usize, end_line: usize) {
        self.dirty_lines.add_range(start_line..end_line);
    }

    /// Get pages that overlap with dirty lines
    fn get_dirty_pages(&self) -> Vec<usize> {
        if self.dirty_lines.is_empty() {
            return Vec::new();
        }

        let mut dirty_pages = Vec::new();
        for range in self.dirty_lines.ranges() {
            let start_page = range.start / MINIMAP_PAGE_SIZE;
            let end_page = (range.end.saturating_sub(1)) / MINIMAP_PAGE_SIZE;

            for page_num in start_page..=end_page {
                if self.pages.contains_key(&page_num) && !dirty_pages.contains(&page_num) {
                    dirty_pages.push(page_num);
                }
            }
        }

        dirty_pages
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
    /// Phase 3: Uses dirty line tracking, debouncing, and visible-first strategy.
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

        // Detect scrollback changes (new content appeared)
        if total_lines != self.last_total_lines {
            if total_lines > self.last_total_lines {
                // New lines added, mark them as dirty
                self.dirty_lines.add_range(self.last_total_lines..total_lines);
                self.has_pending_updates = true;
            } else {
                // Lines removed (rare, but possible) - invalidate all
                self.invalidate_all();
                self.has_pending_updates = true;
            }
            self.last_total_lines = total_lines;
        }

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
                // New pages are automatically dirty (they need initial rasterization)
                self.dirty_lines.add_range(start_line..start_line + line_count);
                self.has_pending_updates = true;
                self.pages.insert(page_num, page);
            }
        }

        // Phase 3: Debounced rasterization with visible-first strategy
        // Check if enough time has passed since last rasterization
        let should_rasterize = if let Some(last_time) = self.last_rasterize_time {
            let elapsed = last_time.elapsed();
            elapsed >= Duration::from_millis(self.debounce_interval_ms)
        } else {
            true  // First rasterization, always proceed
        };

        // Always rasterize if visible page is dirty (prioritize user experience)
        let visible_page_dirty = if let Some(visible_page) = self.visible_page {
            let start_line = visible_page * MINIMAP_PAGE_SIZE;
            let end_line = start_line + MINIMAP_PAGE_SIZE;
            self.dirty_lines.intersects(&(start_line..end_line))
        } else {
            false
        };

        if should_rasterize || visible_page_dirty || !self.has_pending_updates {
            if self.has_pending_updates || !self.dirty_lines.is_empty() {
                self.rasterize_dirty_pages(term);
                self.last_rasterize_time = Some(Instant::now());
                self.has_pending_updates = false;
            }
        }
        // Otherwise, defer rasterization (updates are batched)
    }

    /// Rasterize dirty pages (Phase 3: visible-first, incremental)
    ///
    /// Strategy:
    /// 1. Rasterize visible page first (if dirty)
    /// 2. Rasterize other dirty pages
    /// 3. Only process pages that have dirty lines or are marked stale
    fn rasterize_dirty_pages<L: EventListener>(&mut self, term: &Term<L>) {
        // Get pages that need rasterization
        let dirty_pages = self.get_dirty_pages();
        let stale_pages: Vec<usize> = self.pages.iter()
            .filter(|(_, p)| p.status == PageStatus::Stale)
            .map(|(num, _)| *num)
            .collect();

        // Combine dirty and stale pages
        let mut pages_to_update: Vec<usize> = dirty_pages.clone();
        for page_num in stale_pages {
            if !pages_to_update.contains(&page_num) {
                pages_to_update.push(page_num);
            }
        }

        // Phase 3: Visible-first strategy
        // Sort so visible page is first
        if let Some(visible_page) = self.visible_page {
            pages_to_update.sort_by_key(|&page_num| {
                if page_num == visible_page {
                    0  // Visible page first
                } else {
                    // Other pages by distance from visible
                    page_num.abs_diff(visible_page) + 1
                }
            });
        }

        // Rasterize pages
        let char_sheet = self.char_sheet.clone();
        for page_num in pages_to_update {
            if let Some(page) = self.pages.get_mut(&page_num) {
                Self::rasterize_page(page_num, page, term, &char_sheet);
                page.status = PageStatus::Clean;
            }
        }

        // Clear dirty lines that have been rasterized
        for &page_num in &dirty_pages {
            let start_line = page_num * MINIMAP_PAGE_SIZE;
            let page = self.pages.get(&page_num).unwrap();
            let end_line = start_line + page.line_count;
            self.dirty_lines.remove_range(&(start_line..end_line));
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
