use super::types::{DrawCommand, ShapeStyle};
use crate::types::Rect;

/// Batches 2D primitive draw calls for efficient GPU rendering
pub struct PrimitiveBatcher {
    commands: Vec<DrawCommand>,
}

impl PrimitiveBatcher {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Draw a rectangle with rounded corners, fill, and optional border
    pub fn draw_rect(&mut self, rect: Rect, style: ShapeStyle) {
        self.commands.push(DrawCommand::Rect { rect, style });
    }

    /// Get all draw commands (for rendering)
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
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
