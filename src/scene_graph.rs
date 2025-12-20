use crate::types::{Point, WidgetId};

// ============================================================================
// Scene Graph (Element Tree)
// ============================================================================

/// Tree node containing only IDs, not actual elements
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id: WidgetId,
    pub children: Vec<SceneNode>,
}

impl SceneNode {
    pub fn new(id: WidgetId) -> Self {
        SceneNode {
            id,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }

    /// Traverse tree in pre-order (parent first, then children)
    pub fn traverse<F>(&self, visitor: &mut F)
    where
        F: FnMut(WidgetId),
    {
        visitor(self.id);
        for child in &self.children {
            child.traverse(visitor);
        }
    }

    /// Traverse tree to find element at given position (hit testing)
    pub fn hit_test(&self, point: Point) -> Option<WidgetId> {
        // Traverse in reverse order (top to bottom in z-order)
        for child in self.children.iter().rev() {
            if let Some(id) = child.hit_test(point) {
                return Some(id);
            }
        }
        Some(self.id)
    }

    /// Find parent of given widget ID
    pub fn find_parent(&self, child_id: WidgetId) -> Option<WidgetId> {
        for child in &self.children {
            if child.id == child_id {
                return Some(self.id);
            }
            if let Some(parent) = child.find_parent(child_id) {
                return Some(parent);
            }
        }
        None
    }
}

/// The scene graph holds the hierarchical structure
pub struct SceneGraph {
    root: Option<SceneNode>,
}

impl SceneGraph {
    pub fn new() -> Self {
        SceneGraph { root: None }
    }

    pub fn set_root(&mut self, root: SceneNode) {
        self.root = Some(root);
    }

    pub fn root(&self) -> Option<&SceneNode> {
        self.root.as_ref()
    }

    pub fn root_mut(&mut self) -> Option<&mut SceneNode> {
        self.root.as_mut()
    }

    pub fn hit_test(&self, point: Point) -> Option<WidgetId> {
        self.root.as_ref().and_then(|r| r.hit_test(point))
    }

    pub fn find_parent(&self, child_id: WidgetId) -> Option<WidgetId> {
        self.root.as_ref().and_then(|r| r.find_parent(child_id))
    }
}
