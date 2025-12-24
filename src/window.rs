use crate::element::Element;
use crate::element_manager::ElementManager;
use crate::event::{FocusManager, GuiEvent, HitTester, InputEventEnum, MouseCapture};
use crate::layout::LayoutManager;
use crate::paint::PaintContext;
use crate::render::{RenderContext, WindowRenderer};
use crate::scene_graph::{SceneGraph, SceneNode};
use crate::types::{FrameInfo, Point, Size, WidgetId, WindowId};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
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
/// - Rendering surface and uniforms (WindowRenderer)
///
/// Windows share (via WindowRenderer's Arc<RenderContext>):
/// - GPU resources (device, queue, adapter)
/// - Rendering pipelines (RectPipeline, TextPipeline)
/// - Glyph atlas, fonts, text engine
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
    /// Per-window rendering resources (surface, uniforms, instance buffers)
    /// Holds Arc<RenderContext> for shared GPU resources and pipelines
    window_renderer: WindowRenderer,

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

    /// Event queue for posting events back to application (cross-window drag, etc.)
    event_queue: Arc<Mutex<VecDeque<(WindowId, GuiEvent)>>>,
}

impl Window {
    /// Create a new window (internal - called by Application)
    #[cfg(target_os = "macos")]
    pub(crate) fn new(
        id: WindowId,
        platform_window: PlatformWindowImpl,
        window_renderer: WindowRenderer,
        window_size: Size,
        event_queue: Arc<Mutex<VecDeque<(WindowId, GuiEvent)>>>,
    ) -> Self {
        Window {
            id,
            platform_window,
            element_manager: ElementManager::new(),
            scene_graph: SceneGraph::new(),
            layout_manager: LayoutManager::new(),
            window_size,
            needs_layout: true,
            window_renderer,
            last_frame_time: None,
            frame_number: 0,
            hit_tester: HitTester::new(),
            focus_manager: FocusManager::new(),
            mouse_capture: MouseCapture::new(),
            event_queue,
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

    /// Get reference to window renderer
    pub fn window_renderer(&self) -> &WindowRenderer {
        &self.window_renderer
    }

    /// Get mutable reference to window renderer
    pub fn window_renderer_mut(&mut self) -> &mut WindowRenderer {
        &mut self.window_renderer
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

    // ========================================
    // Unified Widget Management
    // ========================================

    /// Add a widget as the root of the window
    ///
    /// This is a high-level API that coordinates three systems:
    /// 1. Adds element to ElementManager
    /// 2. Creates root node in LayoutManager
    /// 3. Sets root in SceneGraph
    ///
    /// # Arguments
    /// * `element` - The widget to add (must implement Element trait)
    /// * `style` - Layout style (flex, grid, absolute positioning, etc.)
    ///
    /// # Example
    /// ```ignore
    /// let panel = Panel::new(WidgetId::new(1));
    /// window.add_root_widget(Box::new(panel), Style {
    ///     display: Display::Flex,
    ///     flex_direction: FlexDirection::Column,
    ///     ..Default::default()
    /// })?;
    /// ```
    pub fn add_root_widget(
        &mut self,
        element: Box<dyn Element>,
        style: crate::layout::Style,
    ) -> Result<WidgetId, String> {
        let widget_id = element.id();

        // 1. Add to ElementManager
        self.element_manager.add_element(element);

        // 2. Create layout node
        if self.element_manager.get(widget_id).unwrap().needs_measure() {
            self.layout_manager.create_measurable_node(widget_id, style)?;
        } else {
            self.layout_manager.create_node(widget_id, style)?;
        }

        // 3. Set as layout root
        self.layout_manager.set_root(widget_id)?;

        // 4. Create scene graph root
        let root_node = SceneNode::new(widget_id);
        self.scene_graph.set_root(root_node);

        // Mark layout as dirty
        self.needs_layout = true;

        Ok(widget_id)
    }

    /// Add a widget as a child of an existing parent widget
    ///
    /// This is a high-level API that coordinates three systems:
    /// 1. Adds element to ElementManager
    /// 2. Creates child node in LayoutManager and establishes parent-child relationship
    /// 3. Adds child to parent in SceneGraph
    ///
    /// # Arguments
    /// * `element` - The widget to add (must implement Element trait)
    /// * `style` - Layout style for this child
    /// * `parent_id` - The parent widget's ID
    ///
    /// # Example
    /// ```ignore
    /// let button = Button::new(WidgetId::new(2));
    /// window.add_child_widget(Box::new(button), Style {
    ///     size: Size {
    ///         width: Dimension::Length(100.0),
    ///         height: Dimension::Length(30.0),
    ///     },
    ///     ..Default::default()
    /// }, panel_id)?;
    /// ```
    pub fn add_child_widget(
        &mut self,
        element: Box<dyn Element>,
        style: crate::layout::Style,
        parent_id: WidgetId,
    ) -> Result<WidgetId, String> {
        let widget_id = element.id();

        // 1. Add to ElementManager
        self.element_manager.add_element(element);

        // 2. Create layout node
        if self.element_manager.get(widget_id).unwrap().needs_measure() {
            self.layout_manager.create_measurable_node(widget_id, style)?;
        } else {
            self.layout_manager.create_node(widget_id, style)?;
        }

        // 3. Establish parent-child relationship in LayoutManager
        self.layout_manager.add_child(parent_id, widget_id)?;

        // 4. Add to SceneGraph
        self.add_scene_graph_child(parent_id, widget_id)?;

        // Mark layout as dirty
        self.needs_layout = true;

        Ok(widget_id)
    }

    /// Internal helper: Add child to SceneGraph by finding parent node
    fn add_scene_graph_child(&mut self, parent_id: WidgetId, child_id: WidgetId) -> Result<(), String> {
        // Find parent node in scene graph
        if let Some(root) = self.scene_graph.root_mut() {
            Self::add_child_to_node(root, parent_id, child_id)?;
            Ok(())
        } else {
            Err(format!("No root in scene graph - cannot add child {:?}", child_id))
        }
    }

    /// Recursive helper to find parent and add child
    fn add_child_to_node(node: &mut SceneNode, parent_id: WidgetId, child_id: WidgetId) -> Result<(), String> {
        if node.id == parent_id {
            // Found parent - add child
            node.add_child(SceneNode::new(child_id));
            return Ok(());
        }

        // Recursively search children
        for child in &mut node.children {
            if Self::add_child_to_node(child, parent_id, child_id).is_ok() {
                return Ok(());
            }
        }

        Err(format!("Parent {:?} not found in scene graph", parent_id))
    }

    /// Remove a widget and all its descendants
    ///
    /// This coordinates removal across all three systems:
    /// 1. Removes from ElementManager
    /// 2. Removes from LayoutManager (including descendants)
    /// 3. Removes from SceneGraph
    ///
    /// # Example
    /// ```ignore
    /// window.remove_widget(button_id)?;
    /// ```
    pub fn remove_widget(&mut self, widget_id: WidgetId) -> Result<(), String> {
        // 1. Remove from LayoutManager (this removes descendants too)
        self.layout_manager.remove_node(widget_id)?;

        // 2. Collect all descendants from SceneGraph
        let mut to_remove = vec![widget_id];
        if let Some(root) = self.scene_graph.root() {
            Self::collect_descendants(root, widget_id, &mut to_remove);
        }

        // 3. Remove from ElementManager
        for id in &to_remove {
            self.element_manager.remove_element(*id);
        }

        // 4. Remove from SceneGraph
        self.remove_from_scene_graph(widget_id)?;

        // Mark layout as dirty
        self.needs_layout = true;

        Ok(())
    }

    /// Recursive helper to collect all descendants
    fn collect_descendants(node: &SceneNode, target_id: WidgetId, result: &mut Vec<WidgetId>) {
        if node.id == target_id {
            // Found target - collect all children
            Self::collect_all_children(node, result);
            return;
        }

        for child in &node.children {
            Self::collect_descendants(child, target_id, result);
        }
    }

    /// Recursive helper to collect all children
    fn collect_all_children(node: &SceneNode, result: &mut Vec<WidgetId>) {
        for child in &node.children {
            result.push(child.id);
            Self::collect_all_children(child, result);
        }
    }

    /// Remove node from SceneGraph
    fn remove_from_scene_graph(&mut self, widget_id: WidgetId) -> Result<(), String> {
        if let Some(root) = self.scene_graph.root() {
            if root.id == widget_id {
                // Removing root
                self.scene_graph = SceneGraph::new();
                return Ok(());
            }
        }

        if let Some(root) = self.scene_graph.root_mut() {
            Self::remove_child_from_node(root, widget_id)?;
        }

        Ok(())
    }

    /// Recursive helper to remove child from scene graph
    fn remove_child_from_node(node: &mut SceneNode, target_id: WidgetId) -> Result<(), String> {
        // Check direct children
        if let Some(index) = node.children.iter().position(|child| child.id == target_id) {
            node.children.remove(index);
            return Ok(());
        }

        // Recursively search
        for child in &mut node.children {
            if Self::remove_child_from_node(child, target_id).is_ok() {
                return Ok(());
            }
        }

        Err(format!("Widget {:?} not found in scene graph", target_id))
    }

    /// Resize window and update all rendering resources
    #[cfg(target_os = "macos")]
    pub fn resize(&mut self, bounds: crate::types::Rect, _render_context: &RenderContext) {
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
        let scale_factor_changed = (self.window_renderer.scale_factor - scale_factor as f32).abs() > 0.01;
        if scale_factor_changed {
            println!("  ⚠️  DPI CHANGE: {:.1}x → {:.1}x",
                self.window_renderer.scale_factor, scale_factor);
            // Note: No need to invalidate glyph atlas - glyphs at both scales are cached separately
            // via the scale_factor field in GlyphKey. This allows seamless transitions between displays!
        }

        // WindowRenderer.resize() handles surface reconfiguration and uniform buffer updates
        self.window_renderer.resize(bounds, scale_factor);
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
                println!("[DISPATCH] Window {:?} received MouseDown at ({:.1}, {:.1})",
                         self.id, position.x, position.y);

                // Check if mouse is captured
                let target = if let Some(captured_id) = self.mouse_capture.captured_id() {
                    println!("[HIT TEST] Mouse captured by widget {:?}", captured_id);
                    Some(captured_id)
                } else {
                    // Hit test using z-order: find topmost element at this position
                    let hit = self.hit_tester.hit_test(position);
                    println!("[HIT TEST] Hit test at ({:.1}, {:.1}) -> {:?}",
                             position.x, position.y, hit);
                    hit
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

                                // Capture mouse for this widget (for drag operations)
                                self.mouse_capture.capture(widget_id);
                                println!("[Window {:?}] Mouse captured by {:?}", self.id, widget_id);

                                // Check if this is a DraggableRect and start cross-window drag
                                use crate::elements::DraggableRect;
                                if let Some(draggable) = element.as_any().downcast_ref::<DraggableRect>() {
                                    if draggable.is_dragging() {
                                        // Convert window coordinates to screen coordinates
                                        let screen_pos = self.platform_window.window_to_screen(position);

                                        // Get widget bounds and properties
                                        let bounds = element.bounds();

                                        println!("[Window {:?}] Starting cross-window drag for {:?}", self.id, widget_id);

                                        // Emit StartCrossWindowDrag event
                                        self.event_queue.lock().unwrap().push_back((
                                            self.id,
                                            GuiEvent::StartCrossWindowDrag {
                                                widget_id,
                                                color: draggable.color(),
                                                label: draggable.label().to_string(),
                                                size: bounds.size,
                                                drag_offset: draggable.drag_offset(),
                                                screen_position: screen_pos,
                                            },
                                        ));
                                    }
                                }
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
                    // Check if this was a dragging DraggableRect before dispatching
                    let was_dragging = if let Some(element) = self.element_manager.get(widget_id) {
                        use crate::elements::DraggableRect;
                        element.as_any().downcast_ref::<DraggableRect>()
                            .map(|d| d.is_dragging())
                            .unwrap_or(false)
                    } else {
                        false
                    };

                    if let Some(element) = self.element_manager.get_mut(widget_id) {
                        let response = element.dispatch_mouse_event(&mut event);

                        match response {
                            EventResponse::Handled => {
                                println!("[Window {:?}] Element {:?} handled mouse up", self.id, widget_id);

                                // If this was a dragging DraggableRect, emit EndCrossWindowDrag
                                if was_dragging {
                                    // Convert window coordinates to screen coordinates
                                    let screen_pos = self.platform_window.window_to_screen(position);

                                    println!("[Window {:?}] Ending cross-window drag for {:?}", self.id, widget_id);

                                    // Emit EndCrossWindowDrag event
                                    self.event_queue.lock().unwrap().push_back((
                                        self.id,
                                        GuiEvent::EndCrossWindowDrag {
                                            screen_position: screen_pos,
                                        },
                                    ));
                                }

                                // Release mouse capture
                                if self.mouse_capture.is_captured_by(widget_id) {
                                    self.mouse_capture.release();
                                    println!("[Window {:?}] Mouse capture released", self.id);
                                }
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

                        // If this is a dragging DraggableRect, emit UpdateCrossWindowDrag
                        use crate::elements::DraggableRect;
                        if let Some(draggable) = element.as_any().downcast_ref::<DraggableRect>() {
                            if draggable.is_dragging() {
                                // Convert window coordinates to screen coordinates
                                let screen_pos = self.platform_window.window_to_screen(position);

                                // Emit UpdateCrossWindowDrag event
                                self.event_queue.lock().unwrap().push_back((
                                    self.id,
                                    GuiEvent::UpdateCrossWindowDrag {
                                        screen_position: screen_pos,
                                    },
                                ));
                            }
                        }
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

            InputEventEnum::Custom(_) => {
                // Custom events are not yet handled
                // TODO: Implement custom event handling
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
        let surface_texture = match self.window_renderer.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Failed to get surface texture: {:?}", e);
                return;
            }
        };

        // Create texture view with sRGB format
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(self.window_renderer.format.add_srgb_suffix()),
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

        // 2. Build hit test cache from element bounds and z-order
        // This happens AFTER layout (bounds are known) but BEFORE paint
        // Separates spatial/event concerns from rendering
        self.hit_tester.clear();
        if let Some(root) = self.scene_graph.root() {
            let mut z_order = 0u32;
            root.traverse(&mut |widget_id| {
                if let Some(element) = self.element_manager.get(widget_id) {
                    // Only register interactive elements for hit testing
                    if element.is_interactive() {
                        self.hit_tester.add(widget_id, element.bounds(), z_order);
                        z_order += 1;
                    }
                }
            });
        }
        self.hit_tester.finalize();

        // 3. Paint elements in tree order (collect draw commands)
        let scale_factor = self.platform_window.scale_factor() as f32;

        // Begin new frame for window renderer (atlas + text engine)
        self.window_renderer.begin_frame();

        // Paint phase - collect draw commands
        let (rect_instances, sdf_commands, text_instances) = {
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

            // Extract instances before paint_ctx is dropped
            let mut rect_instances = paint_ctx.rect_instances().to_vec();
            let sdf_commands = paint_ctx.sdf_commands().to_vec();
            let mut text_instances = paint_ctx.text_instances().to_vec();

            // Sort by z-order (low to high) for correct overlapping
            // Elements with lower z-order are drawn first (appear behind)
            // Elements with higher z-order are drawn last (appear on top)
            rect_instances.sort_by_key(|inst| inst.z_order);
            text_instances.sort_by_key(|inst| inst.z_order);

            (rect_instances, sdf_commands, text_instances)
        };

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

            // Render all rectangles using shared pipeline
            self.window_renderer.render_rects(&mut render_pass, &rect_instances);

            // Render all SDF rectangles (rounded corners with borders)
            if !sdf_commands.is_empty() {
                // Create a temporary batcher from the collected commands
                let mut sdf_batcher = crate::paint::PrimitiveBatcher::new();
                for cmd in &sdf_commands {
                    if let crate::paint::DrawCommand::Rect { rect, style } = cmd {
                        sdf_batcher.draw_rect(*rect, style.clone());
                    }
                }
                self.window_renderer.render_sdf_rects(&mut render_pass, &sdf_batcher);
            }

            // Render all text using shared pipeline and atlas
            let atlas_lock = render_context.glyph_atlas.lock().unwrap();
            let atlas_texture_view = atlas_lock.texture_view();
            self.window_renderer.render_text(&mut render_pass, &text_instances, atlas_texture_view);
            drop(atlas_lock);
        }

        // Submit commands
        render_context.queue.submit([encoder.finish()]);

        // Present the frame
        surface_texture.present();
    }
}
