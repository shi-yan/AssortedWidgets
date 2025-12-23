use crate::element_manager::ElementManager;
use crate::event::{FocusManager, HitTester, InputEventEnum, MouseCapture};
use crate::layout::LayoutManager;
use crate::paint::PaintContext;
use crate::render::RenderContext;
use crate::scene_graph::SceneGraph;
use crate::types::{FrameInfo, Point, Size, WidgetId, WindowId};
use crate::window_render_state::WindowRenderState;
use std::time::Instant;

#[cfg(target_os = "macos")]
use crate::platform::{PlatformWindow, PlatformWindowImpl};

// ============================================================================
// Window - Per-window state and UI tree
// ============================================================================

/// Represents a single window with its UI tree and rendering state
///
/// Each window has its own:
/// - UI element tree (ElementManager, SceneGraph)
/// - Layout system (LayoutManager)
/// - Rendering surface (WindowRenderState)
///
/// Windows share:
/// - GPU context (RenderContext) - passed by reference
/// - Glyph atlas, fonts, text engine (SharedRenderState) - passed by reference
pub struct Window {
    // ========================================
    // Identity
    // ========================================
    id: WindowId,

    // ========================================
    // Platform Window
    // ========================================
    #[cfg(target_os = "macos")]
    platform_window: PlatformWindowImpl,

    // ========================================
    // UI State (Per-Window)
    // ========================================
    element_manager: ElementManager,
    scene_graph: SceneGraph,
    layout_manager: LayoutManager,
    window_size: Size,
    needs_layout: bool,

    // ========================================
    // Rendering (Per-Window)
    // ========================================
    /// Per-window rendering resources (surface, renderers, scale factor)
    /// References shared state (atlas, fonts) via Arc
    render_state: WindowRenderState,

    // ========================================
    // Animation / Frame Timing
    // ========================================
    /// Timestamp of the last frame (for calculating delta time)
    last_frame_time: Option<Instant>,

    /// Frame counter (increments each frame)
    frame_number: u64,

    // ========================================
    // Event Handling (Per-Window)
    // ========================================
    /// Hit tester for spatial event routing
    /// Updated after each paint pass to match rendering z-order
    hit_tester: HitTester,

    /// Focus manager for keyboard input and Tab navigation
    focus_manager: FocusManager,

    /// Mouse capture for drag operations
    mouse_capture: MouseCapture,
}

impl Window {
    /// Create a new window (internal - called by Application)
    #[cfg(target_os = "macos")]
    pub(crate) fn new(
        id: WindowId,
        platform_window: PlatformWindowImpl,
        render_state: WindowRenderState,
        window_size: Size,
    ) -> Self {
        Window {
            id,
            platform_window,
            element_manager: ElementManager::new(),
            scene_graph: SceneGraph::new(),
            layout_manager: LayoutManager::new(),
            window_size,
            needs_layout: true,
            render_state,
            last_frame_time: None,
            frame_number: 0,
            hit_tester: HitTester::new(),
            focus_manager: FocusManager::new(),
            mouse_capture: MouseCapture::new(),
        }
    }

    /// Get window ID
    pub fn id(&self) -> WindowId {
        self.id
    }

    /// Get window size
    pub fn size(&self) -> Size {
        self.window_size
    }

    /// Set window size and mark layout as dirty
    pub fn set_size(&mut self, size: Size) {
        if self.window_size != size {
            self.window_size = size;
            self.needs_layout = true;
        }
    }

    /// Mark layout as dirty (forces recomputation on next frame)
    pub fn mark_layout_dirty(&mut self) {
        self.needs_layout = true;
    }

    /// Get reference to element manager
    pub fn element_manager(&self) -> &ElementManager {
        &self.element_manager
    }

    /// Get mutable reference to element manager
    pub fn element_manager_mut(&mut self) -> &mut ElementManager {
        &mut self.element_manager
    }

    /// Get reference to scene graph
    pub fn scene_graph(&self) -> &SceneGraph {
        &self.scene_graph
    }

    /// Get mutable reference to scene graph
    pub fn scene_graph_mut(&mut self) -> &mut SceneGraph {
        &mut self.scene_graph
    }

    /// Get reference to layout manager
    pub fn layout_manager(&self) -> &LayoutManager {
        &self.layout_manager
    }

    /// Get mutable reference to layout manager
    pub fn layout_manager_mut(&mut self) -> &mut LayoutManager {
        &mut self.layout_manager
    }

    /// Get reference to render state
    pub fn render_state(&self) -> &WindowRenderState {
        &self.render_state
    }

    /// Get mutable reference to render state
    pub fn render_state_mut(&mut self) -> &mut WindowRenderState {
        &mut self.render_state
    }

    /// Get reference to platform window
    #[cfg(target_os = "macos")]
    pub fn platform_window(&self) -> &PlatformWindowImpl {
        &self.platform_window
    }

    /// Get mutable reference to platform window
    #[cfg(target_os = "macos")]
    pub fn platform_window_mut(&mut self) -> &mut PlatformWindowImpl {
        &mut self.platform_window
    }

    /// Resize window and update all rendering resources
    #[cfg(target_os = "macos")]
    pub fn resize(&mut self, bounds: crate::types::Rect, render_context: &RenderContext) {
        // Get scale factor from platform window
        let scale_factor = self.platform_window.scale_factor();

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📐 Window {:?} resize:", self.id);
        println!("  Logical size:  {:.0}x{:.0}", bounds.size.width, bounds.size.height);
        println!("  Scale factor:  {:.1}x", scale_factor);
        println!("  Physical size: {:.0}x{:.0}",
            bounds.size.width * scale_factor,
            bounds.size.height * scale_factor);

        // Update window size and mark layout as dirty
        self.window_size = bounds.size;
        self.needs_layout = true;

        // Check if scale factor changed (e.g., window moved to different DPI display)
        let scale_factor_changed = (self.render_state.scale_factor - scale_factor as f32).abs() > 0.01;
        if scale_factor_changed {
            println!("  ⚠️  DPI CHANGE: {:.1}x → {:.1}x",
                self.render_state.scale_factor, scale_factor);
            // Update the stored scale factor
            self.render_state.scale_factor = scale_factor as f32;
            // Note: No need to invalidate glyph atlas - glyphs at both scales are cached separately
            // via the scale_factor field in GlyphKey. This allows seamless transitions between displays!
        }

        // Calculate physical pixel size for Retina displays (for surface configuration only)
        let physical_size = Size::new(
            bounds.size.width * scale_factor,
            bounds.size.height * scale_factor
        );

        // Resize window renderer (surface needs physical size for actual pixel buffer)
        self.render_state.renderer.resize(render_context, bounds, scale_factor);

        // Update projection matrices with logical size and scale factor
        // The renderers will scale the uniform by scale_factor to match the physical viewport
        println!("  📊 Updating projection matrices: logical = {:.0}x{:.0}, scale = {:.1}x",
            bounds.size.width, bounds.size.height, scale_factor);

        self.render_state.rect_renderer.update_screen_size(render_context, bounds.size, scale_factor as f32);
        self.render_state.text_renderer.update_screen_size(render_context, bounds.size, scale_factor as f32);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }

    /// Dispatch an input event to elements
    ///
    /// Phase 2.2 implementation with focus management and mouse capture:
    /// - Mouse events: Use capture if active, otherwise hit test with z-order
    /// - Keyboard events: Dispatch to focused element, handle Tab/Shift+Tab
    /// - Wheel events: Hit test using z-order from paint pass
    /// - IME events: Dispatch to focused element
    pub fn dispatch_input_event(&mut self, mut event: InputEventEnum) {
        use crate::event::{EventResponse, Key, NamedKey};

        match &event {
            InputEventEnum::MouseDown(mouse_event) => {
                let position = mouse_event.position;

                // Check if mouse is captured
                let target = if let Some(captured_id) = self.mouse_capture.captured_id() {
                    Some(captured_id)
                } else {
                    // Hit test using z-order: find topmost element at this position
                    self.hit_tester.hit_test(position)
                };

                if let Some(widget_id) = target {
                    // Give focus to clicked element if it's focusable
                    if let Some(element) = self.element_manager.get(widget_id) {
                        if element.is_focusable() {
                            self.focus_manager.set_focus(Some(widget_id));
                        }
                    }

                    // Dispatch to element via dispatch_mouse_event method
                    if let Some(element) = self.element_manager.get_mut(widget_id) {
                        let response = element.dispatch_mouse_event(&mut event);

                        match response {
                            EventResponse::Handled => {
                                println!("[Window {:?}] Element {:?} handled mouse down", self.id, widget_id);
                            }
                            EventResponse::PassThrough => {
                                // TODO Phase 3: Bubble to parent
                            }
                            EventResponse::Ignored => {}
                        }
                    }
                }
            }

            InputEventEnum::MouseUp(mouse_event) => {
                let position = mouse_event.position;

                // Check if mouse is captured
                let target = if let Some(captured_id) = self.mouse_capture.captured_id() {
                    Some(captured_id)
                } else {
                    self.hit_tester.hit_test(position)
                };

                if let Some(widget_id) = target {
                    if let Some(element) = self.element_manager.get_mut(widget_id) {
                        let response = element.dispatch_mouse_event(&mut event);

                        match response {
                            EventResponse::Handled => {
                                println!("[Window {:?}] Element {:?} handled mouse up", self.id, widget_id);
                            }
                            EventResponse::PassThrough => {}
                            EventResponse::Ignored => {}
                        }
                    }
                }
            }

            InputEventEnum::MouseMove(mouse_event) => {
                let position = mouse_event.position;

                // Check if mouse is captured
                let target = if let Some(captured_id) = self.mouse_capture.captured_id() {
                    Some(captured_id)
                } else {
                    self.hit_tester.hit_test(position)
                };

                if let Some(widget_id) = target {
                    if let Some(element) = self.element_manager.get_mut(widget_id) {
                        element.dispatch_mouse_event(&mut event);
                    }
                }
            }

            InputEventEnum::KeyDown(key_event) => {
                // Handle Tab navigation
                if key_event.key == Key::Named(NamedKey::Tab) {
                    if key_event.modifiers.shift {
                        self.focus_manager.focus_previous();
                    } else {
                        self.focus_manager.focus_next();
                    }
                    return;
                }

                // Dispatch to focused element
                if let Some(focused_id) = self.focus_manager.focused_id() {
                    if let Some(element) = self.element_manager.get_mut(focused_id) {
                        let response = element.dispatch_key_event(&mut event);

                        if response == EventResponse::Handled {
                            println!("[Window {:?}] Element {:?} handled key down", self.id, focused_id);
                        }
                    }
                }
            }

            InputEventEnum::KeyUp(key_event) => {
                // Dispatch to focused element
                if let Some(focused_id) = self.focus_manager.focused_id() {
                    if let Some(element) = self.element_manager.get_mut(focused_id) {
                        let response = element.dispatch_key_event(&mut event);

                        if response == EventResponse::Handled {
                            println!("[Window {:?}] Element {:?} handled key up", self.id, focused_id);
                        }
                    }
                }
            }

            InputEventEnum::Wheel(wheel_event) => {
                // Use same hit test as mouse events
                let position = Point::new(0.0, 0.0); // TODO: track mouse position
                let target = self.hit_tester.hit_test(position);

                if let Some(widget_id) = target {
                    if let Some(element) = self.element_manager.get_mut(widget_id) {
                        let response = element.dispatch_wheel_event(&mut wheel_event.clone());

                        if response == EventResponse::Handled {
                            println!("[Window {:?}] Element {:?} handled wheel event", self.id, widget_id);
                        }
                    }
                }
            }

            InputEventEnum::Ime(ime_event) => {
                // Dispatch to focused element
                if let Some(focused_id) = self.focus_manager.focused_id() {
                    if let Some(element) = self.element_manager.get_mut(focused_id) {
                        let response = element.dispatch_ime_event(&mut ime_event.clone());

                        if response == EventResponse::Handled {
                            println!("[Window {:?}] Element {:?} handled IME event", self.id, focused_id);
                        }
                    }
                }
            }
        }
    }

    /// Get access to the focus manager
    pub fn focus_manager(&self) -> &FocusManager {
        &self.focus_manager
    }

    /// Get mutable access to the focus manager
    pub fn focus_manager_mut(&mut self) -> &mut FocusManager {
        &mut self.focus_manager
    }

    /// Get access to the mouse capture
    pub fn mouse_capture(&self) -> &MouseCapture {
        &self.mouse_capture
    }

    /// Get mutable access to the mouse capture
    pub fn mouse_capture_mut(&mut self) -> &mut MouseCapture {
        &mut self.mouse_capture
    }

    /// Update IME cursor position based on focused widget
    ///
    /// This should be called each frame to keep the IME window positioned correctly.
    #[cfg(target_os = "macos")]
    pub fn update_ime_position(&mut self) {
        if let Some(cursor_rect) = self.focus_manager.get_ime_cursor_rect(&self.element_manager) {
            // Convert to screen coordinates
            // For now, assume window coordinates = screen coordinates (no offset)
            // TODO: Add window position offset when we support window positioning

            let scale_factor = self.platform_window.scale_factor();

            // Convert logical coordinates to physical (screen) coordinates
            let screen_x = cursor_rect.origin.x * scale_factor;
            let screen_y = cursor_rect.origin.y * scale_factor;
            let screen_width = cursor_rect.size.width * scale_factor;
            let screen_height = cursor_rect.size.height * scale_factor;

            self.platform_window.set_ime_cursor_area(
                screen_x,
                screen_y,
                screen_width,
                screen_height,
            );
        }
    }

    /// Render a single frame
    ///
    /// This performs the full rendering pipeline:
    /// 1. Compute layout if needed
    /// 2. Paint elements (collect draw commands)
    /// 3. Render batched primitives to screen
    #[cfg(target_os = "macos")]
    pub fn render_frame(
        &mut self,
        render_context: &RenderContext,
    ) {
        // Get surface texture
        let surface_texture = match self.render_state.renderer.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Failed to get surface texture: {:?}", e);
                return;
            }
        };

        // Create texture view with sRGB format
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(self.render_state.renderer.format.add_srgb_suffix()),
            ..Default::default()
        });

        // 0. Update animations and time-based state (before layout)
        let now = Instant::now();
        let dt = self.last_frame_time
            .map(|last| (now - last).as_secs_f64())
            .unwrap_or(0.0);  // First frame has dt=0
        let frame_info = FrameInfo::new(dt, now, self.frame_number);

        // Update all elements that need continuous updates (animations, physics, etc.)
        let widget_ids: Vec<_> = self.element_manager.widget_ids().collect();
        for widget_id in widget_ids {
            if let Some(element) = self.element_manager.get_mut(widget_id) {
                if element.needs_continuous_updates() {
                    element.update(&frame_info);
                }
            }
        }

        // Update frame timing for next frame
        self.last_frame_time = Some(now);
        self.frame_number += 1;

        // 1. Compute layout if needed (skip if no scene graph)
        if self.needs_layout && self.scene_graph.root().is_some() {
            //println!("[Window {:?}] Computing layout...", self.id);

            // Mark all elements that need measurement as dirty in Taffy
            let widget_ids: Vec<_> = self.element_manager.widget_ids().collect();
            for widget_id in widget_ids {
                if let Some(element) = self.element_manager.get(widget_id) {
                    if element.needs_measure() {
                        //println!("[Window {:?}] Marking widget {:?} as dirty for re-measurement", self.id, widget_id);
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
            let widget_ids: Vec<_> = self.element_manager.widget_ids().collect();
            for widget_id in widget_ids {
                if let Some(bounds) = self.layout_manager.get_layout(widget_id) {
                    if let Some(element) = self.element_manager.get_mut(widget_id) {
                        element.set_bounds(bounds);
                        element.set_dirty(false);
                    }
                }
            }

            self.needs_layout = false;
        }

        // 2. Paint elements in tree order (collect draw commands)
        let scale_factor = self.platform_window.scale_factor() as f32;

        // Begin new frame for render state (atlas + text engine)
        self.render_state.begin_frame();

        // Paint phase - collect draw commands
        let (rect_instances, text_instances) = {
            // Lock shared resources for the duration of the frame
            let mut atlas_lock = render_context.glyph_atlas.lock().unwrap();
            let mut font_system_lock = render_context.font_system.lock().unwrap();
            let mut text_engine_lock = render_context.text_engine.lock().unwrap();

            // Create render bundle with references to locked resources
            let bundle = crate::paint::RenderBundle {
                atlas: &mut *atlas_lock,
                font_system: &mut *font_system_lock,
                text_engine: &mut *text_engine_lock,
                queue: &render_context.queue,
                device: &render_context.device,
                scale_factor,
            };

            let mut paint_ctx = PaintContext::new(self.window_size, bundle);

            if let Some(root) = self.scene_graph.root() {
                root.traverse(&mut |widget_id| {
                    if let Some(element) = self.element_manager.get(widget_id) {
                        element.paint(&mut paint_ctx);
                    }
                });
            }

            // Extract instances and hit tester before paint_ctx is dropped
            let mut rect_instances = paint_ctx.rect_instances().to_vec();
            let mut text_instances = paint_ctx.text_instances().to_vec();

            // Sort by z-order (low to high) for correct overlapping
            // Elements with lower z-order are drawn first (appear behind)
            // Elements with higher z-order are drawn last (appear on top)
            rect_instances.sort_by_key(|inst| inst.z_order);
            text_instances.sort_by_key(|inst| inst.z_order);

            // Finalize and clone hit tester for event dispatch
            let hit_tester = paint_ctx.finalized_hit_tester();

            (rect_instances, text_instances, hit_tester)
        };

        // Update window's hit tester with the one from this paint pass
        // This ensures hit testing uses the same z-order as rendering
        self.hit_tester = hit_tester;

        // Rebuild focus manager to sync with current element tree
        // This ensures focusable widgets are up-to-date for Tab navigation
        self.focus_manager.rebuild(&self.element_manager);

        // Update IME cursor position for the focused widget
        self.update_ime_position();

        // 3. Render batched primitives
        let mut encoder = render_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
            self.render_state.rect_renderer.render(render_context, &mut render_pass, &rect_instances);

            // Get atlas texture view from glyph atlas
            let atlas_lock = render_context.glyph_atlas.lock().unwrap();
            let atlas_texture_view = atlas_lock.texture_view();
            self.render_state.text_renderer.render(
                render_context,
                &mut render_pass,
                &text_instances,
                atlas_texture_view,
            );
            drop(atlas_lock);
        }

        // Submit commands
        render_context.queue.submit([encoder.finish()]);

        // Present the frame
        surface_texture.present();
    }
}
