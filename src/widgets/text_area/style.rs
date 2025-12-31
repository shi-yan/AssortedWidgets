//! TextArea styling

use crate::paint::primitives::Color;
use crate::paint::types::{Border, CornerRadius, Shadow};

/// Visual state of the text area
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAreaState {
    Normal,
    Hovered,
    Focused,
    Disabled,
    Error,
}

/// Style configuration for TextArea
#[derive(Debug, Clone)]
pub struct TextAreaStyle {
    pub background: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub selection_color: Color,
    pub cursor_color: Color,
    pub border: Option<Border>,
    pub corner_radius: CornerRadius,
    pub shadow: Option<Shadow>,
}

impl TextAreaStyle {
    /// Normal/default state
    pub fn normal() -> Self {
        Self {
            background: Color::rgb(0.15, 0.15, 0.18),
            text_color: Color::rgb(0.9, 0.9, 0.95),
            placeholder_color: Color::rgba(0.6, 0.6, 0.65, 0.5),
            selection_color: Color::rgba(0.3, 0.5, 0.9, 0.3),
            cursor_color: Color::rgb(0.9, 0.9, 0.95),
            border: Some(Border::new(Color::rgba(0.3, 0.3, 0.35, 0.5), 1.5)),
            corner_radius: CornerRadius::uniform(6.0),
            shadow: None,
        }
    }

    /// Hovered state
    pub fn hovered() -> Self {
        Self {
            border: Some(Border::new(Color::rgba(0.4, 0.4, 0.45, 0.7), 1.5)),
            ..Self::normal()
        }
    }

    /// Focused state
    pub fn focused() -> Self {
        Self {
            border: Some(Border::new(Color::rgb(0.3, 0.6, 0.9), 2.0)),
            shadow: Some(Shadow::new(
                Color::rgba(0.3, 0.6, 0.9, 0.3),
                (0.0, 0.0),
                8.0,
            )),
            ..Self::normal()
        }
    }

    /// Disabled state
    pub fn disabled() -> Self {
        Self {
            background: Color::rgb(0.1, 0.1, 0.12),
            text_color: Color::rgba(0.6, 0.6, 0.65, 0.4),
            placeholder_color: Color::rgba(0.5, 0.5, 0.55, 0.3),
            border: Some(Border::new(Color::rgba(0.2, 0.2, 0.25, 0.3), 1.0)),
            ..Self::normal()
        }
    }

    /// Error state (validation failed)
    pub fn error() -> Self {
        Self {
            border: Some(Border::new(Color::rgb(0.9, 0.3, 0.3), 2.0)),
            shadow: Some(Shadow::new(
                Color::rgba(0.9, 0.3, 0.3, 0.3),
                (0.0, 0.0),
                8.0,
            )),
            ..Self::normal()
        }
    }
}
