//! Text styling configuration

use cosmic_text::{Attrs, Family, Weight, Style, Stretch};
use crate::paint::Color;

/// Text style configuration
#[derive(Clone, Debug)]
pub struct TextStyle {
    /// Font size in pixels
    pub font_size: f32,

    /// Line height (multiplier of font_size)
    /// Default: 1.2 (20% larger than font size)
    pub line_height: f32,

    /// Font family name
    /// Use empty string for system default
    pub font_family: String,

    /// Font weight (100-900)
    pub font_weight: Weight,

    /// Font style (normal, italic, oblique)
    pub font_style: Style,

    /// Font stretch (condensed, normal, expanded)
    pub font_stretch: Stretch,

    /// Text color
    pub text_color: Color,
}

impl TextStyle {
    /// Create a new text style with default values
    pub fn new() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.2,
            font_family: String::new(),
            font_weight: Weight::NORMAL,
            font_style: Style::Normal,
            font_stretch: Stretch::Normal,
            text_color: Color::WHITE,  // Default to white text
        }
    }

    /// Set font size (builder pattern)
    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set line height (builder pattern)
    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = height;
        self
    }

    /// Set font family (builder pattern)
    pub fn family(mut self, family: impl Into<String>) -> Self {
        self.font_family = family.into();
        self
    }

    /// Set font weight (builder pattern)
    pub fn weight(mut self, weight: Weight) -> Self {
        self.font_weight = weight;
        self
    }

    /// Set italic style (builder pattern)
    pub fn italic(mut self) -> Self {
        self.font_style = Style::Italic;
        self
    }

    /// Set bold weight (builder pattern)
    pub fn bold(mut self) -> Self {
        self.font_weight = Weight::BOLD;
        self
    }

    /// Set text color (builder pattern)
    pub fn color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Convert to cosmic-text Attrs
    pub(crate) fn to_attrs(&self) -> Attrs {
        let family = if self.font_family.is_empty() {
            Family::SansSerif
        } else {
            Family::Name(&self.font_family)
        };

        Attrs::new()
            .family(family)
            .weight(self.font_weight)
            .style(self.font_style)
            .stretch(self.font_stretch)
    }

    /// Get actual line height in pixels
    pub fn line_height_pixels(&self) -> f32 {
        self.font_size * self.line_height
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Text truncation mode
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Truncate {
    /// Don't truncate, just clip
    None,
    /// Add "..." at the end when text doesn't fit
    End,
}

impl From<Truncate> for cosmic_text::Wrap {
    fn from(truncate: Truncate) -> Self {
        match truncate {
            Truncate::None => cosmic_text::Wrap::None,
            Truncate::End => cosmic_text::Wrap::None,  // Will handle truncation separately
        }
    }
}
