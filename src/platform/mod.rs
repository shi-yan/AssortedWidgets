//! Platform abstraction layer for cross-platform windowing
//!
//! This module defines traits that abstract over platform-specific windowing implementations.
//!
//! ## Architecture
//!
//! - **macOS**: Uses NSApplication event loop (callback/push model)
//! - **Linux**: Uses calloop with Wayland/X11 (polling model)
//! - **Windows**: Uses message pump (polling model)
//!
//! Despite different event loop paradigms, all platforms implement the same traits.

use crate::types::{rect, Point, Rect, Vector};

// Re-export input types for convenience
pub use crate::event::input::{Modifiers, MouseButton};

#[cfg(target_os = "macos")]
pub mod mac;

// Platform-specific window type alias
#[cfg(target_os = "macos")]
pub type PlatformWindowImpl = mac::MacWindow;

// Re-export platform initialization
#[cfg(target_os = "macos")]
pub use mac::init;

// ============================================================================
// Platform Window Trait
// ============================================================================

/// Callbacks invoked by the platform window
pub struct WindowCallbacks {
    /// Called when the window receives an input event (mouse, keyboard, etc.) - LEGACY
    /// This is deprecated in favor of input_event callback
    pub input: Option<Box<dyn FnMut(PlatformInput) + Send>>,

    /// Called when the window receives an input event (new event system)
    pub input_event: Option<Box<dyn FnMut(crate::event::InputEventEnum) + Send>>,

    /// Called when the window needs to be redrawn
    pub request_frame: Option<Box<dyn FnMut() + Send>>,

    /// Called when the window is resized
    pub resize: Option<Box<dyn FnMut(Rect) + Send>>,

    /// Called when the window is moved
    pub moved: Option<Box<dyn FnMut(Point) + Send>>,

    /// Called when the window is about to close
    pub close: Option<Box<dyn FnMut() + Send>>,

    /// Called when the window becomes active/inactive
    pub active_status_change: Option<Box<dyn FnMut(bool) + Send>>,
}

impl std::fmt::Debug for WindowCallbacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowCallbacks")
            .field("input", &self.input.as_ref().map(|_| "<callback>"))
            .field("input_event", &self.input_event.as_ref().map(|_| "<callback>"))
            .field("request_frame", &self.request_frame.as_ref().map(|_| "<callback>"))
            .field("resize", &self.resize.as_ref().map(|_| "<callback>"))
            .field("moved", &self.moved.as_ref().map(|_| "<callback>"))
            .field("close", &self.close.as_ref().map(|_| "<callback>"))
            .field("active_status_change", &self.active_status_change.as_ref().map(|_| "<callback>"))
            .finish()
    }
}

impl Default for WindowCallbacks {
    fn default() -> Self {
        Self {
            input: None,
            input_event: None,
            request_frame: None,
            resize: None,
            moved: None,
            close: None,
            active_status_change: None,
        }
    }
}

/// Platform input event
#[derive(Debug, Clone)]
pub enum PlatformInput {
    MouseDown {
        position: Point,
        button: MouseButton,
        modifiers: Modifiers,
    },
    MouseUp {
        position: Point,
        button: MouseButton,
        modifiers: Modifiers,
    },
    MouseMove {
        position: Point,
        modifiers: Modifiers,
    },
    MouseWheel {
        delta: Vector,
        modifiers: Modifiers,
    },
    KeyDown {
        key: String,
        modifiers: Modifiers,
    },
    KeyUp {
        key: String,
        modifiers: Modifiers,
    },
}

/// Window options for creation
#[derive(Debug, Clone)]
pub struct WindowOptions {
    pub bounds: Rect,
    pub title: String,
    pub titlebar: Option<TitlebarOptions>,

    /// Create a borderless window (no title bar, no resize controls)
    pub borderless: bool,

    /// Create a transparent window (for floating drag proxies)
    pub transparent: bool,

    /// Keep window always on top of other windows
    pub always_on_top: bool,

    /// Window is a utility window (doesn't appear in Dock/taskbar)
    pub utility: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            bounds: rect(100.0, 100.0, 800.0, 600.0),
            title: "AssortedWidgets Window".to_string(),
            titlebar: Some(TitlebarOptions::default()),
            borderless: false,
            transparent: false,
            always_on_top: false,
            utility: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TitlebarOptions {
    pub appears_transparent: bool,
    pub traffic_light_position: Option<Point>,
}

impl Default for TitlebarOptions {
    fn default() -> Self {
        Self {
            appears_transparent: false,
            traffic_light_position: None,
        }
    }
}

/// Cross-platform window trait
/// Note: Windows are NOT Send - they must stay on the main thread
pub trait PlatformWindow {
    /// Get window bounds in screen coordinates
    fn bounds(&self) -> Rect;

    /// Get content bounds (excludes title bar)
    fn content_bounds(&self) -> Rect;

    /// Get the window's scale factor (for HiDPI displays)
    fn scale_factor(&self) -> f64;

    /// Set window title
    fn set_title(&mut self, title: &str);

    /// Show or hide the window
    fn set_visible(&mut self, visible: bool);

    /// Minimize the window
    fn minimize(&mut self);

    /// Maximize/zoom the window
    fn zoom(&mut self);

    /// Bring window to front and activate
    fn activate(&mut self);

    /// Close the window
    fn close(&mut self);

    /// Request the window to be redrawn
    fn invalidate(&mut self);

    /// Set callbacks for window events
    fn set_callbacks(&mut self, callbacks: WindowCallbacks);

    /// Set the IME cursor area (for positioning the IME composition window)
    ///
    /// # Arguments
    /// * `x` - X coordinate in screen pixels
    /// * `y` - Y coordinate in screen pixels
    /// * `width` - Width of the cursor area in screen pixels
    /// * `height` - Height of the cursor area in screen pixels
    fn set_ime_cursor_area(&mut self, x: f64, y: f64, width: f64, height: f64);

    // ========================================
    // Screen Coordinate Methods (for cross-window drag-drop)
    // ========================================

    /// Get the window's origin in screen coordinates
    /// Returns the bottom-left corner on macOS (standard screen coords)
    fn window_screen_origin(&self) -> Point;

    /// Convert window-local coordinates to screen coordinates
    fn window_to_screen(&self, window_pos: Point) -> Point {
        let origin = self.window_screen_origin();
        Point::new(origin.x + window_pos.x, origin.y + window_pos.y)
    }

    /// Convert screen coordinates to window-local coordinates
    fn screen_to_window(&self, screen_pos: Point) -> Point {
        let origin = self.window_screen_origin();
        Point::new(screen_pos.x - origin.x, screen_pos.y - origin.y)
    }
}

/// Platform event loop trait
pub trait PlatformEventLoop {
    /// Run the event loop
    ///
    /// **macOS**: This calls NSApp.run() and never returns
    /// **Linux/Windows**: This runs a loop that polls for events
    fn run(&mut self) -> !;

    /// Request the event loop to terminate
    fn quit(&mut self);
}
