use super::path::{Path, Stroke};
use super::primitives::Color;
use super::types::{CornerRadius, DrawCommand, ShapeStyle};
use super::layers::layers;
use crate::types::{Point, Rect};

/// Batches 2D primitive draw calls for efficient GPU rendering
///
/// Commands are automatically sorted by z-index before rendering to ensure
/// correct layering (shadows behind shapes, tooltips on top, etc.).
pub struct PrimitiveBatcher {
    commands: Vec<DrawCommand>,
}

impl PrimitiveBatcher {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    // === Shape Drawing ===

    /// Draw a rectangle at the default layer (NORMAL = 0)
    pub fn draw_rect(&mut self, rect: Rect, style: ShapeStyle) {
        self.draw_rect_z(rect, style, layers::NORMAL);
    }

    /// Draw a rectangle with explicit z-index (higher = rendered on top)
    ///
    /// Use `layers::*` constants for predictable layering:
    /// - `layers::SHADOW` (-100) for drop shadows
    /// - `layers::NORMAL` (0) for standard UI elements
    /// - `layers::OVERLAY` (1000) for tooltips and popovers
    pub fn draw_rect_z(&mut self, rect: Rect, style: ShapeStyle, z_index: i32) {
        self.commands.push(DrawCommand::Rect { rect, style, z_index });
    }

    // === Lines ===

    /// Draw a line segment at the default layer (NORMAL = 0)
    pub fn draw_line(&mut self, p1: Point, p2: Point, stroke: Stroke) {
        self.draw_line_z(p1, p2, stroke, layers::NORMAL);
    }

    /// Draw a line segment with explicit z-index
    pub fn draw_line_z(&mut self, p1: Point, p2: Point, stroke: Stroke, z_index: i32) {
        self.commands.push(DrawCommand::Line { p1, p2, stroke, z_index });
    }

    // === Paths ===

    /// Draw a filled path at the default layer (NORMAL = 0)
    pub fn fill_path(&mut self, path: Path, color: Color) {
        self.fill_path_z(path, color, layers::NORMAL);
    }

    /// Draw a filled path with explicit z-index
    pub fn fill_path_z(&mut self, path: Path, color: Color, z_index: i32) {
        self.commands.push(DrawCommand::Path {
            path,
            fill: Some(color),
            stroke: None,
            z_index,
        });
    }

    /// Draw a stroked path at the default layer (NORMAL = 0)
    pub fn stroke_path(&mut self, path: Path, stroke: Stroke) {
        self.stroke_path_z(path, stroke, layers::NORMAL);
    }

    /// Draw a stroked path with explicit z-index
    pub fn stroke_path_z(&mut self, path: Path, stroke: Stroke, z_index: i32) {
        self.commands.push(DrawCommand::Path {
            path,
            fill: None,
            stroke: Some(stroke),
            z_index,
        });
    }

    /// Draw a path with both fill and stroke
    pub fn draw_path(&mut self, path: Path, fill: Option<Color>, stroke: Option<Stroke>) {
        self.draw_path_z(path, fill, stroke, layers::NORMAL);
    }

    /// Draw a path with both fill and stroke with explicit z-index
    pub fn draw_path_z(
        &mut self,
        path: Path,
        fill: Option<Color>,
        stroke: Option<Stroke>,
        z_index: i32,
    ) {
        self.commands.push(DrawCommand::Path {
            path,
            fill,
            stroke,
            z_index,
        });
    }

    // === Icons ===

    /// Draw an icon at the default layer (NORMAL = 0)
    ///
    /// Icons are rendered as glyphs from the Material Icons font.
    /// Use human-readable icon IDs like "search", "home", "settings", etc.
    ///
    /// # Arguments
    /// * `icon_id` - Human-readable icon identifier (e.g., "search")
    /// * `position` - Top-left position of the icon
    /// * `size` - Font size for the icon (in points)
    /// * `color` - Icon color
    pub fn draw_icon(&mut self, icon_id: &str, position: Point, size: f32, color: Color) {
        self.draw_icon_z(icon_id, position, size, color, layers::NORMAL);
    }

    /// Draw an icon with explicit z-index
    pub fn draw_icon_z(&mut self, icon_id: &str, position: Point, size: f32, color: Color, z_index: i32) {
        self.commands.push(DrawCommand::Icon {
            icon_id: icon_id.to_string(),
            position,
            size,
            color,
            z_index,
        });
    }

    // === Images ===

    /// Draw an image at the default layer (NORMAL = 0)
    ///
    /// Images are rendered using individual GPU textures (not atlas-based).
    /// Supports PNG, JPG, WebP formats.
    ///
    /// # Arguments
    /// * `image_id` - Image identifier (from ImageId::from_file)
    /// * `rect` - Destination rectangle (position and size)
    pub fn draw_image(&mut self, image_id: crate::image::ImageId, rect: Rect) {
        self.draw_image_z(image_id, rect, None, layers::NORMAL);
    }

    /// Draw an image with color tinting
    pub fn draw_image_tinted(&mut self, image_id: crate::image::ImageId, rect: Rect, tint: Color) {
        self.draw_image_z(image_id, rect, Some(tint), layers::NORMAL);
    }

    /// Draw an image with explicit z-index and optional tinting
    pub fn draw_image_z(&mut self, image_id: crate::image::ImageId, rect: Rect, tint: Option<Color>, z_index: i32) {
        self.commands.push(DrawCommand::Image {
            image_id,
            rect,
            tint,
            z_index,
        });
    }

    // === Clipping ===

    /// Push a rectangular clipping region (axis-aligned, no rounding)
    pub fn push_clip(&mut self, rect: Rect) {
        self.push_clip_rounded(rect, CornerRadius::zero());
    }

    /// Push a rounded-rect clipping region (for scrollable areas with rounded corners)
    ///
    /// Clipping is evaluated in the fragment shader via SDF, supporting up to 8 nested regions.
    /// Clips must be properly balanced with pop_clip() calls.
    pub fn push_clip_rounded(&mut self, rect: Rect, corner_radius: CornerRadius) {
        self.commands.push(DrawCommand::PushClip { rect, corner_radius });
    }

    /// Pop the most recent clipping region
    ///
    /// Must be paired with a corresponding push_clip or push_clip_rounded call.
    pub fn pop_clip(&mut self) {
        self.commands.push(DrawCommand::PopClip);
    }

    // === Command Access ===

    /// Get all draw commands (for rendering)
    ///
    /// NOTE: Commands are NOT sorted here. Call `sort_commands()` before rendering
    /// to ensure correct z-ordering.
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// Get mutable access to commands (for sorting)
    pub fn commands_mut(&mut self) -> &mut Vec<DrawCommand> {
        &mut self.commands
    }

    /// Sort commands by z-index for correct rendering order
    ///
    /// This implements Phase 1 z-ordering: simple CPU sort by z-index.
    /// Commands with the same z-index maintain their original order (stable sort).
    ///
    /// Call this once per frame before rendering.
    pub fn sort_commands(&mut self) {
        // Stable sort preserves clip command ordering while sorting by z-index
        self.commands.sort_by_key(|cmd| (cmd.z_index(), cmd.batch_key()));
    }

    /// Clear all batched commands (call at start of frame)
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Number of batched commands
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl Default for PrimitiveBatcher {
    fn default() -> Self {
        Self::new()
    }
}
