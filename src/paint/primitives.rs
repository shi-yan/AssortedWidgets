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

    /// Convert to array for GPU upload
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
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
    /// GPU depth value from LayeredBoundsTree (0.0 = near, 1.0 = far)
    pub depth: f32,
    /// Z-order for CPU sorting (before LayeredBoundsTree assignment)
    pub z_order: u32,
    /// Padding to maintain alignment
    _padding: [u32; 2],
}

impl RectInstance {
    pub fn new(rect: Rect, color: Color) -> Self {
        // Default: no clipping, z_order = 0, depth will be assigned later
        RectInstance {
            rect: [
                rect.origin.x as f32,
                rect.origin.y as f32,
                rect.size.width as f32,
                rect.size.height as f32,
            ],
            color: [color.r, color.g, color.b, color.a],
            clip_rect: [0.0, 0.0, 1000000.0, 1000000.0],
            depth: 0.5,  // Default middle depth (will be overwritten)
            z_order: 0,
            _padding: [0; 2],
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

    pub fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        self
    }

    pub fn bounds(&self) -> Rect {
        Rect::new(
            crate::types::Point::new(self.rect[0] as f64, self.rect[1] as f64),
            crate::types::Size::new(self.rect[2] as f64, self.rect[3] as f64),
        )
    }
}
