/// Icon rendering system using Google Material Icons font
///
/// Icons are rendered as glyphs from an embedded icon font, sharing the same
/// GlyphAtlas infrastructure as text rendering for efficient batching.

mod embedded;
mod engine;

pub use embedded::IconAssets;
pub use engine::IconEngine;
