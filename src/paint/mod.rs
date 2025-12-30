pub mod batcher;
pub mod clip;
pub mod context;
pub mod gradient;
pub mod layered_bounds_tree;
pub mod layers;
pub mod path;
pub mod primitives;
pub mod types;
pub mod z_order;

pub use batcher::PrimitiveBatcher;
pub use clip::{ClipRegion, ClipStack};
pub use context::{PaintContext, RenderBundle};
pub use gradient::{ColorStop, LinearGradient, RadialGradient, MAX_GRADIENT_STOPS};
pub use layered_bounds_tree::LayeredBoundsTree;
// Re-export layer constants at module level
pub use layers::*;
pub use path::{LineCap, LineJoin, Path, PathCommand, Stroke};
pub use primitives::{Color, RectInstance};
pub use types::{Border, Brush, CornerRadius, DrawCommand, ShapeStyle, Shadow};
pub use z_order::{z_index_to_depth, temp_z_index_to_depth_simple};
