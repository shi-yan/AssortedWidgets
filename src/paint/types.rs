use crate::types::Rect;
pub use super::primitives::Color;

/// Fill brush (solid color or gradient)
#[derive(Debug, Clone, PartialEq)]
pub enum Brush {
    Solid(Color),
    // Future: gradients
    // LinearGradient(LinearGradient),
    // RadialGradient(RadialGradient),
}

impl Brush {
    /// Get solid color (for Phase 1, gradients will panic)
    pub fn to_color(&self) -> Color {
        match self {
            Brush::Solid(color) => *color,
        }
    }
}

impl From<Color> for Brush {
    fn from(color: Color) -> Self {
        Brush::Solid(color)
    }
}

/// Per-corner radius control
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl CornerRadius {
    /// All corners have the same radius
    pub const fn uniform(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    /// No rounding (sharp corners)
    pub const fn zero() -> Self {
        Self::uniform(0.0)
    }

    /// Convert to array for GPU upload [TL, TR, BR, BL]
    pub fn to_array(&self) -> [f32; 4] {
        [self.top_left, self.top_right, self.bottom_right, self.bottom_left]
    }
}

impl Default for CornerRadius {
    fn default() -> Self {
        Self::zero()
    }
}

/// Border styling
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Border {
    pub color: Color,
    pub width: f32,
}

impl Border {
    pub fn new(color: Color, width: f32) -> Self {
        Self { color, width }
    }
}

/// Box shadow (soft, analytical)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub color: Color,
    pub offset: (f32, f32), // (x, y)
    pub blur_radius: f32,
    pub spread_radius: f32, // Expands/contracts shadow before blur
}

impl Shadow {
    pub fn new(color: Color, offset: (f32, f32), blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
            spread_radius: 0.0,
        }
    }
}

/// Complete styling for a shape
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeStyle {
    /// Fill brush (solid color or gradient)
    pub fill: Brush,

    /// Corner radius (0.0 = sharp corners)
    pub corner_radius: CornerRadius,

    /// Optional border
    pub border: Option<Border>,

    /// Optional drop shadow
    pub shadow: Option<Shadow>,
}

impl ShapeStyle {
    /// Simple solid color rectangle
    pub fn solid(color: Color) -> Self {
        Self {
            fill: Brush::Solid(color),
            corner_radius: CornerRadius::zero(),
            border: None,
            shadow: None,
        }
    }

    /// Rounded rectangle with solid color
    pub fn rounded(color: Color, radius: f32) -> Self {
        Self {
            fill: Brush::Solid(color),
            corner_radius: CornerRadius::uniform(radius),
            border: None,
            shadow: None,
        }
    }

    /// With border
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    /// With drop shadow
    pub fn with_shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = Some(shadow);
        self
    }
}

/// Draw command for batching
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    Rect {
        rect: Rect,
        style: ShapeStyle,
        z_index: i32,
    },
    /// Push a clipping region (rounded rectangle)
    PushClip {
        rect: Rect,
        corner_radius: CornerRadius,
    },
    /// Pop the most recent clipping region
    PopClip,
    // Future commands:
    // Circle { center: Point, radius: f32, style: ShapeStyle, z_index: i32 },
    // Line { p1: Point, p2: Point, stroke: Stroke, z_index: i32 },
}

impl DrawCommand {
    /// Get the z-index for sorting (clip commands return 0)
    pub fn z_index(&self) -> i32 {
        match self {
            DrawCommand::Rect { z_index, .. } => *z_index,
            DrawCommand::PushClip { .. } | DrawCommand::PopClip => 0,
        }
    }

    /// Get the batch key for grouping (primitives of same type can batch)
    pub fn batch_key(&self) -> u32 {
        match self {
            DrawCommand::Rect { .. } => 0,
            DrawCommand::PushClip { .. } => u32::MAX - 1,
            DrawCommand::PopClip => u32::MAX,
        }
    }
}
