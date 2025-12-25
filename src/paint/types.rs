use crate::types::{Point, Rect};
use crate::image::ImageId;
pub use super::gradient::{LinearGradient, RadialGradient};
pub use super::path::{Path, Stroke};
pub use super::primitives::Color;

/// Fill brush (solid color or gradient)
#[derive(Debug, Clone, PartialEq)]
pub enum Brush {
    Solid(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
}

impl Brush {
    /// Get solid color (only valid for solid brush)
    pub fn to_color(&self) -> Color {
        match self {
            Brush::Solid(color) => *color,
            _ => panic!("to_color() called on gradient brush"),
        }
    }

    /// Check if this is a solid color
    pub fn is_solid(&self) -> bool {
        matches!(self, Brush::Solid(_))
    }

    /// Check if this is a gradient
    pub fn is_gradient(&self) -> bool {
        !self.is_solid()
    }
}

impl From<Color> for Brush {
    fn from(color: Color) -> Self {
        Brush::Solid(color)
    }
}

impl From<LinearGradient> for Brush {
    fn from(gradient: LinearGradient) -> Self {
        Brush::LinearGradient(gradient)
    }
}

impl From<RadialGradient> for Brush {
    fn from(gradient: RadialGradient) -> Self {
        Brush::RadialGradient(gradient)
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
    /// Draw a line segment
    Line {
        p1: Point,
        p2: Point,
        stroke: Stroke,
        z_index: i32,
    },
    /// Draw a custom path (filled or stroked)
    Path {
        path: Path,
        fill: Option<Color>,
        stroke: Option<Stroke>,
        z_index: i32,
    },
    /// Draw an icon by ID (from Material Icons font)
    Icon {
        icon_id: String,
        position: Point,
        size: f32,
        color: Color,
        z_index: i32,
    },
    /// Draw an image (photo, avatar, etc.)
    Image {
        image_id: ImageId,
        rect: Rect,
        tint: Option<Color>,
        z_index: i32,
    },
    /// Push a clipping region (rounded rectangle)
    PushClip {
        rect: Rect,
        corner_radius: CornerRadius,
    },
    /// Pop the most recent clipping region
    PopClip,
}

impl DrawCommand {
    /// Get the z-index for sorting (clip commands return 0)
    pub fn z_index(&self) -> i32 {
        match self {
            DrawCommand::Rect { z_index, .. } => *z_index,
            DrawCommand::Line { z_index, .. } => *z_index,
            DrawCommand::Path { z_index, .. } => *z_index,
            DrawCommand::Icon { z_index, .. } => *z_index,
            DrawCommand::Image { z_index, .. } => *z_index,
            DrawCommand::PushClip { .. } | DrawCommand::PopClip => 0,
        }
    }

    /// Get the batch key for grouping (primitives of same type can batch)
    pub fn batch_key(&self) -> u32 {
        match self {
            DrawCommand::Rect { .. } => 0,
            DrawCommand::Line { .. } => 1,
            DrawCommand::Path { .. } => 2,
            DrawCommand::Icon { .. } => 3,
            DrawCommand::Image { .. } => 4,
            DrawCommand::PushClip { .. } => u32::MAX - 1,
            DrawCommand::PopClip => u32::MAX,
        }
    }
}
