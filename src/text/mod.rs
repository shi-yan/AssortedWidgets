//! Text rendering system for AssortedWidgets
//!
//! This module implements a production-quality text rendering system using:
//! - cosmic-text: Font discovery, fallback, shaping, and rasterization
//! - etagere: 2D bin packing for glyph atlas
//! - Multi-page RGBA8 texture atlas for efficient GPU rendering
//!
//! Architecture:
//! - `FontSystem`: Wrapper around cosmic-text's FontSystem and SwashCache
//! - `GlyphAtlas`: GPU texture atlas with bin packing and caching
//! - `TextRenderer`: Instanced quad renderer for batched text drawing

mod atlas;
mod font_system;
mod renderer;

pub use atlas::{GlyphAtlas, GlyphKey, GlyphLocation, UvRect};
pub use font_system::FontSystemWrapper;
pub use renderer::{TextRenderer, TextInstance};
