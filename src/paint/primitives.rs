use crate::types::Rect;

/// RGBA color
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    // Common colors
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
    pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
}

/// Instance data for a single rectangle
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
    /// Position (x, y) and size (width, height)
    pub rect: [f32; 4],
    /// Color (r, g, b, a)
    pub color: [f32; 4],
    /// Clip rect (x, y, width, height) - pixels outside are discarded
    pub clip_rect: [f32; 4],
    /// Z-order for depth sorting (higher = on top)
    /// Used for sorting primitives before rendering to ensure correct overlapping
    pub z_order: u32,
    /// Padding to maintain alignment
    _padding: [u32; 3],
}

impl RectInstance {
    pub fn new(rect: Rect, color: Color) -> Self {
        // Default: no clipping (use huge bounds), z_order = 0
        RectInstance {
            rect: [
                rect.origin.x as f32,
                rect.origin.y as f32,
                rect.size.width as f32,
                rect.size.height as f32,
            ],
            color: [color.r, color.g, color.b, color.a],
            clip_rect: [0.0, 0.0, 1000000.0, 1000000.0],
            z_order: 0,
            _padding: [0; 3],
        }
    }

    pub fn with_clip(mut self, clip: Rect) -> Self {
        self.clip_rect = [
            clip.origin.x as f32,
            clip.origin.y as f32,
            clip.size.width as f32,
            clip.size.height as f32,
        ];
        self
    }

    pub fn with_z_order(mut self, z_order: u32) -> Self {
        self.z_order = z_order;
        self
    }
}
