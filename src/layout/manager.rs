use crate::types::{Rect, WidgetId, Point, Size};
use std::collections::HashMap;
use taffy::{TaffyTree, NodeId, Style, AvailableSpace};

/// Context data stored per-node for measure functions
///
/// This holds the data needed to calculate a node's intrinsic size.
/// The actual measurement logic is provided as a closure to `compute_layout_with_measure()`.
#[derive(Default, Clone)]
pub struct MeasureContext {
    /// The widget ID for this node (used to look up element data during measurement)
    pub widget_id: WidgetId,

    /// Whether this node needs custom measurement
    pub needs_measure: bool,
}

/// Manages layout using Taffy
///
/// This wraps the Taffy layout engine and syncs it with our WidgetId system.
/// Each widget gets a corresponding Taffy NodeId for layout calculations.
pub struct LayoutManager {
    /// The Taffy layout engine
    /// Stores MeasureContext per node for elements that need dynamic sizing
    taffy: TaffyTree<MeasureContext>,

    /// Mapping from our WidgetId to Taffy's NodeId
    nodes: HashMap<WidgetId, NodeId>,

    /// Reverse mapping for cleanup
    widget_ids: HashMap<NodeId, WidgetId>,

    /// Cached layout results (updated after compute_layout)
    layouts: HashMap<WidgetId, Rect>,

    /// Root node (represents the window)
    root: Option<NodeId>,
}

impl LayoutManager {
    pub fn new() -> Self {
        LayoutManager {
            taffy: TaffyTree::new(),
            nodes: HashMap::new(),
            widget_ids: HashMap::new(),
            layouts: HashMap::new(),
            root: None,
        }
    }

    /// Create a new layout node with the given style
    pub fn create_node(&mut self, widget_id: WidgetId, style: Style) -> Result<(), String> {
        // Create node with empty context (no measurement needed)
        let context = MeasureContext {
            widget_id,
            needs_measure: false,
        };

        let node = self.taffy.new_leaf_with_context(style, context)
            .map_err(|e| format!("Failed to create Taffy node: {:?}", e))?;

        self.nodes.insert(widget_id, node);
        self.widget_ids.insert(node, widget_id);

        // If this is the first node, make it the root
        if self.root.is_none() {
            self.root = Some(node);
        }

        Ok(())
    }

    /// Create a new layout node that needs custom measurement
    ///
    /// Use this for elements that need dynamic sizing based on content (like text).
    /// The actual measurement logic should be provided to `compute_layout_with_measure()`.
    pub fn create_measurable_node(
        &mut self,
        widget_id: WidgetId,
        style: Style,
    ) -> Result<(), String> {
        // Create node with measure context
        let context = MeasureContext {
            widget_id,
            needs_measure: true,
        };

        let node = self.taffy.new_leaf_with_context(style, context)
            .map_err(|e| format!("Failed to create Taffy node: {:?}", e))?;

        self.nodes.insert(widget_id, node);
        self.widget_ids.insert(node, widget_id);

        // If this is the first node, make it the root
        if self.root.is_none() {
            self.root = Some(node);
        }

        Ok(())
    }

    /// Add a child to a parent node
    pub fn add_child(&mut self, parent_id: WidgetId, child_id: WidgetId) -> Result<(), String> {
        let parent_node = self.nodes.get(&parent_id)
            .ok_or_else(|| format!("Parent widget {:?} has no layout node", parent_id))?;

        let child_node = self.nodes.get(&child_id)
            .ok_or_else(|| format!("Child widget {:?} has no layout node", child_id))?;

        self.taffy.add_child(*parent_node, *child_node)
            .map_err(|e| format!("Failed to add child to layout: {:?}", e))?;

        Ok(())
    }

    /// Update the style of a node
    pub fn set_style(&mut self, widget_id: WidgetId, style: Style) -> Result<(), String> {
        let node = self.nodes.get(&widget_id)
            .ok_or_else(|| format!("Widget {:?} has no layout node", widget_id))?;

        self.taffy.set_style(*node, style)
            .map_err(|e| format!("Failed to set style: {:?}", e))?;

        Ok(())
    }

    /// Mark a node as dirty (needs re-layout)
    ///
    /// Call this when an element's content changes in a way that affects its size
    /// (e.g., text content changes, image loads). This triggers a layout recalculation
    /// from this node upwards to the root.
    pub fn mark_dirty(&mut self, widget_id: WidgetId) -> Result<(), String> {
        let node = self.nodes.get(&widget_id)
            .ok_or_else(|| format!("Widget {:?} has no layout node", widget_id))?;

        self.taffy.mark_dirty(*node)
            .map_err(|e| format!("Failed to mark dirty: {:?}", e))?;

        Ok(())
    }

    /// Compute layout for the entire tree
    ///
    /// This should be called before rendering when layout is dirty.
    /// The available_size is typically the window size.
    ///
    /// **Note**: This uses simple layout without custom measurement.
    /// For elements with dynamic sizing (like text), use `compute_layout_with_measure()` instead.
    pub fn compute_layout(&mut self, available_size: Size) -> Result<(), String> {
        let root = self.root
            .ok_or_else(|| "No root node set".to_string())?;

        // Compute layout starting from root
        self.taffy.compute_layout(
            root,
            taffy::Size {
                width: AvailableSpace::Definite(available_size.width as f32),
                height: AvailableSpace::Definite(available_size.height as f32),
            }
        ).map_err(|e| format!("Failed to compute layout: {:?}", e))?;

        // Copy results to our cache
        self.cache_layouts();

        Ok(())
    }

    /// Compute layout with a custom measure function
    ///
    /// Use this when you have elements that need dynamic sizing based on content.
    /// The measure function will be called for nodes marked with `needs_measure = true`.
    ///
    /// # Arguments
    /// * `available_size` - The available space (typically window size)
    /// * `measure_fn` - Closure that calculates size for measurable nodes
    ///
    /// # Example
    /// ```ignore
    /// layout_manager.compute_layout_with_measure(window_size, |known, available, _node_id, context, _style| {
    ///     if let Some(ctx) = context {
    ///         if ctx.needs_measure {
    ///             // Look up element and call its measure() method
    ///             if let Some(element) = element_manager.get(ctx.widget_id) {
    ///                 if let Some(size) = element.measure(known, available) {
    ///                     return taffy::Size { width: size.width as f32, height: size.height as f32 };
    ///                 }
    ///             }
    ///         }
    ///     }
    ///     taffy::Size::ZERO
    /// })
    /// ```
    pub fn compute_layout_with_measure<F>(
        &mut self,
        available_size: Size,
        measure_fn: F,
    ) -> Result<(), String>
    where
        F: FnMut(
            taffy::Size<Option<f32>>,
            taffy::Size<AvailableSpace>,
            NodeId,
            Option<&mut MeasureContext>,
            &Style,
        ) -> taffy::Size<f32>,
    {
        let root = self.root
            .ok_or_else(|| "No root node set".to_string())?;

        // Compute layout with measure function
        self.taffy.compute_layout_with_measure(
            root,
            taffy::Size {
                width: AvailableSpace::Definite(available_size.width as f32),
                height: AvailableSpace::Definite(available_size.height as f32),
            },
            measure_fn,
        ).map_err(|e| format!("Failed to compute layout with measure: {:?}", e))?;

        // Copy results to our cache
        self.cache_layouts();

        Ok(())
    }

    /// Cache layout results from Taffy
    fn cache_layouts(&mut self) {
        self.layouts.clear();

        for (widget_id, node_id) in &self.nodes {
            if let Ok(layout) = self.taffy.layout(*node_id) {
                self.layouts.insert(*widget_id, Rect {
                    origin: Point::new(layout.location.x as f64, layout.location.y as f64),
                    size: Size::new(layout.size.width as f64, layout.size.height as f64),
                });
            }
        }
    }

    /// Get the cached layout for a widget
    pub fn get_layout(&self, widget_id: WidgetId) -> Option<Rect> {
        self.layouts.get(&widget_id).copied()
    }

    /// Remove a node and all its descendants
    pub fn remove_node(&mut self, widget_id: WidgetId) -> Result<(), String> {
        if let Some(node) = self.nodes.remove(&widget_id) {
            self.widget_ids.remove(&node);
            self.layouts.remove(&widget_id);

            // Taffy will handle removing descendants
            self.taffy.remove(node)
                .map_err(|e| format!("Failed to remove node: {:?}", e))?;
        }

        Ok(())
    }

    /// Set the root node explicitly
    pub fn set_root(&mut self, widget_id: WidgetId) -> Result<(), String> {
        let node = self.nodes.get(&widget_id)
            .ok_or_else(|| format!("Widget {:?} has no layout node", widget_id))?;

        self.root = Some(*node);
        Ok(())
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
