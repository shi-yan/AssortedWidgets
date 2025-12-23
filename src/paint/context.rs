use crate::paint::primitives::{Color, RectInstance};
use crate::text::{TextInstance, TextLayout, GlyphAtlas, FontSystemWrapper, TextEngine, TextStyle};
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

/// Bundle of rendering resources passed to PaintContext
///
/// This bundles all the resources needed for text rendering into a single struct,
/// dramatically simplifying the API (from 7 parameters down to 3).
pub struct RenderBundle<'a> {
    /// Glyph atlas for caching rasterized glyphs
    pub atlas: &'a mut GlyphAtlas,

    /// Font system for discovering fonts and rasterizing glyphs
    pub font_system: &'a mut FontSystemWrapper,

    /// Text engine with managed cache (for high-level API)
    pub text_engine: &'a mut TextEngine,

    /// WebGPU queue for uploading textures
    pub queue: &'a wgpu::Queue,

    /// WebGPU device for creating buffers
    pub device: &'a wgpu::Device,

    /// Display scale factor (1.0 for standard, 2.0 for Retina)
    pub scale_factor: f32,
}

/// Paint context for drawing primitives
///
/// This collects draw calls during the paint pass and then
/// renders them all at once in a batched manner.
///
/// # Clean API with RenderBundle
///
/// PaintContext now bundles all rendering resources, providing a clean API:
/// - `draw_text()`: High-level API with automatic caching (3 parameters)
/// - `draw_layout()`: Low-level API for pre-shaped text (3 parameters)
///
/// This is a dramatic improvement over the previous 7-parameter API!
pub struct PaintContext<'a> {
    /// Collected rectangle instances
    rects: Vec<RectInstance>,

    /// Collected text glyph instances
    text: Vec<TextInstance>,

    /// Window size (for projection matrix)
    window_size: Size,

    /// Clip stack for hierarchical clipping
    /// The current clip rect is the intersection of all rects on the stack
    clip_stack: Vec<Rect>,

    /// Bundled rendering resources (atlas, fonts, queue, etc.)
    bundle: RenderBundle<'a>,
}

impl<'a> PaintContext<'a> {
    /// Create a new paint context with bundled rendering resources
    pub fn new(window_size: Size, bundle: RenderBundle<'a>) -> Self {
        PaintContext {
            rects: Vec::new(),
            text: Vec::new(),
            window_size,
            clip_stack: Vec::new(),
            bundle,
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

        let logical_rect = Rect::new(
            Point::new(rect.origin.x , rect.origin.y ),
            Size::new(rect.size.width , rect.size.height ),
        );

        let instance = RectInstance::new(logical_rect, color);

        // Apply clipping if there's a clip rect on the stack
        let instance = if let Some(clip) = self.current_clip_rect() {
            let logical_clip = Rect::new(
                Point::new(clip.origin.x , clip.origin.y ),
                Size::new(clip.size.width , clip.size.height ),
            );
            instance.with_clip(logical_clip)
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

    /// Draw text with automatic caching (High-level managed API)
    ///
    /// **Use this for:** Buttons, labels, menus, tooltips - simple widgets with static text
    ///
    /// This is the easiest way to draw text. The TextEngine automatically caches
    /// shaped text layouts and reuses them across frames. Perfect for UI elements
    /// that display the same text repeatedly.
    ///
    /// # Arguments
    /// * `text` - The text to display
    /// * `style` - Text style (font size, weight, etc.)
    /// * `position` - Top-left corner where text should be drawn
    /// * `max_width` - Optional width constraint for wrapping
    ///
    /// # Example
    /// ```rust,ignore
    /// // Simple button label - automatically cached!
    /// ctx.draw_text(
    ///     "Click Me",
    ///     &TextStyle::new().size(14.0),
    ///     Point::new(10.0, 10.0),
    ///     None,
    /// );
    /// ```
    pub fn draw_text(
        &mut self,
        text: &str,
        style: &TextStyle,
        position: Point,
        max_width: Option<f32>,
    ) {
        // Get or create managed layout (cached by TextEngine)
        // We need to do this in two steps to avoid borrow checker issues
        let color = style.text_color;

        // Get clip rect before borrowing bundle
        let clip_rect = self.current_clip_rect();
        let window_size = self.window_size;

        // First, get the layout (borrows text_engine)
        let layout = self.bundle.text_engine.get_or_create_managed(text, style, max_width);

        // Extract bundle fields (without text_engine which is already borrowed)
        let scale_factor = self.bundle.scale_factor;

        // Then render it using direct field access to avoid double borrow
        Self::render_text_layout_internal(
            layout,
            position,
            color,
            &mut self.text,
            &clip_rect,
            window_size,
            self.bundle.atlas,
            self.bundle.font_system,
            self.bundle.queue,
            scale_factor,
        );
    }

    /// Create a text layout with manual control (Low-level manual API)
    ///
    /// **Use this for:** Custom text rendering with truncation, custom caching, etc.
    ///
    /// This creates a TextLayout that you render with `draw_layout()`.
    /// Unlike `draw_text()`, this gives you full control over truncation and caching.
    ///
    /// # Arguments
    /// * `text` - The text to shape
    /// * `style` - Text style
    /// * `max_width` - Optional width constraint
    /// * `truncate` - Truncation mode (None or End with ellipsis)
    ///
    /// # Example
    /// ```rust,ignore
    /// // Create layout with truncation
    /// let layout = ctx.create_text_layout(
    ///     "Long text that will be truncated...",
    ///     &TextStyle::new().size(14.0),
    ///     Some(200.0),
    ///     Truncate::End,
    /// );
    ///
    /// // Render it
    /// ctx.draw_layout(&layout, position, color);
    /// ```
    pub fn create_text_layout(
        &mut self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f32>,
        truncate: crate::text::Truncate,
    ) -> crate::text::TextLayout {
        self.bundle.text_engine.create_layout(text, style, max_width, truncate)
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
    ///
    /// # Example
    /// ```rust,ignore
    /// // Widget owns the layout
    /// struct EditorLine {
    ///     layout: TextLayout,
    /// }
    ///
    /// // Paint: just render the pre-shaped layout
    /// ctx.draw_layout(&line.layout, Point::new(0, 0), Color::BLACK);
    /// ```
    pub fn draw_layout(
        &mut self,
        layout: &TextLayout,
        position: Point,
        color: Color,
    ) {
        // Precompute values to avoid borrowing issues
        let clip_rect = self.current_clip_rect();
        let window_size = self.window_size;

        Self::render_text_layout(
            layout,
            position,
            color,
            &mut self.text,
            &clip_rect,
            window_size,
            &mut self.bundle,
        );
    }

    /// Internal method for rendering text layouts (static to avoid borrow issues)
    /// Takes individual fields instead of bundle to avoid double-borrow issues
    fn render_text_layout_internal(
        layout: &TextLayout,
        position: Point,
        color: Color,
        text_instances: &mut Vec<TextInstance>,
        clip_rect_opt: &Option<Rect>,
        window_size: Size,
        atlas: &mut GlyphAtlas,
        font_system: &mut FontSystemWrapper,
        queue: &wgpu::Queue,
        scale_factor: f32,
    ) {
        use crate::text::GlyphKey;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Get current clip rect
        let clip = clip_rect_opt.unwrap_or_else(|| {
            Rect::new(Point::new(0.0, 0.0), window_size)
        });

        let clip_rect = [
            clip.origin.x as f32,
            clip.origin.y as f32,
            clip.size.width as f32,
            clip.size.height as f32,
        ];

        // Extract text from buffer for character lookup
        let buffer = layout.buffer();

        // Iterate through shaped glyphs
        for run in buffer.layout_runs() {
            // Get the baseline Y for this line
            let line_y = run.line_y;

            // Get the text for this specific line
            let line_text = buffer.lines[run.line_i].text();

            for glyph in run.glyphs.iter() {
                // Extract the character from the line's text using glyph's byte range
                // IMPORTANT: glyph.start/glyph.end are byte indices into THIS line, not the whole buffer
                let glyph_char = line_text[glyph.start..glyph.end].chars().next().unwrap_or('?');

                // Convert to PhysicalGlyph with baseline Y offset
                // cosmic-text already applies alignment offsets to glyph.x
                let physical_glyph = glyph.physical(
                    (position.x as f32 * scale_factor, position.y  as f32 * scale_factor+ line_y),
                    scale_factor
                );

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
                    scale_factor: (scale_factor * 100.0) as u8, // 100 = 1.0x, 200 = 2.0x
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

                // PhysicalGlyph.x/y represents the baseline position (pen position)
                // We must add the rasterization offsets to get the bitmap's top-left corner
                //
                // COORDINATE SYSTEM: cosmic-text uses bottom-left origin (Y goes UP)
                // but WebGPU uses top-left origin (Y goes DOWN), so we flip Y by subtracting
                //
                // CRITICAL ARCHITECTURE: Logical Coordinates with Scaled Projection
                // ===================================================================
                // - ALL drawing coordinates are in LOGICAL pixels (DPI-independent)
                // - Projection matrix is scaled: screen_size = logical_size * scale_factor (PHYSICAL)
                // - This ensures logical coords map correctly to the physical viewport
                //
                // Why this works:
                // 1. physical_glyph gives us PHYSICAL coordinates (e.g., 200px on 2x display)
                // 2. We divide by scale_factor to convert to LOGICAL (200 / 2 = 100)
                // 3. Projection matrix: logical_size * scale_factor = physical_size (1200 * 2 = 2400)
                // 4. Shader: logical_coord / physical_screen_size = correct NDC (100 / 2400)
                // 5. Viewport maps NDC to physical pixels correctly
                let glyph_x = (physical_glyph.x + location.offset_x) as f32 / scale_factor;
                let glyph_y = (physical_glyph.y - location.offset_y) as f32 / scale_factor;

                // Glyph dimensions from atlas are PHYSICAL (high-res bitmap)
                // Convert to LOGICAL for consistent coordinate system
                let glyph_width = location.width as f32 / scale_factor;
                let glyph_height = location.height as f32 / scale_factor;

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

                text_instances.push(instance);
            }
        }
    }

    /// Internal method for rendering text layouts (wrapper that takes bundle)
    fn render_text_layout(
        layout: &TextLayout,
        position: Point,
        color: Color,
        text_instances: &mut Vec<TextInstance>,
        clip_rect_opt: &Option<Rect>,
        window_size: Size,
        bundle: &mut RenderBundle<'_>,
    ) {
        Self::render_text_layout_internal(
            layout,
            position,
            color,
            text_instances,
            clip_rect_opt,
            window_size,
            bundle.atlas,
            bundle.font_system,
            bundle.queue,
            bundle.scale_factor,
        )
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
