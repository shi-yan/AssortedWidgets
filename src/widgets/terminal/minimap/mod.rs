// Terminal minimap module
//
// Provides visual overview of terminal content for navigation.
// Phase 2: Synchronous rasterization.
// Phase 5: Multi-threaded background rasterization.

mod color;
mod page;

pub use color::{encode_rgb565, decode_rgb565};
pub use page::{TerminalMinimapPage, PageStatus};

use std::collections::BTreeMap;
use std::sync::Arc;

use super::config::MINIMAP_PAGE_SIZE;
use crate::widgets::code_editor::CharSheet;

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
