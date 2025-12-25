//! Gradient types for 2D rendering
//!
//! Supports linear and radial gradients with multiple color stops.

use super::primitives::Color;
use crate::types::Point;

/// Maximum number of color stops per gradient
pub const MAX_GRADIENT_STOPS: usize = 8;

/// A color stop in a gradient
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorStop {
    /// Position along gradient (0.0 = start, 1.0 = end)
    pub offset: f32,
    /// Color at this position
    pub color: Color,
}

impl ColorStop {
    pub fn new(offset: f32, color: Color) -> Self {
        Self { offset, color }
    }
}

/// Linear gradient (color transition along a line)
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    /// Start point (in local rect coordinates, 0.0-1.0)
    pub start: Point,
    /// End point (in local rect coordinates, 0.0-1.0)
    pub end: Point,
    /// Color stops (2-8 stops)
    pub stops: Vec<ColorStop>,
}

impl LinearGradient {
    /// Create a new linear gradient
    pub fn new(start: Point, end: Point, stops: Vec<ColorStop>) -> Self {
        assert!(stops.len() >= 2, "Linear gradient requires at least 2 color stops");
        assert!(stops.len() <= MAX_GRADIENT_STOPS, "Linear gradient supports at most {} color stops", MAX_GRADIENT_STOPS);

        Self { start, end, stops }
    }

    /// Create a vertical gradient (top to bottom)
    pub fn vertical(top: Color, bottom: Color) -> Self {
        Self::new(
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            vec![
                ColorStop::new(0.0, top),
                ColorStop::new(1.0, bottom),
            ],
        )
    }

    /// Create a horizontal gradient (left to right)
    pub fn horizontal(left: Color, right: Color) -> Self {
        Self::new(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            vec![
                ColorStop::new(0.0, left),
                ColorStop::new(1.0, right),
            ],
        )
    }

    /// Create a diagonal gradient (top-left to bottom-right)
    pub fn diagonal(top_left: Color, bottom_right: Color) -> Self {
        Self::new(
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            vec![
                ColorStop::new(0.0, top_left),
                ColorStop::new(1.0, bottom_right),
            ],
        )
    }
}

/// Radial gradient (color transition from center outward)
#[derive(Debug, Clone, PartialEq)]
pub struct RadialGradient {
    /// Center point (in local rect coordinates, 0.0-1.0)
    pub center: Point,
    /// Radius (in local rect coordinates, 0.0-1.0)
    pub radius: f32,
    /// Color stops (2-8 stops)
    pub stops: Vec<ColorStop>,
}

impl RadialGradient {
    /// Create a new radial gradient
    pub fn new(center: Point, radius: f32, stops: Vec<ColorStop>) -> Self {
        assert!(stops.len() >= 2, "Radial gradient requires at least 2 color stops");
        assert!(stops.len() <= MAX_GRADIENT_STOPS, "Radial gradient supports at most {} color stops", MAX_GRADIENT_STOPS);

        Self { center, radius, stops }
    }

    /// Create a centered radial gradient
    pub fn centered(inner: Color, outer: Color) -> Self {
        Self::new(
            Point::new(0.5, 0.5),
            0.5,
            vec![
                ColorStop::new(0.0, inner),
                ColorStop::new(1.0, outer),
            ],
        )
    }
}
