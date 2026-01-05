// Editor theme - shared colors between editor and minimap

use crate::paint::primitives::Color;

/// Editor color theme
///
/// This theme is shared between the editor view and the minimap
/// to ensure consistent colors throughout.
#[derive(Clone, Debug)]
pub struct EditorTheme {
    // Background colors
    pub background: Color,
    pub gutter_background: Color,
    pub current_line_background: Color,

    // Text colors
    pub default_text: Color,
    pub line_number: Color,
    pub line_number_active: Color,

    // Syntax highlighting colors
    pub keyword: Color,      // fn, let, pub, if, else, etc.
    pub string: Color,       // "hello", 'c'
    pub comment: Color,      // // comment, /* comment */
    pub number: Color,       // 42, 3.14
    pub function: Color,     // function names
    pub type_name: Color,    // struct, enum, trait names
    pub operator: Color,     // +, -, *, /, =, etc.
    pub bracket: Color,      // {}, [], () - for folding

    // UI colors
    pub cursor: Color,
    pub selection: Color,
}

impl EditorTheme {
    /// Default dark theme (VS Code-like)
    pub fn dark() -> Self {
        Self {
            // Backgrounds
            background: Color::rgb(0.11, 0.12, 0.13),                // #1c1e21
            gutter_background: Color::rgb(0.08, 0.09, 0.10),         // #141719
            current_line_background: Color::rgba(0.3, 0.3, 0.4, 0.1), // Subtle highlight

            // Text
            default_text: Color::rgb(0.85, 0.85, 0.85),              // Light gray
            line_number: Color::rgb(0.4, 0.4, 0.4),                  // Medium gray
            line_number_active: Color::rgb(0.7, 0.7, 0.7),           // Lighter gray

            // Syntax (inspired by VS Code Dark+)
            keyword: Color::rgb(0.33, 0.61, 0.84),                   // Blue (#569cd6)
            string: Color::rgb(0.81, 0.62, 0.45),                    // Orange/tan (#ce9178)
            comment: Color::rgb(0.38, 0.55, 0.38),                   // Green (#608b4e)
            number: Color::rgb(0.71, 0.82, 0.72),                    // Light green (#b5cea8)
            function: Color::rgb(0.86, 0.86, 0.55),                  // Yellow (#dcdcaa)
            type_name: Color::rgb(0.31, 0.78, 0.63),                 // Teal (#4ec9b0)
            operator: Color::rgb(0.85, 0.85, 0.85),                  // Same as default
            bracket: Color::rgb(1.0, 0.85, 0.0),                     // Gold (FFD700)

            // UI
            cursor: Color::WHITE,
            selection: Color::rgba(0.3, 0.5, 0.8, 0.3),              // Blue transparent
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            // Backgrounds
            background: Color::rgb(1.0, 1.0, 1.0),                   // White
            gutter_background: Color::rgb(0.95, 0.95, 0.95),         // Light gray
            current_line_background: Color::rgba(0.9, 0.9, 0.95, 0.5),

            // Text
            default_text: Color::rgb(0.0, 0.0, 0.0),                 // Black
            line_number: Color::rgb(0.6, 0.6, 0.6),
            line_number_active: Color::rgb(0.3, 0.3, 0.3),

            // Syntax
            keyword: Color::rgb(0.0, 0.0, 0.8),                      // Blue
            string: Color::rgb(0.6, 0.2, 0.0),                       // Brown
            comment: Color::rgb(0.0, 0.5, 0.0),                      // Green
            number: Color::rgb(0.0, 0.4, 0.4),                       // Teal
            function: Color::rgb(0.5, 0.5, 0.0),                     // Olive
            type_name: Color::rgb(0.0, 0.5, 0.5),                    // Cyan
            operator: Color::rgb(0.0, 0.0, 0.0),                     // Black
            bracket: Color::rgb(0.6, 0.4, 0.0),                      // Dark gold

            // UI
            cursor: Color::BLACK,
            selection: Color::rgba(0.5, 0.7, 1.0, 0.3),
        }
    }
}

impl Default for EditorTheme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Color index constants for minimap palette
/// These map to the EditorTheme colors
pub const COLOR_DEFAULT: u8 = 0;
pub const COLOR_KEYWORD: u8 = 1;
pub const COLOR_STRING: u8 = 2;
pub const COLOR_COMMENT: u8 = 3;
pub const COLOR_NUMBER: u8 = 4;
pub const COLOR_FUNCTION: u8 = 5;
pub const COLOR_TYPE: u8 = 6;
pub const COLOR_OPERATOR: u8 = 7;
pub const COLOR_BRACKET: u8 = 8;

/// Create a minimap palette from this theme
impl EditorTheme {
    pub fn to_minimap_palette(&self) -> super::minimap::MinimapPalette {
        use super::minimap::MinimapPalette;

        let mut palette = MinimapPalette::new();

        // These indices must match the COLOR_* constants above
        palette.add_entry(self.default_text, self.background); // 0: COLOR_DEFAULT
        palette.add_entry(self.keyword, self.background);      // 1: COLOR_KEYWORD
        palette.add_entry(self.string, self.background);       // 2: COLOR_STRING
        palette.add_entry(self.comment, self.background);      // 3: COLOR_COMMENT
        palette.add_entry(self.number, self.background);       // 4: COLOR_NUMBER
        palette.add_entry(self.function, self.background);     // 5: COLOR_FUNCTION
        palette.add_entry(self.type_name, self.background);    // 6: COLOR_TYPE
        palette.add_entry(self.operator, self.background);     // 7: COLOR_OPERATOR
        palette.add_entry(self.bracket, self.background);      // 8: COLOR_BRACKET

        palette
    }
}
