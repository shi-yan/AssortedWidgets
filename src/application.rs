use crate::event::{GuiEvent, InputEventEnum};
use crate::handle::GuiHandle;
use crate::paint::Color;
use crate::render::{RenderContext, WindowRenderer};
use crate::types::{Point, Rect, Size, WidgetId, WindowId};
use crate::window::Window;

#[cfg(target_os = "macos")]
use crate::platform::{PlatformWindow, PlatformWindowImpl, WindowCallbacks, WindowOptions};

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

// ============================================================================
// Drag State (for cross-window drag-drop)
// ============================================================================

/// Data transferred during drag operation
#[derive(Debug, Clone)]
pub struct DragData {
    /// Widget being dragged
    pub widget_id: WidgetId,

    /// Visual appearance (for proxy window)
    pub color: Color,
    pub label: String,
    pub size: Size,

    /// Offset from mouse position to widget origin
    pub drag_offset: Point,
}

/// Global drag state (application-wide)
#[derive(Debug, Clone)]
pub struct DragState {
    /// Source window where drag started
    pub source_window: WindowId,

    /// Dragged widget data
    pub drag_data: DragData,

    /// Floating proxy window (shows dragged element)
    pub proxy_window: Option<WindowId>,

    /// Current screen position (global coordinates)
    pub screen_position: Point,
}

// ============================================================================
// Application - Root container (ONE per process)
// ============================================================================

/// Main application that manages windows and the event loop
///
/// # Architecture
///
/// **Single Application per Process:**
/// - Owns the main event loop (`run() -> !`)
/// - Manages all windows (multi-window support)
/// - Owns shared rendering context (GPU + pipelines + atlas + fonts)
///
/// **Per-Window State:**
/// - Each window has its own UI tree (ElementManager, SceneGraph)
/// - Each window has its own rendering surface and uniforms (WindowRenderer)
/// - Windows share the RenderContext (GPU, pipelines, glyph atlas, fonts)
///
/// # Platform Notes
///
/// On macOS: There is ONE NSApplication per process, managed by this struct.
/// On Linux/Windows: Similar platform-specific singleton event loop.
pub struct Application {
    // ========================================
    // Windows (Multi-window support)
    // ========================================
    /// All windows indexed by WindowId
    windows: HashMap<WindowId, Window>,

    /// Next window ID to allocate
    next_window_id: u64,

    // ========================================
    // Shared Resources (All windows)
    // ========================================
    /// Shared rendering context (GPU + atlas + fonts + text engine)
    /// Contains both low-level GPU resources and high-level rendering state
    /// Created once, shared across all windows via Arc
    render_context: Arc<RenderContext>,

    /// App-level widget ID counter (shared across all windows)
    /// Ensures globally unique widget IDs in multi-window applications
    _next_widget_id: Arc<AtomicU64>,

    /// Thread-safe handle for GUI operations (cloned and passed to windows)
    /// Provides app-level widget ID generation and cross-thread messaging
    gui_handle: GuiHandle,

    // ========================================
    // Event Queue (Application-wide)
    // ========================================
    /// Event queue for all windows
    /// Events are tagged with WindowId to route to correct window
    event_queue: Arc<Mutex<VecDeque<(WindowId, GuiEvent)>>>,

    // ========================================
    // Cross-Window Drag State
    // ========================================
    /// Active drag operation (cross-window)
    /// None when no drag is in progress
    drag_state: Option<DragState>,
}

impl Application {
    /// Create a new application with rendering support
    ///
    /// This initializes the WebGPU rendering context and shared resources
    /// (GPU, atlas, fonts, text engine).
    ///
    /// Use `pollster::block_on(Application::new())` for simple blocking initialization.
    pub async fn new() -> Result<Self, String> {
        // Initialize platform (NSApplication on macOS)
        #[cfg(target_os = "macos")]
        crate::platform::init();

        let render_context = RenderContext::new().await?;
        let render_context_arc = Arc::new(render_context);

        // Create app-level widget ID counter (starts at 1)
        let next_widget_id = Arc::new(AtomicU64::new(1));

        // Create message channel for cross-thread communication
        let (message_tx, _message_rx) = mpsc::channel();

        // Create GuiHandle with app-level ID generation
        let gui_handle = GuiHandle::new(message_tx, Arc::clone(&next_widget_id));

        Ok(Application {
            windows: HashMap::new(),
            next_window_id: 1,
            render_context: render_context_arc,
            _next_widget_id: next_widget_id,
            gui_handle,
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            drag_state: None,
        })
    }

    /// Create a window
    #[cfg(target_os = "macos")]
    pub fn create_window(&mut self, options: WindowOptions) -> Result<WindowId, String> {
        self.create_window_with_layout(options, None)
    }

    /// Create a window with custom root layout (Qt-style API)
    pub fn create_window_with_layout(
        &mut self,
        options: WindowOptions,
        root_layout: Option<crate::layout::Style>,
    ) -> Result<WindowId, String> {
        // Allocate window ID
        let window_id = WindowId::new(self.next_window_id);
        self.next_window_id += 1;

        let mut platform_window = PlatformWindowImpl::new(options);

        // Get window content bounds (excludes titlebar) and scale factor
        let content_bounds = platform_window.content_bounds();
        let scale_factor = platform_window.scale_factor();
        let window_size = content_bounds.size;

        println!("Creating window {:?} with logical size {:.0}x{:.0}, scale factor: {:.1}x",
                 window_id, window_size.width, window_size.height, scale_factor);

        // Create window renderer (surface + uniforms + instance buffers)
        // This accepts Arc<RenderContext> which contains shared pipelines
        let window_renderer = WindowRenderer::new(Arc::clone(&self.render_context), &platform_window)?;

        // Clone event queue Arc for callbacks to use
        let event_queue_input_event = self.event_queue.clone();
        let event_queue_frame = self.event_queue.clone();
        let event_queue_resize = self.event_queue.clone();
        let event_queue_close = self.event_queue.clone();
        let event_queue_scale_factor = self.event_queue.clone();

        // Set up callbacks to push events to queue (tagged with window_id)
        let callbacks = WindowCallbacks {
            input_event: Some(Box::new(move |input_event| {
                event_queue_input_event.lock().unwrap().push_back((window_id, GuiEvent::InputEvent(input_event)));
            })),
            request_frame: Some(Box::new(move || {
                event_queue_frame.lock().unwrap().push_back((window_id, GuiEvent::RedrawRequested));
            })),
            resize: Some(Box::new(move |bounds| {
                event_queue_resize.lock().unwrap().push_back((window_id, GuiEvent::Resize(bounds)));
            })),
            moved: Some(Box::new(|_position| {
                // Window moved (not important for demo)
            })),
            close: Some(Box::new(move || {
                event_queue_close.lock().unwrap().push_back((window_id, GuiEvent::Close));
            })),
            active_status_change: Some(Box::new(|_active| {
                // Window activation status changed (not important for demo)
            })),
            scale_factor_changed: Some(Box::new(move |scale_factor| {
                event_queue_scale_factor.lock().unwrap().push_back((window_id, GuiEvent::ScaleFactorChanged(scale_factor)));
            })),
        };

        platform_window.set_callbacks(callbacks);

        // Create window (uses logical size for layout calculations)
        // Pass cloned GuiHandle for app-level widget ID generation
        // Pass optional root_layout for Qt-style implicit root container
        let window = Window::new(
            window_id,
            platform_window,
            window_renderer,
            window_size,
            Arc::clone(&self.event_queue),
            self.gui_handle.clone(),
            root_layout,
        );

        // Store window
        self.windows.insert(window_id, window);

        Ok(window_id)
    }

    /// Get reference to a window by ID
    pub fn window(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    /// Get mutable reference to a window by ID
    pub fn window_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    /// Get handle for interacting with UI elements
    ///
    /// Returns the app-level GuiHandle which provides:
    /// - App-level widget ID generation (globally unique across all windows)
    /// - Cross-thread messaging to GUI thread
    pub fn get_handle(&self) -> GuiHandle {
        self.gui_handle.clone()
    }

    /// Get reference to shared render context
    pub fn render_context(&self) -> &Arc<RenderContext> {
        &self.render_context
    }

    // ========================================
    // Cross-Window Drag-Drop API
    // ========================================

    /// Start a cross-window drag operation
    ///
    /// Creates a floating proxy window to visualize the dragged element.
    ///
    /// # Arguments
    /// * `source_window` - Window where drag started
    /// * `drag_data` - Data about the widget being dragged
    /// * `screen_position` - Initial mouse position in screen coordinates
    #[cfg(target_os = "macos")]
    pub fn start_drag(
        &mut self,
        source_window: WindowId,
        drag_data: DragData,
        screen_position: Point,
    ) -> Result<(), String> {
        println!("[Drag] Starting drag from window {:?} at screen ({:.1}, {:.1})",
                 source_window, screen_position.x, screen_position.y);

        // Create borderless, transparent, always-on-top proxy window
        let proxy_bounds = Rect::new(
            Point::new(
                screen_position.x - drag_data.drag_offset.x,
                screen_position.y - drag_data.drag_offset.y,
            ),
            drag_data.size,
        );

        let proxy_options = WindowOptions {
            bounds: proxy_bounds,
            title: "Drag Proxy".to_string(),
            titlebar: None,
            borderless: true,
            transparent: true,
            always_on_top: true,
            utility: true,
        };

        // Create proxy window
        let proxy_id = self.create_window(proxy_options)?;

        println!("═══════════════════════════════════════════════════════");
        println!("[PROXY CREATE] Lifted rect into new floating window");
        println!("  Proxy Window ID: {:?}", proxy_id);
        println!("  Size: {}x{}", drag_data.size.width, drag_data.size.height);
        println!("  Position: ({:.1}, {:.1})", proxy_bounds.origin.x, proxy_bounds.origin.y);
        println!("  Label: {}", drag_data.label);
        println!("  Drag Offset: ({:.1}, {:.1})", drag_data.drag_offset.x, drag_data.drag_offset.y);
        println!("═══════════════════════════════════════════════════════");

        // Add draggable rect to proxy window to visualize the dragged element
        {
            use crate::elements::DraggableRect;

            let proxy_window = self.windows.get_mut(&proxy_id).ok_or("Proxy window not found")?;

            // Create a visual representation of the dragged element
            // Position it at (0, 0) since the proxy window itself is positioned at the drag location
            let proxy_rect = DraggableRect::new(
                WidgetId::new(9999), // Special ID for proxy
                Rect::new(Point::new(0.0, 0.0), drag_data.size),
                drag_data.color,
                &drag_data.label,
            );

            // Use clean Window API to add as root widget
            proxy_window.add_root(Box::new(proxy_rect), taffy::Style::default())
                .map_err(|e| format!("Failed to add proxy widget: {}", e))?;
        }

        // Store drag state
        self.drag_state = Some(DragState {
            source_window,
            drag_data,
            proxy_window: Some(proxy_id),
            screen_position,
        });

        Ok(())
    }

    /// Update drag position (moves the proxy window)
    ///
    /// # Arguments
    /// * `screen_position` - New mouse position in screen coordinates
    pub fn update_drag(&mut self, screen_position: Point) {
        if let Some(drag_state) = &mut self.drag_state {
            drag_state.screen_position = screen_position;

            // Move proxy window to follow mouse
            if let Some(proxy_id) = drag_state.proxy_window {
                if let Some(proxy_window) = self.windows.get_mut(&proxy_id) {
                    let new_origin = Point::new(
                        screen_position.x - drag_state.drag_data.drag_offset.x,
                        screen_position.y - drag_state.drag_data.drag_offset.y,
                    );

                    println!("[DRAG MOVE] Mouse: ({:.1}, {:.1}) → Window Position: ({:.1}, {:.1})",
                             screen_position.x, screen_position.y,
                             new_origin.x, new_origin.y);

                    // Update proxy window position
                    #[cfg(target_os = "macos")]
                    proxy_window.platform_window_mut().set_position(new_origin);
                }
            }
        }
    }

    /// End drag operation
    ///
    /// Detects the target window under the cursor and transfers the widget if applicable.
    /// Closes the proxy window and clears drag state.
    ///
    /// # Arguments
    /// * `screen_position` - Final mouse position in screen coordinates
    ///
    /// # Returns
    /// * `Some(WindowId)` if dropped on a valid target window (not the source)
    /// * `None` if dropped outside or on the source window
    pub fn end_drag(&mut self, screen_position: Point) -> Option<WindowId> {
        let drag_state = self.drag_state.take()?;

        println!("═══════════════════════════════════════════════════════");
        println!("[MOUSE RELEASE] Drag ended at screen ({:.1}, {:.1})",
                 screen_position.x, screen_position.y);

        // Close proxy window
        if let Some(proxy_id) = drag_state.proxy_window {
            println!("  [c] Deleting proxy window {:?}", proxy_id);
            self.windows.remove(&proxy_id);
        }

        // Find target window under cursor
        println!("  [a] Checking if mouse is over an app window...");
        let target_window = match self.get_window_at_screen_position(screen_position) {
            Some(win) => {
                println!("  [a] ✓ Found target window: {:?}", win);
                win
            }
            None => {
                println!("  [a] ✗ No window at cursor position - drag cancelled");
                println!("═══════════════════════════════════════════════════════");
                return None;
            }
        };

        // Don't allow dropping on source window
        if target_window == drag_state.source_window {
            println!("  [a] ✗ Target is source window - drag cancelled");
            println!("═══════════════════════════════════════════════════════");
            return None;
        }

        // Calculate offset on target window and transfer widget
        let target_origin = self.windows.get(&target_window)
            .map(|w| w.platform_window().window_screen_origin())?;

        // Convert screen position to target window coordinates
        let target_window_pos = self.windows.get(&target_window)
            .map(|w| w.platform_window().screen_to_window(screen_position))?;

        println!("  [b] Target window origin: ({:.1}, {:.1})", target_origin.x, target_origin.y);
        println!("  [b] Mouse position on target window: ({:.1}, {:.1})",
                 target_window_pos.x, target_window_pos.y);

        // Calculate the position for the new rect (accounting for drag offset)
        let rect_x = target_window_pos.x - drag_state.drag_data.drag_offset.x;
        let rect_y = target_window_pos.y - drag_state.drag_data.drag_offset.y;
        println!("  [d] Creating new rect at target window position: ({:.1}, {:.1})", rect_x, rect_y);

        // Transfer widget: Remove from source, add to target
        {
            use crate::elements::DraggableRect;

            // 1. Remove widget from source window
            if let Some(source_window) = self.windows.get_mut(&drag_state.source_window) {
                println!("  [d] Removing widget {:?} from source window {:?}",
                         drag_state.drag_data.widget_id, drag_state.source_window);

                // Use clean Window API to remove the widget
                source_window.remove(drag_state.drag_data.widget_id).ok();
            }

            // 2. Create new widget at calculated position on target window
            if let Some(target_window_mut) = self.windows.get_mut(&target_window) {
                println!("  [d] Adding widget to target window {:?} at ({:.1}, {:.1})",
                         target_window, rect_x, rect_y);

                // Create new rect at the drop position
                let new_rect = DraggableRect::new(
                    drag_state.drag_data.widget_id,
                    Rect::new(Point::new(rect_x, rect_y), drag_state.drag_data.size),
                    drag_state.drag_data.color,
                    &drag_state.drag_data.label,
                );

                // Create layout style with explicit position and size
                let style = taffy::Style {
                    margin: taffy::Rect {
                        left: taffy::LengthPercentageAuto::length(rect_x as f32),
                        top: taffy::LengthPercentageAuto::length(rect_y as f32),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    size: taffy::Size {
                        width: taffy::Dimension::length(drag_state.drag_data.size.width as f32),
                        height: taffy::Dimension::length(drag_state.drag_data.size.height as f32),
                    },
                    ..Default::default()
                };

                // Use clean Window API to add as root widget
                target_window_mut.add_root(Box::new(new_rect), style)
                    .map_err(|e| format!("Failed to add widget to target: {}", e))
                    .ok();

                println!("  [d] ✓ Widget transferred successfully!");
            }
        }

        println!("═══════════════════════════════════════════════════════");
        Some(target_window)
    }

    /// Find which window is at the given screen position
    ///
    /// # Arguments
    /// * `screen_pos` - Position in screen coordinates
    ///
    /// # Returns
    /// * `Some(WindowId)` if a window is at that position
    /// * `None` if no window found (or only proxy window)
    pub fn get_window_at_screen_position(&self, screen_pos: Point) -> Option<WindowId> {
        // Check each window's bounds in screen coordinates
        for (window_id, window) in &self.windows {
            // Skip proxy windows
            if let Some(drag_state) = &self.drag_state {
                if Some(*window_id) == drag_state.proxy_window {
                    continue;
                }
            }

            let origin = window.platform_window().window_screen_origin();
            let bounds = window.platform_window().content_bounds();

            // Convert to screen-space rect
            let screen_bounds = Rect::new(
                Point::new(origin.x, origin.y),
                bounds.size,
            );

            // Check if point is inside window
            if screen_pos.x >= screen_bounds.origin.x
                && screen_pos.x <= screen_bounds.origin.x + screen_bounds.size.width
                && screen_pos.y >= screen_bounds.origin.y
                && screen_pos.y <= screen_bounds.origin.y + screen_bounds.size.height
            {
                return Some(*window_id);
            }
        }

        None
    }

    /// Check if a drag operation is in progress
    pub fn is_dragging(&self) -> bool {
        self.drag_state.is_some()
    }

    /// Get the current drag state (if any)
    pub fn drag_state(&self) -> Option<&DragState> {
        self.drag_state.as_ref()
    }

    // ========================================
    // High-Level API (Hides async/pollster details)
    // ========================================

    /// Launch the application with a setup callback
    ///
    /// This is the ergonomic entry point that hides async initialization details.
    /// The callback receives a mutable reference to the Application for setup.
    ///
    /// # Example
    /// ```ignore
    /// Application::launch(|app| {
    ///     app.spawn_window("My App", 800.0, 600.0, |window| {
    ///         window.set_main_widget(MyWidget::new());
    ///     });
    /// });
    /// ```
    #[cfg(target_os = "macos")]
    pub fn launch<F>(setup: F) -> !
    where
        F: FnOnce(&mut Application),
    {
        // Hide the async initialization
        let mut app = pollster::block_on(async { Self::new().await })
            .expect("Failed to initialize application");

        // Call user setup code
        setup(&mut app);

        // Start the event loop (never returns)
        app.run()
    }

    /// Spawn a new window with a setup callback
    ///
    /// This is the ergonomic API for creating windows without dealing with WindowId.
    /// The callback receives a mutable reference to the newly created Window.
    ///
    /// # Arguments
    /// * `title` - Window title
    /// * `width` - Window width in logical pixels
    /// * `height` - Window height in logical pixels
    /// * `setup` - Callback to configure the window (receives &mut Window)
    ///
    /// # Returns
    /// WindowId if you need to reference the window later (optional)
    ///
    /// # Example
    /// ```ignore
    /// app.spawn_window("Main Window", 800.0, 600.0, |window| {
    ///     window.set_main_widget(MyWidget::new());
    /// });
    /// ```
    #[cfg(target_os = "macos")]
    /// Spawn a window with default root layout (Qt-style API)
    ///
    /// The window has an implicit root container with a column flex layout.
    /// Use `window.add_to_root()` inside the setup closure to add widgets.
    ///
    /// # Example
    /// ```ignore
    /// app.spawn_window("My App", 800.0, 600.0, |window| {
    ///     window.add_to_root(Box::new(Label::new("Hello")), Style::default());
    /// });
    /// ```
    pub fn spawn_window<F>(
        &mut self,
        title: &str,
        width: f64,
        height: f64,
        setup: F,
    ) -> WindowId
    where
        F: FnOnce(&mut Window),
    {
        self.spawn_window_with_layout(title, width, height, None, setup)
    }

    /// Spawn a window with custom root layout (Qt-style API - Option C)
    ///
    /// The window has an implicit root container. If `root_layout` is provided,
    /// it configures the root container's layout. Otherwise, a default column layout is used.
    /// You can also call `window.set_root_layout()` inside the setup closure.
    ///
    /// # Example
    /// ```ignore
    /// let root_style = Style {
    ///     flex_direction: FlexDirection::Row,
    ///     gap: taffy::Size::length(10.0),
    ///     ..Default::default()
    /// };
    ///
    /// app.spawn_window_with_layout("My App", 800.0, 600.0, Some(root_style), |window| {
    ///     window.add_to_root(Box::new(Label::new("Left")), Style::default());
    ///     window.add_to_root(Box::new(Label::new("Right")), Style::default());
    /// });
    /// ```
    pub fn spawn_window_with_layout<F>(
        &mut self,
        title: &str,
        width: f64,
        height: f64,
        root_layout: Option<crate::layout::Style>,
        setup: F,
    ) -> WindowId
    where
        F: FnOnce(&mut Window),
    {
        let options = WindowOptions {
            title: title.to_string(),
            bounds: Rect::new(Point::new(100.0, 100.0), Size::new(width, height)),
            titlebar: None,
            borderless: false,
            transparent: false,
            always_on_top: false,
            utility: false,
        };

        let window_id = self.create_window_with_layout(options, root_layout)
            .expect("Failed to create window");

        // Immediately get the window and call setup
        if let Some(window) = self.window_mut(window_id) {
            setup(window);
        }

        window_id
    }

    /// Run the main event loop (never returns)
    ///
    /// This manually pumps the platform event loop and processes events from the queue.
    /// On macOS, this pumps NSApplication events.
    ///
    /// This function never returns - it runs until the application exits.
    #[cfg(target_os = "macos")]
    pub fn run(&mut self) -> ! {
        use objc2::rc::autoreleasepool;
        use objc2_app_kit::{NSApplication, NSEventMask};
        use objc2_foundation::{MainThreadMarker, NSDate, NSDefaultRunLoopMode, NSRunLoop};

        println!("Starting application event loop...");

        // Trigger initial frame for all windows
        for window in self.windows.values_mut() {
            window.platform_window_mut().invalidate();
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
                    Some((_window_id, GuiEvent::RedrawRequested)) => {
                        // Will render at end of loop iteration
                    }
                    Some((window_id, GuiEvent::Resize(bounds))) => {
                        if let Some(window) = self.windows.get_mut(&window_id) {
                            window.resize(bounds, &self.render_context);

                            // Request redraw after resize
                            window.platform_window_mut().invalidate();
                        }
                    }
                    Some((window_id, GuiEvent::ScaleFactorChanged(scale_factor))) => {
                        if let Some(window) = self.windows.get_mut(&window_id) {
                            println!("🔄 Window {:?} scale factor changed to {:.1}x", window_id, scale_factor);

                            // Update renderer with new scale factor (reconfigures surface)
                            let bounds = window.platform_window().content_bounds();
                            window.window_renderer_mut().resize(bounds, scale_factor);

                            // Request redraw to re-render with new DPI
                            window.platform_window_mut().invalidate();
                        }
                    }
                    Some((window_id, GuiEvent::InputEvent(input_event))) => {
                        // Check if there's an active drag and this is a mouse up event
                        if self.drag_state.is_some() {
                            if let InputEventEnum::MouseUp(mouse_event) = &input_event {
                                // Mouse up during active drag - end the drag!
                                println!("[App] Mouse up detected during active drag on window {:?}", window_id);

                                // Convert window coordinates to screen coordinates
                                if let Some(window) = self.windows.get(&window_id) {
                                    let screen_pos = window.platform_window().window_to_screen(mouse_event.position);

                                    println!("[App] Triggering end drag at screen ({:.1}, {:.1})",
                                             screen_pos.x, screen_pos.y);

                                    // End the drag
                                    if let Some(target_window) = self.end_drag(screen_pos) {
                                        println!("[App] Drag completed - dropped on window {:?}", target_window);
                                    } else {
                                        println!("[App] Drag cancelled");
                                    }
                                }

                                // Don't dispatch this event to the window - we've handled it
                                continue;
                            }
                        }

                        // Dispatch event to window's element manager
                        if let Some(window) = self.windows.get_mut(&window_id) {
                            window.dispatch_input_event(input_event);
                        }
                    }
                    Some((window_id, GuiEvent::Close)) => {
                        println!("Window {:?} closing", window_id);
                        self.windows.remove(&window_id);

                        // Exit if no windows left
                        if self.windows.is_empty() {
                            println!("All windows closed - goodbye!");
                            std::process::exit(0);
                        }
                    }
                    Some((window_id, GuiEvent::StartCrossWindowDrag {
                        widget_id,
                        color,
                        label,
                        size,
                        drag_offset,
                        screen_position,
                    })) => {
                        println!("[App] Starting cross-window drag for widget {:?} from window {:?}",
                                 widget_id, window_id);

                        let drag_data = crate::application::DragData {
                            widget_id,
                            color,
                            label,
                            size,
                            drag_offset,
                        };

                        if let Err(e) = self.start_drag(window_id, drag_data, screen_position) {
                            eprintln!("[App] Failed to start drag: {}", e);
                        }
                    }
                    Some((_window_id, GuiEvent::UpdateCrossWindowDrag { screen_position })) => {
                        self.update_drag(screen_position);
                    }
                    Some((_window_id, GuiEvent::EndCrossWindowDrag { screen_position })) => {
                        if let Some(target_window) = self.end_drag(screen_position) {
                            println!("[App] Drag completed - dropped on window {:?}", target_window);
                            // TODO: Transfer widget to target window
                        } else {
                            println!("[App] Drag cancelled - no valid drop target");
                        }
                    }
                    None => break, // No more events to process
                }
            }

            // Process element manager messages for all windows (signal/slot system)
            for window in self.windows.values_mut() {
                window.process_messages();
            }

            // Check if any windows have animated elements
            for window in self.windows.values_mut() {
                let has_animations = window.widget_ids()
                    .filter_map(|id| window.get(id))
                    .any(|element| element.is_dirty());

                if has_animations {
                    window.mark_layout_dirty();
                }
            }

            // Render all windows
            for window in self.windows.values_mut() {
                window.render_frame(&self.render_context);
            }

            // Request next frame for continuous animation (all windows)
            for window in self.windows.values_mut() {
                window.platform_window_mut().invalidate();
            }
        }
    }
}
