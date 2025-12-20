use std::any::Any;

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for each widget/element
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn new(id: u64) -> Self {
        WidgetId(id)
    }
}

/// Generic message type for signal/slot communication
/// Uses manual serialization - can be JSON, binary, or Any
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
// Geometry Types (using euclid)
// ============================================================================

/// Unit type for screen coordinates
///
/// We use euclid's unit system to provide type safety for different
/// coordinate spaces. This can be extended to support:
/// - `ScreenPixels` - Physical screen pixels
/// - `LogicalPixels` - Logical pixels (DPI-independent)
/// - `LayoutPixels` - Layout coordinates
///
/// For now, we use a single unit type for simplicity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScreenPixels;

/// 2D point in screen coordinates
///
/// Uses euclid::Point2D with SIMD acceleration.
/// Supports HiDPI scaling and coordinate space transformations.
pub type Point = euclid::Point2D<f64, ScreenPixels>;

/// 2D rectangle in screen coordinates
///
/// Uses euclid::Rect with SIMD acceleration.
/// Provides methods like `contains()`, `intersects()`, etc.
pub type Rect = euclid::Rect<f64, ScreenPixels>;

/// 2D size in screen coordinates
pub type Size = euclid::Size2D<f64, ScreenPixels>;

/// 2D vector in screen coordinates
pub type Vector = euclid::Vector2D<f64, ScreenPixels>;

// ============================================================================
// Convenience constructors
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
