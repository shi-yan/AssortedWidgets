// Configuration constants and types for the code editor

/// Maximum memory budget for the LRU text layout cache (100 MB)
pub const LRU_CACHE_MAX_SIZE: usize = 100 * 1024 * 1024;

/// Maximum number of entries in the LRU cache
pub const LRU_CACHE_MAX_ENTRIES: usize = 1000;

/// Debounce time for window resize events (milliseconds)
pub const RESIZE_DEBOUNCE_MS: u64 = 100;

/// Debounce time for text edit events (milliseconds)
pub const EDIT_DEBOUNCE_MS: u64 = 50;

/// Number of background threads for minimap rasterization
pub const MINIMAP_THREAD_POOL_SIZE: usize = 4;

/// Number of logical lines per minimap page
pub const MINIMAP_PAGE_SIZE: usize = 512;

/// Growth increment when expanding minimap pages (lines)
pub const MINIMAP_PAGE_GROWTH: usize = 256;

/// Text wrapping mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    /// No wrapping - horizontal scroll only
    NoWrap,
    /// Soft wrap at word boundaries
    SoftWrap,
}

/// Configuration for the code editor widget
#[derive(Debug, Clone)]
pub struct EditorConfig {
    /// Text wrapping mode
    pub wrap_mode: WrapMode,

    /// Tab size in spaces
    pub tab_size: usize,

    /// Show line numbers in the gutter
    pub show_line_numbers: bool,

    /// Highlight the current line
    pub highlight_current_line: bool,

    /// Font size in pixels
    pub font_size: f32,

    /// Font family name
    pub font_family: String,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            wrap_mode: WrapMode::NoWrap,
            tab_size: 4,
            show_line_numbers: true,
            highlight_current_line: true,
            font_size: 14.0,
            font_family: "Menlo".to_string(),
        }
    }
}
