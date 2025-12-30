use crate::paint::primitives::Color;
use crate::paint::types::{Border, CornerRadius, Shadow};

/// Visual state of the text input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    /// Normal state (not focused, not hovered)
    Normal,
    /// Mouse hovering over input
    Hovered,
    /// Input has keyboard focus
    Focused,
    /// Input is disabled
    Disabled,
    /// Validation error state
    Error,
}

/// Styling for a text input in a specific state
#[derive(Debug, Clone)]
pub struct InputStyle {
    /// Background color
    pub background: Color,
    /// Text color
    pub text_color: Color,
    /// Placeholder text color (dimmed)
    pub placeholder_color: Color,
    /// Border
    pub border: Option<Border>,
    /// Corner radius for rounded corners
    pub corner_radius: CornerRadius,
    /// Shadow effect
    pub shadow: Option<Shadow>,
    /// Cursor color (blinking vertical line)
    pub cursor_color: Color,
    /// Selection background color (highlight)
    pub selection_color: Color,
    /// Icon color (for left icon and eye button)
    pub icon_color: Color,
}

impl InputStyle {
    /// Create default style for normal state (dark theme)
    pub fn normal() -> Self {
        Self {
            background: Color::rgba(0.25, 0.25, 0.28, 1.0),  // Lighter for visibility
            text_color: Color::rgba(0.9, 0.9, 0.95, 1.0),
            placeholder_color: Color::rgba(0.5, 0.5, 0.55, 1.0),
            border: Some(Border::new(Color::rgba(0.4, 0.4, 0.45, 1.0), 1.5)),  // Thicker border
            corner_radius: CornerRadius::uniform(8.0),  // More rounded
            shadow: None,
            cursor_color: Color::rgba(0.4, 0.6, 1.0, 1.0),
            selection_color: Color::rgba(0.3, 0.5, 0.8, 0.5),  // Increased alpha for visibility
            icon_color: Color::rgba(0.7, 0.7, 0.75, 1.0),  // Lighter icons
        }
    }

    /// Create default style for hovered state
    pub fn hovered() -> Self {
        Self {
            background: Color::rgba(0.28, 0.28, 0.31, 1.0),  // Lighter on hover
            text_color: Color::rgba(0.9, 0.9, 0.95, 1.0),
            placeholder_color: Color::rgba(0.5, 0.5, 0.55, 1.0),
            border: Some(Border::new(Color::rgba(0.5, 0.5, 0.55, 1.0), 1.5)),  // Lighter border
            corner_radius: CornerRadius::uniform(8.0),
            shadow: None,
            cursor_color: Color::rgba(0.4, 0.6, 1.0, 1.0),
            selection_color: Color::rgba(0.3, 0.5, 0.8, 0.5),  // Increased alpha for visibility
            icon_color: Color::rgba(0.8, 0.8, 0.85, 1.0),  // Brighter icons
        }
    }

    /// Create default style for focused state
    pub fn focused() -> Self {
        Self {
            background: Color::rgba(0.3, 0.3, 0.33, 1.0),  // Brighter when focused
            text_color: Color::rgba(0.95, 0.95, 1.0, 1.0),
            placeholder_color: Color::rgba(0.5, 0.5, 0.55, 1.0),
            border: Some(Border::new(Color::rgba(0.4, 0.6, 1.0, 1.0), 2.5)),  // Prominent blue border
            corner_radius: CornerRadius::uniform(8.0),
            shadow: Some(Shadow::new(
                Color::rgba(0.3, 0.5, 1.0, 0.5),  // More visible shadow
                (0.0, 0.0),
                12.0,
            )),
            cursor_color: Color::rgba(0.4, 0.6, 1.0, 1.0),
            selection_color: Color::rgba(0.3, 0.5, 0.8, 0.5),  // Increased alpha for visibility
            icon_color: Color::rgba(0.9, 0.9, 0.95, 1.0),  // Brightest icons when focused
        }
    }

    /// Create default style for disabled state
    pub fn disabled() -> Self {
        Self {
            background: Color::rgba(0.1, 0.1, 0.12, 1.0),
            text_color: Color::rgba(0.4, 0.4, 0.45, 1.0),
            placeholder_color: Color::rgba(0.3, 0.3, 0.35, 1.0),
            border: Some(Border::new(Color::rgba(0.2, 0.2, 0.25, 1.0), 1.0)),
            corner_radius: CornerRadius::uniform(6.0),
            shadow: None,
            cursor_color: Color::rgba(0.4, 0.4, 0.45, 1.0),
            selection_color: Color::rgba(0.2, 0.2, 0.25, 0.3),
            icon_color: Color::rgba(0.3, 0.3, 0.35, 1.0),
        }
    }

    /// Create default style for error state (validation failed)
    pub fn error() -> Self {
        Self {
            background: Color::rgba(0.2, 0.15, 0.15, 1.0),
            text_color: Color::rgba(0.95, 0.95, 1.0, 1.0),
            placeholder_color: Color::rgba(0.5, 0.5, 0.55, 1.0),
            border: Some(Border::new(Color::rgba(1.0, 0.3, 0.3, 1.0), 2.0)),
            corner_radius: CornerRadius::uniform(6.0),
            shadow: Some(Shadow::new(
                Color::rgba(1.0, 0.2, 0.2, 0.3),
                (0.0, 0.0),
                8.0,
            )),
            cursor_color: Color::rgba(0.4, 0.6, 1.0, 1.0),
            selection_color: Color::rgba(0.3, 0.5, 0.8, 0.5),  // Increased alpha for visibility
            icon_color: Color::rgba(1.0, 0.4, 0.4, 1.0),
        }
    }
}

impl Default for InputStyle {
    fn default() -> Self {
        Self::normal()
    }
}
