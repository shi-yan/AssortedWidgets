// Minimap manager for code editor
//
// Provides a pixel-perfect minimap view of the entire document.
// Uses background rasterization for smooth performance.

use std::collections::BTreeMap;
use std::sync::Arc;

mod char_sheet;
mod palette;
mod page;

pub use char_sheet::{CharSheet, GlyphWidth};
pub use palette::{MinimapPalette, PaletteEntry, COLOR_DEFAULT, COLOR_KEYWORD, COLOR_STRING, COLOR_COMMENT};
pub use page::{MinimapPage, PageStatus};

use super::model::EditorModel;
use super::config::MINIMAP_PAGE_SIZE;

/// Minimap manager
///
/// Manages minimap pages and handles rasterization.
/// For Phase 2, this uses synchronous rasterization.
/// Threading will be added in later iterations.
pub struct MinimapManager {
    /// Page storage (keyed by page number)
    pages: BTreeMap<usize, MinimapPage>,

    /// Character sheet for glyph rendering
    char_sheet: Arc<CharSheet>,

    /// Color palette
    palette: MinimapPalette,

    /// Minimap width in pixels
    width_pixels: usize,

    /// Currently visible page (for prioritization)
    visible_page: Option<usize>,
}

impl MinimapManager {
    /// Create a new minimap manager
    ///
    /// # Arguments
    /// * `width_pixels` - Width of the minimap in pixels
    pub fn new(width_pixels: usize) -> Self {
        Self {
            pages: BTreeMap::new(),
            char_sheet: Arc::new(CharSheet::with_defaults()),
            palette: MinimapPalette::with_defaults(),
            width_pixels,
            visible_page: None,
        }
    }

    /// Update minimap for the current editor state
    ///
    /// This should be called each frame to keep the minimap in sync.
    ///
    /// # Arguments
    /// * `model` - The editor's text model
    /// * `center_line` - The line currently at the center of the viewport
    pub fn update(&mut self, model: &EditorModel, center_line: usize) {
        // Determine which page is currently visible
        self.visible_page = Some(center_line / MINIMAP_PAGE_SIZE);

        // Create pages as needed
        let total_lines = model.len_lines();
        let num_pages = (total_lines + MINIMAP_PAGE_SIZE - 1) / MINIMAP_PAGE_SIZE;

        // Ensure all needed pages exist
        for page_num in 0..num_pages {
            if !self.pages.contains_key(&page_num) {
                let start_line = page_num * MINIMAP_PAGE_SIZE;
                let line_count = (MINIMAP_PAGE_SIZE).min(total_lines - start_line);
                let page = MinimapPage::new(start_line, line_count, self.width_pixels);
                self.pages.insert(page_num, page);
            }
        }

        // Rasterize dirty pages (synchronous for Phase 2)
        self.rasterize_dirty_pages(model);
    }

    /// Rasterize all dirty pages
    fn rasterize_dirty_pages(&mut self, model: &EditorModel) {
        let char_sheet = &self.char_sheet;
        for (page_num, page) in self.pages.iter_mut() {
            if page.status == PageStatus::Dirty {
                Self::rasterize_page(*page_num, page, model, char_sheet);
            }
        }
    }

    /// Rasterize a single page
    fn rasterize_page(
        page_num: usize,
        page: &mut MinimapPage,
        model: &EditorModel,
        char_sheet: &CharSheet,
    ) {
        page.clear();

        let start_line = page_num * MINIMAP_PAGE_SIZE;
        let end_line = (start_line + page.line_count).min(model.len_lines());

        for line_idx in start_line..end_line {
            if let Some(line_text) = model.line(line_idx) {
                let page_line_idx = line_idx - start_line;
                page.rasterize_line(
                    page_line_idx,
                    &line_text,
                    char_sheet,
                    COLOR_DEFAULT,
                );
            }
        }

        page.mark_clean();
    }

    /// Get a page by number
    pub fn get_page(&self, page_num: usize) -> Option<&MinimapPage> {
        self.pages.get(&page_num)
    }

    /// Get a mutable page by number
    pub fn get_page_mut(&mut self, page_num: usize) -> Option<&mut MinimapPage> {
        self.pages.get_mut(&page_num)
    }

    /// Mark a range of lines as dirty
    ///
    /// Call this when text changes to trigger re-rasterization.
    pub fn invalidate_lines(&mut self, line_range: std::ops::Range<usize>) {
        let start_page = line_range.start / MINIMAP_PAGE_SIZE;
        let end_page = line_range.end / MINIMAP_PAGE_SIZE;

        for page_num in start_page..=end_page {
            if let Some(page) = self.pages.get_mut(&page_num) {
                page.mark_dirty();
            }
        }
    }

    /// Clear all pages
    pub fn clear(&mut self) {
        self.pages.clear();
    }

    /// Get the number of pages
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Get the color palette
    pub fn palette(&self) -> &MinimapPalette {
        &self.palette
    }

    /// Get the character sheet
    pub fn char_sheet(&self) -> &CharSheet {
        &self.char_sheet
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
        let manager = MinimapManager::new(100);
        assert_eq!(manager.page_count(), 0);
        assert_eq!(manager.width_pixels, 100);
    }

    #[test]
    fn test_update_creates_pages() {
        let mut manager = MinimapManager::new(100);
        let mut model = EditorModel::new();

        // Create text with 1000 lines
        let text = (0..1000)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        model.set_text(&text);

        manager.update(&model, 500);

        // Should have created 2 pages (512 lines each)
        assert!(manager.page_count() >= 2);
    }

    #[test]
    fn test_invalidate_lines() {
        let mut manager = MinimapManager::new(100);
        let mut model = EditorModel::new();

        let text = (0..1000)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        model.set_text(&text);

        manager.update(&model, 0);

        // Mark first page as clean
        if let Some(page) = manager.get_page_mut(0) {
            page.mark_clean();
        }

        // Invalidate lines 10-20 (should mark page 0 as dirty)
        manager.invalidate_lines(10..20);

        if let Some(page) = manager.get_page(0) {
            assert_eq!(page.status, PageStatus::Dirty);
        }
    }

    #[test]
    fn test_memory_usage() {
        let mut manager = MinimapManager::new(100);
        let mut model = EditorModel::new();

        let text = (0..100)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        model.set_text(&text);

        manager.update(&model, 0);

        let usage = manager.memory_usage();
        assert!(usage > 0);
    }

    #[test]
    fn test_clear() {
        let mut manager = MinimapManager::new(100);
        let mut model = EditorModel::new();

        model.set_text("Hello\nWorld");
        manager.update(&model, 0);

        assert!(manager.page_count() > 0);

        manager.clear();
        assert_eq!(manager.page_count(), 0);
    }
}
