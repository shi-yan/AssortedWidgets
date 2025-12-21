use crate::paint::primitives::{Color, RectInstance};
use crate::text::TextInstance;
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

    /// Collected text glyph instances
    text: Vec<TextInstance>,

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
            text: Vec::new(),
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

    /// Draw a single text glyph
    ///
    /// For Phase 3.1, this is used to draw individual characters manually positioned.
    /// Phase 3.2 will add higher-level text rendering with shaping.
    pub fn draw_glyph(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        uv_min_x: f32,
        uv_min_y: f32,
        uv_max_x: f32,
        uv_max_y: f32,
        color: Color,
        page_index: u32,
        is_color: bool,
    ) {
        // Get current clip rect or use full window
        let clip = self.current_clip_rect().unwrap_or_else(|| {
            Rect::new(Point::new(0.0, 0.0), self.window_size)
        });

        let clip_rect = [
            clip.origin.x as f32,
            clip.origin.y as f32,
            clip.size.width as f32,
            clip.size.height as f32,
        ];

        let instance = TextInstance::new(
            x,
            y,
            width,
            height,
            uv_min_x,
            uv_min_y,
            uv_max_x,
            uv_max_y,
            color,
            page_index,
            is_color,
            clip_rect,
        );

        self.text.push(instance);
    }

    /// Get the collected rectangle instances
    pub fn rect_instances(&self) -> &[RectInstance] {
        &self.rects
    }

    /// Get the collected text glyph instances
    pub fn text_instances(&self) -> &[TextInstance] {
        &self.text
    }

    /// Clear all collected primitives
    pub fn clear(&mut self) {
        self.rects.clear();
        self.text.clear();
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
