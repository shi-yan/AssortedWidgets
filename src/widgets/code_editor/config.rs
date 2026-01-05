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

    /// Monospace font configuration
    pub monospace_config: MonospaceFontConfig,
}

/// Monospace font configuration for mixed-width character support
///
/// Handles ASCII (1x width), CJK (2x width), and emoji (2x width)
/// to maintain grid alignment in the code editor.
#[derive(Debug, Clone)]
pub struct MonospaceFontConfig {
    /// Font fallback chain for narrow characters (ASCII, Latin, symbols)
    pub narrow_fonts: Vec<String>,

    /// Font fallback chain for wide characters (CJK)
    pub wide_fonts: Vec<String>,

    /// Emoji font
    pub emoji_font: String,

    /// Base advance width (calculated from narrow font)
    pub base_advance: Option<f32>,
}

impl Default for MonospaceFontConfig {
    fn default() -> Self {
        Self {
            // macOS monospace fonts
            narrow_fonts: vec![
                "Menlo".to_string(),
                "Monaco".to_string(),
                "Courier New".to_string(),
            ],
            wide_fonts: vec![
                "Hiragino Sans GB".to_string(),
                "PingFang SC".to_string(),
                "Microsoft YaHei".to_string(),
            ],
            emoji_font: "Apple Color Emoji".to_string(),
            base_advance: None, // Calculated at runtime
        }
    }
}

impl MonospaceFontConfig {
    /// Detect if a character is CJK (Chinese, Japanese, Korean)
    pub fn is_cjk(ch: char) -> bool {
        matches!(ch as u32,
            0x4E00..=0x9FFF |  // CJK Unified Ideographs
            0x3400..=0x4DBF |  // CJK Extension A
            0x3040..=0x309F |  // Hiragana
            0x30A0..=0x30FF    // Katakana
        )
    }

    /// Detect if a character is emoji
    pub fn is_emoji(ch: char) -> bool {
        matches!(ch as u32,
            0x1F600..=0x1F64F |  // Emoticons
            0x1F300..=0x1F5FF |  // Misc Symbols and Pictographs
            0x1F680..=0x1F6FF |  // Transport and Map Symbols
            0x2600..=0x26FF   |  // Misc Symbols
            0x2700..=0x27BF      // Dingbats
        )
    }

    /// Get the expected width multiplier for a character
    /// Returns 1.0 for ASCII/Latin, 2.0 for CJK/emoji
    pub fn get_width_multiplier(ch: char) -> f32 {
        if Self::is_cjk(ch) || Self::is_emoji(ch) {
            2.0
        } else {
            1.0
        }
    }
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
            monospace_config: MonospaceFontConfig::default(),
        }
    }
}
