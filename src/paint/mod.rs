pub mod batcher;
pub mod clip;
pub mod context;
pub mod gradient;
pub mod layers;
pub mod primitives;
pub mod types;

pub use batcher::PrimitiveBatcher;
pub use clip::{ClipRegion, ClipStack};
pub use context::{PaintContext, RenderBundle};
pub use gradient::{ColorStop, LinearGradient, RadialGradient, MAX_GRADIENT_STOPS};
// Re-export layer constants at module level
pub use layers::*;
pub use primitives::{Color, RectInstance};
pub use types::{Border, Brush, CornerRadius, DrawCommand, ShapeStyle, Shadow};
