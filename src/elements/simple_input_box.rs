//! Simple input box for testing IME functionality
//!
//! This is a minimal text input widget that demonstrates:
//! - Focus management (click to focus)
//! - IME support (preedit and commit)
//! - Visual feedback for preedit vs committed text

use crate::widget::Widget;
use crate::event::{
    EventResponse, ImeEvent, ImeEventType, ImeHandler, Key, KeyEvent, KeyboardHandler, MouseEvent,
    MouseHandler, NamedKey,
};
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::text::TextStyle;
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};

/// Simple input box for IME testing
///
/// Features:
/// - Clickable to gain focus
/// - Shows committed text in white
/// - Shows preedit (composition) text in yellow
/// - Handles basic character input
/// - IME composition support (setMarkedText/insertText)
pub struct SimpleInputBox {
    id: WidgetId,
    bounds: Rect,
    is_dirty: bool,

    /// Committed text (final text)
    text: String,

    /// Preedit text (IME composition)
    preedit: String,
}

impl SimpleInputBox {
    /// Create a new simple input box
    pub fn new(id: WidgetId) -> Self {
        Self {
            id,
            bounds: Rect::default(),
            is_dirty: true,
            text: String::new(),
            preedit: String::new(),
        }
    }

    /// Get the text content
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the text content
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.is_dirty = true;
    }
}

impl Widget for SimpleInputBox {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &crate::event::OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        if self.bounds != bounds {
            self.bounds = bounds;
            self.is_dirty = true;
        }
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn layout(&self) -> Style {
        Style {
            size: taffy::Size {
                width: taffy::Dimension::length(400.0),
                height: taffy::Dimension::length(60.0),
            },
            padding: taffy::Rect {
                left: taffy::LengthPercentage::length(10.0),
                right: taffy::LengthPercentage::length(10.0),
                top: taffy::LengthPercentage::length(10.0),
                bottom: taffy::LengthPercentage::length(10.0),
            },
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw border (outer rect) - inflate by 2.0 to make it larger
        let border = self.bounds.inflate(2.0, 2.0);
        ctx.draw_rect(border, Color::rgba(100.0/255.0, 100.0/255.0, 120.0/255.0, 1.0));

        // Draw background
        ctx.draw_rect(self.bounds, Color::rgba(40.0/255.0, 40.0/255.0, 45.0/255.0, 1.0));

        // Draw committed text in white
        if !self.text.is_empty() {
            let text_pos = Point::new(self.bounds.origin.x + 10.0, self.bounds.origin.y + 25.0);
            let text_style = TextStyle::new().size(18.0).color(Color::WHITE);
            ctx.draw_text(&self.text, &text_style, text_pos, None);
        }

        // Draw preedit text in yellow (right after committed text)
        if !self.preedit.is_empty() {
            // Calculate width of committed text to position preedit after it
            let committed_width = if self.text.is_empty() {
                0.0
            } else {
                self.text.len() as f64 * 10.0 // Rough estimate
            };

            let preedit_pos = Point::new(
                self.bounds.origin.x + 10.0 + committed_width,
                self.bounds.origin.y + 25.0
            );
            let preedit_style = TextStyle::new()
                .size(18.0)
                .color(Color::rgba(1.0, 1.0, 0.0, 1.0)); // Yellow
            ctx.draw_text(&self.preedit, &preedit_style, preedit_pos, None);

            // Draw underline under preedit text
            let underline_y = preedit_pos.y + 5.0;
            let underline_width = self.preedit.len() as f64 * 10.0;
            let underline_rect = Rect::new(
                Point::new(preedit_pos.x, underline_y),
                Size::new(underline_width, 2.0),
            );
            ctx.draw_rect(underline_rect, Color::rgba(1.0, 1.0, 0.0, 1.0));
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    // Event handling

    fn is_interactive(&self) -> bool {
        true
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn ime_cursor_rect(&self) -> Option<Rect> {
        // Return cursor position at the end of committed text
        let cursor_x = 10.0 + (self.text.len() as f64 * 10.0);
        let cursor_y = 15.0;

        Some(Rect::new(
            Point::new(self.bounds.origin.x + cursor_x, self.bounds.origin.y + cursor_y),
            Size::new(2.0, 30.0),
        ))
    }

    fn dispatch_mouse_event(
        &mut self,
        event: &mut crate::event::InputEventEnum,
    ) -> crate::event::EventResponse {
        if let crate::event::InputEventEnum::MouseDown(_) = event {
            println!("[SimpleInputBox] Clicked - gaining focus");
            EventResponse::Handled
        } else {
            EventResponse::Ignored
        }
    }

    fn dispatch_key_event(
        &mut self,
        event: &mut crate::event::InputEventEnum,
    ) -> crate::event::EventResponse {
        use crate::event::InputEventEnum;

        match event {
            InputEventEnum::KeyDown(key_event) => {
                match &key_event.key {
                    Key::Character(c) => {
                        // Simple character insertion (no IME)
                        self.text.push(*c);
                        self.is_dirty = true;
                        println!("[SimpleInputBox] Typed: {}", c);
                        EventResponse::Handled
                    }
                    Key::Named(NamedKey::Backspace) => {
                        if !self.text.is_empty() {
                            self.text.pop();
                            self.is_dirty = true;
                            println!("[SimpleInputBox] Backspace");
                        }
                        EventResponse::Handled
                    }
                    _ => EventResponse::Ignored,
                }
            }
            _ => EventResponse::Ignored,
        }
    }

    fn dispatch_ime_event(
        &mut self,
        event: &mut crate::event::ImeEvent,
    ) -> crate::event::EventResponse {
        match &event.event_type {
            ImeEventType::Preedit(text) => {
                self.preedit = text.clone();
                self.is_dirty = true;
                println!("[SimpleInputBox] IME Preedit: {}", text);
                EventResponse::Handled
            }
            ImeEventType::Commit(text) => {
                self.text.push_str(text);
                self.preedit.clear();
                self.is_dirty = true;
                println!("[SimpleInputBox] IME Commit: {}", text);
                EventResponse::Handled
            }
            ImeEventType::Cancel => {
                self.preedit.clear();
                self.is_dirty = true;
                println!("[SimpleInputBox] IME Cancel");
                EventResponse::Handled
            }
        }
    }
}

impl MouseHandler for SimpleInputBox {
    fn on_mouse_down(&mut self, _event: &mut MouseEvent) -> EventResponse {
        println!("[SimpleInputBox] Mouse down - focusing");
        EventResponse::Handled
    }
}

impl KeyboardHandler for SimpleInputBox {
    fn on_key_down(&mut self, event: &mut KeyEvent) -> EventResponse {
        match &event.key {
            Key::Character(c) => {
                self.text.push(*c);
                self.is_dirty = true;
                EventResponse::Handled
            }
            Key::Named(NamedKey::Backspace) => {
                if !self.text.is_empty() {
                    self.text.pop();
                    self.is_dirty = true;
                }
                EventResponse::Handled
            }
            _ => EventResponse::Ignored,
        }
    }
}

impl ImeHandler for SimpleInputBox {
    fn on_ime(&mut self, event: &mut ImeEvent) -> EventResponse {
        match &event.event_type {
            ImeEventType::Preedit(text) => {
                self.preedit = text.clone();
                self.is_dirty = true;
                EventResponse::Handled
            }
            ImeEventType::Commit(text) => {
                self.text.push_str(text);
                self.preedit.clear();
                self.is_dirty = true;
                EventResponse::Handled
            }
            ImeEventType::Cancel => {
                self.preedit.clear();
                self.is_dirty = true;
                EventResponse::Handled
            }
        }
    }
}
