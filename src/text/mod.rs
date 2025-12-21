//! Text rendering system for AssortedWidgets
//!
//! This module implements a production-quality text rendering system using:
//! - cosmic-text: Font discovery, fallback, shaping, and rasterization
//! - etagere: 2D bin packing for glyph atlas
//! - Multi-page RGBA8 texture atlas for efficient GPU rendering
//!
//! ## Two-Tier API Architecture
//!
//! ### High-Level Managed API (For Simple Widgets)
//! ```rust,ignore
//! ctx.draw_text("Hello", &TextStyle::default(), Point::new(0, 0), None);
//! ```
//! - Automatic LRU caching
//! - Deduplication (multiple widgets with same text share cache)
//! - Generational eviction (unused text auto-removed)
//!
//! ### Low-Level Manual API (For Advanced Widgets)
//! ```rust,ignore
//! let layout = engine.create_layout("Hello", &style, None, Truncate::None);
//! ctx.draw_layout(&layout, Point::new(0, 0), Color::BLACK);
//! ```
//! - Widget owns the TextLayout
//! - No caching overhead
//! - Precise invalidation control
//!
//! Architecture:
//! - `FontSystemWrapper`: Wrapper around cosmic-text's FontSystem and SwashCache
//! - `GlyphAtlas`: GPU texture atlas with bin packing and caching
//! - `TextRenderer`: Instanced quad renderer for batched text drawing
//! - `TextEngine`: Dual-mode caching (managed + manual)
//! - `TextLayout`: Pre-shaped text ready for rendering
//! - `TextStyle`: Font configuration

mod atlas;
mod engine;
mod font_system;
mod layout;
mod renderer;
mod style;

pub use atlas::{GlyphAtlas, GlyphKey, GlyphLocation, UvRect};
pub use engine::{TextEngine, CacheStats};
pub use font_system::{FontSystemWrapper, RasterizedGlyph};
pub use layout::TextLayout;
pub use renderer::{TextRenderer, TextInstance};
pub use style::{TextStyle, Truncate};
