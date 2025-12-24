//! Focus management system for keyboard input and IME
//!
//! The FocusManager tracks which widget has keyboard focus and manages
//! Tab/Shift+Tab navigation through focusable elements.

use crate::widget_manager::WidgetManager;
use crate::types::{Rect, WidgetId};

/// Manages keyboard focus and focusable widget navigation
///
/// # Focus Model
/// - Only one widget can have focus at a time
/// - Tab moves to next focusable widget
/// - Shift+Tab moves to previous focusable widget
/// - Clicking a focusable widget gives it focus
/// - Focus can be cleared (None) if no widgets are focusable
///
/// # IME Support
/// The focused widget's ime_cursor_rect() is used to position the
/// Input Method Editor (IME) composition window for complex text input
/// (e.g., Chinese, Japanese, Korean).
pub struct FocusManager {
    /// Currently focused widget (receives keyboard events)
    focused_id: Option<WidgetId>,

    /// List of focusable widgets (for Tab navigation)
    /// Sorted in traversal order (typically tree order or custom tab order)
    focusable_widgets: Vec<WidgetId>,

    /// Current focus index in focusable_widgets
    /// Valid only when focused_id is Some
    focus_index: usize,
}

impl FocusManager {
    /// Create a new focus manager with no focused widget
    pub fn new() -> Self {
        FocusManager {
            focused_id: None,
            focusable_widgets: Vec::new(),
            focus_index: 0,
        }
    }

    /// Get the currently focused widget
    pub fn focused_id(&self) -> Option<WidgetId> {
        self.focused_id
    }

    /// Set focus to a specific widget
    ///
    /// If the widget is not in the focusable list, this does nothing.
    /// To clear focus, pass `None`.
    pub fn set_focus(&mut self, widget_id: Option<WidgetId>) {
        self.focused_id = widget_id;

        // Update focus_index to match the focused widget
        if let Some(id) = widget_id {
            self.focus_index = self
                .focusable_widgets
                .iter()
                .position(|w| *w == id)
                .unwrap_or(0);
        }
    }

    /// Move focus to the next focusable widget (Tab key)
    ///
    /// Wraps around to the first widget after the last.
    /// Returns the newly focused widget ID, or None if no focusable widgets.
    pub fn focus_next(&mut self) -> Option<WidgetId> {
        if self.focusable_widgets.is_empty() {
            self.focused_id = None;
            return None;
        }

        self.focus_index = (self.focus_index + 1) % self.focusable_widgets.len();
        self.focused_id = Some(self.focusable_widgets[self.focus_index]);
        self.focused_id
    }

    /// Move focus to the previous focusable widget (Shift+Tab)
    ///
    /// Wraps around to the last widget before the first.
    /// Returns the newly focused widget ID, or None if no focusable widgets.
    pub fn focus_previous(&mut self) -> Option<WidgetId> {
        if self.focusable_widgets.is_empty() {
            self.focused_id = None;
            return None;
        }

        self.focus_index = if self.focus_index == 0 {
            self.focusable_widgets.len() - 1
        } else {
            self.focus_index - 1
        };

        self.focused_id = Some(self.focusable_widgets[self.focus_index]);
        self.focused_id
    }

    /// Get IME cursor position from the focused widget
    ///
    /// Returns the rectangle where the IME composition window should be
    /// positioned, or None if no widget is focused or the focused widget
    /// doesn't support IME.
    pub fn get_ime_cursor_rect(&self, widget_manager: &WidgetManager) -> Option<Rect> {
        let focused_id = self.focused_id?;

        widget_manager
            .get(focused_id)
            .and_then(|element| element.ime_cursor_rect())
    }

    /// Rebuild the focusable widget list from the widget tree
    ///
    /// This should be called when:
    /// - Widgets are added/removed
    /// - A widget's is_focusable() state changes
    /// - The widget tree structure changes
    ///
    /// The order of focusable widgets determines Tab navigation order.
    pub fn rebuild(&mut self, widget_manager: &WidgetManager) {
        self.focusable_widgets.clear();

        for widget_id in widget_manager.widget_ids() {
            if let Some(element) = widget_manager.get(widget_id) {
                if element.is_focusable() {
                    self.focusable_widgets.push(widget_id);
                }
            }
        }

        // If the currently focused widget is no longer focusable, clear focus
        if let Some(focused_id) = self.focused_id {
            if !self.focusable_widgets.contains(&focused_id) {
                self.focused_id = None;
                self.focus_index = 0;
            }
        }
    }

    /// Check if a widget has focus
    pub fn has_focus(&self, widget_id: WidgetId) -> bool {
        self.focused_id == Some(widget_id)
    }

    /// Get the number of focusable widgets
    pub fn focusable_count(&self) -> usize {
        self.focusable_widgets.len()
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}
