//! Spatial bounds tree for efficient z-ordering and batching
//!
//! This is an R-tree variant optimized for finding maximum ordering among intersecting bounds.
//! Ported from gpui (zed/crates/gpui/src/bounds_tree.rs) and adapted for AssortedWidgets.
//!
//! ## Key Features
//!
//! - **Overlap Detection**: Efficiently finds all bounds that intersect with a query
//! - **Automatic Z-Ordering**: Assigns minimum z-values for correct layering
//! - **Batching Optimization**: Non-overlapping elements get same z-value → batch together!
//! - **Fast Path**: O(1) lookup for global max z-order (common case)
//!
//! ## Algorithm
//!
//! ```text
//! Insert Rect1 (0,0,100,100)   → intersections: []       → z=1
//! Insert Rect2 (200,0,100,100) → intersections: []       → z=1 ✅ SAME (batched!)
//! Insert Rect3 (50,50,100,100) → intersections: [Rect1] → z=2
//! ```

use crate::types::{Point, Rect, Size};

/// Maximum children per internal node (R-tree branching factor)
///
/// Higher values = shorter tree = fewer cache misses, but more work per node.
/// Value of 12 is empirically optimal from gpui benchmarks.
const MAX_CHILDREN: usize = 12;

/// A spatial tree optimized for finding maximum ordering among intersecting bounds
///
/// This is an R-tree variant specifically designed for assigning z-order values
/// to overlapping UI elements with minimal allocations.
#[derive(Debug)]
pub struct BoundsTree {
    /// All nodes stored contiguously for cache efficiency
    nodes: Vec<Node>,

    /// Index of the root node, if any
    root: Option<usize>,

    /// Index of the leaf with the highest ordering (for fast-path lookups)
    max_leaf: Option<usize>,

    /// Reusable stack for tree traversal during insertion
    insert_path: Vec<usize>,

    /// Reusable stack for search operations
    search_stack: Vec<usize>,
}

/// A node in the bounds tree
#[derive(Debug, Clone)]
struct Node {
    /// Bounding box containing this node and all descendants
    bounds: Rect,

    /// Maximum ordering value in this subtree
    max_order: u32,

    /// Node-specific data
    kind: NodeKind,
}

#[derive(Debug, Clone)]
enum NodeKind {
    /// Leaf node containing actual bounds data
    Leaf {
        /// The ordering assigned to this bounds
        order: u32,
    },

    /// Internal node with children
    Internal {
        /// Indices of child nodes (2 to MAX_CHILDREN)
        children: NodeChildren,
    },
}

/// Fixed-size array for child indices, avoiding heap allocation
#[derive(Debug, Clone)]
struct NodeChildren {
    /// Invariant: max order child is always at the end
    indices: [usize; MAX_CHILDREN],
    len: u8,
}

impl NodeChildren {
    fn new() -> Self {
        Self {
            indices: [0; MAX_CHILDREN],
            len: 0,
        }
    }

    fn push(&mut self, index: usize) {
        debug_assert!((self.len as usize) < MAX_CHILDREN);
        self.indices[self.len as usize] = index;
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn as_slice(&self) -> &[usize] {
        &self.indices[..self.len as usize]
    }
}

impl BoundsTree {
    /// Create a new empty bounds tree
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all nodes from the tree
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
        self.max_leaf = None;
        self.insert_path.clear();
        self.search_stack.clear();
    }

    /// Inserts bounds into the tree and returns its assigned ordering
    ///
    /// The ordering is one greater than the maximum ordering of any
    /// existing bounds that intersect with the new bounds.
    ///
    /// # Returns
    ///
    /// The assigned z-order value (1-based, where 1 is the lowest)
    ///
    /// # Example
    ///
    /// ```
    /// use assorted_widgets::bounds_tree::BoundsTree;
    /// use assorted_widgets::types::{Rect, Point, Size};
    ///
    /// let mut tree = BoundsTree::new();
    ///
    /// // First element: no overlaps
    /// let rect1 = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));
    /// assert_eq!(tree.insert(rect1), 1);
    ///
    /// // Second element: doesn't overlap, gets same z-order (batching!)
    /// let rect2 = Rect::new(Point::new(200.0, 0.0), Size::new(100.0, 100.0));
    /// assert_eq!(tree.insert(rect2), 1);
    ///
    /// // Third element: overlaps rect1, gets higher z-order
    /// let rect3 = Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0));
    /// assert_eq!(tree.insert(rect3), 2);
    /// ```
    pub fn insert(&mut self, new_bounds: Rect) -> u32 {
        // Find maximum ordering among intersecting bounds
        let max_intersecting = self.find_max_ordering(&new_bounds);
        let ordering = max_intersecting + 1;

        // Insert the new leaf
        let new_leaf_idx = self.insert_leaf(new_bounds, ordering);

        // Update max_leaf tracking
        self.max_leaf = match self.max_leaf {
            None => Some(new_leaf_idx),
            Some(old_idx) if self.nodes[old_idx].max_order < ordering => Some(new_leaf_idx),
            some => some,
        };

        ordering
    }

    /// Finds the maximum ordering among all bounds that intersect with the query
    fn find_max_ordering(&mut self, query: &Rect) -> u32 {
        let Some(root_idx) = self.root else {
            return 0;
        };

        // Fast path: check if the max-ordering leaf intersects
        if let Some(max_idx) = self.max_leaf {
            let max_node = &self.nodes[max_idx];
            if query.intersects(&max_node.bounds) {
                return max_node.max_order;
            }
        }

        // Slow path: search the tree
        self.search_stack.clear();
        self.search_stack.push(root_idx);

        let mut max_found = 0u32;

        while let Some(node_idx) = self.search_stack.pop() {
            let node = &self.nodes[node_idx];

            // Pruning: skip if this subtree can't improve our result
            if node.max_order <= max_found {
                continue;
            }

            // Spatial pruning: skip if bounds don't intersect
            if !query.intersects(&node.bounds) {
                continue;
            }

            match &node.kind {
                NodeKind::Leaf { order } => {
                    max_found = max_found.max(*order);
                }
                NodeKind::Internal { children } => {
                    // Children are maintained with highest max_order at the end.
                    // Push in forward order so highest (last) is popped first.
                    for &child_idx in children.as_slice() {
                        if self.nodes[child_idx].max_order > max_found {
                            self.search_stack.push(child_idx);
                        }
                    }
                }
            }
        }

        max_found
    }

    /// Inserts a leaf node with the given bounds and ordering.
    /// Returns the index of the new leaf.
    fn insert_leaf(&mut self, bounds: Rect, order: u32) -> usize {
        let new_leaf_idx = self.nodes.len();
        self.nodes.push(Node {
            bounds,
            max_order: order,
            kind: NodeKind::Leaf { order },
        });

        let Some(root_idx) = self.root else {
            // Tree is empty, new leaf becomes root
            self.root = Some(new_leaf_idx);
            return new_leaf_idx;
        };

        // If root is a leaf, create internal node with both
        if matches!(self.nodes[root_idx].kind, NodeKind::Leaf { .. }) {
            let root_bounds = self.nodes[root_idx].bounds;
            let root_order = self.nodes[root_idx].max_order;

            let mut children = NodeChildren::new();
            // Max end invariant
            if order > root_order {
                children.push(root_idx);
                children.push(new_leaf_idx);
            } else {
                children.push(new_leaf_idx);
                children.push(root_idx);
            }

            let new_root_idx = self.nodes.len();
            self.nodes.push(Node {
                bounds: root_bounds.union(&bounds),
                max_order: root_order.max(order),
                kind: NodeKind::Internal { children },
            });
            self.root = Some(new_root_idx);
            return new_leaf_idx;
        }

        // Descend to find the best internal node to insert into
        self.insert_path.clear();
        let mut current_idx = root_idx;

        loop {
            let current = &self.nodes[current_idx];
            let NodeKind::Internal { children } = &current.kind else {
                unreachable!("Should only traverse internal nodes");
            };

            self.insert_path.push(current_idx);

            // Find the best child to descend into
            let mut best_child_idx = children.as_slice()[0];
            let mut best_child_pos = 0;
            let mut best_cost = bounds.union(&self.nodes[best_child_idx].bounds).half_perimeter();

            for (pos, &child_idx) in children.as_slice().iter().enumerate().skip(1) {
                let cost = bounds.union(&self.nodes[child_idx].bounds).half_perimeter();
                if cost < best_cost {
                    best_cost = cost;
                    best_child_idx = child_idx;
                    best_child_pos = pos;
                }
            }

            // Check if best child is a leaf or internal
            if matches!(self.nodes[best_child_idx].kind, NodeKind::Leaf { .. }) {
                // Best child is a leaf. Check if current node has room for another child.
                if children.len() < MAX_CHILDREN {
                    // Add new leaf directly to this node
                    let node = &mut self.nodes[current_idx];

                    if let NodeKind::Internal { children } = &mut node.kind {
                        children.push(new_leaf_idx);
                        // Swap new leaf only if it has the highest max_order
                        if order <= node.max_order {
                            let last = children.len() - 1;
                            children.indices.swap(last - 1, last);
                        }
                    }

                    node.bounds = node.bounds.union(&bounds);
                    node.max_order = node.max_order.max(order);
                    break;
                } else {
                    // Node is full, create new internal with [best_leaf, new_leaf]
                    let sibling_bounds = self.nodes[best_child_idx].bounds;
                    let sibling_order = self.nodes[best_child_idx].max_order;

                    let mut new_children = NodeChildren::new();
                    // Max end invariant
                    if order > sibling_order {
                        new_children.push(best_child_idx);
                        new_children.push(new_leaf_idx);
                    } else {
                        new_children.push(new_leaf_idx);
                        new_children.push(best_child_idx);
                    }

                    let new_internal_idx = self.nodes.len();
                    let new_internal_max = sibling_order.max(order);
                    self.nodes.push(Node {
                        bounds: sibling_bounds.union(&bounds),
                        max_order: new_internal_max,
                        kind: NodeKind::Internal {
                            children: new_children,
                        },
                    });

                    // Replace the leaf with the new internal in parent
                    let parent = &mut self.nodes[current_idx];
                    if let NodeKind::Internal { children } = &mut parent.kind {
                        let children_len = children.len();

                        children.indices[best_child_pos] = new_internal_idx;

                        // If new internal has highest max_order, swap it to the end
                        // to maintain sorting invariant
                        if new_internal_max > parent.max_order {
                            children.indices.swap(best_child_pos, children_len - 1);
                        }
                    }
                    break;
                }
            } else {
                // Best child is internal, continue descent
                current_idx = best_child_idx;
            }
        }

        // Propagate bounds and max_order updates up the tree
        let mut updated_child_idx = None;
        for &node_idx in self.insert_path.iter().rev() {
            let node = &mut self.nodes[node_idx];
            node.bounds = node.bounds.union(&bounds);

            if node.max_order < order {
                node.max_order = order;

                // Swap updated child to end (skip first iteration since the invariant is already handled by previous cases)
                if let Some(child_idx) = updated_child_idx {
                    if let NodeKind::Internal { children } = &mut node.kind {
                        if let Some(pos) = children.as_slice().iter().position(|&c| c == child_idx)
                        {
                            let last = children.len() - 1;
                            if pos != last {
                                children.indices.swap(pos, last);
                            }
                        }
                    }
                }
            }

            updated_child_idx = Some(node_idx);
        }

        new_leaf_idx
    }
}

impl Default for BoundsTree {
    fn default() -> Self {
        BoundsTree {
            nodes: Vec::new(),
            root: None,
            max_leaf: None,
            insert_path: Vec::new(),
            search_stack: Vec::new(),
        }
    }
}

// Helper trait for Rect to calculate union and half-perimeter
trait RectExt {
    #[allow(dead_code)]
    fn union(&self, other: &Self) -> Self;
    fn half_perimeter(&self) -> f64;
}

impl RectExt for Rect {
    fn union(&self, other: &Rect) -> Rect {
        let min_x = self.origin.x.min(other.origin.x);
        let min_y = self.origin.y.min(other.origin.y);
        let max_x = (self.origin.x + self.size.width).max(other.origin.x + other.size.width);
        let max_y = (self.origin.y + self.size.height).max(other.origin.y + other.size.height);

        Rect::new(
            Point::new(min_x, min_y),
            Size::new(max_x - min_x, max_y - min_y),
        )
    }

    fn half_perimeter(&self) -> f64 {
        self.size.width + self.size.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut tree = BoundsTree::new();
        let bounds1 = Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0));
        let bounds2 = Rect::new(Point::new(5.0, 5.0), Size::new(10.0, 10.0));
        let bounds3 = Rect::new(Point::new(10.0, 10.0), Size::new(10.0, 10.0));

        // Insert the bounds into the tree and verify the order is correct
        assert_eq!(tree.insert(bounds1), 1);
        assert_eq!(tree.insert(bounds2), 2); // Overlaps bounds1
        assert_eq!(tree.insert(bounds3), 3); // Overlaps bounds2

        // Insert non-overlapping bounds and verify they can reuse orders
        let bounds4 = Rect::new(Point::new(20.0, 20.0), Size::new(10.0, 10.0));
        let bounds5 = Rect::new(Point::new(40.0, 40.0), Size::new(10.0, 10.0));
        let bounds6 = Rect::new(Point::new(25.0, 25.0), Size::new(10.0, 10.0));

        assert_eq!(tree.insert(bounds4), 1); // Non-overlapping
        assert_eq!(tree.insert(bounds5), 1); // Non-overlapping
        assert_eq!(tree.insert(bounds6), 2); // Overlaps bounds4
    }

    #[test]
    fn test_batching() {
        let mut tree = BoundsTree::new();

        // Three non-overlapping rects should all get order=1 (batched!)
        let rect1 = Rect::new(Point::new(0.0, 0.0), Size::new(50.0, 50.0));
        let rect2 = Rect::new(Point::new(100.0, 0.0), Size::new(50.0, 50.0));
        let rect3 = Rect::new(Point::new(200.0, 0.0), Size::new(50.0, 50.0));

        assert_eq!(tree.insert(rect1), 1);
        assert_eq!(tree.insert(rect2), 1); // ✅ Same order (batched!)
        assert_eq!(tree.insert(rect3), 1); // ✅ Same order (batched!)

        // Overlapping rect gets order=2
        let rect4 = Rect::new(Point::new(25.0, 25.0), Size::new(50.0, 50.0));
        assert_eq!(tree.insert(rect4), 2);
    }
}
