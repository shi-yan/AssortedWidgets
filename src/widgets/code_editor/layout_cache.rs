// LRU cache for text layout buffers

use std::num::NonZeroUsize;
use std::ops::Range;
use lru::LruCache;
use cosmic_text::Buffer;

use super::config::{LRU_CACHE_MAX_SIZE, LRU_CACHE_MAX_ENTRIES};

/// Key for identifying a cached text layout
///
/// A layout is uniquely identified by:
/// - Line range (logical line indices)
/// - Wrap width (for soft-wrap mode)
/// - Font size
/// - Font family
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct LayoutKey {
    /// Range of logical lines (e.g., 100..150 for lines 100-149)
    pub line_range: Range<usize>,

    /// Wrap width in pixels (0 = no wrap)
    pub wrap_width: u32,

    /// Font size as u32 (multiply by 10, e.g., 14.0 → 140)
    pub font_size_tenths: u32,

    /// Font family name
    pub font_family: String,
}

impl LayoutKey {
    /// Create a new layout key
    pub fn new(
        line_range: Range<usize>,
        wrap_width: f32,
        font_size: f32,
        font_family: String,
    ) -> Self {
        Self {
            line_range,
            wrap_width: wrap_width as u32,
            font_size_tenths: (font_size * 10.0) as u32,
            font_family,
        }
    }
}

/// Cached text layout entry
pub struct CachedLayout {
    /// The cosmic-text buffer with shaped text
    pub buffer: Buffer,

    /// Approximate memory size in bytes (for LRU eviction tracking)
    pub byte_size: usize,
}

impl CachedLayout {
    /// Estimate memory usage of a buffer
    ///
    /// This is an approximation based on:
    /// - Number of lines
    /// - Average characters per line
    /// - Glyph data structure size
    fn estimate_size(buffer: &Buffer) -> usize {
        let num_lines = buffer.lines.len();

        // Estimate: each line has ~80 chars, each char has ~100 bytes of layout data
        // (glyph info, positions, metrics, etc.)
        const BYTES_PER_CHAR: usize = 100;
        const AVG_CHARS_PER_LINE: usize = 80;

        num_lines * AVG_CHARS_PER_LINE * BYTES_PER_CHAR
    }

    /// Create a new cached layout
    pub fn new(buffer: Buffer) -> Self {
        let byte_size = Self::estimate_size(&buffer);
        Self { buffer, byte_size }
    }
}

/// LRU cache for text layouts
///
/// This cache stores cosmic-text buffers for ranges of lines to enable efficient
/// virtualized rendering of large files. Only visible line ranges are kept in cache.
pub struct LayoutCache {
    /// LRU cache mapping layout keys to buffers
    cache: LruCache<LayoutKey, CachedLayout>,

    /// Current total cache size in bytes
    current_size_bytes: usize,

    /// Maximum cache size in bytes (from config)
    max_size_bytes: usize,
}

impl LayoutCache {
    /// Create a new layout cache with default limits
    pub fn new() -> Self {
        Self::with_limits(LRU_CACHE_MAX_SIZE, LRU_CACHE_MAX_ENTRIES)
    }

    /// Create a layout cache with custom limits
    ///
    /// # Arguments
    /// * `max_size_bytes` - Maximum total memory usage
    /// * `max_entries` - Maximum number of cached layouts
    pub fn with_limits(max_size_bytes: usize, max_entries: usize) -> Self {
        let capacity = NonZeroUsize::new(max_entries).unwrap();
        Self {
            cache: LruCache::new(capacity),
            current_size_bytes: 0,
            max_size_bytes,
        }
    }

    /// Get a cached layout
    ///
    /// Returns None if not cached. Marks the entry as recently used.
    pub fn get(&mut self, key: &LayoutKey) -> Option<&Buffer> {
        self.cache.get(key).map(|layout| &layout.buffer)
    }

    /// Insert a layout into the cache
    ///
    /// Evicts old entries if the cache is full.
    pub fn insert(&mut self, key: LayoutKey, buffer: Buffer) {
        let layout = CachedLayout::new(buffer);
        let new_size = layout.byte_size;

        // Evict entries until we have room for the new entry
        while self.current_size_bytes + new_size > self.max_size_bytes && !self.cache.is_empty() {
            if let Some((_, evicted)) = self.cache.pop_lru() {
                self.current_size_bytes = self.current_size_bytes.saturating_sub(evicted.byte_size);
            }
        }

        // Insert the new entry
        if let Some((_old_key, old_layout)) = self.cache.push(key, layout) {
            // If we're replacing an existing entry, subtract its size
            self.current_size_bytes = self.current_size_bytes.saturating_sub(old_layout.byte_size);
        }

        self.current_size_bytes += new_size;
    }

    /// Remove a specific entry from the cache
    pub fn remove(&mut self, key: &LayoutKey) -> Option<Buffer> {
        self.cache.pop(key).map(|layout| {
            self.current_size_bytes = self.current_size_bytes.saturating_sub(layout.byte_size);
            layout.buffer
        })
    }

    /// Clear all cached layouts
    pub fn clear(&mut self) {
        self.cache.clear();
        self.current_size_bytes = 0;
    }

    /// Invalidate layouts that overlap with a range of lines
    ///
    /// Call this when text content changes to invalidate affected layouts.
    pub fn invalidate_range(&mut self, changed_lines: Range<usize>) {
        // Collect keys to remove (can't modify while iterating)
        let keys_to_remove: Vec<LayoutKey> = self.cache
            .iter()
            .filter_map(|(key, _)| {
                // Check if the key's line range overlaps with changed_lines
                if key.line_range.start < changed_lines.end
                    && key.line_range.end > changed_lines.start
                {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();

        // Remove all overlapping entries
        for key in keys_to_remove {
            self.remove(&key);
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entry_count: self.cache.len(),
            total_bytes: self.current_size_bytes,
            max_bytes: self.max_size_bytes,
        }
    }

    /// Get the number of cached entries
    #[inline]
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if the cache is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl Default for LayoutCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics for debugging/monitoring
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cached layout entries
    pub entry_count: usize,

    /// Total memory used by cache (bytes)
    pub total_bytes: usize,

    /// Maximum allowed memory (bytes)
    pub max_bytes: usize,
}

impl CacheStats {
    /// Get cache utilization as a percentage (0.0 to 100.0)
    pub fn utilization_percent(&self) -> f64 {
        if self.max_bytes == 0 {
            0.0
        } else {
            (self.total_bytes as f64 / self.max_bytes as f64) * 100.0
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dummy_buffer() -> Buffer {
        // Create a minimal buffer for testing
        // Note: In real usage, this would be created via cosmic_text::FontSystem
        Buffer::new_empty(cosmic_text::Metrics::new(14.0, 20.0))
    }

    #[test]
    fn test_cache_insert_and_get() {
        let mut cache = LayoutCache::with_limits(1_000_000, 10);

        let key = LayoutKey::new(0..10, 500.0, 14.0, "Menlo".to_string());
        let buffer = create_dummy_buffer();

        cache.insert(key.clone(), buffer);

        assert_eq!(cache.len(), 1);
        assert!(cache.get(&key).is_some());
    }

    #[test]
    fn test_cache_eviction() {
        // Create a cache with very small size limit
        let mut cache = LayoutCache::with_limits(1000, 100);

        // Insert multiple entries until eviction happens
        for i in 0..10 {
            let key = LayoutKey::new(i * 10..(i + 1) * 10, 500.0, 14.0, "Menlo".to_string());
            cache.insert(key, create_dummy_buffer());
        }

        // Cache should have evicted some entries
        assert!(cache.len() < 10);
    }

    #[test]
    fn test_invalidate_range() {
        let mut cache = LayoutCache::with_limits(1_000_000, 10);

        // Insert layouts for different line ranges
        cache.insert(
            LayoutKey::new(0..50, 500.0, 14.0, "Menlo".to_string()),
            create_dummy_buffer(),
        );
        cache.insert(
            LayoutKey::new(50..100, 500.0, 14.0, "Menlo".to_string()),
            create_dummy_buffer(),
        );
        cache.insert(
            LayoutKey::new(100..150, 500.0, 14.0, "Menlo".to_string()),
            create_dummy_buffer(),
        );

        assert_eq!(cache.len(), 3);

        // Invalidate lines 40-60 (should remove first two entries)
        cache.invalidate_range(40..60);

        assert_eq!(cache.len(), 1);

        // The remaining entry should be 100..150
        let key = LayoutKey::new(100..150, 500.0, 14.0, "Menlo".to_string());
        assert!(cache.get(&key).is_some());
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = LayoutCache::with_limits(1_000_000, 10);

        cache.insert(
            LayoutKey::new(0..50, 500.0, 14.0, "Menlo".to_string()),
            create_dummy_buffer(),
        );

        assert_eq!(cache.len(), 1);
        assert!(cache.current_size_bytes > 0);

        cache.clear();

        assert_eq!(cache.len(), 0);
        assert_eq!(cache.current_size_bytes, 0);
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = LayoutCache::with_limits(1_000_000, 10);

        cache.insert(
            LayoutKey::new(0..50, 500.0, 14.0, "Menlo".to_string()),
            create_dummy_buffer(),
        );

        let stats = cache.stats();
        assert_eq!(stats.entry_count, 1);
        assert!(stats.total_bytes > 0);
        assert_eq!(stats.max_bytes, 1_000_000);
        assert!(stats.utilization_percent() > 0.0);
        assert!(stats.utilization_percent() < 100.0);
    }
}
