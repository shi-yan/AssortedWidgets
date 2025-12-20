use crate::element_manager::ElementManager;
use crate::event::GuiEvent;
use crate::handle::GuiHandle;
use crate::render::{RenderContext, WindowRenderer};
use crate::scene_graph::SceneGraph;

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
    render_context: Arc<RenderContext>,
    event_queue: Arc<Mutex<VecDeque<GuiEvent>>>,
    render_fn: Option<Box<dyn FnMut(&WindowRenderer, &RenderContext)>>,
    #[cfg(target_os = "macos")]
    window: Option<PlatformWindowImpl>,
    #[cfg(target_os = "macos")]
    renderer: Option<WindowRenderer>,
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
        use cocoa::appkit::NSApp;
        use cocoa::base::{id, nil};
        use cocoa::foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop};
        use objc::runtime::Class;
        use objc::{class, msg_send, sel, sel_impl};

        println!("Starting manual event loop...");

        // Trigger initial frame
        if let Some(window) = self.window.as_mut() {
            window.invalidate();
        }

        loop {
            unsafe {
                // Process NSApplication events first
                let app = NSApp();
                let until_date: id = msg_send![class!(NSDate), distantPast];

                loop {
                    let event: id = msg_send![app,
                        nextEventMatchingMask:0xffffffffu64
                        untilDate:until_date
                        inMode:NSDefaultRunLoopMode
                        dequeue:1u8
                    ];

                    if event == nil {
                        break;
                    }

                    let _: () = msg_send![app, sendEvent: event];
                }

                // Now pump the runloop briefly to handle timers/sources
                let run_loop: id = msg_send![class!(NSRunLoop), currentRunLoop];
                let date: id = msg_send![class!(NSDate), dateWithTimeIntervalSinceNow: 0.001f64];
                let _: () = msg_send![run_loop, runMode:NSDefaultRunLoopMode beforeDate:date];
            }

            // Process all queued events posted by platform callbacks
            loop {
                let event = self.event_queue.lock().unwrap().pop_front();
                match event {
                    Some(GuiEvent::RedrawRequested) => {
                        // Will render at end of loop iteration
                    }
                    Some(GuiEvent::Resize(bounds)) => {
                        println!("Window resized to {:.0}x{:.0}", bounds.size.width, bounds.size.height);
                        if let Some(renderer) = self.renderer.as_mut() {
                            renderer.resize(&self.render_context, bounds);
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

            // Render frame if we have a renderer and render function
            // This happens every loop iteration (immediate mode rendering)
            if let (Some(renderer), Some(ref mut render_fn)) =
                (self.renderer.as_ref(), self.render_fn.as_mut()) {
                render_fn(renderer, &self.render_context);
            }
        }
    }

    pub fn get_handle(&self) -> GuiHandle {
        self.element_manager.get_handle()
    }

    pub fn render_context(&self) -> &RenderContext {
        &self.render_context
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
}
