//! Dirty tracking for widgets
//!
//! Hierarchical dirty level system where higher levels imply lower levels:
//! - Clean (0): No changes
//! - Visual (1): Visual changes only (color, opacity) → needs repaint
//! - Layout (2): Size/content changes → needs layout + repaint

/// Dirty level for widgets
///
/// Hierarchical system where:
/// - `Layout` implies `Visual` (can't change layout without repainting)
/// - `Visual` doesn't imply `Layout` (can change color without re-layout)
/// - `Clean` means no changes needed
///
/// # Examples
/// ```
/// use assorted_widgets::types::DirtyLevel;
///
/// let level = DirtyLevel::Visual;
/// assert!(level >= DirtyLevel::Visual);  // Needs repaint
/// assert!(level < DirtyLevel::Layout);   // Doesn't need layout
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum DirtyLevel {
    /// No changes - widget is clean
    Clean = 0,

    /// Visual changes only (color, opacity, transforms)
    /// Needs repaint but NOT layout recalculation
    Visual = 1,

    /// Layout-affecting changes (size, content, position)
    /// Needs BOTH layout recalculation AND repaint
    Layout = 2,
}

impl DirtyLevel {
    /// Check if widget needs repainting
    #[inline]
    pub fn needs_repaint(self) -> bool {
        self >= DirtyLevel::Visual
    }

    /// Check if widget needs layout recalculation
    #[inline]
    pub fn needs_layout(self) -> bool {
        self >= DirtyLevel::Layout
    }

    /// Merge two dirty levels (take the higher/dirtier one)
    #[inline]
    pub fn merge(self, other: DirtyLevel) -> DirtyLevel {
        if self > other { self } else { other }
    }
}

impl Default for DirtyLevel {
    fn default() -> Self {
        DirtyLevel::Clean
    }
}

impl From<bool> for DirtyLevel {
    /// Convert old-style bool dirty flag to DirtyLevel
    /// `true` → Visual, `false` → Clean
    fn from(dirty: bool) -> Self {
        if dirty {
            DirtyLevel::Visual
        } else {
            DirtyLevel::Clean
        }
    }
}

impl From<DirtyLevel> for bool {
    /// Convert to bool for backward compatibility
    /// Any dirty level → true
    fn from(level: DirtyLevel) -> bool {
        level != DirtyLevel::Clean
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy() {
        assert!(DirtyLevel::Layout > DirtyLevel::Visual);
        assert!(DirtyLevel::Visual > DirtyLevel::Clean);
        assert!(DirtyLevel::Layout > DirtyLevel::Clean);
    }

    #[test]
    fn test_needs_repaint() {
        assert!(!DirtyLevel::Clean.needs_repaint());
        assert!(DirtyLevel::Visual.needs_repaint());
        assert!(DirtyLevel::Layout.needs_repaint());
    }

    #[test]
    fn test_needs_layout() {
        assert!(!DirtyLevel::Clean.needs_layout());
        assert!(!DirtyLevel::Visual.needs_layout());
        assert!(DirtyLevel::Layout.needs_layout());
    }

    #[test]
    fn test_merge() {
        assert_eq!(
            DirtyLevel::Visual.merge(DirtyLevel::Clean),
            DirtyLevel::Visual
        );
        assert_eq!(
            DirtyLevel::Layout.merge(DirtyLevel::Visual),
            DirtyLevel::Layout
        );
        assert_eq!(
            DirtyLevel::Clean.merge(DirtyLevel::Layout),
            DirtyLevel::Layout
        );
    }
}
