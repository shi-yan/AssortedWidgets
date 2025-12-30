//! Z-ordering utilities for depth buffer management
//!
//! This module implements the formal z-ordering architecture described in Z_ORDER_ARCHITECTURE.md.
//! It provides:
//! - Depth value conversion from user z-index to GPU depth
//! - Layer-based depth range mapping
//! - Support for both opaque and transparent rendering passes

/// Convert user z-index to GPU depth value
///
/// Maps user-facing z-index layers to GPU depth buffer values [0.0, 1.0].
/// Higher z-index = LOWER depth value (closer to camera, rendered on top).
///
/// # Layer Depth Ranges
///
/// - MODAL (10000+):     [0.0, 0.1)   - Closest to camera
/// - OVERLAY (1000):     [0.1, 0.2)   - Tooltips, popovers
/// - FOREGROUND (100):   [0.2, 0.3)   - Raised elements
/// - NORMAL (0):         [0.3, 0.6)   - Standard UI elements
/// - SHADOW (-100):      [0.6, 0.8)   - Drop shadows
/// - BACKGROUND (-1000): [0.8, 1.0]   - Furthest from camera
///
/// # Arguments
///
/// * `z_index` - User-facing z-index (from layers::SHADOW, layers::NORMAL, etc.)
/// * `fine_z` - Fine-grained offset within layer [0.0, 1.0], from BoundsTree
///
/// # Returns
///
/// GPU depth value clamped to [0.0, 1.0]
///
/// # Examples
///
/// ```
/// use assorted_widgets::paint::z_order::z_index_to_depth;
/// use assorted_widgets::paint::layers;
///
/// // Shadow at base depth (no overlap)
/// let depth = z_index_to_depth(layers::SHADOW, 0.0);
/// assert!((depth - 0.6).abs() < 0.001);
///
/// // Overlapping shadows (fine_z separates them)
/// let shadow1 = z_index_to_depth(layers::SHADOW, 0.0);
/// let shadow2 = z_index_to_depth(layers::SHADOW, 0.00001);
/// assert!(shadow2 > shadow1); // shadow2 slightly further back
///
/// // Normal UI element (closer than shadows)
/// let normal = z_index_to_depth(layers::NORMAL, 0.0);
/// assert!(normal < shadow1); // Lower depth = closer
/// ```
pub fn z_index_to_depth(z_index: i32, fine_z: f32) -> f32 {
    // Clamp fine_z to valid range
    let fine_z = fine_z.clamp(0.0, 1.0);

    // Map z-index to depth ranges
    // Note: Lower depth values = closer to camera (rendered on top)
    let base_depth = match z_index {
        // Layer MODAL (10000+) → depth [0.0, 0.1)
        10000..=i32::MAX => 0.0 + fine_z * 0.1,

        // Layer OVERLAY (1000-9999) → depth [0.1, 0.2)
        1000..=9999 => 0.1 + fine_z * 0.1,

        // Layer FOREGROUND (100-999) → depth [0.2, 0.3)
        100..=999 => 0.2 + fine_z * 0.1,

        // Layer NORMAL (0-99) → depth [0.3, 0.6)
        // Widest range for most UI elements
        0..=99 => 0.3 + fine_z * 0.3,

        // Layer SHADOW (-100 to -1) → depth [0.6, 0.8)
        -100..=-1 => 0.6 + fine_z * 0.2,

        // Layer BACKGROUND (-1000+) → depth [0.8, 1.0]
        i32::MIN..=-101 => 0.8 + fine_z * 0.2,
    };

    // Ensure depth is in valid range
    base_depth.clamp(0.0, 1.0)
}

/// Temporary depth calculation for Phase 1 (before BoundsTree)
///
/// This function provides simple depth values without overlap detection.
/// It will be replaced by BoundsTree-based assignment in Phase 2.
///
/// # Arguments
///
/// * `z_index` - User-facing z-index
/// * `paint_order` - Sequential index within frame (0, 1, 2, ...)
///
/// # Returns
///
/// GPU depth value
///
/// # Note
///
/// This is a **temporary** implementation for Phase 1. Phase 2 will use
/// BoundsTree to assign `fine_z` values based on actual overlap detection.
pub fn temp_z_index_to_depth_simple(z_index: i32, paint_order: usize) -> f32 {
    // Use paint order as fine-grained offset
    // Divide by large number to keep within layer range
    let fine_z = (paint_order as f32) * 0.00001;
    z_index_to_depth(z_index, fine_z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paint::layers;

    #[test]
    fn test_depth_ranges() {
        // Test layer depth ranges are non-overlapping
        let modal = z_index_to_depth(layers::MODAL, 0.0);
        let overlay = z_index_to_depth(layers::OVERLAY, 0.0);
        let foreground = z_index_to_depth(layers::FOREGROUND, 0.0);
        let normal = z_index_to_depth(layers::NORMAL, 0.0);
        let shadow = z_index_to_depth(layers::SHADOW, 0.0);
        let background = z_index_to_depth(layers::BACKGROUND, 0.0);

        // Lower depth = closer (rendered on top)
        assert!(modal < overlay);
        assert!(overlay < foreground);
        assert!(foreground < normal);
        assert!(normal < shadow);
        assert!(shadow < background);
    }

    #[test]
    fn test_fine_z_ordering() {
        // Within same layer, fine_z should separate elements
        let shadow1 = z_index_to_depth(layers::SHADOW, 0.0);
        let shadow2 = z_index_to_depth(layers::SHADOW, 0.5);

        assert!(shadow2 > shadow1, "Higher fine_z should be further back");
    }

    #[test]
    fn test_depth_clamping() {
        // Extreme values should clamp to [0.0, 1.0]
        let near = z_index_to_depth(i32::MAX, 0.0);
        let far = z_index_to_depth(i32::MIN, 1.0);

        assert!(near >= 0.0 && near <= 1.0);
        assert!(far >= 0.0 && far <= 1.0);
    }

    #[test]
    fn test_shadow_vs_normal_ordering() {
        // Critical test: Shadows must render BEHIND normal elements
        let shadow = z_index_to_depth(layers::SHADOW, 0.0);
        let normal = z_index_to_depth(layers::NORMAL, 0.0);

        assert!(
            shadow > normal,
            "Shadows (depth={}) must be BEHIND normal elements (depth={})",
            shadow,
            normal
        );
    }

    #[test]
    fn test_temp_simple_conversion() {
        // Test temporary conversion maintains ordering
        let shadow1 = temp_z_index_to_depth_simple(layers::SHADOW, 0);
        let shadow2 = temp_z_index_to_depth_simple(layers::SHADOW, 1);
        let normal = temp_z_index_to_depth_simple(layers::NORMAL, 0);

        assert!(shadow2 > shadow1, "Later paint order = further back");
        assert!(shadow1 > normal, "Shadows behind normal");
    }
}
