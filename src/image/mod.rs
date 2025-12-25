/// Image loading and caching system
///
/// Provides LRU-cached loading of images (PNG/JPG/WebP) as GPU textures.
/// Unlike icons, images use individual textures (not atlas-based) to support
/// arbitrary sizes.

mod cache;

pub use cache::{CachedImage, ImageCache, ImageError, ImageId};
