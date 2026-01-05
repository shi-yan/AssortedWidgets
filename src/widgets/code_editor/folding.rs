// Code folding support
//
// Detects matching brackets and allows folding/unfolding code blocks.

use std::collections::HashSet;

/// A foldable region in the code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldableRegion {
    /// Starting line (0-indexed, inclusive)
    pub start_line: usize,

    /// Ending line (0-indexed, inclusive)
    pub end_line: usize,

    /// Type of fold (for future expansion)
    pub kind: FoldKind,
}

/// Type of foldable region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldKind {
    /// Curly braces { ... }
    Braces,
}

/// Manages foldable regions and their state
pub struct FoldManager {
    /// All detected foldable regions
    regions: Vec<FoldableRegion>,

    /// Set of folded region indices
    folded: HashSet<usize>,
}

impl FoldManager {
    /// Create a new fold manager
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            folded: HashSet::new(),
        }
    }

    /// Detect foldable regions from text
    ///
    /// This scans the text for matching brackets and creates fold regions.
    pub fn detect_regions(&mut self, text: &str) {
        self.regions.clear();

        let mut stack: Vec<(usize, char)> = Vec::new(); // (line_number, bracket_char)

        for (line_idx, line) in text.lines().enumerate() {
            for ch in line.chars() {
                match ch {
                    '{' => {
                        stack.push((line_idx, '{'));
                    }
                    '}' => {
                        if let Some((start_line, '{')) = stack.pop() {
                            // Only create fold if it spans multiple lines
                            if line_idx > start_line {
                                self.regions.push(FoldableRegion {
                                    start_line,
                                    end_line: line_idx,
                                    kind: FoldKind::Braces,
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Sort regions by start line for easier lookup
        self.regions.sort_by_key(|r| r.start_line);
    }

    /// Toggle fold state for a region at the given line
    ///
    /// Returns true if a region was toggled.
    pub fn toggle_fold_at_line(&mut self, line: usize) -> bool {
        // Find a region that starts at this line
        if let Some(region_idx) = self.find_region_starting_at(line) {
            if self.folded.contains(&region_idx) {
                self.folded.remove(&region_idx);
            } else {
                self.folded.insert(region_idx);
            }
            true
        } else {
            false
        }
    }

    /// Check if a line is visible (not inside a folded region)
    pub fn is_line_visible(&self, line: usize) -> bool {
        for &region_idx in &self.folded {
            if let Some(region) = self.regions.get(region_idx) {
                // Lines between start (exclusive) and end (inclusive) are hidden
                if line > region.start_line && line <= region.end_line {
                    return false;
                }
            }
        }
        true
    }

    /// Check if a region is folded
    pub fn is_folded(&self, region_idx: usize) -> bool {
        self.folded.contains(&region_idx)
    }

    /// Find a foldable region starting at the given line
    fn find_region_starting_at(&self, line: usize) -> Option<usize> {
        self.regions
            .iter()
            .position(|r| r.start_line == line)
    }

    /// Find a foldable region at the given line (starting or containing)
    pub fn find_region_at_line(&self, line: usize) -> Option<usize> {
        self.regions
            .iter()
            .position(|r| r.start_line == line || (line > r.start_line && line <= r.end_line))
    }

    /// Get all regions
    pub fn regions(&self) -> &[FoldableRegion] {
        &self.regions
    }

    /// Get the number of regions
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Fold all regions
    pub fn fold_all(&mut self) {
        self.folded = (0..self.regions.len()).collect();
    }

    /// Unfold all regions
    pub fn unfold_all(&mut self) {
        self.folded.clear();
    }

    /// Clear all regions and folded state
    pub fn clear(&mut self) {
        self.regions.clear();
        self.folded.clear();
    }
}

impl Default for FoldManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_simple_braces() {
        let mut manager = FoldManager::new();
        let code = "fn main() {\n    println!(\"hello\");\n}";

        manager.detect_regions(code);

        assert_eq!(manager.region_count(), 1);
        let region = &manager.regions()[0];
        assert_eq!(region.start_line, 0);
        assert_eq!(region.end_line, 2);
        assert_eq!(region.kind, FoldKind::Braces);
    }

    #[test]
    fn test_detect_nested_braces() {
        let mut manager = FoldManager::new();
        let code = "fn main() {\n    if true {\n        code();\n    }\n}";

        manager.detect_regions(code);

        // Should detect 2 regions: outer fn and inner if
        assert_eq!(manager.region_count(), 2);

        // Inner region (if block)
        let inner = &manager.regions()[0];
        assert_eq!(inner.start_line, 1);
        assert_eq!(inner.end_line, 3);

        // Outer region (fn block)
        let outer = &manager.regions()[1];
        assert_eq!(outer.start_line, 0);
        assert_eq!(outer.end_line, 4);
    }

    #[test]
    fn test_toggle_fold() {
        let mut manager = FoldManager::new();
        let code = "fn main() {\n    code();\n}";

        manager.detect_regions(code);

        // Initially not folded
        assert!(!manager.is_folded(0));

        // Toggle fold
        let toggled = manager.toggle_fold_at_line(0);
        assert!(toggled);
        assert!(manager.is_folded(0));

        // Toggle again
        manager.toggle_fold_at_line(0);
        assert!(!manager.is_folded(0));
    }

    #[test]
    fn test_line_visibility() {
        let mut manager = FoldManager::new();
        let code = "fn main() {\n    line1();\n    line2();\n}";

        manager.detect_regions(code);

        // All lines visible initially
        assert!(manager.is_line_visible(0));
        assert!(manager.is_line_visible(1));
        assert!(manager.is_line_visible(2));
        assert!(manager.is_line_visible(3));

        // Fold the region
        manager.toggle_fold_at_line(0);

        // Start line still visible, interior lines hidden
        assert!(manager.is_line_visible(0));
        assert!(!manager.is_line_visible(1));
        assert!(!manager.is_line_visible(2));
        assert!(!manager.is_line_visible(3)); // End line is hidden
    }

    #[test]
    fn test_single_line_braces_ignored() {
        let mut manager = FoldManager::new();
        let code = "fn foo() {}"; // Single line - should not create fold

        manager.detect_regions(code);

        assert_eq!(manager.region_count(), 0);
    }

    #[test]
    fn test_fold_all_unfold_all() {
        let mut manager = FoldManager::new();
        let code = "fn main() {\n    if true {\n        code();\n    }\n}";

        manager.detect_regions(code);

        // Fold all
        manager.fold_all();
        assert_eq!(manager.folded.len(), 2);

        // Unfold all
        manager.unfold_all();
        assert_eq!(manager.folded.len(), 0);
    }

    #[test]
    fn test_mismatched_brackets() {
        let mut manager = FoldManager::new();
        let code = "fn main() {\n    code();\n"; // Missing closing brace

        manager.detect_regions(code);

        // Should handle gracefully (no crash)
        assert_eq!(manager.region_count(), 0);
    }
}
