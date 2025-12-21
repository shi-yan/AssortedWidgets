use assorted_widgets::{GuiEventLoop, WindowOptions};
use assorted_widgets::paint::{Color, PaintContext};
use assorted_widgets::text::{TextEngine, TextLayout, TextStyle, Truncate};
use assorted_widgets::types::{Point, Rect, Size};
use std::time::Instant;

fn main() {
    println!("AssortedWidgets - Text Rendering Demo (Phase 3.2)");
    println!("=================================================");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        let mut event_loop = pollster::block_on(async {
            GuiEventLoop::new_with_window(WindowOptions {
                title: "AssortedWidgets - Text Rendering Demo".to_string(),
                width: 1200,
                height: 900,
                ..Default::default()
            })
            .await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!");
        println!();

        println!("Creating text rendering demo...");
        println!();
        println!("This demonstrates:");
        println!("  - Text shaping with kerning and ligatures");
        println!("  - Bidirectional text (English + Arabic + Hebrew + Chinese)");
        println!("  - Emoji rendering");
        println!("  - Animated labels (sin wave position)");
        println!("  - Text truncation with ellipsis");
        println!("  - Text wrapping (multi-line)");
        println!();

        // Create text engine
        let mut text_engine = TextEngine::new();

        // Create text demo state
        let mut demo = TextDemo::new(&mut text_engine);

        println!("Starting continuous rendering...");
        println!("Press Cmd+Q to quit.");
        println!();

        let start_time = Instant::now();

        // Set custom render function
        event_loop.set_custom_render(move |ctx, atlas, font_system| {
            let elapsed = start_time.elapsed().as_secs_f32();

            // Begin frame for cache management
            text_engine.begin_frame();

            // Render demo
            demo.render(ctx, &mut text_engine, atlas, font_system, elapsed);
        });

        // Run event loop
        event_loop.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo only runs on macOS currently.");
    }
}

/// Text rendering demo state
struct TextDemo {
    // Manually owned layouts (low-level API demonstration)
    shaped_text_layout: TextLayout,
    bidirectional_layout: TextLayout,
    emoji_layout: TextLayout,

    // Animation parameters
    animation_amplitude: f32,
    animation_frequency: f32,
}

impl TextDemo {
    fn new(engine: &mut TextEngine) -> Self {
        // Create text styles
        let heading_style = TextStyle::new().size(32.0).bold();
        let body_style = TextStyle::new().size(18.0);
        let large_style = TextStyle::new().size(24.0);

        // Create pre-shaped layouts (low-level API)
        let shaped_text_layout = engine.create_layout(
            "The office offers efficient service",  // Tests ligatures: ffi, ff
            &body_style,
            None,
            Truncate::None,
        );

        let bidirectional_layout = engine.create_layout(
            "Hello שלום مرحبا 你好 👋",  // English + Hebrew + Arabic + Chinese + Emoji
            &large_style,
            None,
            Truncate::None,
        );

        let emoji_layout = engine.create_layout(
            "🚀 ⭐ 💡 🎨 🔥 ✨ 🌈 🎯",  // Color emoji test
            &heading_style,
            None,
            Truncate::None,
        );

        Self {
            shaped_text_layout,
            bidirectional_layout,
            emoji_layout,
            animation_amplitude: 50.0,
            animation_frequency: 1.0,
        }
    }

    fn render(
        &mut self,
        ctx: &mut PaintContext,
        engine: &mut TextEngine,
        atlas: &mut assorted_widgets::text::GlyphAtlas,
        font_system: &mut assorted_widgets::text::FontSystemWrapper,
        queue: &wgpu::Queue,
        elapsed: f32,
    ) {
        let window_size = ctx.window_size();

        // Clear background
        ctx.draw_rect(
            Rect::new(Point::new(0.0, 0.0), window_size),
            Color::rgb(0.95, 0.95, 0.95),
        );

        let mut y = 20.0;

        // ================================================================
        // Section 1: Static Text (High-Level Managed API)
        // ================================================================

        self.draw_section_title(ctx, atlas, font_system, "High-Level Managed API (Automatic Caching)", Point::new(20.0, y));
        y += 50.0;

        // Using high-level API - cached transparently
        // Note: In the actual implementation, this would be:
        // let layout = engine.get_or_create_managed("Hello World!", &style, None);
        // ctx.draw_layout(layout, ...);
        // For now, we'll use the low-level API to demonstrate

        let hello_layout = engine.create_layout(
            "Hello World! This text demonstrates automatic caching.",
            &TextStyle::new().size(20.0),
            None,
            Truncate::None,
        );
        ctx.draw_layout(&hello_layout, Point::new(40.0, y), Color::BLACK, atlas, font_system, queue);
        y += 40.0;

        // ================================================================
        // Section 2: Shaped Text with Ligatures
        // ================================================================

        y += 20.0;
        self.draw_section_title(ctx, atlas, font_system, "Text Shaping (Ligatures & Kerning)", Point::new(20.0, y));
        y += 50.0;

        ctx.draw_layout(&self.shaped_text_layout, Point::new(40.0, y), Color::rgb(0.2, 0.2, 0.8), atlas, font_system);
        y += 40.0;

        let ligature_note = engine.create_layout(
            "↑ Notice: 'ffi' and 'ff' should render as ligatures",
            &TextStyle::new().size(14.0).italic(),
            None,
            Truncate::None,
        );
        ctx.draw_layout(&ligature_note, Point::new(40.0, y), Color::rgb(0.5, 0.5, 0.5), atlas, font_system);
        y += 40.0;

        // ================================================================
        // Section 3: Bidirectional & Multi-Language Text
        // ================================================================

        y += 20.0;
        self.draw_section_title(ctx, atlas, font_system, "Bidirectional & Multi-Language", Point::new(20.0, y));
        y += 50.0;

        ctx.draw_layout(&self.bidirectional_layout, Point::new(40.0, y), Color::rgb(0.8, 0.2, 0.2), atlas, font_system);
        y += 50.0;

        // ================================================================
        // Section 4: Emoji Rendering
        // ================================================================

        y += 20.0;
        self.draw_section_title(ctx, atlas, font_system, "Color Emoji", Point::new(20.0, y));
        y += 50.0;

        ctx.draw_layout(&self.emoji_layout, Point::new(40.0, y), Color::BLACK, atlas, font_system);
        y += 60.0;

        // ================================================================
        // Section 5: Animated Labels (Sin Wave)
        // ================================================================

        y += 20.0;
        self.draw_section_title(ctx, atlas, font_system, "Animated Labels", Point::new(20.0, y));
        y += 50.0;

        // Animated label 1: horizontal oscillation
        let anim_x1 = 40.0 + (elapsed * self.animation_frequency).sin() * self.animation_amplitude;
        let anim_text1 = engine.create_layout(
            "← Oscillating horizontally →",
            &TextStyle::new().size(20.0).bold(),
            None,
            Truncate::None,
        );
        ctx.draw_layout(&anim_text1, Point::new(anim_x1 as f64, y), Color::rgb(0.0, 0.6, 0.0), atlas, font_system);
        y += 40.0;

        // Animated label 2: vertical oscillation
        let anim_y2 = y + (elapsed * self.animation_frequency * 0.7).sin() * (self.animation_amplitude * 0.5);
        let anim_text2 = engine.create_layout(
            "↑ Oscillating vertically ↓",
            &TextStyle::new().size(20.0).bold(),
            None,
            Truncate::None,
        );
        ctx.draw_layout(&anim_text2, Point::new(40.0, anim_y2), Color::rgb(0.6, 0.0, 0.6), atlas, font_system);
        y += 80.0;

        // ================================================================
        // Section 6: Text Truncation
        // ================================================================

        y += 20.0;
        self.draw_section_title(ctx, atlas, font_system, "Text Truncation", Point::new(20.0, y));
        y += 50.0;

        // Draw container box
        let truncate_box = Rect::new(Point::new(40.0, y), Size::new(300.0, 30.0));
        ctx.draw_rect(truncate_box, Color::rgb(1.0, 1.0, 0.9));
        ctx.draw_rect(
            Rect::new(truncate_box.origin, Size::new(truncate_box.size.width, 2.0)),
            Color::rgb(0.8, 0.8, 0.6),
        );
        ctx.draw_rect(
            Rect::new(
                Point::new(truncate_box.origin.x, truncate_box.origin.y + truncate_box.size.height - 2.0),
                Size::new(truncate_box.size.width, 2.0)
            ),
            Color::rgb(0.8, 0.8, 0.6),
        );

        // Truncated text (will implement ellipsis in Phase 3.3)
        let truncate_text = engine.create_layout(
            "This is a very long text that will be truncated because it doesn't fit in the box",
            &TextStyle::new().size(16.0),
            Some(300.0),  // Constrained width
            Truncate::End,
        );
        ctx.draw_layout(&truncate_text, Point::new(45.0, y + 5.0), Color::rgb(0.3, 0.3, 0.3), atlas, font_system);
        y += 50.0;

        // ================================================================
        // Section 7: Text Wrapping (Multi-line)
        // ================================================================

        y += 20.0;
        self.draw_section_title(ctx, atlas, font_system, "Text Wrapping (Multi-line)", Point::new(20.0, y));
        y += 50.0;

        // Draw container box
        let wrap_box = Rect::new(Point::new(40.0, y), Size::new(400.0, 120.0));
        ctx.draw_rect(wrap_box, Color::rgb(0.9, 1.0, 0.9));
        ctx.draw_rect(
            Rect::new(wrap_box.origin, Size::new(wrap_box.size.width, 2.0)),
            Color::rgb(0.6, 0.8, 0.6),
        );

        // Wrapped text
        let wrap_text = engine.create_layout(
            "This is a longer paragraph of text that will wrap to multiple lines when it reaches the edge of the container. Text wrapping is essential for any UI framework, and cosmic-text handles it automatically with proper line breaking at word boundaries.",
            &TextStyle::new().size(16.0).line_height(1.4),
            Some(400.0),  // Constrained width triggers wrapping
            Truncate::None,
        );
        ctx.draw_layout(&wrap_text, Point::new(45.0, y + 10.0), Color::rgb(0.2, 0.5, 0.2), atlas, font_system);

        // Display cache stats
        let stats = engine.cache_stats();
        let stats_text = format!(
            "Cache: {} entries | Frame: {}",
            stats.entry_count,
            stats.current_frame
        );
        let stats_layout = engine.create_layout(
            &stats_text,
            &TextStyle::new().size(12.0),
            None,
            Truncate::None,
        );
        ctx.draw_layout(
            &stats_layout,
            Point::new(20.0, window_size.height - 30.0),
            Color::rgb(0.5, 0.5, 0.5),
            atlas,
            font_system
        );
    }

    fn draw_section_title(
        &self,
        ctx: &mut PaintContext,
        atlas: &mut assorted_widgets::text::GlyphAtlas,
        font_system: &mut assorted_widgets::text::FontSystemWrapper,
        title: &str,
        position: Point,
    ) {
        // Draw background bar
        ctx.draw_rect(
            Rect::new(position, Size::new(ctx.window_size().width - 40.0, 35.0)),
            Color::rgb(0.2, 0.3, 0.5),
        );

        // Temporarily create a layout for the title
        // In a real app, these would be cached
        let mut temp_engine = TextEngine::new();
        let title_layout = temp_engine.create_layout(
            title,
            &TextStyle::new().size(20.0).bold(),
            None,
            Truncate::None,
        );

        ctx.draw_layout(
            &title_layout,
            Point::new(position.x + 10.0, position.y + 7.0),
            Color::WHITE,
            atlas,
            font_system
        );
    }
}
