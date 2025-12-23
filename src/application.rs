use crate::event::GuiEvent;
use crate::handle::GuiHandle;
use crate::render::{RenderContext, WindowRenderer};
use crate::types::{Size, WindowId};
use crate::window::Window;
use crate::window_render_state::WindowRenderState;

#[cfg(target_os = "macos")]
use crate::platform::{PlatformInput, PlatformWindow, PlatformWindowImpl, WindowCallbacks, WindowOptions};

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

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
/// - Owns shared rendering context (GPU + atlas + fonts)
///
/// **Per-Window State:**
/// - Each window has its own UI tree (ElementManager, SceneGraph)
/// - Each window has its own rendering surface (WindowRenderState)
/// - Windows share the RenderContext (GPU resources + glyph atlas + fonts)
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

    // ========================================
    // Event Queue (Application-wide)
    // ========================================
    /// Event queue for all windows
    /// Events are tagged with WindowId to route to correct window
    event_queue: Arc<Mutex<VecDeque<(WindowId, GuiEvent)>>>,
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

        Ok(Application {
            windows: HashMap::new(),
            next_window_id: 1,
            render_context: render_context_arc,
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
        })
    }

    /// Create a window
    #[cfg(target_os = "macos")]
    pub fn create_window(&mut self, options: WindowOptions) -> Result<WindowId, String> {
        use crate::paint::RectRenderer;
        use crate::text::TextRenderer;

        // Allocate window ID
        let window_id = WindowId::new(self.next_window_id);
        self.next_window_id += 1;

        let mut platform_window = PlatformWindowImpl::new(options);

        // Create window renderer (surface + format management)
        let renderer = WindowRenderer::new(&self.render_context, &platform_window)?;

        // Get window bounds and scale factor
        let bounds = platform_window.bounds();
        let scale_factor = platform_window.scale_factor();
        let window_size = bounds.size;

        println!("Creating window {:?} with size {:.0}x{:.0}, scale factor: {}",
                 window_id, window_size.width, window_size.height, scale_factor);

        // Calculate physical pixel size for Retina displays
        let physical_size = Size::new(
            bounds.size.width * scale_factor,
            bounds.size.height * scale_factor
        );

        // Create rectangle renderer (stateless - just pipeline/shaders)
        let mut rect_renderer = RectRenderer::new(
            &self.render_context,
            renderer.format,
        );
        rect_renderer.update_screen_size(&self.render_context, physical_size);

        // Create text renderer (stateless - just pipeline/shaders)
        let mut text_renderer = TextRenderer::new(
            &self.render_context,
            renderer.format,
        );
        text_renderer.update_screen_size(&self.render_context, physical_size);

        // Bundle per-window resources
        let render_state = WindowRenderState::new(
            renderer,
            rect_renderer,
            text_renderer,
            scale_factor as f32,
            Arc::clone(&self.render_context),
        );

        // Clone event queue Arc for callbacks to use
        let event_queue_input = self.event_queue.clone();
        let event_queue_input_event = self.event_queue.clone();
        let event_queue_frame = self.event_queue.clone();
        let event_queue_resize = self.event_queue.clone();
        let event_queue_close = self.event_queue.clone();

        // Set up callbacks to push events to queue (tagged with window_id)
        let callbacks = WindowCallbacks {
            input: Some(Box::new(move |input| {
                event_queue_input.lock().unwrap().push_back((window_id, GuiEvent::Input(input)));
            })),
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
        };

        platform_window.set_callbacks(callbacks);

        // Create window
        let window = Window::new(window_id, platform_window, render_state, window_size);

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
    /// For now, this returns the handle from the first window.
    /// In a multi-window app, you'd specify which window's handle you want.
    pub fn get_handle(&self) -> Option<GuiHandle> {
        self.windows.values().next().map(|w| w.element_manager().get_handle())
    }

    /// Get reference to shared render context
    pub fn render_context(&self) -> &Arc<RenderContext> {
        &self.render_context
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
                    Some((window_id, GuiEvent::Input(input))) => {
                        // Handle input events (LEGACY)
                        match input {
                            PlatformInput::MouseDown { position, button, .. } => {
                                println!("Window {:?}: Mouse {:?} clicked at ({:.1}, {:.1})",
                                         window_id, button, position.x, position.y);
                            }
                            PlatformInput::KeyDown { key, .. } => {
                                println!("Window {:?}: Key pressed: {}", window_id, key);
                            }
                            _ => {}
                        }
                        // TODO: Convert to OsEvent and dispatch to ElementManager
                    }
                    Some((window_id, GuiEvent::InputEvent(input_event))) => {
                        // Handle new event system
                        use crate::event::InputEventEnum;
                        match input_event {
                            InputEventEnum::MouseDown(evt) => {
                                println!("[NEW] Window {:?}: Mouse {:?} down at ({:.1}, {:.1}), click_count={}",
                                         window_id, evt.button, evt.position.x, evt.position.y, evt.click_count);
                            }
                            InputEventEnum::MouseUp(evt) => {
                                println!("[NEW] Window {:?}: Mouse {:?} up at ({:.1}, {:.1})",
                                         window_id, evt.button, evt.position.x, evt.position.y);
                            }
                            InputEventEnum::MouseMove(evt) => {
                                // Too noisy to log every move
                                let _ = evt;
                            }
                            InputEventEnum::KeyDown(evt) => {
                                println!("[NEW] Window {:?}: Key down: {:?}, repeat={}",
                                         window_id, evt.key, evt.is_repeat);
                            }
                            InputEventEnum::KeyUp(evt) => {
                                println!("[NEW] Window {:?}: Key up: {:?}",
                                         window_id, evt.key);
                            }
                            InputEventEnum::Wheel(evt) => {
                                println!("[NEW] Window {:?}: Wheel delta=({:.1}, {:.1}), phase={:?}",
                                         window_id, evt.delta.dx, evt.delta.dy, evt.phase);
                            }
                        }
                        // TODO: Dispatch to ElementManager when hit testing is implemented
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
                    None => break, // No more events to process
                }
            }

            // Process element manager messages for all windows (signal/slot system)
            for window in self.windows.values_mut() {
                window.element_manager_mut().process_messages();
            }

            // Check if any windows have animated elements
            for window in self.windows.values_mut() {
                let has_animations = window.element_manager().widget_ids()
                    .filter_map(|id| window.element_manager().get(id))
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
