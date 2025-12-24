use super::types::{CornerRadius, DrawCommand, ShapeStyle};
use super::layers::layers;
use crate::types::Rect;

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
