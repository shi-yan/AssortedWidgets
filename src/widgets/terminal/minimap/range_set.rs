/// Efficient tracking of dirty line ranges
///
/// Uses a sorted list of non-overlapping ranges to track which lines
/// need rasterization. Supports efficient insertion, merging, and querying.
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct RangeSet {
    /// Sorted, non-overlapping ranges
    ranges: Vec<Range<usize>>,
}

impl RangeSet {
    /// Create an empty range set
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    /// Add a single line to the dirty set
    pub fn add_line(&mut self, line: usize) {
        self.add_range(line..line + 1);
    }

    /// Add a range of lines to the dirty set
    pub fn add_range(&mut self, range: Range<usize>) {
        if range.is_empty() {
            return;
        }

        // Find where to insert/merge this range
        let mut new_range = range;
        let mut to_remove = Vec::new();

        for (i, existing) in self.ranges.iter().enumerate() {
            // Check if ranges can be merged
            // Ranges overlap if: new_start <= existing_end AND existing_start <= new_end
            if new_range.start <= existing.end && existing.start <= new_range.end {
                // Merge ranges
                new_range.start = new_range.start.min(existing.start);
                new_range.end = new_range.end.max(existing.end);
                to_remove.push(i);
            }
        }

        // Remove merged ranges (in reverse order to maintain indices)
        for i in to_remove.iter().rev() {
            self.ranges.remove(*i);
        }

        // Insert new merged range in sorted position
        let insert_pos = self.ranges.binary_search_by_key(&new_range.start, |r| r.start)
            .unwrap_or_else(|pos| pos);
        self.ranges.insert(insert_pos, new_range);
    }

    /// Check if a line is dirty
    pub fn contains(&self, line: usize) -> bool {
        self.ranges.iter().any(|r| r.contains(&line))
    }

    /// Check if any lines in the range are dirty
    pub fn intersects(&self, range: &Range<usize>) -> bool {
        self.ranges.iter().any(|r| {
            r.start < range.end && range.start < r.end
        })
    }

    /// Get all ranges that intersect with the given range
    pub fn intersecting_ranges(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        self.ranges.iter()
            .filter(|r| r.start < range.end && range.start < r.end)
            .cloned()
            .collect()
    }

    /// Clear all dirty ranges
    pub fn clear(&mut self) {
        self.ranges.clear();
    }

    /// Remove a specific range from the dirty set
    pub fn remove_range(&mut self, range: &Range<usize>) {
        if range.is_empty() {
            return;
        }

        let mut new_ranges = Vec::new();

        for existing in &self.ranges {
            // If no overlap, keep as-is
            if existing.end <= range.start || existing.start >= range.end {
                new_ranges.push(existing.clone());
                continue;
            }

            // If existing range is completely contained, skip it
            if existing.start >= range.start && existing.end <= range.end {
                continue;
            }

            // If partial overlap, split the range
            if existing.start < range.start {
                // Keep the part before the removed range
                new_ranges.push(existing.start..range.start);
            }
            if existing.end > range.end {
                // Keep the part after the removed range
                new_ranges.push(range.end..existing.end);
            }
        }

        self.ranges = new_ranges;
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Get total number of dirty lines
    pub fn len(&self) -> usize {
        self.ranges.iter().map(|r| r.len()).sum()
    }

    /// Get all ranges
    pub fn ranges(&self) -> &[Range<usize>] {
        &self.ranges
    }
}

impl Default for RangeSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_single_line() {
        let mut set = RangeSet::new();
        set.add_line(5);
        assert!(set.contains(5));
        assert!(!set.contains(4));
        assert!(!set.contains(6));
    }

    #[test]
    fn test_add_range() {
        let mut set = RangeSet::new();
        set.add_range(10..20);
        assert!(set.contains(10));
        assert!(set.contains(15));
        assert!(set.contains(19));
        assert!(!set.contains(9));
        assert!(!set.contains(20));
    }

    #[test]
    fn test_merge_adjacent_ranges() {
        let mut set = RangeSet::new();
        set.add_range(10..20);
        set.add_range(20..30);
        assert_eq!(set.ranges().len(), 1);
        assert_eq!(set.ranges()[0], 10..30);
    }

    #[test]
    fn test_merge_overlapping_ranges() {
        let mut set = RangeSet::new();
        set.add_range(10..20);
        set.add_range(15..25);
        assert_eq!(set.ranges().len(), 1);
        assert_eq!(set.ranges()[0], 10..25);
    }

    #[test]
    fn test_merge_multiple_ranges() {
        let mut set = RangeSet::new();
        set.add_range(10..15);
        set.add_range(20..25);
        set.add_range(30..35);
        // Add range that merges all three
        set.add_range(12..32);
        assert_eq!(set.ranges().len(), 1);
        assert_eq!(set.ranges()[0], 10..35);
    }

    #[test]
    fn test_intersects() {
        let mut set = RangeSet::new();
        set.add_range(10..20);
        set.add_range(30..40);

        assert!(set.intersects(&(15..25)));
        assert!(set.intersects(&(35..45)));
        assert!(!set.intersects(&(20..30)));
        assert!(!set.intersects(&(40..50)));
    }

    #[test]
    fn test_remove_range() {
        let mut set = RangeSet::new();
        set.add_range(10..50);

        // Remove middle section
        set.remove_range(&(20..30));
        assert_eq!(set.ranges().len(), 2);
        assert!(set.contains(15));
        assert!(!set.contains(25));
        assert!(set.contains(35));
    }

    #[test]
    fn test_clear() {
        let mut set = RangeSet::new();
        set.add_range(10..20);
        set.add_range(30..40);
        set.clear();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_len() {
        let mut set = RangeSet::new();
        set.add_range(10..20);  // 10 lines
        set.add_range(30..35);  // 5 lines
        assert_eq!(set.len(), 15);
    }
}
