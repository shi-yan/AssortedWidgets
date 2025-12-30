//! Layered BoundsTree for per-layer z-ordering
//!
//! This module wraps multiple BoundsTree instances (one per z-index layer) to provide
//! final depth assignment for rendering. Each layer maintains its own tree for overlap
//! detection, enabling batching of non-overlapping elements within the same layer.

use crate::bounds_tree::BoundsTree;
use crate::paint::z_order::z_index_to_depth;
use crate::types::Rect;
use std::collections::HashMap;

/// Manages separate BoundsTree instances per z-index layer
///
/// This enables batching optimization: non-overlapping elements within the same layer
/// get identical depth values and can be rendered in a single draw call.
///
/// # Example
///
/// ```
/// use assorted_widgets::paint::LayeredBoundsTree;
/// use assorted_widgets::types::{Rect, Point, Size};
///
/// let mut tree = LayeredBoundsTree::new();
///
/// // Two non-overlapping rects at z=0 → same depth (batched!)
/// let rect1 = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));
/// let rect2 = Rect::new(Point::new(200.0, 0.0), Size::new(100.0, 100.0));
/// let depth1 = tree.insert(rect1, 0);
/// let depth2 = tree.insert(rect2, 0);
/// assert_eq!(depth1, depth2); // ✅ Same depth → batched!
///
/// // Overlapping rect → different depth
/// let rect3 = Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0));
/// let depth3 = tree.insert(rect3, 0);
/// assert!(depth3 > depth1); // Higher depth (further back visually)
/// ```
#[derive(Debug)]
pub struct LayeredBoundsTree {
    /// One BoundsTree per z-index layer
    layers: HashMap<i32, BoundsTree>,
}

impl LayeredBoundsTree {
    /// Create a new empty layered bounds tree
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
        }
    }

    /// Clear all layers (call at start of each frame)
    pub fn clear(&mut self) {
        for tree in self.layers.values_mut() {
            tree.clear();
        }
    }

    /// Insert bounds into the tree and return the assigned GPU depth value
    ///
    /// # Algorithm
    ///
    /// 1. Get or create BoundsTree for this z-index layer
    /// 2. Insert bounds → get overlap ordering (1, 2, 3, ...)
    /// 3. Convert overlap ordering to fine_z value (0.0-1.0 within layer)
    /// 4. Map z_index + fine_z → final GPU depth via z_index_to_depth
    ///
    /// # Returns
    ///
    /// GPU depth value in [0.0, 1.0] where 0.0 is near (on top), 1.0 is far (behind)
    ///
    /// # Example
    ///
    /// ```text
    /// z_index=0, overlap_order=1 → fine_z=0.00001 → depth≈0.30001
    /// z_index=0, overlap_order=1 → fine_z=0.00001 → depth≈0.30001 (SAME!)
    /// z_index=0, overlap_order=2 → fine_z=0.00002 → depth≈0.30002
    /// ```
    pub fn insert(&mut self, bounds: Rect, z_index: i32) -> f32 {
        // Get or create BoundsTree for this layer
        let tree = self.layers.entry(z_index).or_insert_with(BoundsTree::new);

        // Insert into layer-specific tree → get overlap ordering
        let overlap_order = tree.insert(bounds);

        // Convert overlap ordering to fine_z (0.0-1.0 within layer)
        // Using small increments to fit many elements within each layer's depth range
        // Example: order=1 → 0.00001, order=2 → 0.00002, ..., order=10000 → 0.1
        let fine_z = (overlap_order as f32) / 100000.0;

        // Map z_index + fine_z → final GPU depth
        z_index_to_depth(z_index, fine_z)
    }
}

impl Default for LayeredBoundsTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Point, Size};

    #[test]
    fn test_batching_within_layer() {
        let mut tree = LayeredBoundsTree::new();

        // Three non-overlapping rects at z=0 should get same depth (batched!)
        let rect1 = Rect::new(Point::new(0.0, 0.0), Size::new(50.0, 50.0));
        let rect2 = Rect::new(Point::new(100.0, 0.0), Size::new(50.0, 50.0));
        let rect3 = Rect::new(Point::new(200.0, 0.0), Size::new(50.0, 50.0));

        let depth1 = tree.insert(rect1, 0);
        let depth2 = tree.insert(rect2, 0);
        let depth3 = tree.insert(rect3, 0);

        assert_eq!(depth1, depth2, "Non-overlapping rects should batch");
        assert_eq!(depth2, depth3, "Non-overlapping rects should batch");
    }

    #[test]
    fn test_overlapping_different_depth() {
        let mut tree = LayeredBoundsTree::new();

        let rect1 = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));
        let rect2 = Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0));

        let depth1 = tree.insert(rect1, 0);
        let depth2 = tree.insert(rect2, 0);

        assert!(
            depth2 > depth1,
            "Overlapping rect should have higher depth (further back)"
        );
    }

    #[test]
    fn test_different_layers() {
        let mut tree = LayeredBoundsTree::new();

        // Same bounds, different z-index layers → different depths
        let bounds = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));

        let depth_normal = tree.insert(bounds, 0); // NORMAL layer
        let depth_shadow = tree.insert(bounds, -100); // SHADOW layer

        // Shadow layer (z=-100) should be further back (higher depth)
        assert!(
            depth_shadow > depth_normal,
            "Shadow layer should be behind normal layer"
        );
    }

    #[test]
    fn test_clear() {
        let mut tree = LayeredBoundsTree::new();

        let rect = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));
        let depth1 = tree.insert(rect, 0);

        tree.clear();

        // After clear, inserting same rect should give same depth (order=1 again)
        let depth2 = tree.insert(rect, 0);
        assert_eq!(depth1, depth2, "Clear should reset ordering");
    }
}
