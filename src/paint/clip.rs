use crate::types::Rect;
use super::types::CornerRadius;

/// Maximum number of nested clip regions (shader uniform array limit)
pub const MAX_CLIPS: usize = 8;

/// A single clip region (rounded rectangle)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClipRegion {
    pub rect: Rect,
    pub corner_radius: CornerRadius,
}

impl ClipRegion {
    /// Create a new clip region
    pub fn new(rect: Rect, corner_radius: CornerRadius) -> Self {
        Self { rect, corner_radius }
    }

    /// Create an axis-aligned rectangular clip (no rounding)
    pub fn rect_only(rect: Rect) -> Self {
        Self {
            rect,
            corner_radius: CornerRadius::zero(),
        }
    }

    /// Convert to GPU uniform format [x, y, width, height, tl, tr, br, bl]
    pub fn to_gpu_array(&self) -> [f32; 8] {
        let radii = self.corner_radius.to_array();
        [
            self.rect.origin.x as f32,
            self.rect.origin.y as f32,
            self.rect.size.width as f32,
            self.rect.size.height as f32,
            radii[0], // top_left
            radii[1], // top_right
            radii[2], // bottom_right
            radii[3], // bottom_left
        ]
    }
}

/// Stack-based clip region management
///
/// Supports up to MAX_CLIPS (8) nested clipping regions.
/// Each region is a rounded rectangle evaluated in the fragment shader via SDF.
#[derive(Debug, Clone)]
pub struct ClipStack {
    stack: Vec<ClipRegion>,
}

impl ClipStack {
    /// Create an empty clip stack
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push a new clip region onto the stack
    ///
    /// Returns an error if the stack is full (MAX_CLIPS exceeded).
    pub fn push(&mut self, rect: Rect, corner_radius: CornerRadius) -> Result<(), ClipError> {
        if self.stack.len() >= MAX_CLIPS {
            return Err(ClipError::StackOverflow);
        }

        // Intersect with current clip (optimization for culling)
        let intersected_rect = if let Some(current) = self.stack.last() {
            current.rect.intersection(&rect).unwrap_or(rect)
        } else {
            rect
        };

        self.stack.push(ClipRegion {
            rect: intersected_rect,
            corner_radius,
        });
        Ok(())
    }

    /// Pop the most recent clip region from the stack
    ///
    /// Returns an error if the stack is empty.
    pub fn pop(&mut self) -> Result<(), ClipError> {
        if self.stack.is_empty() {
            return Err(ClipError::StackUnderflow);
        }
        self.stack.pop();
        Ok(())
    }

    /// Get the current number of active clip regions
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Check if the clip stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Get all active clip regions as a slice
    pub fn regions(&self) -> &[ClipRegion] {
        &self.stack
    }

    /// Clear all clip regions
    pub fn clear(&mut self) {
        self.stack.clear();
    }

    /// Convert clip stack to GPU uniform data
    ///
    /// Returns a flat array suitable for uploading to a shader uniform buffer.
    /// Format: [count, padding, padding, padding, region0..., region1..., ...]
    /// Each region: [x, y, w, h, tl, tr, br, bl]
    pub fn to_uniform_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity(4 + MAX_CLIPS * 8);

        // Header: [count, 0, 0, 0] (vec4 for alignment)
        data.push(self.stack.len() as f32);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);

        // Regions
        for region in &self.stack {
            data.extend_from_slice(&region.to_gpu_array());
        }

        // Pad to MAX_CLIPS
        let remaining = MAX_CLIPS - self.stack.len();
        for _ in 0..remaining {
            data.extend_from_slice(&[0.0; 8]);
        }

        data
    }
}

impl Default for ClipStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur during clip stack operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipError {
    /// Tried to push more than MAX_CLIPS regions
    StackOverflow,
    /// Tried to pop from an empty stack
    StackUnderflow,
}

impl std::fmt::Display for ClipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipError::StackOverflow => write!(f, "Clip stack overflow (max {} regions)", MAX_CLIPS),
            ClipError::StackUnderflow => write!(f, "Clip stack underflow (pop on empty stack)"),
        }
    }
}

impl std::error::Error for ClipError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_stack_push_pop() {
        let mut stack = ClipStack::new();
        assert_eq!(stack.len(), 0);

        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        stack.push(rect1, CornerRadius::zero()).unwrap();
        assert_eq!(stack.len(), 1);

        let rect2 = Rect::new(10.0, 10.0, 50.0, 50.0);
        stack.push(rect2, CornerRadius::uniform(5.0)).unwrap();
        assert_eq!(stack.len(), 2);

        stack.pop().unwrap();
        assert_eq!(stack.len(), 1);

        stack.pop().unwrap();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_clip_stack_overflow() {
        let mut stack = ClipStack::new();
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);

        // Fill up to MAX_CLIPS
        for _ in 0..MAX_CLIPS {
            assert!(stack.push(rect, CornerRadius::zero()).is_ok());
        }

        // Next push should fail
        assert_eq!(stack.push(rect, CornerRadius::zero()), Err(ClipError::StackOverflow));
    }

    #[test]
    fn test_clip_stack_underflow() {
        let mut stack = ClipStack::new();
        assert_eq!(stack.pop(), Err(ClipError::StackUnderflow));
    }
}
