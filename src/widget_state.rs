use crate::types::{WidgetId, Rect, DirtyLevel};

/// Common widget state for composition pattern
///
/// All widgets should contain a `state: WidgetState` field and delegate
/// id/bounds/dirty management to it via `impl_widget_essentials!()` macro.
///
/// # Example
///
/// ```
/// use assorted_widgets::{Widget, WidgetState, PaintContext};
///
/// pub struct MyWidget {
///     state: WidgetState,  // Common state (id, bounds, dirty)
///     // ... widget-specific fields
/// }
///
/// impl Widget for MyWidget {
///     impl_widget_essentials!();  // Delegates to self.state
///
///     fn paint(&self, ctx: &mut PaintContext) {
///         // ... implementation
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct WidgetState {
    /// Unique widget identifier
    pub id: WidgetId,

    /// Widget bounds (position and size)
    pub bounds: Rect,

    /// Dirty tracking level (Visual or Layout)
    pub dirty: DirtyLevel,
}

impl WidgetState {
    /// Create new widget state (starts with Layout dirty for initial render)
    pub fn new() -> Self {
        Self {
            id: WidgetId::default(),
            bounds: Rect::zero(),
            dirty: DirtyLevel::Layout,
        }
    }

    /// Create with specific ID
    pub fn with_id(id: WidgetId) -> Self {
        Self {
            id,
            bounds: Rect::zero(),
            dirty: DirtyLevel::Layout,
        }
    }

    /// Mark as dirty (merges with existing dirty level)
    ///
    /// Uses max(current, new) to ensure we don't downgrade dirty level.
    /// For example, if already marked Layout, setting Visual won't downgrade it.
    pub fn mark_dirty(&mut self, level: DirtyLevel) {
        self.dirty = self.dirty.merge(level);
    }

    /// Clear dirty flag (after render)
    pub fn clear_dirty(&mut self) {
        self.dirty = DirtyLevel::Clean;
    }
}

impl Default for WidgetState {
    fn default() -> Self {
        Self::new()
    }
}
