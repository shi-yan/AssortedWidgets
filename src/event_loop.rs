use crate::element_manager::ElementManager;
use crate::event::GuiEvent;
use crate::handle::GuiHandle;
use crate::layout::LayoutManager;
use crate::paint::{PaintContext, RectRenderer};
use crate::render::{RenderContext, WindowRenderer};
use crate::scene_graph::SceneGraph;
use crate::text::{GlyphAtlas, FontSystemWrapper, TextRenderer, TextEngine, TextLayout, TextStyle, Truncate};
use crate::types::{Size, Point};

#[cfg(target_os = "macos")]
use crate::platform::{PlatformInput, PlatformWindow, PlatformWindowImpl, WindowCallbacks, WindowOptions};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// ============================================================================
// Main Event Loop
// ============================================================================

pub struct GuiEventLoop {
    element_manager: ElementManager,
    scene_graph: SceneGraph,
    layout_manager: LayoutManager,
    rect_renderer: Option<RectRenderer>,
    text_renderer: Option<TextRenderer>,
    glyph_atlas: Option<GlyphAtlas>,
    font_system: FontSystemWrapper,
    // Phase 3.2: Text engine with dual-mode caching
    text_engine: TextEngine,
    // Demo state for text rendering test
    demo_layouts: Option<DemoTextLayouts>,
    demo_start_time: std::time::Instant,
    // Demo frame counter (replaces static mut for Rust 2024 compatibility)
    demo_frame_count: u64,
    demo_atlas_dumped: bool,
    window_size: Size,
    needs_layout: bool,
    render_context: Arc<RenderContext>,
    event_queue: Arc<Mutex<VecDeque<GuiEvent>>>,
    render_fn: Option<Box<dyn FnMut(&WindowRenderer, &RenderContext)>>,
    #[cfg(target_os = "macos")]
    window: Option<PlatformWindowImpl>,
    #[cfg(target_os = "macos")]
    renderer: Option<WindowRenderer>,
}

/// Pre-shaped text layouts for demo (low-level API demonstration)
struct DemoTextLayouts {
    shaped_text: TextLayout,
    bidirectional: TextLayout,
    emoji: TextLayout,
}

impl GuiEventLoop {
    /// Create a new event loop with rendering support
    ///
    /// This initializes the WebGPU rendering context.
    /// Use `pollster::block_on(GuiEventLoop::new())` for simple blocking initialization.
    pub async fn new() -> Result<Self, String> {
        let render_context = RenderContext::new().await?;

        Ok(GuiEventLoop {
            element_manager: ElementManager::new(),
            scene_graph: SceneGraph::new(),
            layout_manager: LayoutManager::new(),
            rect_renderer: None,  // Created when window is created
            text_renderer: None,  // Created when window is created
            glyph_atlas: None,    // Created when window is created
            font_system: FontSystemWrapper::new(),
            text_engine: TextEngine::new(),
            demo_layouts: None,  // Will be initialized after window is created
            demo_start_time: std::time::Instant::now(),
            demo_frame_count: 0,
            demo_atlas_dumped: false,
            window_size: Size::new(800.0, 600.0),  // Default size
            needs_layout: true,
            render_context: Arc::new(render_context),
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            render_fn: None,
            #[cfg(target_os = "macos")]
            window: None,
            #[cfg(target_os = "macos")]
            renderer: None,
        })
    }

    /// Create a new event loop with a window (async)
    #[cfg(target_os = "macos")]
    pub async fn new_with_window(options: WindowOptions) -> Result<Self, String> {
        let mut event_loop = Self::new().await?;
        event_loop.create_window(options)?;
        Ok(event_loop)
    }

    /// Create a window with rendering surface
    #[cfg(target_os = "macos")]
    pub fn create_window(&mut self, options: WindowOptions) -> Result<(), String> {
        let mut window = PlatformWindowImpl::new(options);

        // Create window renderer
        let renderer = WindowRenderer::new(&self.render_context, &window)?;

        // Initialize window size
        let bounds = window.bounds();
        let scale_factor = window.scale_factor();
        self.window_size = bounds.size;

        println!("Initializing renderers with scale factor: {}", scale_factor);

        // Create rectangle renderer with the surface format
        let mut rect_renderer = RectRenderer::new(
            &self.render_context,
            renderer.format,
        );

        // Set initial screen size (physical pixels for Retina)
        let physical_size = Size::new(
            bounds.size.width * scale_factor,
            bounds.size.height * scale_factor
        );
        rect_renderer.update_screen_size(&self.render_context, physical_size);
        self.rect_renderer = Some(rect_renderer);

        // Create glyph atlas (2048x2048 pages, max 8 pages)
        self.glyph_atlas = Some(GlyphAtlas::new(
            &self.render_context.device,
            2048,
            8,
        ));

        // Create text renderer with the surface format
        let mut text_renderer = TextRenderer::new(
            &self.render_context,
            renderer.format,
        );

        // Set initial screen size (physical pixels for Retina)
        text_renderer.update_screen_size(&self.render_context, physical_size);
        self.text_renderer = Some(text_renderer);

        // Mark that we need to compute layout
        self.needs_layout = true;

        // Clone event queue Arc for callbacks to use
        let event_queue_input = self.event_queue.clone();
        let event_queue_frame = self.event_queue.clone();
        let event_queue_resize = self.event_queue.clone();
        let event_queue_close = self.event_queue.clone();

        // Set up callbacks to push events to queue
        let callbacks = WindowCallbacks {
            input: Some(Box::new(move |input| {
                event_queue_input.lock().unwrap().push_back(GuiEvent::Input(input));
            })),
            request_frame: Some(Box::new(move || {
                event_queue_frame.lock().unwrap().push_back(GuiEvent::RedrawRequested);
            })),
            resize: Some(Box::new(move |bounds| {
                event_queue_resize.lock().unwrap().push_back(GuiEvent::Resize(bounds));
            })),
            moved: Some(Box::new(|_position| {
                // Window moved (not important for demo)
            })),
            close: Some(Box::new(move || {
                event_queue_close.lock().unwrap().push_back(GuiEvent::Close);
            })),
            active_status_change: Some(Box::new(|_active| {
                // Window activation status changed (not important for demo)
            })),
        };

        window.set_callbacks(callbacks);
        self.window = Some(window);
        self.renderer = Some(renderer);

        Ok(())
    }

    /// Set the render function to be called each frame
    ///
    /// The render function receives references to the WindowRenderer and RenderContext
    /// and should perform all rendering operations.
    pub fn set_render_fn<F>(&mut self, f: F)
    where
        F: FnMut(&WindowRenderer, &RenderContext) + 'static,
    {
        self.render_fn = Some(Box::new(f));
    }

    /// Main event loop iteration
    /// This is for platforms that use a polling model (Linux, Windows)
    pub fn process_events(&mut self) {
        // Phase 1: Capture OS events
        // (For macOS, events come through callbacks)
        // (For Linux/Windows, would poll here)

        // Phase 2: Handle events
        // self.element_manager.handle_os_event(os_event);

        // Phase 3: Process all queued messages (signal/slot dispatch)
        self.element_manager.process_messages();

        // Phase 4: Render dirty elements
        // (Would trigger redraw for any elements marked dirty)
    }

    /// Run the macOS event loop with manual runloop control
    ///
    /// This manually pumps the macOS runloop and processes events from the queue.
    /// This gives us full control over the event loop and allows clean event handling
    /// without RefCell or runtime borrow checking.
    ///
    /// This function never returns on macOS.
    #[cfg(target_os = "macos")]
    pub fn run(&mut self) -> ! {
        use objc2::rc::autoreleasepool;
        use objc2_app_kit::{NSApplication, NSEventMask};
        use objc2_foundation::{MainThreadMarker, NSDate, NSDefaultRunLoopMode, NSRunLoop};

        println!("Starting manual event loop...");

        // Trigger initial frame
        if let Some(window) = self.window.as_mut() {
            window.invalidate();
        }

        let mtm = MainThreadMarker::new().expect("Must be on main thread");

        loop {
            autoreleasepool(|_| {
                unsafe {
                    // Process NSApplication events first
                    let app = NSApplication::sharedApplication(mtm);
                    let until_date = NSDate::distantPast();

                    loop {
                        let event = app.nextEventMatchingMask_untilDate_inMode_dequeue(
                            NSEventMask::Any,
                            Some(&until_date),
                            NSDefaultRunLoopMode,
                            true,
                        );

                        if event.is_none() {
                            break;
                        }

                        app.sendEvent(&event.unwrap());
                    }

                    // Now pump the runloop briefly to handle timers/sources
                    let run_loop = NSRunLoop::currentRunLoop();
                    let date = NSDate::dateWithTimeIntervalSinceNow(0.001);
                    run_loop.runMode_beforeDate(NSDefaultRunLoopMode, &date);
                }
            });

            // Process all queued events posted by platform callbacks
            loop {
                let event = self.event_queue.lock().unwrap().pop_front();
                match event {
                    Some(GuiEvent::RedrawRequested) => {
                        // Will render at end of loop iteration
                    }
                    Some(GuiEvent::Resize(bounds)) => {
                        println!("Window resized to {:.0}x{:.0}", bounds.size.width, bounds.size.height);

                        // Update window size and mark layout as dirty
                        self.window_size = bounds.size;
                        self.needs_layout = true;

                        // Update renderer surface with scale factor for Retina displays
                        if let Some(renderer) = self.renderer.as_mut() {
                            let scale_factor = self.window.as_ref().map(|w| w.scale_factor()).unwrap_or(1.0);
                            renderer.resize(&self.render_context, bounds, scale_factor);
                        }

                        // Update rect renderer with physical pixel dimensions
                        if let Some(rect_renderer) = self.rect_renderer.as_mut() {
                            let scale_factor = self.window.as_ref().map(|w| w.scale_factor()).unwrap_or(1.0);
                            let physical_size = Size::new(
                                bounds.size.width * scale_factor,
                                bounds.size.height * scale_factor
                            );
                            rect_renderer.update_screen_size(&self.render_context, physical_size);
                        }

                        // Update text renderer with physical pixel dimensions
                        if let Some(text_renderer) = self.text_renderer.as_mut() {
                            let scale_factor = self.window.as_ref().map(|w| w.scale_factor()).unwrap_or(1.0);
                            let physical_size = Size::new(
                                bounds.size.width * scale_factor,
                                bounds.size.height * scale_factor
                            );
                            text_renderer.update_screen_size(&self.render_context, physical_size);
                        }

                        // Request redraw after resize
                        if let Some(window) = self.window.as_mut() {
                            window.invalidate();
                        }
                    }
                    Some(GuiEvent::Input(input)) => {
                        // Handle input events
                        match input {
                            PlatformInput::MouseDown { position, button, .. } => {
                                println!("Mouse {:?} clicked at ({:.1}, {:.1})", button, position.x, position.y);
                            }
                            PlatformInput::KeyDown { key, .. } => {
                                println!("Key pressed: {}", key);
                            }
                            _ => {}
                        }
                        // TODO: Convert to OsEvent and dispatch to ElementManager
                    }
                    Some(GuiEvent::Close) => {
                        println!("Window closing - goodbye!");
                        std::process::exit(0);
                    }
                    None => break, // No more events to process
                }
            }

            // Process element manager messages (signal/slot system)
            self.element_manager.process_messages();

            // Mark layout dirty for continuous animation updates
            // This triggers re-measurement each frame, allowing AnimatedRect's
            // measure function to return updated width based on elapsed time
            self.needs_layout = true;

            // Render frame using built-in layout → paint → render flow
            if self.renderer.is_some() && self.rect_renderer.is_some() {
                self.render_frame_internal();
            } else if let (Some(renderer), Some(ref mut render_fn)) =
                (self.renderer.as_ref(), self.render_fn.as_mut()) {
                // Fallback to external render function if no rect_renderer
                render_fn(renderer, &self.render_context);
            }

            // Request next frame for continuous animation
            if let Some(window) = self.window.as_mut() {
                window.invalidate();
            }
        }
    }

    /// Internal frame rendering with layout → paint → render flow
    #[cfg(target_os = "macos")]
    fn render_frame_internal(&mut self) {
        let renderer = self.renderer.as_ref().unwrap();

        // Get surface texture
        let surface_texture = match renderer.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Failed to get surface texture: {:?}", e);
                return;
            }
        };

        // Create texture view with sRGB format
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(renderer.format.add_srgb_suffix()),
            ..Default::default()
        });

        // 1. Compute layout if needed (skip if no scene graph)
        if self.needs_layout && self.scene_graph.root().is_some() {
            println!("[EventLoop] Computing layout...");

            // Mark all elements that need measurement as dirty in Taffy
            // This forces Taffy to re-invoke their measure functions
            let widget_ids: Vec<_> = self.element_manager.widget_ids().collect();
            for widget_id in widget_ids {
                if let Some(element) = self.element_manager.get(widget_id) {
                    if element.needs_measure() {
                        println!("[EventLoop] Marking widget {:?} as dirty for re-measurement", widget_id);
                        if let Err(e) = self.layout_manager.mark_dirty(widget_id) {
                            eprintln!("Failed to mark widget {:?} dirty: {}", widget_id, e);
                        }
                    }
                }
            }

            // Use compute_layout_with_measure to support elements with dynamic sizing
            if let Err(e) = self.layout_manager.compute_layout_with_measure(
                self.window_size,
                |known, available, _node_id, context, _style| {
                    // Dispatch to element's measure method if needed
                    if let Some(ctx) = context {
                        if ctx.needs_measure {
                            // Look up element and call its measure() method
                            if let Some(element) = self.element_manager.get(ctx.widget_id) {
                                if let Some(size) = element.measure(known, available) {
                                    return taffy::Size {
                                        width: size.width as f32,
                                        height: size.height as f32,
                                    };
                                }
                            }
                        }
                    }
                    // Default: no intrinsic size
                    taffy::Size::ZERO
                },
            ) {
                eprintln!("Layout computation failed: {}", e);
                return;
            }

            // Apply layout results to elements
            // Collect IDs first to avoid borrow checker issues
            let widget_ids: Vec<_> = self.element_manager.widget_ids().collect();
            for widget_id in widget_ids {
                if let Some(bounds) = self.layout_manager.get_layout(widget_id) {
                    if let Some(element) = self.element_manager.get_mut(widget_id) {
                        element.set_bounds(bounds);
                    }
                }
            }

            self.needs_layout = false;
        }

        // 2. Paint elements in tree order (collect draw commands)
        let mut paint_ctx = PaintContext::new(self.window_size);
        if let Some(root) = self.scene_graph.root() {
            root.traverse(&mut |widget_id| {
                if let Some(element) = self.element_manager.get(widget_id) {
                    element.paint(&mut paint_ctx);
                }
            });
        }

        // 2.5. Phase 3.1 Test: Render text manually
        // This demonstrates English, Chinese, and emoji rendering
        self.render_test_text(&mut paint_ctx);

        // 3. Render batched primitives
        let mut encoder = self.render_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Scene Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Scene Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            // Render all rectangles
            if let Some(rect_renderer) = self.rect_renderer.as_mut() {
                rect_renderer.render(&self.render_context, &mut render_pass, paint_ctx.rect_instances());
            }

            // Render all text glyphs
            if let (Some(text_renderer), Some(glyph_atlas)) =
                (self.text_renderer.as_mut(), self.glyph_atlas.as_ref()) {
                text_renderer.render(
                    &self.render_context,
                    &mut render_pass,
                    paint_ctx.text_instances(),
                    glyph_atlas.texture_view(),
                );
            }
        }

        // Submit commands
        self.render_context.queue.submit([encoder.finish()]);

        // Present the frame
        surface_texture.present();
    }

    pub fn get_handle(&self) -> GuiHandle {
        self.element_manager.get_handle()
    }

    pub fn render_context(&self) -> &RenderContext {
        &self.render_context
    }

    pub fn layout_manager(&self) -> &LayoutManager {
        &self.layout_manager
    }

    pub fn layout_manager_mut(&mut self) -> &mut LayoutManager {
        &mut self.layout_manager
    }

    pub fn element_manager(&self) -> &ElementManager {
        &self.element_manager
    }

    pub fn element_manager_mut(&mut self) -> &mut ElementManager {
        &mut self.element_manager
    }

    pub fn scene_graph(&self) -> &SceneGraph {
        &self.scene_graph
    }

    pub fn scene_graph_mut(&mut self) -> &mut SceneGraph {
        &mut self.scene_graph
    }

    pub fn mark_layout_dirty(&mut self) {
        self.needs_layout = true;
    }

    #[cfg(target_os = "macos")]
    pub fn window(&self) -> Option<&PlatformWindowImpl> {
        self.window.as_ref()
    }

    #[cfg(target_os = "macos")]
    pub fn window_mut(&mut self) -> Option<&mut PlatformWindowImpl> {
        self.window.as_mut()
    }

    #[cfg(target_os = "macos")]
    pub fn renderer(&self) -> Option<&WindowRenderer> {
        self.renderer.as_ref()
    }

    #[cfg(target_os = "macos")]
    pub fn renderer_mut(&mut self) -> Option<&mut WindowRenderer> {
        self.renderer.as_mut()
    }

    pub fn glyph_atlas(&self) -> Option<&GlyphAtlas> {
        self.glyph_atlas.as_ref()
    }

    pub fn glyph_atlas_mut(&mut self) -> Option<&mut GlyphAtlas> {
        self.glyph_atlas.as_mut()
    }

    pub fn font_system(&self) -> &FontSystemWrapper {
        &self.font_system
    }

    pub fn font_system_mut(&mut self) -> &mut FontSystemWrapper {
        &mut self.font_system
    }

    /// Phase 3.2 Test: Render text using TextEngine and draw_layout()
    #[cfg(target_os = "macos")]
    fn render_test_text(&mut self, paint_ctx: &mut PaintContext) {
        use crate::paint::Color;
        use crate::text::{TextStyle, Truncate};
        use crate::types::Point;

        // Get scale factor for Retina display support
        let scale_factor = self.window.as_ref().map(|w| w.scale_factor()).unwrap_or(1.0) as f32;

        // Early return if text systems aren't ready
        let (glyph_atlas, queue) = match (self.glyph_atlas.as_mut(), Some(&self.render_context.queue)) {
            (Some(atlas), Some(q)) => (atlas, q),
            _ => return,
        };

        // Begin new frame for both atlas and text engine
        glyph_atlas.begin_frame();
        self.text_engine.begin_frame();

        // Log atlas stats at start of frame (using instance fields for Rust 2024 compatibility)
        let stats = glyph_atlas.stats();

        /*if self.demo_frame_count % 60 == 0 {  // Log every 60 frames
            println!("[ATLAS] Frame {}: {} pages, {} glyphs, current frame: {}",
                self.demo_frame_count, stats.page_count, stats.total_glyphs, stats.current_frame);
        }*/

        // Dump atlas once after we have some glyphs
       /*  if !self.demo_atlas_dumped && stats.total_glyphs >= 6 {
            println!("[DEBUG] Dumping atlas to PNG...");
            if let Err(e) = glyph_atlas.dump_page_to_png(
                &self.render_context.device,
                &self.render_context.queue,
                0,  // First page
                "atlas_debug.png"
            ) {
                eprintln!("Failed to dump atlas: {}", e);
            } else {
                println!("[DEBUG] Atlas dumped successfully to atlas_debug.png");
            }
            self.demo_atlas_dumped = true;
        }*/

        self.demo_frame_count += 1;

        // ================================================================
        // Phase 3.2 Demo: Using TextEngine and draw_layout()
        // ================================================================

        // Test styles
        let heading_style = TextStyle::new().size(32.0).bold();
        let body_style = TextStyle::new().size(18.0);
        let large_style = TextStyle::new().size(24.0);

        let mut y = 50.0;
 
        // 1. Basic text with ligatures (demonstrates shaping)
        let shaped_layout = self.text_engine.create_layout(
            "The office offers efficient service",  // Tests ligatures: ffi, ff
            &body_style,
            None,
            Truncate::None,
        );
        paint_ctx.draw_layout(
            &shaped_layout,
            Point::new(40.0, y),
            Color::WHITE,
            glyph_atlas,
            &mut self.font_system,
            queue,
            scale_factor,
        );
        y += 50.0;

        // 2. Bidirectional multi-language text (English + Hebrew + Arabic + Chinese + Emoji)
        let bidi_layout = self.text_engine.create_layout(
            "Hello שלום مرحبا 你好 👋",
            &large_style,
            None,
            Truncate::None,
        );
        paint_ctx.draw_layout(
            &bidi_layout,
            Point::new(40.0, y),
            Color { r: 0.5, g: 1.0, b: 0.5, a: 1.0 },
            glyph_atlas,
            &mut self.font_system,
            queue,
            scale_factor,
        );
        y += 60.0;

        // 3. Color emoji test
        let emoji_layout = self.text_engine.create_layout(
            "🚀 ⭐ 💡 🎨 🔥 ✨",
            &heading_style,
            None,
            Truncate::None,
        );
        paint_ctx.draw_layout(
            &emoji_layout,
            Point::new(40.0, y),
            Color { r: 1.0, g: 1.0, b: 0.5, a: 1.0 },
            glyph_atlas,
            &mut self.font_system,
            queue,
            scale_factor,
        );
        y += 60.0;

        // 4. Text truncation demo
        let truncate_layout = self.text_engine.create_layout(
            "This is a very long text that will be truncated",
            &body_style,
            Some(250.0),  // Constrained width
            Truncate::End,
        );
        paint_ctx.draw_layout(
            &truncate_layout,
            Point::new(40.0, y),
            Color { r: 1.0, g: 0.8, b: 0.5, a: 1.0 },
            glyph_atlas,
            &mut self.font_system,
            queue,
            scale_factor,
        );
        y += 50.0;

        // 5. Text wrapping (multi-line)
        let wrap_layout = self.text_engine.create_layout(
            "This is a longer paragraph that will wrap to multiple lines when it reaches the edge of the container.",
            &body_style,
            Some(350.0),  // Constrained width triggers wrapping
            Truncate::None,
        );
        paint_ctx.draw_layout(
            &wrap_layout,
            Point::new(40.0, y),
            Color { r: 0.5, g: 0.8, b: 1.0, a: 1.0 },
            glyph_atlas,
            &mut self.font_system,
            queue,
            scale_factor,
        );

        // Display cache stats
        let engine_stats = self.text_engine.cache_stats();
        let atlas_stats = glyph_atlas.stats();
        let stats_text = format!(
            "TextEngine: {} entries, frame {} | Atlas: {} glyphs, {} pages",
            engine_stats.entry_count, engine_stats.current_frame,
            atlas_stats.total_glyphs, atlas_stats.page_count
        );
        let stats_layout = self.text_engine.create_layout(
            &stats_text,
            &TextStyle::new().size(12.0),
            None,
            Truncate::None,
        );
        paint_ctx.draw_layout(
            &stats_layout,
            Point::new(20.0, self.window_size.height - 30.0),
            Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 },
            glyph_atlas,
            &mut self.font_system,
            queue,
            scale_factor,
        );
    }
}
