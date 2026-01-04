//! Text Layout Cache
//!
//! Dual-cache system for text layouts to avoid reshaping during Taffy's constraint solving.
//! Taffy alternates between intrinsic (None) and constrained (Some(width)) queries during
//! flexbox/grid layout computation. This cache stores both types to avoid constant reshaping.

use std::cell::RefCell;
use crate::text::{TextEngine, TextLayout};

/// Dual-cache for text layouts
///
/// Stores two separate caches:
/// - **Intrinsic**: For unconstrained width queries (max_width=None)
/// - **Constrained**: For wrapped width queries (max_width=Some(width))
///
/// This prevents reshaping when Taffy alternates between asking for intrinsic size
/// and constrained size during layout solving.
pub struct TextLayoutCache {
    /// Cache for intrinsic layout (no width constraint)
    intrinsic: RefCell<Option<TextLayout>>,

    /// Cache for constrained layout (specific width)
    /// Stores (width, layout) tuple to check if width changed
    constrained: RefCell<Option<(f32, TextLayout)>>,
}

impl TextLayoutCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            intrinsic: RefCell::new(None),
            constrained: RefCell::new(None),
        }
    }

    /// Get or create a text layout for the given constraints
    ///
    /// # Arguments
    /// * `engine` - TextEngine for creating new layouts
    /// * `max_width` - Maximum width constraint (None = unconstrained)
    /// * `create_fn` - Function to create a new layout if cache miss
    ///
    /// # Returns
    /// Reference to the cached TextLayout (either intrinsic or constrained)
    ///
    /// # Panics
    /// Panics if cache is empty after calling create_fn (shouldn't happen)
    pub fn get_or_create<F>(
        &self,
        engine: &mut TextEngine,
        max_width: Option<f32>,
        create_fn: F,
    ) -> std::cell::Ref<TextLayout>
    where
        F: FnOnce(&mut TextEngine, Option<f32>) -> TextLayout,
    {
        match max_width {
            None => {
                // Intrinsic query - check intrinsic cache
                if self.intrinsic.borrow().is_none() {
                    let layout = create_fn(engine, None);
                    *self.intrinsic.borrow_mut() = Some(layout);
                }

                std::cell::Ref::map(self.intrinsic.borrow(), |opt| {
                    opt.as_ref().expect("Intrinsic cache should be populated")
                })
            }
            Some(width) => {
                // Constrained query - check constrained cache
                let needs_reshape = self.constrained.borrow()
                    .as_ref()
                    .map(|(cached_width, _)| (*cached_width - width).abs() > 0.01)
                    .unwrap_or(true);

                if needs_reshape {
                    let layout = create_fn(engine, Some(width));
                    *self.constrained.borrow_mut() = Some((width, layout));
                }

                std::cell::Ref::map(self.constrained.borrow(), |opt| {
                    let (_width, layout) = opt.as_ref().expect("Constrained cache should be populated");
                    layout
                })
            }
        }
    }

    /// Get the size of the cached layout for the given constraint
    ///
    /// This is a convenience method that calls get_or_create and returns the size.
    pub fn get_or_create_size<F>(
        &self,
        engine: &mut TextEngine,
        max_width: Option<f32>,
        create_fn: F,
    ) -> crate::types::Size
    where
        F: FnOnce(&mut TextEngine, Option<f32>) -> TextLayout,
    {
        let layout = self.get_or_create(engine, max_width, create_fn);
        let size = layout.size();
        crate::types::Size::new(size.width as f64, size.height as f64)
    }

    /// Invalidate all caches
    ///
    /// Call this when widget content changes (text, style, wrap mode, etc.)
    pub fn invalidate(&self) {
        *self.intrinsic.borrow_mut() = None;
        *self.constrained.borrow_mut() = None;
    }

    /// Try to get the cached intrinsic layout without creating it
    pub fn get_intrinsic(&self) -> Option<std::cell::Ref<TextLayout>> {
        if self.intrinsic.borrow().is_some() {
            Some(std::cell::Ref::map(self.intrinsic.borrow(), |opt| {
                opt.as_ref().unwrap()
            }))
        } else {
            None
        }
    }

    /// Try to get the cached constrained layout without creating it
    pub fn get_constrained(&self) -> Option<std::cell::Ref<TextLayout>> {
        if self.constrained.borrow().is_some() {
            Some(std::cell::Ref::map(self.constrained.borrow(), |opt| {
                &opt.as_ref().unwrap().1
            }))
        } else {
            None
        }
    }
}

impl Default for TextLayoutCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic functionality tests would go here
    // Note: Requires TextEngine setup which is non-trivial
}
