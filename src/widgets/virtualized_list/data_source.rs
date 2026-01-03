//! DataSource trait for virtualized lists
//!
//! Inspired by AppKit's NSTableViewDataSource pattern, this trait allows
//! the list widget to query for data without owning it.

use crate::widget::Widget;

/// Data source trait for virtualized lists
///
/// Similar to NSTableViewDataSource, this trait allows the list widget
/// to query for data without owning it. The list will call these methods
/// to determine what to display.
///
/// # Example
///
/// ```rust,ignore
/// struct MyDataSource {
///     items: Vec<String>,
/// }
///
/// impl ListDataSource for MyDataSource {
///     fn item_count(&self) -> usize {
///         self.items.len()
///     }
///
///     fn item_height(&self, _index: usize) -> Option<f64> {
///         Some(40.0)  // Fixed height for all items
///     }
///
///     fn widget_for_item(
///         &mut self,
///         index: usize,
///         reused_widget: Option<Box<dyn Widget>>,
///     ) -> Box<dyn Widget> {
///         let mut widget = reused_widget.unwrap_or_else(|| Box::new(Label::new("")));
///
///         if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
///             label.set_text(&self.items[index]);
///         }
///
///         widget
///     }
/// }
/// ```
pub trait ListDataSource {
    /// Return the total number of items in the list
    fn item_count(&self) -> usize;

    /// Return the height for a specific item
    ///
    /// If None, the list will measure the widget to determine its height.
    /// For performance, returning a fixed value is recommended when possible.
    ///
    /// # Arguments
    /// * `index` - The data item index (0-based)
    ///
    /// # Returns
    /// Some(height) if the height is known, None to measure the widget
    fn item_height(&self, index: usize) -> Option<f64>;

    /// Create or configure a widget for the given item
    ///
    /// The list provides a reused widget if available (may be None for first use).
    /// The data source should update the widget's content to match the item at `index`.
    ///
    /// **Important:** If `reused_widget` is provided, check its type before reusing it!
    /// Use `widget.as_any().downcast_ref::<YourType>()` to verify compatibility.
    ///
    /// # Arguments
    /// * `index` - The data item index (0-based)
    /// * `reused_widget` - A widget to reuse if available (None for first use)
    ///
    /// # Returns
    /// A configured widget for this item. May be the reused widget or a new one.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn widget_for_item(
    ///     &mut self,
    ///     index: usize,
    ///     reused_widget: Option<Box<dyn Widget>>,
    /// ) -> Box<dyn Widget> {
    ///     // Try to reuse the widget if it's the right type
    ///     if let Some(mut widget) = reused_widget {
    ///         if widget.as_any().downcast_ref::<Label>().is_some() {
    ///             // It's a Label, we can reuse it
    ///             if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
    ///                 label.set_text(&self.items[index]);
    ///             }
    ///             return widget;
    ///         }
    ///     }
    ///
    ///     // Create a new widget
    ///     Box::new(Label::new(&self.items[index]))
    /// }
    /// ```
    fn widget_for_item(
        &mut self,
        index: usize,
        reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget>;
}
