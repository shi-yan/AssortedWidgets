//! Text engine with dual-mode caching
//!
//! Provides two APIs:
//! 1. High-level managed API: Transparent LRU caching for simple widgets
//! 2. Low-level manual API: Widget owns TextLayout lifecycle

use super::{TextLayout, TextStyle, TextAlign, Truncate};
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

    /// Text alignment
    alignment: TextAlign,
}

impl TextCacheKey {
    fn new(text: &str, style: &TextStyle, max_width: Option<f32>) -> Self {
        Self {
            text: text.to_string(),
            font_size_bits: style.font_size.to_bits(),
            max_width_bits: max_width.map(|w| w.to_bits()),
            font_family: style.font_family.clone(),
            font_weight: style.font_weight.0,
            alignment: style.alignment,
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

    // Performance tracking
    /// Cache hits this frame
    cache_hits: u64,

    /// Cache misses this frame
    cache_misses: u64,

    /// Total shapes this frame (manual + managed misses)
    shapes_this_frame: u64,
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
            cache_hits: 0,
            cache_misses: 0,
            shapes_this_frame: 0,
        }
    }

    /// Begin a new frame (updates frame counter)
    ///
    /// Call this once per frame from the event loop.
    /// Triggers cache cleanup every N frames and resets frame stats.
    pub fn begin_frame(&mut self) {
        self.current_frame += 1;

        // Reset frame stats
        self.cache_hits = 0;
        self.cache_misses = 0;
        self.shapes_this_frame = 0;

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
        self.shapes_this_frame += 1;  // Track manual API usage
        let buffer = self.shape_text_internal(text, style, max_width, truncate);
        TextLayout::new(buffer, style.alignment, max_width)
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
            self.cache_misses += 1;
            self.shapes_this_frame += 1;

            let buffer = self.shape_text_internal(text, style, max_width, Truncate::None);
            let layout = TextLayout::new(buffer, style.alignment, max_width);

            // Insert into cache
            self.managed_cache.insert(
                key.clone(),
                CachedTextLayout {
                    layout,
                    last_used_frame: current_frame,
                },
            );
        } else {
            // Cache hit
            self.cache_hits += 1;
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

        // Set wrapping mode BEFORE shaping
        if truncate == Truncate::End {
            // Disable wrapping for single-line truncation
            buffer.set_wrap(font_system, cosmic_text::Wrap::None);
        } else {
            // Enable word wrapping for normal multi-line text
            buffer.set_wrap(font_system, cosmic_text::Wrap::Word);
        }

        // Set alignment on all buffer lines
        use cosmic_text::Align as CosmicAlign;
        let cosmic_align = match style.alignment {
            crate::text::TextAlign::Left => CosmicAlign::Left,
            crate::text::TextAlign::Center => CosmicAlign::Center,
            crate::text::TextAlign::Right => CosmicAlign::Right,
        };

        for line in buffer.lines.iter_mut() {
            line.set_align(Some(cosmic_align));
        }

        // Shape the text
        buffer.shape_until_scroll(font_system, false);

        // Apply truncation with ellipsis if requested
        // Note: cosmic-text 0.15 doesn't have built-in ellipsis, so we implement it manually
        if truncate == Truncate::End {
            if let Some(width) = max_width {
                // Check if truncation is needed
                let needs_truncation = buffer.layout_runs().any(|run| run.line_w > width);

                if needs_truncation {
                    // Manual ellipsis truncation via binary search
                    let ellipsis = "…";

                    // Measure ellipsis width
                    let mut ellipsis_buffer = Buffer::new(font_system, metrics);
                    ellipsis_buffer.set_text(font_system, ellipsis, &attrs, Shaping::Advanced, None);
                    ellipsis_buffer.shape_until_scroll(font_system, false);
                    let ellipsis_width = ellipsis_buffer.layout_runs()
                        .next()
                        .map(|run| run.line_w)
                        .unwrap_or(0.0);

                    // Available width for actual text (excluding ellipsis)
                    let available_width = width - ellipsis_width;

                    // Binary search for truncation point
                    let char_count = text.chars().count();
                    let mut left = 0;
                    let mut right = char_count;
                    let mut best_fit = 0;

                    while left <= right {
                        let mid = (left + right) / 2;

                        // Get substring up to mid
                        let truncated: String = text.chars().take(mid).collect();

                        // Measure width
                        let mut test_buffer = Buffer::new(font_system, metrics);
                        test_buffer.set_text(font_system, &truncated, &attrs, Shaping::Advanced, None);
                        test_buffer.shape_until_scroll(font_system, false);
                        let test_width = test_buffer.layout_runs()
                            .map(|run| run.line_w)
                            .max_by(|a, b| a.partial_cmp(b).unwrap())
                            .unwrap_or(0.0);

                        if test_width <= available_width {
                            best_fit = mid;
                            left = mid + 1;
                        } else {
                            if mid == 0 {
                                break;
                            }
                            right = mid - 1;
                        }
                    }

                    // Create truncated text with ellipsis
                    let truncated_text: String = text.chars().take(best_fit).collect();
                    let final_text = format!("{}{}", truncated_text, ellipsis);

                    // Re-shape with final text (wrapping already disabled above)
                    buffer.set_text(font_system, &final_text, &attrs, Shaping::Advanced, None);
                    buffer.set_size(font_system, Some(width), None);
                    buffer.shape_until_scroll(font_system, false);
                }
            }
        }

        buffer
    }

    /// Clean up stale cache entries (generational eviction)
    fn cleanup_cache(&mut self) {
        self.managed_cache.retain(|_, cached| {
            let age = self.current_frame.saturating_sub(cached.last_used_frame);
            age < self.eviction_threshold
        });
    }

    /// Get cache statistics (for performance monitoring)
    pub fn cache_stats(&self) -> CacheStats {
        let total_lookups = self.cache_hits + self.cache_misses;
        let hit_rate = if total_lookups > 0 {
            self.cache_hits as f32 / total_lookups as f32
        } else {
            0.0
        };

        CacheStats {
            entry_count: self.managed_cache.len(),
            current_frame: self.current_frame,
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            shapes_this_frame: self.shapes_this_frame,
            hit_rate,
        }
    }

    /// Print performance stats to console (for debugging)
    pub fn print_stats(&self) {
        let stats = self.cache_stats();
        println!("=== TextEngine Stats (Frame {}) ===", stats.current_frame);
        println!("  Cache entries: {}", stats.entry_count);
        println!("  Cache hits: {}", stats.cache_hits);
        println!("  Cache misses: {}", stats.cache_misses);
        println!("  Hit rate: {:.1}%", stats.hit_rate * 100.0);
        println!("  Shapes this frame: {}", stats.shapes_this_frame);
    }
}

impl Default for TextEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics for performance monitoring
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of entries in managed cache
    pub entry_count: usize,

    /// Current frame number
    pub current_frame: u64,

    /// Cache hits this frame
    pub cache_hits: u64,

    /// Cache misses this frame (new layouts created)
    pub cache_misses: u64,

    /// Total layouts shaped this frame
    pub shapes_this_frame: u64,

    /// Cache hit rate (0.0 - 1.0)
    pub hit_rate: f32,
}
