use crate::paint::primitives::{Color, RectInstance};
use crate::text::{TextInstance, TextLayout, GlyphAtlas, FontSystemWrapper};
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

    /// Draw a pre-shaped text layout (Low-level manual API)
    ///
    /// **Use this for:** Editors, terminals, widgets with thousands of unique texts
    ///
    /// Renders a TextLayout that was created by TextEngine.create_layout().
    /// The widget owns the TextLayout and decides when to re-shape.
    ///
    /// # Arguments
    /// * `layout` - Pre-shaped text layout
    /// * `position` - Top-left corner where text should be drawn
    /// * `color` - Text color
    /// * `atlas` - Glyph atlas for looking up glyph locations
    /// * `font_system` - Font system for rasterizing missing glyphs
    /// * `queue` - WebGPU queue for uploading missing glyphs
    ///
    /// # Example
    /// ```rust,ignore
    /// // Widget owns the layout
    /// struct EditorLine {
    ///     layout: TextLayout,
    /// }
    ///
    /// // Paint: just render the pre-shaped layout
    /// ctx.draw_layout(&line.layout, Point::new(0, 0), Color::BLACK, atlas, font_system, queue);
    /// ```
    pub fn draw_layout(
        &mut self,
        layout: &TextLayout,
        position: Point,
        color: Color,
        atlas: &mut GlyphAtlas,
        font_system: &mut FontSystemWrapper,
        queue: &wgpu::Queue,
    ) {
        use crate::text::GlyphKey;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Get current clip rect
        let clip = self.current_clip_rect().unwrap_or_else(|| {
            Rect::new(Point::new(0.0, 0.0), self.window_size)
        });

        let clip_rect = [
            clip.origin.x as f32,
            clip.origin.y as f32,
            clip.size.width as f32,
            clip.size.height as f32,
        ];

        // Extract text from buffer for character lookup
        let buffer = layout.buffer();
        let text = buffer.lines.iter()
            .flat_map(|line| line.text().chars())
            .collect::<String>();

        // Iterate through shaped glyphs
        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                // Extract the character from the text using glyph's byte range
                let glyph_char = text[glyph.start..glyph.end].chars().next().unwrap_or('?');

                // Convert to PhysicalGlyph to get the cache_key for rasterization
                let physical_glyph = glyph.physical((position.x as f32, position.y as f32), 1.0);
                let cache_key = physical_glyph.cache_key;

                // Create glyph key for atlas (hash the font_id for consistent caching)
                let mut hasher = DefaultHasher::new();
                cache_key.font_id.hash(&mut hasher);
                cache_key.glyph_id.hash(&mut hasher);
                let font_id_hash = hasher.finish() as usize;

                let glyph_key = GlyphKey {
                    font_id: font_id_hash,
                    size_bits: (glyph.font_size * 1024.0) as u32,
                    character: glyph_char,
                    subpixel_offset: 0,
                };

                // Get or insert glyph in atlas
                let location = if let Some(&loc) = atlas.get(&glyph_key) {
                    atlas.mark_glyph_used(&glyph_key);
                    loc
                } else {
                    // Rasterize and insert
                    if let Some(rasterized) = font_system.rasterize_glyph(cache_key) {
                        match atlas.insert(
                            queue,
                            glyph_key,
                            &rasterized.pixels,
                            rasterized.width,
                            rasterized.height,
                            rasterized.offset_x,
                            rasterized.offset_y,
                            rasterized.is_color,
                        ) {
                            Ok(loc) => loc,
                            Err(_e) => {
                                // Failed to insert glyph, skip it
                                continue;
                            }
                        }
                    } else {
                        continue;  // Skip if rasterization failed
                    }
                };

                // Calculate glyph position
                let glyph_x = position.x as f32 + glyph.x + glyph.x_offset;
                let glyph_y = position.y as f32 + run.line_y + glyph.y_offset;

                // Get metrics from cosmic-text
                let glyph_width = glyph.w;
                let glyph_height = run.line_height;

                // Push text instance
                let instance = TextInstance::new(
                    glyph_x,
                    glyph_y,
                    glyph_width,
                    glyph_height,
                    location.uv_rect.min_x,
                    location.uv_rect.min_y,
                    location.uv_rect.max_x,
                    location.uv_rect.max_y,
                    color,
                    location.page_index,
                    location.is_color,
                    clip_rect,
                );

                self.text.push(instance);
            }
        }
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
