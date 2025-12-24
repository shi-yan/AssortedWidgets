/// Standard Z-Index Layers for predictable UI layering
///
/// These constants define semantic layers that ensure consistent z-ordering
/// across the UI. Shadows always render behind shapes, tooltips always on top, etc.
///
/// Example:
/// ```
/// ctx.primitives.draw_rect_z(bounds, style, layers::SHADOW);   // Behind everything
/// ctx.primitives.draw_rect_z(bounds, style, layers::NORMAL);   // Normal layer
/// ctx.primitives.draw_rect_z(bounds, style, layers::OVERLAY);  // Tooltips, popovers
/// ```
pub mod layers {
    /// Background layers (wallpapers, large background images)
    pub const BACKGROUND: i32 = -1000;

    /// Shadow layer (all drop shadows render here)
    pub const SHADOW: i32 = -100;

    /// Normal widget layer (default for most UI elements)
    pub const NORMAL: i32 = 0;

    /// Foreground layer (raised elements like hovered buttons)
    pub const FOREGROUND: i32 = 100;

    /// Overlay layer (tooltips, popovers, dropdown menus)
    pub const OVERLAY: i32 = 1000;

    /// Modal layer (modal dialogs, always on top)
    pub const MODAL: i32 = 10000;
}

// Re-export for convenience
pub use layers::*;
