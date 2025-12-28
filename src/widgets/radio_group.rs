//! Radio Group widget - multiple options with single selection
//!
//! Features:
//! - Multiple radio button options
//! - Single selection (radio button behavior)
//! - Vertical layout
//! - Per-state styling based on selection
//! - Selection change callbacks
//! - Signal/slot integration via deferred commands
//!
//! # Example
//! ```rust,ignore
//! let radio = RadioGroup::new()
//!     .add_item("Option A")
//!     .add_item("Option B")
//!     .add_item("Option C")
//!     .selected(0)
//!     .on_selection_change(|index| println!("Selected: {}", index));
//! ```

use std::any::Any;
use std::cell::RefCell;

use crate::event::handlers::{KeyboardHandler, MouseHandler};
use crate::event::input::{EventResponse, InputEventEnum, KeyEvent, MouseEvent};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::primitives::Color;
use crate::paint::PaintContext;
use crate::text::{TextAlign, TextEngine, TextStyle, Truncate};
use crate::types::{CursorType, DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;

/// Individual radio button item within a RadioGroup
#[derive(Clone, Debug)]
pub struct RadioGroupItem {
    pub label: String,
    pub enabled: bool,
}

impl RadioGroupItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            enabled: true,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Style configuration for radio button in a specific state
#[derive(Clone, Debug)]
pub struct RadioStyle {
    pub icon_color: Color,
    pub label_color: Color,
}

impl RadioStyle {
    pub fn new(icon_color: Color, label_color: Color) -> Self {
        Self {
            icon_color,
            label_color,
        }
    }
}

/// Radio Group widget - multiple options with single selection
pub struct RadioGroup {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Items
    items: Vec<RadioGroupItem>,

    // Styling
    font_size: f32,
    font_family: Option<String>,
    item_spacing: f32,
    icon_size: f32,
    gap: f32,

    // Per-state styles
    normal_style: RadioStyle,
    hovered_style: RadioStyle,
    selected_style: RadioStyle,
    disabled_style: RadioStyle,

    // State
    selected_index: Option<usize>,
    hovered_index: Option<usize>,
    pressed_index: Option<usize>,

    // Callbacks
    on_selection_change: Option<Box<dyn FnMut(usize) + 'static>>,

    // Cached item bounds for hit testing
    item_bounds: RefCell<Vec<Rect>>,

    // Deferred commands
    pending_commands: Vec<DeferredCommand>,
}

impl RadioGroup {
    /// Create a new empty radio group
    pub fn new() -> Self {
        let (normal, hovered, selected, disabled) = Self::default_styles();

        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            items: Vec::new(),
            font_size: 16.0,
            font_family: None,
            item_spacing: 16.0,
            icon_size: 20.0,
            gap: 8.0,
            normal_style: normal,
            hovered_style: hovered,
            selected_style: selected,
            disabled_style: disabled,
            selected_index: None,
            hovered_index: None,
            pressed_index: None,
            on_selection_change: None,
            item_bounds: RefCell::new(Vec::new()),
            pending_commands: Vec::new(),
        }
    }

    // ========================================================================
    // Default Styles
    // ========================================================================

    fn default_styles() -> (RadioStyle, RadioStyle, RadioStyle, RadioStyle) {
        let normal = RadioStyle::new(
            Color::rgb(0.5, 0.5, 0.5),     // Gray icon
            Color::rgb(0.9, 0.9, 0.9),     // Light gray label
        );

        let hovered = RadioStyle::new(
            Color::rgb(0.6, 0.6, 0.6),     // Lighter gray icon
            Color::rgb(1.0, 1.0, 1.0),     // White label
        );

        let selected = RadioStyle::new(
            Color::rgb(0.2, 0.4, 0.8),     // Blue icon
            Color::rgb(1.0, 1.0, 1.0),     // White label
        );

        let disabled = RadioStyle::new(
            Color::rgb(0.3, 0.3, 0.3),     // Dark gray icon
            Color::rgb(0.5, 0.5, 0.5),     // Gray label
        );

        (normal, hovered, selected, disabled)
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Add an item to the radio group
    pub fn add_item(mut self, label: impl Into<String>) -> Self {
        self.items.push(RadioGroupItem::new(label));
        self
    }

    /// Add a radio group item
    pub fn add_radio_item(mut self, item: RadioGroupItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add a disabled item
    pub fn add_disabled_item(mut self, label: impl Into<String>) -> Self {
        self.items.push(RadioGroupItem::new(label).disabled());
        self
    }

    /// Set the selected item index
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.selected_index = Some(index);
        }
        self
    }

    /// Set item spacing (vertical gap between items)
    pub fn item_spacing(mut self, spacing: f32) -> Self {
        self.item_spacing = spacing;
        self
    }

    /// Set icon size
    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    /// Set gap between icon and label
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set font family
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font_family = Some(font.into());
        self
    }

    /// Set selection change callback
    pub fn on_selection_change<F>(mut self, callback: F) -> Self
    where
        F: FnMut(usize) + 'static,
    {
        self.on_selection_change = Some(Box::new(callback));
        self
    }

    /// Set layout style
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    /// Set normal state style
    pub fn normal_style(mut self, style: RadioStyle) -> Self {
        self.normal_style = style;
        self
    }

    /// Set hovered state style
    pub fn hovered_style(mut self, style: RadioStyle) -> Self {
        self.hovered_style = style;
        self
    }

    /// Set selected state style
    pub fn selected_style(mut self, style: RadioStyle) -> Self {
        self.selected_style = style;
        self
    }

    /// Set disabled state style
    pub fn disabled_style(mut self, style: RadioStyle) -> Self {
        self.disabled_style = style;
        self
    }

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Get the currently selected item index
    pub fn get_selected(&self) -> Option<usize> {
        self.selected_index
    }

    /// Set the selected item index at runtime
    pub fn set_selected(&mut self, index: Option<usize>) {
        if index != self.selected_index {
            if let Some(idx) = index {
                if idx < self.items.len() && self.items[idx].enabled {
                    self.selected_index = index;
                    self.dirty = true;
                    self.notify_selection_changed(idx);
                }
            } else {
                self.selected_index = None;
                self.dirty = true;
            }
        }
    }

    /// Enable/disable a specific item
    pub fn set_item_enabled(&mut self, index: usize, enabled: bool) {
        if index < self.items.len() {
            self.items[index].enabled = enabled;

            // Deselect if currently selected and being disabled
            if !enabled && self.selected_index == Some(index) {
                self.selected_index = None;
            }

            self.dirty = true;
        }
    }

    // ========================================================================
    // State Management
    // ========================================================================

    /// Get the appropriate style for an item at the given index
    fn get_item_style(&self, index: usize) -> &RadioStyle {
        if !self.items[index].enabled {
            &self.disabled_style
        } else if self.selected_index == Some(index) {
            &self.selected_style
        } else if self.hovered_index == Some(index) {
            &self.hovered_style
        } else {
            &self.normal_style
        }
    }

    /// Calculate bounds for each item
    fn calculate_item_bounds(&self) -> Vec<Rect> {
        let mut bounds = Vec::new();
        let count = self.items.len();

        if count == 0 {
            return bounds;
        }

        let mut y = self.bounds.origin.y;

        // Calculate height for each item (icon size is the minimum)
        let item_height = self.icon_size.max(self.font_size);

        for _i in 0..count {
            bounds.push(Rect::new(
                Point::new(self.bounds.origin.x, y),
                Size::new(self.bounds.size.width, item_height as f64),
            ));

            y += item_height as f64 + self.item_spacing as f64;
        }

        bounds
    }

    /// Hit test to find which item contains a point
    fn hit_test(&self, point: Point) -> Option<usize> {
        let bounds = self.item_bounds.borrow();
        for (i, rect) in bounds.iter().enumerate() {
            if rect.contains(point) {
                return Some(i);
            }
        }
        None
    }

    /// Render a single radio item
    fn render_item(
        &self,
        ctx: &mut PaintContext,
        index: usize,
        bounds: Rect,
    ) {
        let item = &self.items[index];
        let style = self.get_item_style(index);

        // Determine icon ID based on selection
        let icon_id = if self.selected_index == Some(index) {
            "radio_button_checked"
        } else {
            "radio_button_unchecked"
        };

        // Calculate icon position (vertically centered within item bounds)
        let icon_y = bounds.origin.y + (bounds.size.height - self.icon_size as f64) / 2.0;
        let icon_pos = Point::new(bounds.origin.x, icon_y);

        // Draw radio button icon
        ctx.draw_icon(icon_id, icon_pos, self.icon_size, style.icon_color);

        // Draw label
        let mut text_style = TextStyle::new()
            .size(self.font_size)
            .color(style.label_color)
            .align(TextAlign::Left);

        if let Some(ref font_family) = self.font_family {
            text_style = text_style.family(font_family.clone());
        }

        // Calculate label position (after icon + gap, vertically centered)
        let label_x = bounds.origin.x + self.icon_size as f64 + self.gap as f64;
        let label_y = bounds.origin.y + (bounds.size.height - self.font_size as f64) / 2.0;
        let label_pos = Point::new(label_x, label_y);

        ctx.draw_text(&item.label, &text_style, label_pos, None);
    }

    /// Notify listeners that the selection changed
    fn notify_selection_changed(&mut self, index: usize) {
        // Call local callback
        if let Some(ref mut callback) = self.on_selection_change {
            callback(index);
        }

        // Queue deferred command for signal/slot system
        self.pending_commands.push(DeferredCommand {
            target: self.id,
            message: GuiMessage::Custom {
                source: self.id,
                signal_type: "selection_changed".to_string(),
                data: Box::new(index),
            },
        });
    }

    // ========================================================================
    // Layout Measurement
    // ========================================================================

    /// Measure radio group for layout system
    pub fn measure_with_engine(
        &self,
        engine: &mut TextEngine,
        _known_dimensions: taffy::Size<Option<f32>>,
    ) -> Size {
        if self.items.is_empty() {
            return Size::new(0.0, 0.0);
        }

        let mut text_style = TextStyle::new()
            .size(self.font_size)
            .align(TextAlign::Left);

        if let Some(ref font_family) = self.font_family {
            text_style = text_style.family(font_family.clone());
        }

        // Measure all item labels
        let mut max_width = 0.0_f64;
        let item_height = self.icon_size.max(self.font_size);

        for item in &self.items {
            let layout = engine.create_layout_with_wrap(
                &item.label,
                &text_style,
                None,
                Truncate::End,
                cosmic_text::Wrap::Word,
            );

            let text_size = layout.size();
            let item_width = self.icon_size as f64 + self.gap as f64 + text_size.width;
            max_width = max_width.max(item_width);
        }

        // Calculate total height
        let total_height = (self.items.len() as f32 * item_height
            + (self.items.len() - 1) as f32 * self.item_spacing) as f64;

        Size::new(max_width, total_height)
    }
}

impl Default for RadioGroup {
    fn default() -> Self {
        Self::new()
    }
}

// ========================================================================
// Event Handlers
// ========================================================================

impl MouseHandler for RadioGroup {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        if let Some(index) = self.hit_test(event.position) {
            if self.items[index].enabled {
                self.pressed_index = Some(index);
                self.dirty = true;
                return EventResponse::Handled;
            }
        }
        EventResponse::Ignored
    }

    fn on_mouse_up(&mut self, event: &mut MouseEvent) -> EventResponse {
        if let Some(pressed_idx) = self.pressed_index {
            if let Some(index) = self.hit_test(event.position) {
                if index == pressed_idx && self.items[index].enabled {
                    // Click completed on the same item
                    self.set_selected(Some(index));
                }
            }
            self.pressed_index = None;
            self.dirty = true;
            return EventResponse::Handled;
        }
        EventResponse::Ignored
    }

    fn on_mouse_move(&mut self, event: &mut MouseEvent) -> EventResponse {
        let new_hovered = self.hit_test(event.position);
        if new_hovered != self.hovered_index {
            self.hovered_index = new_hovered;
            self.dirty = true;
            return EventResponse::Handled;
        }
        EventResponse::PassThrough
    }

    fn on_mouse_enter(&mut self, event: &mut MouseEvent) -> EventResponse {
        self.hovered_index = self.hit_test(event.position);
        self.dirty = true;
        EventResponse::Handled
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        self.hovered_index = None;
        self.pressed_index = None;
        self.dirty = true;
        EventResponse::Handled
    }
}

impl KeyboardHandler for RadioGroup {
    fn on_key_down(&mut self, _event: &mut KeyEvent) -> EventResponse {
        // Could add arrow key navigation here in the future
        EventResponse::Ignored
    }

    fn on_key_up(&mut self, _event: &mut KeyEvent) -> EventResponse {
        EventResponse::Ignored
    }
}

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for RadioGroup {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        if self.bounds != bounds {
            self.bounds = bounds;
            self.dirty = true;
        }
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        if self.items.is_empty() {
            return;
        }

        // Calculate item bounds
        let item_bounds = self.calculate_item_bounds();

        // Store for hit testing using RefCell
        *self.item_bounds.borrow_mut() = item_bounds.clone();

        // Render each item
        for (i, bounds) in item_bounds.iter().enumerate() {
            self.render_item(ctx, i, *bounds);
        }
    }

    fn needs_measure(&self) -> bool {
        true
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        // Actual measurement happens in measure_with_engine() called by Window
        None
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn preferred_cursor(&self) -> Option<CursorType> {
        Some(CursorType::Pointer)
    }

    fn on_mouse_enter(&mut self, event: &mut MouseEvent) -> EventResponse {
        MouseHandler::on_mouse_enter(self, event)
    }

    fn on_mouse_leave(&mut self, event: &mut MouseEvent) -> EventResponse {
        MouseHandler::on_mouse_leave(self, event)
    }

    fn dispatch_mouse_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::MouseDown(e) => self.on_mouse_down(e),
            InputEventEnum::MouseUp(e) => self.on_mouse_up(e),
            InputEventEnum::MouseMove(e) => self.on_mouse_move(e),
            _ => EventResponse::Ignored,
        }
    }

    fn dispatch_key_event(&mut self, event: &mut InputEventEnum) -> EventResponse {
        match event {
            InputEventEnum::KeyDown(e) => self.on_key_down(e),
            InputEventEnum::KeyUp(e) => self.on_key_up(e),
            _ => EventResponse::Ignored,
        }
    }

    fn drain_deferred_commands(&mut self) -> Vec<DeferredCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
