use crate::paint::primitives::{Color, RectInstance};
use crate::render::RenderContext;
use crate::types::{Point, Rect, Size};

/// Calculate the intersection of two rectangles
fn intersect_rects(a: Rect, b: Rect) -> Rect {
    let x1 = a.origin.x.max(b.origin.x);
    let y1 = a.origin.y.max(b.origin.y);
    let x2 = (a.origin.x + a.size.width).min(b.origin.x + b.size.width);
    let y2 = (a.origin.y + a.size.height).min(b.origin.y + b.size.height);

    Rect::new(
        Point::new(x1, y1),
        Size::new((x2 - x1).max(0.0), (y2 - y1).max(0.0)),
    )
}

/// Paint context for drawing primitives
///
/// This collects draw calls during the paint pass and then
/// renders them all at once in a batched manner.
pub struct PaintContext {
    /// Collected rectangle instances
    rects: Vec<RectInstance>,

    /// Window size (for projection matrix)
    window_size: Size,

    /// Clip stack for hierarchical clipping
    /// The current clip rect is the intersection of all rects on the stack
    clip_stack: Vec<Rect>,
}

impl PaintContext {
    pub fn new(window_size: Size) -> Self {
        PaintContext {
            rects: Vec::new(),
            window_size,
            clip_stack: Vec::new(),
        }
    }

    /// Push a clip rectangle onto the stack
    /// All subsequent draw calls will be clipped to this rect (and any parent clip rects)
    pub fn push_clip(&mut self, rect: Rect) {
        self.clip_stack.push(rect);
    }

    /// Pop the current clip rectangle from the stack
    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    /// Get the current effective clip rect (intersection of all clip rects on stack)
    fn current_clip_rect(&self) -> Option<Rect> {
        if self.clip_stack.is_empty() {
            return None;
        }

        // Start with the first clip rect
        let mut result = self.clip_stack[0];

        // Intersect with all subsequent clip rects
        for clip in &self.clip_stack[1..] {
            result = intersect_rects(result, *clip);
        }

        Some(result)
    }

    /// Draw a filled rectangle
    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        let instance = RectInstance::new(rect, color);

        // Apply clipping if there's a clip rect on the stack
        let instance = if let Some(clip) = self.current_clip_rect() {
            instance.with_clip(clip)
        } else {
            instance
        };

        self.rects.push(instance);
    }

    /// Get the collected rectangle instances
    pub fn rect_instances(&self) -> &[RectInstance] {
        &self.rects
    }

    /// Clear all collected primitives
    pub fn clear(&mut self) {
        self.rects.clear();
    }

    /// Get window size
    pub fn window_size(&self) -> Size {
        self.window_size
    }

    /// Set window size (call this on resize)
    pub fn set_window_size(&mut self, size: Size) {
        self.window_size = size;
    }
}
