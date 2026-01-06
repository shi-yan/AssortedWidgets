// Terminal configuration constants

use crate::widgets::code_editor::MonospaceFontConfig;

/// Default terminal width in columns
pub const DEFAULT_COLS: usize = 80;

/// Default terminal height in rows
pub const DEFAULT_ROWS: usize = 24;

/// Default scrollback buffer size (lines)
pub const DEFAULT_SCROLLBACK: usize = 10_000;

/// Default font size for terminal text
pub const DEFAULT_FONT_SIZE: f32 = 14.0;

/// Default line height multiplier
pub const DEFAULT_LINE_HEIGHT: f32 = 1.2;

/// Minimap configuration (for Phase 2+)
pub const MINIMAP_PAGE_SIZE: usize = 512;
pub const MINIMAP_WIDTH_PIXELS: usize = 100;

/// Terminal configuration
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    /// Font size
    pub font_size: f32,

    /// Font family (primary)
    pub font_family: String,

    /// Monospace font configuration (shared with code editor)
    /// Handles ASCII (narrow), CJK (wide), and emoji fonts
    pub monospace_config: MonospaceFontConfig,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            font_size: DEFAULT_FONT_SIZE,
            font_family: "Menlo".to_string(),
            monospace_config: MonospaceFontConfig::default(),
        }
    }
}
