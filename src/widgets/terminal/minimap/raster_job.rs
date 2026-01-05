/// Background rasterization job types
///
/// Phase 5: Multi-threading support for off-screen page rasterization.
/// Strategy: Extract grid data on main thread, rasterize in background.

use std::sync::Arc;
use crate::widgets::code_editor::CharSheet;

/// Snapshot of a single terminal cell
///
/// Owned data that can be sent to background threads.
#[derive(Debug, Clone)]
pub struct CellSnapshot {
    /// Character to render
    pub c: char,

    /// Foreground color (RGB)
    pub fg: (u8, u8, u8),

    /// Background color (RGB)
    pub bg: (u8, u8, u8),

    /// Cell width (1 for narrow, 2 for wide/CJK)
    pub width: u8,
}

impl CellSnapshot {
    /// Create a snapshot of a cell
    pub fn new(c: char, fg: (u8, u8, u8), bg: (u8, u8, u8), width: u8) -> Self {
        Self { c, fg, bg, width }
    }

    /// Create an empty cell
    pub fn empty() -> Self {
        Self {
            c: ' ',
            fg: (229, 229, 229),  // Default foreground
            bg: (15, 15, 20),     // Default background
            width: 1,
        }
    }
}

/// Snapshot of a single grid line
///
/// Contains all cells for one terminal line.
#[derive(Debug, Clone)]
pub struct GridLine {
    /// Cell snapshots for this line
    pub cells: Vec<CellSnapshot>,
}

impl GridLine {
    /// Create a new grid line with the given cells
    pub fn new(cells: Vec<CellSnapshot>) -> Self {
        Self { cells }
    }

    /// Create an empty line with the given width
    pub fn empty(width: usize) -> Self {
        Self {
            cells: vec![CellSnapshot::empty(); width],
        }
    }
}

/// Job for background rasterization
///
/// Contains all data needed to rasterize a page without accessing the Grid.
pub struct RasterJob {
    /// Page number being rasterized
    pub page_num: usize,

    /// Grid generation at job creation
    /// If grid generation changes (resize), this job is stale
    pub grid_generation: usize,

    /// Start line in grid (for debugging/validation)
    pub start_line: usize,

    /// Grid data snapshot (owned, can be sent to background thread)
    pub grid_snapshot: Vec<GridLine>,

    /// Minimap dimensions
    pub width_pixels: usize,
    pub height_pixels: usize,

    /// Shared character sheet (immutable, Arc)
    pub char_sheet: Arc<CharSheet>,

    /// Priority (true = high priority, false = low priority)
    pub high_priority: bool,
}

/// Result from background rasterization
///
/// Contains rasterized pixel data to be uploaded to the page.
pub struct RasterResult {
    /// Page number that was rasterized
    pub page_num: usize,

    /// Grid generation this result is for
    /// If current generation differs, result is stale
    pub grid_generation: usize,

    /// Rasterized intensity data
    pub intensity: Vec<u8>,

    /// Rasterized color data (RGB565)
    pub color_rgb565: Vec<u16>,
}

impl RasterResult {
    /// Create a new raster result
    pub fn new(
        page_num: usize,
        grid_generation: usize,
        intensity: Vec<u8>,
        color_rgb565: Vec<u16>,
    ) -> Self {
        Self {
            page_num,
            grid_generation,
            intensity,
            color_rgb565,
        }
    }
}
