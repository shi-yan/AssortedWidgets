use crate::element_manager::ElementManager;
use crate::handle::GuiHandle;
use crate::render::{RenderContext, WindowRenderer};
use crate::scene_graph::SceneGraph;

#[cfg(target_os = "macos")]
use crate::platform::{PlatformInput, PlatformWindow, PlatformWindowImpl, WindowCallbacks, WindowOptions};

use std::sync::Arc;

// ============================================================================
// Main Event Loop
// ============================================================================

pub struct GuiEventLoop {
    element_manager: ElementManager,
    scene_graph: SceneGraph,
    render_context: Arc<RenderContext>,
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

        // Set up callbacks
        let callbacks = WindowCallbacks {
            input: Some(Box::new(move |input| {
                // Handle input events
                match input {
                    PlatformInput::MouseDown { position, button, .. } => {
                        println!("Mouse {:?} clicked at ({:.1}, {:.1})", button, position.x, position.y);
                    }
                    PlatformInput::MouseUp { position, button, .. } => {
                        println!("Mouse {:?} released at ({:.1}, {:.1})", button, position.x, position.y);
                    }
                    PlatformInput::MouseMove { position, .. } => {
                        // Uncomment to see mouse move events (very verbose)
                        // println!("Mouse moved to ({:.1}, {:.1})", position.x, position.y);
                    }
                    PlatformInput::MouseWheel { delta, .. } => {
                        println!("Mouse wheel scrolled: dx={:.1}, dy={:.1}", delta.x, delta.y);
                    }
                    PlatformInput::KeyDown { key, .. } => {
                        println!("Key pressed: {}", key);
                    }
                    PlatformInput::KeyUp { key, .. } => {
                        println!("Key released: {}", key);
                    }
                }
                // TODO: Convert to OsEvent and dispatch to ElementManager
            })),
            request_frame: Some(Box::new(|| {
                // TODO: Render frame callback will be set from main.rs
            })),
            resize: Some(Box::new(|bounds| {
                println!("Window resized to {:.0}x{:.0}", bounds.size.width, bounds.size.height);
                // TODO: Resize surface
            })),
            moved: Some(Box::new(|_position| {
                // Window moved (not important for demo)
            })),
            close: Some(Box::new(|| {
                println!("Window closing - goodbye!");
                // TODO: Cleanup and quit
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

    /// Run the macOS event loop
    /// This function never returns on macOS
    #[cfg(target_os = "macos")]
    pub fn run(&mut self) -> ! {
        unsafe {
            use cocoa::appkit::NSApplication;
            use cocoa::base::nil;

            let app = NSApplication::sharedApplication(nil);
            app.run();

            // This line is never reached
            std::process::exit(0);
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
