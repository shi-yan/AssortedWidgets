//! Button widget - interactive button with rich styling and multiple states
//!
//! Features:
//! - Three flavors: text-only, icon-only, icon+text
//! - Multiple states: normal, hovered, pressed, active (toggled), disabled, focused
//! - Per-state styling with gradients, shadows, borders
//! - Configurable padding, colors, fonts
//! - Click callbacks
//! - Keyboard activation (Space/Enter)
//! - Focus ring indicator
//! - Toggleable mode
//! - Flexible layout (stretch or fit content)

use std::any::Any;
use std::cell::RefCell;

use crate::event::handlers::{KeyboardHandler, MouseHandler};
use crate::event::input::{EventResponse, InputEventEnum, Key, KeyEvent, MouseEvent, NamedKey};
use crate::layout::Style;
use crate::paint::gradient::LinearGradient;
use crate::paint::primitives::Color;
use crate::paint::types::{Border, Brush, CornerRadius, Shadow, ShapeStyle};
use crate::paint::PaintContext;
use crate::text::{TextAlign, TextEngine, TextLayout, TextStyle, Truncate};
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;
use crate::event::OsEvent;

// Re-export Padding from label widget
pub use crate::widgets::label::Padding;

/// Button content type
#[derive(Clone, Debug)]
pub enum ButtonContent {
    /// Text-only button
    Text(String),
    /// Icon-only button (Material Icons font)
    Icon(String),
    /// Icon + text button (icon on left)
    IconText { icon: String, text: String },
}

/// Button state (determines which style to use)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Active,    // When toggled on
    Disabled,
    Focused,   // Keyboard focus (shows focus ring)
}

/// Button styling for a specific state
#[derive(Clone, Debug)]
pub struct ButtonStyle {
    pub background: Brush,           // Solid color or gradient
    pub text_color: Color,
    pub icon_color: Color,
    pub border: Option<Border>,
    pub shadow: Option<Shadow>,
    pub corner_radius: f32,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            background: Brush::Solid(Color::rgb(0.2, 0.4, 0.8)),
            text_color: Color::WHITE,
            icon_color: Color::WHITE,
            border: Some(Border::new(Color::rgb(0.1, 0.2, 0.6), 2.0)),
            shadow: Some(Shadow::new(
                Color::rgba(0.0, 0.0, 0.0, 0.4),
                (2.0, 4.0),
                8.0,
            )),
            corner_radius: 8.0,
        }
    }
}

/// Button widget - interactive button with rich styling
///
/// # Example
/// ```rust,ignore
/// // Simple text button
/// let button = Button::text("Click Me")
///     .on_click(|| println!("Clicked!"));
///
/// // Icon button
/// let button = Button::icon("search");
///
/// // Icon + text button
/// let button = Button::icon_text("settings", "Settings")
///     .on_click(|| println!("Settings!"));
///
/// // Toggleable button
/// let button = Button::text("Toggle")
///     .togglable()
///     .on_click(|| println!("Toggled!"));
///
/// // Custom styling
/// let button = Button::text("Custom")
///     .padding(Padding::uniform(16.0))
///     .font_size(20.0)
///     .hovered_style(custom_hover_style);
/// ```
pub struct Button {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Content
    content: ButtonContent,

    // Styling
    font_size: f32,
    font_family: Option<String>,
    padding: Padding,

    // Per-state styles
    normal_style: ButtonStyle,
    hovered_style: ButtonStyle,
    pressed_style: ButtonStyle,
    active_style: ButtonStyle,
    disabled_style: ButtonStyle,
    focused_style: ButtonStyle,

    // State
    current_state: ButtonState,
    is_hovered: bool,
    is_pressed: bool,
    is_togglable: bool,
    is_toggled: bool,
    is_disabled: bool,
    is_focused: bool,

    // Layout
    stretch_width: bool,  // true = full width, false = fit content

    // Click callback
    on_click: Option<Box<dyn FnMut() + 'static>>,

    // Cached text layout
    cached_text_layout: RefCell<Option<TextLayout>>,
    cached_text_max_width: RefCell<Option<f32>>,
}

impl Button {
    /// Create a new button with the given content
    pub fn new(content: ButtonContent) -> Self {
        let (normal, hovered, pressed, active, disabled, focused) = Self::default_styles();

        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            content,
            font_size: 16.0,
            font_family: None,
            padding: Padding::symmetric(16.0, 8.0),
            normal_style: normal,
            hovered_style: hovered,
            pressed_style: pressed,
            active_style: active,
            disabled_style: disabled,
            focused_style: focused,
            current_state: ButtonState::Normal,
            is_hovered: false,
            is_pressed: false,
            is_togglable: false,
            is_toggled: false,
            is_disabled: false,
            is_focused: false,
            stretch_width: false,
            on_click: None,
            cached_text_layout: RefCell::new(None),
            cached_text_max_width: RefCell::new(None),
        }
    }

    /// Create a text-only button
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(ButtonContent::Text(text.into()))
    }

    /// Create an icon-only button
    pub fn icon(icon: impl Into<String>) -> Self {
        Self::new(ButtonContent::Icon(icon.into()))
    }

    /// Create an icon + text button (icon on left)
    pub fn icon_text(icon: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new(ButtonContent::IconText {
            icon: icon.into(),
            text: text.into(),
        })
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        *self.cached_text_layout.borrow_mut() = None;
        self
    }

    /// Set font family
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font_family = Some(font.into());
        *self.cached_text_layout.borrow_mut() = None;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        *self.cached_text_layout.borrow_mut() = None;
        self
    }

    /// Make button stretch to full width
    pub fn stretch(mut self) -> Self {
        self.stretch_width = true;
        self
    }

    /// Make button fit content (default)
    pub fn fit(mut self) -> Self {
        self.stretch_width = false;
        self
    }

    /// Make button toggleable
    pub fn togglable(mut self) -> Self {
        self.is_togglable = true;
        self
    }

    /// Set button as disabled
    pub fn disabled(mut self) -> Self {
        self.is_disabled = true;
        self.current_state = ButtonState::Disabled;
        self
    }

    /// Set click callback
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.on_click = Some(Box::new(callback));
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

    /// Set pressed state style
    pub fn pressed_style(mut self, style: ButtonStyle) -> Self {
        self.pressed_style = style;
        self
    }

    /// Set active (toggled) state style
    pub fn active_style(mut self, style: ButtonStyle) -> Self {
        self.active_style = style;
        self
    }

    /// Set disabled state style
    pub fn disabled_style(mut self, style: ButtonStyle) -> Self {
        self.disabled_style = style;
        self
    }

    /// Set focused state style
    pub fn focused_style(mut self, style: ButtonStyle) -> Self {
        self.focused_style = style;
        self
    }

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Set disabled state at runtime
    pub fn set_disabled(&mut self, disabled: bool) {
        if self.is_disabled != disabled {
            self.is_disabled = disabled;
            self.update_state();
            self.dirty = true;
        }
    }

    /// Set toggled state at runtime
    pub fn set_toggled(&mut self, toggled: bool) {
        if self.is_togglable && self.is_toggled != toggled {
            self.is_toggled = toggled;
            self.update_state();
            self.dirty = true;
        }
    }

    /// Get toggled state
    pub fn is_toggled(&self) -> bool {
        self.is_toggled
    }

    // ========================================================================
    // Default Styles
    // ========================================================================

    fn default_styles() -> (ButtonStyle, ButtonStyle, ButtonStyle, ButtonStyle, ButtonStyle, ButtonStyle) {
        let normal = ButtonStyle::default();

        let hovered = ButtonStyle {
            background: Brush::LinearGradient(LinearGradient::vertical(
                Color::rgb(0.3, 0.5, 0.9),
                Color::rgb(0.2, 0.4, 0.8),
            )),
            shadow: Some(Shadow::new(
                Color::rgba(0.0, 0.0, 0.0, 0.5),
                (3.0, 6.0),
                12.0,
            )),
            ..normal.clone()
        };

        let pressed = ButtonStyle {
            background: Brush::Solid(Color::rgb(0.15, 0.3, 0.6)),
            shadow: Some(Shadow::new(
                Color::rgba(0.0, 0.0, 0.0, 0.3),
                (1.0, 2.0),
                4.0,
            )),
            ..normal.clone()
        };

        let active = ButtonStyle {
            background: Brush::Solid(Color::rgb(0.1, 0.5, 0.2)),
            ..normal.clone()
        };

        let disabled = ButtonStyle {
            background: Brush::Solid(Color::rgb(0.5, 0.5, 0.5)),
            text_color: Color::rgb(0.7, 0.7, 0.7),
            icon_color: Color::rgb(0.7, 0.7, 0.7),
            shadow: None,
            ..normal.clone()
        };

        let focused = ButtonStyle {
            // Same as normal, focus ring is drawn separately
            ..normal.clone()
        };

        (normal, hovered, pressed, active, disabled, focused)
    }

    // ========================================================================
    // State Management
    // ========================================================================

    fn update_state(&mut self) {
        self.current_state = if self.is_disabled {
            ButtonState::Disabled
        } else if self.is_pressed {
            ButtonState::Pressed
        } else if self.is_toggled {
            ButtonState::Active
        } else if self.is_hovered {
            ButtonState::Hovered
        } else if self.is_focused {
            ButtonState::Focused
        } else {
            ButtonState::Normal
        };
    }

    fn get_current_style(&self) -> &ButtonStyle {
        match self.current_state {
            ButtonState::Normal => &self.normal_style,
            ButtonState::Hovered => &self.hovered_style,
            ButtonState::Pressed => &self.pressed_style,
            ButtonState::Active => &self.active_style,
            ButtonState::Disabled => &self.disabled_style,
            ButtonState::Focused => &self.focused_style,
        }
    }

    // ========================================================================
    // Rendering Helpers
    // ========================================================================

    fn render_text(&self, ctx: &mut PaintContext, area: Rect, text: &str, style: &ButtonStyle) {
        // Measure text to center it properly
        let mut measured_width = 0.0;
        ctx.with_text_engine(|engine| {
            let mut text_style = TextStyle::new()
                .size(self.font_size)
                .align(TextAlign::Left);

            if let Some(font_family) = &self.font_family {
                text_style = text_style.family(font_family.clone());
            }

            let layout = engine.create_layout_with_wrap(
                text,
                &text_style,
                Some(area.size.width as f32),
                Truncate::End,
                cosmic_text::Wrap::Word,
            );
            measured_width = layout.size().width;
        });

        // Center the text content
        let content_x = area.origin.x + (area.size.width - measured_width) / 2.0;

        let mut text_style = TextStyle::new()
            .size(self.font_size)
            .color(style.text_color)
            .align(TextAlign::Left);

        if let Some(ref font_family) = self.font_family {
            text_style = text_style.family(font_family.clone());
        }

        let text_origin = Point::new(
            content_x,
            area.origin.y + (area.size.height - self.font_size as f64) / 2.0,
        );

        ctx.draw_text(text, &text_style, text_origin, Some(area.size.width as f32));
    }

    fn render_icon(&self, ctx: &mut PaintContext, area: Rect, icon_id: &str, style: &ButtonStyle) {
        // Center icon in area
        // NOTE: Material Icons font has metrics that make glyphs render smaller than text
        // at the same font size. Scale by 1.5x for visual consistency with text.
        let icon_size = self.font_size * 1.5;
        let icon_x = area.origin.x + (area.size.width - icon_size as f64) / 2.0;
        let icon_y_center = area.origin.y + (area.size.height - icon_size as f64) / 2.0;

        // IMPORTANT: Icon rendering uses baseline positioning (like text), where logical_offset_y
        // (bearing) is subtracted. To position icons by their bounding box top-left, we need to
        // ADD the bearing offset here to compensate. For Material Icons, bearing is ~75% of em size.
        let bearing_compensation = icon_size as f64 * 0.75;
        let icon_y = icon_y_center + bearing_compensation;

        ctx.draw_icon(
            icon_id,
            Point::new(icon_x, icon_y),
            icon_size,
            style.icon_color,
        );
    }

    fn render_icon_text(&self, ctx: &mut PaintContext, area: Rect, icon_id: &str, text: &str, style: &ButtonStyle) {
        let gap = 8.0_f64;
        // NOTE: Material Icons font scaled by 1.5x for visual consistency with text
        let icon_size = (self.font_size * 1.5) as f64;

        // Measure text width
        let mut text_width = 0.0;
        ctx.with_text_engine(|engine| {
            let mut text_style = TextStyle::new()
                .size(self.font_size)
                .align(TextAlign::Left);

            if let Some(font_family) = &self.font_family {
                text_style = text_style.family(font_family.clone());
            }

            let layout = engine.create_layout_with_wrap(
                text,
                &text_style,
                Some(area.size.width as f32),
                Truncate::End,
                cosmic_text::Wrap::Word,
            );
            text_width = layout.size().width;
        });

        // Calculate total content width and center it
        let content_width = icon_size + gap + text_width;
        let content_start_x = area.origin.x + (area.size.width - content_width) / 2.0;

        // Icon position (centered content, left side)
        let icon_x = content_start_x;
        let icon_y_center = area.origin.y + (area.size.height - icon_size) / 2.0;

        // Bearing compensation for baseline positioning (same as render_icon)
        let bearing_compensation = icon_size * 0.75;
        let icon_y = icon_y_center + bearing_compensation;

        ctx.draw_icon(icon_id, Point::new(icon_x, icon_y), icon_size as f32, style.icon_color);

        // Text position (after icon + gap)
        let text_x = content_start_x + icon_size + gap;

        let mut text_style = TextStyle::new()
            .size(self.font_size)
            .color(style.text_color)
            .align(TextAlign::Left);

        if let Some(ref font_family) = self.font_family {
            text_style = text_style.family(font_family.clone());
        }

        let text_origin = Point::new(
            text_x,
            area.origin.y + (area.size.height - self.font_size as f64) / 2.0,
        );

        // Available width for text = total area width - icon size - gap
        let text_max_width = (area.size.width - icon_size - gap) as f32;
        ctx.draw_text(text, &text_style, text_origin, Some(text_max_width));
    }

    // ========================================================================
    // Layout Measurement
    // ========================================================================

    /// Measure button for layout system
    pub fn measure_with_engine(
        &self,
        engine: &mut TextEngine,
        known_dimensions: taffy::Size<Option<f32>>,
    ) -> Size {
        match &self.content {
            ButtonContent::Text(text) => {
                // Measure text
                let mut text_style = TextStyle::new()
                    .size(self.font_size)
                    .align(TextAlign::Center);

                if let Some(ref font_family) = self.font_family {
                    text_style = text_style.family(font_family.clone());
                }

                let max_width = known_dimensions.width.map(|w| (w - self.padding.horizontal()).max(0.0));
                let layout = engine.create_layout_with_wrap(
                    text,
                    &text_style,
                    max_width,
                    Truncate::End,
                    cosmic_text::Wrap::Word,
                );

                let text_size = layout.size();

                Size::new(
                    text_size.width + self.padding.horizontal() as f64,
                    text_size.height + self.padding.vertical() as f64,
                )
            }
            ButtonContent::Icon(_) => {
                // Icon size = font_size * 1.5 (Material Icons scaling)
                let icon_size = self.font_size * 1.5;
                Size::new(
                    icon_size as f64 + self.padding.horizontal() as f64,
                    icon_size as f64 + self.padding.vertical() as f64,
                )
            }
            ButtonContent::IconText { text, .. } => {
                // Icon size + gap + text width
                let gap = 8.0_f32;
                let icon_size = self.font_size * 1.5;  // Material Icons scaling

                let mut text_style = TextStyle::new()
                    .size(self.font_size)
                    .align(TextAlign::Center);

                if let Some(ref font_family) = self.font_family {
                    text_style = text_style.family(font_family.clone());
                }

                let text_max_width = known_dimensions.width.map(|w| {
                    (w - self.padding.horizontal() - icon_size - gap).max(0.0)
                });

                let layout = engine.create_layout_with_wrap(
                    text,
                    &text_style,
                    text_max_width,
                    Truncate::End,
                    cosmic_text::Wrap::Word,
                );

                let text_size = layout.size();

                let total_width = icon_size as f64 + gap as f64 + text_size.width + self.padding.horizontal() as f64;
                let total_height = icon_size.max(text_size.height as f32) as f64 + self.padding.vertical() as f64;

                Size::new(total_width, total_height)
            }
        }
    }
}

// ========================================================================
// Event Handlers
// ========================================================================

impl MouseHandler for Button {
    fn on_mouse_down(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        self.is_pressed = true;
        self.update_state();
        self.dirty = true;

        EventResponse::Handled
    }

    fn on_mouse_up(&mut self, _event: &mut MouseEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        if self.is_pressed {
            // Click happened
            if self.is_togglable {
                self.is_toggled = !self.is_toggled;
            }

            self.is_pressed = false;
            self.update_state();
            self.dirty = true;

            // Call callback if set
            if let Some(ref mut callback) = self.on_click {
                callback();
            }
        }

        EventResponse::Handled
    }

    fn on_mouse_move(&mut self, _event: &mut MouseEvent) -> EventResponse {
        EventResponse::PassThrough
    }

    fn on_mouse_enter(&mut self) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        self.is_hovered = true;
        self.update_state();
        self.dirty = true;

        EventResponse::Handled
    }

    fn on_mouse_leave(&mut self) -> EventResponse {
        self.is_hovered = false;
        self.is_pressed = false;
        self.update_state();
        self.dirty = true;

        EventResponse::Handled
    }
}

impl KeyboardHandler for Button {
    fn on_key_down(&mut self, event: &mut KeyEvent) -> EventResponse {
        if self.is_disabled {
            return EventResponse::Ignored;
        }

        match event.key {
            Key::Named(NamedKey::Space) | Key::Named(NamedKey::Enter) => {
                // Activate button
                if self.is_togglable {
                    self.is_toggled = !self.is_toggled;
                }

                self.update_state();
                self.dirty = true;

                // Call callback if set
                if let Some(ref mut callback) = self.on_click {
                    callback();
                }

                EventResponse::Handled
            }
            _ => EventResponse::Ignored,
        }
    }

    fn on_key_up(&mut self, _event: &mut KeyEvent) -> EventResponse {
        EventResponse::Ignored
    }
}

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for Button {
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
        self.bounds = bounds;
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
        let style = self.get_current_style();

        // Draw button background with current style
        ctx.draw_styled_rect(
            self.bounds,
            ShapeStyle {
                fill: style.background.clone(),
                corner_radius: CornerRadius::uniform(style.corner_radius),
                border: style.border.clone(),
                shadow: style.shadow.clone(),
            },
        );

        // Draw focus ring if focused (outside the button)
        if self.is_focused && !self.is_disabled {
            let focus_ring_rect = Rect::new(
                Point::new(
                    self.bounds.origin.x - 2.0,
                    self.bounds.origin.y - 2.0,
                ),
                Size::new(
                    self.bounds.size.width + 4.0,
                    self.bounds.size.height + 4.0,
                ),
            );

            ctx.draw_styled_rect(
                focus_ring_rect,
                ShapeStyle {
                    fill: Brush::Solid(Color::TRANSPARENT),
                    corner_radius: CornerRadius::uniform(style.corner_radius + 2.0),
                    border: Some(Border::new(Color::rgb(0.3, 0.6, 0.9), 2.0)),
                    shadow: None,
                },
            );
        }

        // Calculate content area (bounds minus padding)
        let content_area = Rect::new(
            Point::new(
                self.bounds.origin.x + self.padding.left as f64,
                self.bounds.origin.y + self.padding.top as f64,
            ),
            Size::new(
                (self.bounds.size.width - self.padding.horizontal() as f64).max(0.0),
                (self.bounds.size.height - self.padding.vertical() as f64).max(0.0),
            ),
        );

        // Render content based on type
        match &self.content {
            ButtonContent::Text(text) => {
                self.render_text(ctx, content_area, text, style);
            }
            ButtonContent::Icon(icon_id) => {
                self.render_icon(ctx, content_area, icon_id, style);
            }
            ButtonContent::IconText { icon, text } => {
                self.render_icon_text(ctx, content_area, icon, text, style);
            }
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
        !self.is_disabled
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
