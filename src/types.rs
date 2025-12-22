use std::any::Any;
use std::time::Instant;

// ============================================================================
// Core Framework Types
// ============================================================================

/// Frame timing information for animations
///
/// Passed to Element::update() to enable frame-rate independent animations.
#[derive(Debug, Clone, Copy)]
pub struct FrameInfo {
    /// Delta time since last frame (in seconds)
    ///
    /// Use this for frame-rate independent animations:
    /// ```ignore
    /// position += velocity * frame.dt;
    /// ```
    pub dt: f64,

    /// Absolute timestamp of this frame
    ///
    /// Use this to calculate elapsed time:
    /// ```ignore
    /// let elapsed = (frame.timestamp - self.start_time).as_secs_f64();
    /// ```
    pub timestamp: Instant,

    /// Frame number (starts at 0, increments each frame)
    ///
    /// Useful for debugging and frame-based logic.
    pub frame_number: u64,
}

impl FrameInfo {
    /// Create a new FrameInfo
    pub fn new(dt: f64, timestamp: Instant, frame_number: u64) -> Self {
        Self { dt, timestamp, frame_number }
    }
}

/// Unique identifier for each widget/element
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn new(id: u64) -> Self {
        WidgetId(id)
    }
}

/// Unique identifier for each window
///
/// ## Design Decision: Simple u64 ID vs raw-window-handle
///
/// We use a simple `u64` counter instead of platform-specific window handles for several reasons:
///
/// 1. **Cross-platform uniformity**: raw-window-handle types vary per platform (NSWindow*, HWND, xcb_window_t)
/// 2. **HashMap compatibility**: u64 is trivially hashable and comparable
/// 3. **Stable identity**: Window handle might change on some platforms, but our ID is stable
/// 4. **Decoupling**: Separates our logical window ID from platform implementation details
///
/// The platform-specific window handle is stored in `PlatformWindowImpl` and accessed via the
/// `raw-window-handle` trait when needed for surface creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(u64);

impl WindowId {
    pub fn new(id: u64) -> Self {
        WindowId(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Generic message type for signal/slot communication
#[derive(Debug)]
pub enum GuiMessage {
    /// Widget was clicked
    Clicked(WidgetId),
    /// Value changed (e.g., slider, input field)
    ValueChanged(WidgetId, f64),
    /// Text input changed
    TextChanged(WidgetId, String),
    /// Custom signal with serialized data
    Custom {
        source: WidgetId,
        signal_type: String,
        data: Box<dyn Any + Send>,
    },
    /// Parent requests child modification
    ParentToChild {
        parent: WidgetId,
        child: WidgetId,
        command: Box<dyn Any + Send>,
    },
    /// Child requests parent modification
    ChildToParent {
        child: WidgetId,
        parent: WidgetId,
        command: Box<dyn Any + Send>,
    },
}

impl GuiMessage {
    pub(crate) fn clone_for_target(&self, _target: WidgetId) -> GuiMessage {
        match self {
            GuiMessage::Clicked(id) => GuiMessage::Clicked(*id),
            GuiMessage::ValueChanged(id, val) => GuiMessage::ValueChanged(*id, *val),
            GuiMessage::TextChanged(id, text) => GuiMessage::TextChanged(*id, text.clone()),
            // For Custom messages, we can't clone the Box<dyn Any>
            // In practice, you'd need to implement a custom cloning strategy
            // or use Arc for shared data
            _ => panic!("Cannot clone this message type"),
        }
    }
}

/// Represents a deferred command that will be processed later
#[derive(Debug)]
pub struct DeferredCommand {
    pub target: WidgetId,
    pub message: GuiMessage,
}

// ============================================================================
// Geometry Type Re-exports (using euclid for SIMD benefits)
// ============================================================================

/// Unit type for screen coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScreenPixels;

/// 2D point in screen coordinates (uses euclid for SIMD acceleration)
pub type Point = euclid::Point2D<f64, ScreenPixels>;

/// 2D rectangle in screen coordinates (uses euclid for SIMD acceleration)
pub type Rect = euclid::Rect<f64, ScreenPixels>;

/// 2D size in screen coordinates (uses euclid for SIMD acceleration)
pub type Size = euclid::Size2D<f64, ScreenPixels>;

/// 2D vector in screen coordinates (uses euclid for SIMD acceleration)
pub type Vector = euclid::Vector2D<f64, ScreenPixels>;

// ============================================================================
// Convenience Constructors
// ============================================================================

/// Create a point from x, y coordinates
pub fn point(x: f64, y: f64) -> Point {
    Point::new(x, y)
}

/// Create a rect from x, y, width, height
pub fn rect(x: f64, y: f64, width: f64, height: f64) -> Rect {
    Rect::new(Point::new(x, y), Size::new(width, height))
}

/// Create a size from width, height
pub fn size(width: f64, height: f64) -> Size {
    Size::new(width, height)
}

/// Create a vector from x, y
pub fn vector(x: f64, y: f64) -> Vector {
    Vector::new(x, y)
}
