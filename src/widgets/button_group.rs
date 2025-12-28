//! Button Group widget - connected buttons with single selection
//!
//! Features:
//! - Multiple buttons visually connected (no gaps)
//! - Single selection mode (radio button behavior)
//! - Smart corner radius (only outer edges are rounded)
//! - Per-button styling based on state
//! - Selection change callbacks
//! - Horizontal layout only (vertical can be added later)
//!
//! # Example
//! ```rust,ignore
//! let button_group = ButtonGroup::new()
//!     .add_button("Left")
//!     .add_button("Middle")
//!     .add_button("Right")
//!     .selected(1)  // Select "Middle" by default
//!     .on_selection_change(|index| println!("Selected: {}", index));
//! ```

use std::any::Any;
use std::cell::RefCell;

use crate::event::handlers::{KeyboardHandler, MouseHandler};
use crate::event::input::{EventResponse, InputEventEnum, KeyEvent, MouseEvent};
use crate::layout::Style;
use crate::paint::gradient::LinearGradient;
use crate::paint::primitives::Color;
use crate::paint::types::{Border, Brush, CornerRadius, ShapeStyle};
use crate::paint::PaintContext;
use crate::text::{TextAlign, TextEngine, TextStyle, Truncate};
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;
use crate::event::OsEvent;

pub use crate::widgets::label::Padding;
use crate::widgets::button::ButtonStyle;

/// Individual button item within a ButtonGroup
#[derive(Clone, Debug)]
pub struct ButtonGroupItem {
    pub label: String,
    pub enabled: bool,
}

impl ButtonGroupItem {
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

/// Position of a button within the group (affects corner radius)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ButtonPosition {
    Left,      // First button (left corners rounded)
    Middle,    // Middle buttons (no corners rounded)
    Right,     // Last button (right corners rounded)
    Single,    // Only one button (all corners rounded)
}

/// Button Group widget - connected buttons with single selection
pub struct ButtonGroup {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Content
    items: Vec<ButtonGroupItem>,

    // Styling
    font_size: f32,
    font_family: Option<String>,
    padding: Padding,
    gap: f32,  // Visual gap between buttons (usually 0 or 1px for divider)

    // Per-state styles (shared by all buttons)
    normal_style: ButtonStyle,
    hovered_style: ButtonStyle,
    selected_style: ButtonStyle,
    disabled_style: ButtonStyle,

    // State
    selected_index: Option<usize>,
    hovered_index: Option<usize>,
    pressed_index: Option<usize>,

    // Callbacks
    on_selection_change: Option<Box<dyn FnMut(usize) + 'static>>,

    // Cached button bounds for hit testing (using RefCell for interior mutability)
    button_bounds: RefCell<Vec<Rect>>,
}

impl ButtonGroup {
    /// Create a new empty button group
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
            padding: Padding::symmetric(16.0, 8.0),
            gap: 1.0,
            normal_style: normal,
            hovered_style: hovered,
            selected_style: selected,
            disabled_style: disabled,
            selected_index: None,
            hovered_index: None,
            pressed_index: None,
            on_selection_change: None,
            button_bounds: RefCell::new(Vec::new()),
        }
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Add a button to the group
    pub fn add_button(mut self, label: impl Into<String>) -> Self {
        self.items.push(ButtonGroupItem::new(label));
        self
    }

    /// Add a button item to the group
    pub fn add_item(mut self, item: ButtonGroupItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set the selected button index
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.selected_index = Some(index);
        }
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

    /// Set padding for each button
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Set gap between buttons (0 = no gap, 1 = thin divider)
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
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

    /// Set layout style (for Taffy)
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    /// Set normal state style
    pub fn normal_style(mut self, style: ButtonStyle) -> Self {
        self.normal_style = style;
        self
    }

    /// Set hovered state style
    pub fn hovered_style(mut self, style: ButtonStyle) -> Self {
        self.hovered_style = style;
        self
    }

    /// Set selected state style
    pub fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.selected_style = style;
        self
    }

    /// Set disabled state style
    pub fn disabled_style(mut self, style: ButtonStyle) -> Self {
        self.disabled_style = style;
        self
    }

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Get the currently selected button index
    pub fn get_selected(&self) -> Option<usize> {
        self.selected_index
    }

    /// Set the selected button index at runtime
    pub fn set_selected(&mut self, index: Option<usize>) {
        if index != self.selected_index {
            if let Some(idx) = index {
                if idx < self.items.len() && self.items[idx].enabled {
                    self.selected_index = index;
                    self.dirty = true;

                    if let Some(ref mut callback) = self.on_selection_change {
                        callback(idx);
                    }
                }
            } else {
                self.selected_index = None;
                self.dirty = true;
            }
        }
    }

    /// Enable/disable a specific button
    pub fn set_button_enabled(&mut self, index: usize, enabled: bool) {
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
    // Default Styles
    // ========================================================================

    fn default_styles() -> (ButtonStyle, ButtonStyle, ButtonStyle, ButtonStyle) {
        let normal = ButtonStyle {
            background: Brush::Solid(Color::rgb(0.2, 0.4, 0.8)),
            text_color: Color::WHITE,
            icon_color: Color::WHITE,
            border: Some(Border::new(Color::rgb(0.1, 0.2, 0.6), 2.0)),
            shadow: None,  // No shadow for connected buttons
            corner_radius: 8.0,
        };

        let hovered = ButtonStyle {
            background: Brush::LinearGradient(LinearGradient::vertical(
                Color::rgb(0.3, 0.5, 0.9),
                Color::rgb(0.2, 0.4, 0.8),
            )),
            ..normal.clone()
        };

        let selected = ButtonStyle {
            background: Brush::Solid(Color::rgb(0.15, 0.3, 0.6)),
            ..normal.clone()
        };

        let disabled = ButtonStyle {
            background: Brush::Solid(Color::rgb(0.5, 0.5, 0.5)),
            text_color: Color::rgb(0.7, 0.7, 0.7),
            icon_color: Color::rgb(0.7, 0.7, 0.7),
            ..normal.clone()
        };

        (normal, hovered, selected, disabled)
    }

    // ========================================================================
    // Layout Helpers
    // ========================================================================

    fn get_button_position(&self, index: usize) -> ButtonPosition {
        let count = self.items.len();
        if count == 1 {
            ButtonPosition::Single
        } else if index == 0 {
            ButtonPosition::Left
        } else if index == count - 1 {
            ButtonPosition::Right
        } else {
            ButtonPosition::Middle
        }
    }

    fn get_corner_radius(&self, position: ButtonPosition, base_radius: f32) -> CornerRadius {
        match position {
            ButtonPosition::Single => CornerRadius::uniform(base_radius),
            ButtonPosition::Left => CornerRadius {
                top_left: base_radius,
                top_right: 0.0,
                bottom_right: 0.0,
                bottom_left: base_radius,
            },
            ButtonPosition::Right => CornerRadius {
                top_left: 0.0,
                top_right: base_radius,
                bottom_right: base_radius,
                bottom_left: 0.0,
            },
            ButtonPosition::Middle => CornerRadius::uniform(0.0),
        }
    }

    /// Calculate button bounds within the group
    fn calculate_button_bounds(&self, engine: &mut TextEngine) -> Vec<Rect> {
        let mut bounds = Vec::new();
        let count = self.items.len();

        if count == 0 {
            return bounds;
        }

        // Calculate width for each button (equal width distribution)
        let total_gaps = (count - 1) as f64 * self.gap as f64;
        let available_width = self.bounds.size.width - total_gaps;
        let button_width = available_width / count as f64;
        let button_height = self.bounds.size.height;

        let mut x = self.bounds.origin.x;

        for _i in 0..count {
            bounds.push(Rect::new(
                Point::new(x, self.bounds.origin.y),
                Size::new(button_width, button_height),
            ));

            x += button_width + self.gap as f64;
        }

        bounds
    }

    /// Hit test to find which button contains a point
    fn hit_test(&self, point: Point) -> Option<usize> {
        let bounds = self.button_bounds.borrow();
        for (i, rect) in bounds.iter().enumerate() {
            if rect.contains(point) {
                return Some(i);
            }
        }
        None
    }

    /// Get the appropriate style for a button at the given index
    fn get_button_style(&self, index: usize) -> &ButtonStyle {
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

    /// Render a single button within the group
    fn render_button(
        &self,
        ctx: &mut PaintContext,
        index: usize,
        bounds: Rect,
    ) {
        let item = &self.items[index];
        let style = self.get_button_style(index);
        let position = self.get_button_position(index);
        let corner_radius = self.get_corner_radius(position, style.corner_radius);

        // Draw button background
        ctx.draw_styled_rect(
            bounds,
            ShapeStyle {
                fill: style.background.clone(),
                corner_radius,
                border: style.border.clone(),
                shadow: style.shadow.clone(),
            },
        );

        // Draw text (centered)
        let content_area = Rect::new(
            Point::new(
                bounds.origin.x + self.padding.left as f64,
                bounds.origin.y + self.padding.top as f64,
            ),
            Size::new(
                (bounds.size.width - self.padding.horizontal() as f64).max(0.0),
                (bounds.size.height - self.padding.vertical() as f64).max(0.0),
            ),
        );

        // Measure text to center it
        let mut measured_width = 0.0;
        ctx.with_text_engine(|engine| {
            let mut text_style = TextStyle::new()
                .size(self.font_size)
                .align(TextAlign::Left);

            if let Some(font_family) = &self.font_family {
                text_style = text_style.family(font_family.clone());
            }

            let layout = engine.create_layout_with_wrap(
                &item.label,
                &text_style,
                Some(content_area.size.width as f32),
                Truncate::End,
                cosmic_text::Wrap::Word,
            );
            measured_width = layout.size().width;
        });

        // Center the text
        let content_x = content_area.origin.x + (content_area.size.width - measured_width) / 2.0;

        let mut text_style = TextStyle::new()
            .size(self.font_size)
            .color(style.text_color)
            .align(TextAlign::Left);

        if let Some(ref font_family) = self.font_family {
            text_style = text_style.family(font_family.clone());
        }

        let text_origin = Point::new(
            content_x,
            content_area.origin.y + (content_area.size.height - self.font_size as f64) / 2.0,
        );

        ctx.draw_text(&item.label, &text_style, text_origin, Some(content_area.size.width as f32));
    }

    // ========================================================================
    // Layout Measurement
    // ========================================================================

    /// Measure button group for layout system
    pub fn measure_with_engine(
        &self,
        engine: &mut TextEngine,
        known_dimensions: taffy::Size<Option<f32>>,
    ) -> Size {
        if self.items.is_empty() {
            return Size::new(0.0, 0.0);
        }

        let mut text_style = TextStyle::new()
            .size(self.font_size)
            .align(TextAlign::Center);

        if let Some(ref font_family) = self.font_family {
            text_style = text_style.family(font_family.clone());
        }

        // Measure all button labels
        let mut max_height = 0.0_f64;
        let mut total_width = 0.0_f64;

        for item in &self.items {
            let layout = engine.create_layout_with_wrap(
                &item.label,
                &text_style,
                None,
                Truncate::End,
                cosmic_text::Wrap::Word,
            );

            let text_size = layout.size();
            max_height = max_height.max(text_size.height);
            total_width += text_size.width + self.padding.horizontal() as f64;
        }

        // Add gaps between buttons
        total_width += (self.items.len() - 1) as f64 * self.gap as f64;

        // Add padding to height
        let total_height = max_height + self.padding.vertical() as f64;

        Size::new(total_width, total_height)
    }
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}

// ========================================================================
// Event Handlers
// ========================================================================

impl MouseHandler for ButtonGroup {
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
                    // Click completed on the same button
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

impl KeyboardHandler for ButtonGroup {
    fn on_key_down(&mut self, _event: &mut KeyEvent) -> EventResponse {
        // Could add arrow key navigation here
        EventResponse::Ignored
    }

    fn on_key_up(&mut self, _event: &mut KeyEvent) -> EventResponse {
        EventResponse::Ignored
    }
}

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for ButtonGroup {
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
            // Button bounds will be recalculated in paint()
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

        // Calculate button bounds
        let button_bounds = ctx.with_text_engine(|engine| {
            self.calculate_button_bounds(engine)
        });

        // Store for hit testing using RefCell
        *self.button_bounds.borrow_mut() = button_bounds.clone();

        // Render each button
        for (i, bounds) in button_bounds.iter().enumerate() {
            self.render_button(ctx, i, *bounds);
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

    fn preferred_cursor(&self) -> Option<crate::types::CursorType> {
        Some(crate::types::CursorType::Pointer)
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
