//! Hit testing for spatial event routing (mouse, touch, stylus)
//!
//! The HitTester maintains a cache of interactive element bounds with z-order
//! from the rendering system. This ensures that hit testing respects the same
//! visual layering as rendering: if element A is drawn on top of element B,
//! then clicking the overlapping area should hit element A, not B.

use crate::types::{Point, Rect, WidgetId};

/// Hit test entry for a single interactive element
#[derive(Debug, Clone, Copy)]
pub struct HitTestEntry {
    /// Widget ID
    pub widget_id: WidgetId,

    /// Bounding rectangle
    pub bounds: Rect,

    /// Z-order from rendering (higher = on top)
    pub z_order: u32,
}

/// Hit tester for spatial event routing (mouse, touch, stylus)
///
/// The HitTester caches interactive element bounds with z-order values
/// from the paint pass. This ensures hit testing matches visual layering.
///
/// # Design
///
/// - **Z-Order**: Uses the same z-order as rendering for consistency
/// - **Interactive Elements**: Only caches elements marked as interactive
/// - **Top-to-Bottom**: Returns the topmost (highest z-order) element at a point
/// - **Rebuild**: Must be rebuilt after paint pass when z-orders are assigned
///
/// # Performance
///
/// - Linear scan through interactive elements (O(n))
/// - Fast enough for < 1000 interactive elements (< 0.1ms)
/// - For larger UIs, consider spatial hash (Phase 5 optimization)
#[derive(Clone)]
pub struct HitTester {
    /// Cache of interactive element bounds with z-order
    /// Sorted by z_order (lowest to highest)
    entries: Vec<HitTestEntry>,
}

impl HitTester {
    /// Create a new empty hit tester
    pub fn new() -> Self {
        HitTester {
            entries: Vec::new(),
        }
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Add an interactive element to the hit test cache
    ///
    /// This should be called during the paint pass for each interactive element,
    /// with the z-order value assigned by PaintContext.
    pub fn add(&mut self, widget_id: WidgetId, bounds: Rect, z_order: u32) {
        self.entries.push(HitTestEntry {
            widget_id,
            bounds,
            z_order,
        });
    }

    /// Finalize the hit test cache by sorting entries by z-order
    ///
    /// This should be called after all elements have been added.
    /// The entries are sorted so we can iterate from highest to lowest z-order.
    pub fn finalize(&mut self) {
        // Sort by z-order (lowest to highest)
        // We'll iterate in reverse to find the topmost element
        self.entries.sort_by_key(|entry| entry.z_order);
    }

    /// Find the topmost widget at the given position
    ///
    /// Returns the widget ID of the topmost (highest z-order) interactive element
    /// that contains the given point, or None if no element is hit.
    ///
    /// # Algorithm
    ///
    /// Iterates through entries in reverse order (highest to lowest z-order).
    /// The first element that contains the point is the topmost and is returned.
    pub fn hit_test(&self, position: Point) -> Option<WidgetId> {
        // Iterate from highest to lowest z-order (reverse = top to bottom)
        self.entries
            .iter()
            .rev()
            .find(|entry| entry.bounds.contains(position))
            .map(|entry| entry.widget_id)
    }

    /// Get all widgets at the given position, from top to bottom
    ///
    /// Returns a list of widget IDs, ordered from highest to lowest z-order.
    /// This is useful for event bubbling: we can dispatch to the topmost element,
    /// and if it doesn't handle the event, bubble to the next element, etc.
    pub fn hit_test_all(&self, position: Point) -> Vec<WidgetId> {
        // Collect all hits from highest to lowest z-order
        self.entries
            .iter()
            .rev()
            .filter(|entry| entry.bounds.contains(position))
            .map(|entry| entry.widget_id)
            .collect()
    }

    /// Get the number of cached entries
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Check if the hit tester is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get all entries (for debugging)
    pub fn entries(&self) -> &[HitTestEntry] {
        &self.entries
    }
}

impl Default for HitTester {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Size;

    #[test]
    fn test_hit_test_single_element() {
        let mut hit_tester = HitTester::new();

        let widget_id = WidgetId::from(1);
        let bounds = Rect::new(Point::new(10.0, 10.0), Size::new(100.0, 100.0));

        hit_tester.add(widget_id, bounds, 0);
        hit_tester.finalize();

        // Inside bounds
        assert_eq!(hit_tester.hit_test(Point::new(50.0, 50.0)), Some(widget_id));

        // Outside bounds
        assert_eq!(hit_tester.hit_test(Point::new(5.0, 5.0)), None);
        assert_eq!(hit_tester.hit_test(Point::new(200.0, 200.0)), None);
    }

    #[test]
    fn test_hit_test_z_order() {
        let mut hit_tester = HitTester::new();

        let widget_a = WidgetId::from(1);
        let widget_b = WidgetId::from(2);

        // Widget A: z-order = 0, bounds = (0, 0, 100, 100)
        hit_tester.add(widget_a, Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0)), 0);

        // Widget B: z-order = 1, bounds = (50, 50, 100, 100) - overlaps A
        hit_tester.add(widget_b, Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0)), 1);

        hit_tester.finalize();

        // Point in A only
        assert_eq!(hit_tester.hit_test(Point::new(25.0, 25.0)), Some(widget_a));

        // Point in both A and B - should return B (higher z-order)
        assert_eq!(hit_tester.hit_test(Point::new(75.0, 75.0)), Some(widget_b));

        // Point in B only
        assert_eq!(hit_tester.hit_test(Point::new(125.0, 125.0)), Some(widget_b));
    }

    #[test]
    fn test_hit_test_all() {
        let mut hit_tester = HitTester::new();

        let widget_a = WidgetId::from(1);
        let widget_b = WidgetId::from(2);
        let widget_c = WidgetId::from(3);

        // Three overlapping widgets at (50, 50, 100, 100) with different z-orders
        hit_tester.add(widget_a, Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0)), 0);
        hit_tester.add(widget_b, Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0)), 1);
        hit_tester.add(widget_c, Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0)), 2);

        hit_tester.finalize();

        // Point in all three - should return all in order (highest to lowest z-order)
        let hits = hit_tester.hit_test_all(Point::new(75.0, 75.0));
        assert_eq!(hits, vec![widget_c, widget_b, widget_a]);
    }
}
