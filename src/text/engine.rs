//! Text engine with dual-mode caching
//!
//! Provides two APIs:
//! 1. High-level managed API: Transparent LRU caching for simple widgets
//! 2. Low-level manual API: Widget owns TextLayout lifecycle

use super::{TextLayout, TextStyle, Truncate};
use super::font_system::FontSystemWrapper;
use cosmic_text::{Buffer, Metrics, Shaping};
use std::collections::HashMap;

/// Key for content-addressable text caching
///
/// NOTE: Does NOT include widget ID - this allows deduplication!
/// If 1000 folders all say "Folder", we shape it once and cache it once.
#[derive(Hash, Eq, PartialEq, Clone)]
struct TextCacheKey {
    /// The text content
    text: String,

    /// Font size as fixed-point bits (avoids f32 hash issues)
    font_size_bits: u32,

    /// Max width as fixed-point bits (None = unlimited)
    max_width_bits: Option<u32>,

    /// Font family
    font_family: String,

    /// Font weight as u16
    font_weight: u16,
}

impl TextCacheKey {
    fn new(text: &str, style: &TextStyle, max_width: Option<f32>) -> Self {
        Self {
            text: text.to_string(),
            font_size_bits: style.font_size.to_bits(),
            max_width_bits: max_width.map(|w| w.to_bits()),
            font_family: style.font_family.clone(),
            font_weight: style.font_weight.0,
        }
    }
}

/// Cached text layout with frame tracking for LRU eviction
struct CachedTextLayout {
    layout: TextLayout,
    last_used_frame: u64,
}

/// Text engine managing font system and caching
///
/// Provides two modes of operation:
/// - **Managed mode**: Transparent caching via `get_or_create_managed()`
/// - **Manual mode**: Just create and return via `create_layout()`
pub struct TextEngine {
    /// Font system (wraps cosmic-text)
    font_system: FontSystemWrapper,

    /// Global LRU cache for managed mode
    managed_cache: HashMap<TextCacheKey, CachedTextLayout>,

    /// Frame counter for generational eviction
    current_frame: u64,

    /// Cache eviction threshold (frames)
    eviction_threshold: u64,

    /// Cache cleanup interval (frames)
    cleanup_interval: u64,
}

impl TextEngine {
    /// Create a new text engine
    pub fn new() -> Self {
        Self {
            font_system: FontSystemWrapper::new(),
            managed_cache: HashMap::new(),
            current_frame: 0,
            eviction_threshold: 120,  // Evict after 2 seconds at 60fps
            cleanup_interval: 60,     // Clean up every 1 second
        }
    }

    /// Begin a new frame (updates frame counter)
    ///
    /// Call this once per frame from the event loop.
    /// Triggers cache cleanup every N frames.
    pub fn begin_frame(&mut self) {
        self.current_frame += 1;

        // Clean up stale entries periodically
        if self.current_frame % self.cleanup_interval == 0 {
            self.cleanup_cache();
        }
    }

    /// Get a reference to the font system
    pub fn font_system(&self) -> &FontSystemWrapper {
        &self.font_system
    }

    /// Get a mutable reference to the font system
    pub fn font_system_mut(&mut self) -> &mut FontSystemWrapper {
        &mut self.font_system
    }

    // ========================================================================
    // LOW-LEVEL MANUAL API (No caching - widget owns the TextLayout)
    // ========================================================================

    /// Create a text layout (manual mode - no caching)
    ///
    /// **Use this for:** Editors, terminals, widgets with thousands of unique texts
    ///
    /// The widget owns the returned TextLayout and decides when to re-shape.
    ///
    /// # Arguments
    /// * `text` - The text to shape
    /// * `style` - Font styling
    /// * `max_width` - Optional width constraint for wrapping
    /// * `truncate` - Truncation mode (None or End with ellipsis)
    ///
    /// # Returns
    /// Owned TextLayout ready for rendering
    pub fn create_layout(
        &mut self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f32>,
        truncate: Truncate,
    ) -> TextLayout {
        let buffer = self.shape_text_internal(text, style, max_width, truncate);
        TextLayout::new(buffer)
    }

    // ========================================================================
    // HIGH-LEVEL MANAGED API (Transparent caching)
    // ========================================================================

    /// Get or create a text layout (managed mode - transparently cached)
    ///
    /// **Use this for:** Buttons, labels, menus, tooltips
    ///
    /// The TextEngine owns the layout and caches it. If the same text with
    /// the same style is requested again, it returns the cached version.
    ///
    /// # Arguments
    /// * `text` - The text to shape
    /// * `style` - Font styling
    /// * `max_width` - Optional width constraint for wrapping
    ///
    /// # Returns
    /// Reference to cached TextLayout (lifetime tied to this TextEngine)
    pub fn get_or_create_managed(
        &mut self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f32>,
    ) -> &TextLayout {
        let key = TextCacheKey::new(text, style, max_width);
        let current_frame = self.current_frame;

        // Check if already cached (need to check without borrowing)
        let needs_creation = !self.managed_cache.contains_key(&key);

        if needs_creation {
            // Cache miss - create new layout
            let buffer = self.shape_text_internal(text, style, max_width, Truncate::None);
            let layout = TextLayout::new(buffer);

            // Insert into cache
            self.managed_cache.insert(
                key.clone(),
                CachedTextLayout {
                    layout,
                    last_used_frame: current_frame,
                },
            );
        }

        // Update last used frame (mutable borrow in scope, then drop)
        {
            let cached = self.managed_cache.get_mut(&key).unwrap();
            cached.last_used_frame = current_frame;
        }

        // Return immutable reference (new borrow)
        &self.managed_cache.get(&key).unwrap().layout
    }

    // ========================================================================
    // Internal Implementation
    // ========================================================================

    /// Shape text using cosmic-text (internal)
    fn shape_text_internal(
        &mut self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f32>,
        truncate: Truncate,
    ) -> Buffer {
        let font_system = self.font_system.font_system_mut();

        // Create metrics
        let metrics = Metrics::new(style.font_size, style.line_height_pixels());

        // Create buffer
        let mut buffer = Buffer::new(font_system, metrics);

        // Set size (width constraint for wrapping)
        if let Some(width) = max_width {
            buffer.set_size(font_system, Some(width), None);
        }

        // Set text with attributes
        let attrs = style.to_attrs();
        buffer.set_text(font_system, text, &attrs, Shaping::Advanced, None);

        // Apply truncation if requested
        if truncate == Truncate::End {
            if let Some(width) = max_width {
                // cosmic-text doesn't have built-in ellipsis truncation,
                // so we'll implement it manually in a later phase
                // For now, just wrap
                buffer.set_size(font_system, Some(width), None);
            }
        }

        // Shape the text
        buffer.shape_until_scroll(font_system, false);

        buffer
    }

    /// Clean up stale cache entries (generational eviction)
    fn cleanup_cache(&mut self) {
        self.managed_cache.retain(|_, cached| {
            let age = self.current_frame.saturating_sub(cached.last_used_frame);
            age < self.eviction_threshold
        });
    }

    /// Get cache statistics (for debugging)
    #[allow(dead_code)]
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            entry_count: self.managed_cache.len(),
            current_frame: self.current_frame,
        }
    }
}

impl Default for TextEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics for debugging
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: usize,
    pub current_frame: u64,
}
