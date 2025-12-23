//! Simple input box for testing IME functionality
//!
//! This is a minimal text input widget that demonstrates:
//! - Focus management (click to focus)
//! - IME support (preedit and commit)
//! - Visual feedback for preedit vs committed text

use crate::element::Element;
use crate::event::{
    EventResponse, ImeEvent, ImeEventType, ImeHandler, KeyEvent, KeyboardHandler, MouseEvent,
    MouseHandler,
};
use crate::layout::Style;
use crate::paint::PaintContext;
use crate::types::{color, DeferredCommand, FrameInfo, GuiMessage, Rect, Size, WidgetId};
use taffy::AvailableSpace;

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

impl Element for SimpleInputBox {
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
                width: taffy::Dimension::Length(400.0),
                height: taffy::Dimension::Length(60.0),
            },
            padding: taffy::Rect {
                left: taffy::LengthPercentage::Length(10.0),
                right: taffy::LengthPercentage::Length(10.0),
                top: taffy::LengthPercentage::Length(10.0),
                bottom: taffy::LengthPercentage::Length(10.0),
            },
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw border (outer rect)
        let border = self.bounds.inset(-2.0);
        ctx.draw_rect(border, color::rgba(100, 100, 120, 255));

        // Draw background
        ctx.draw_rect(self.bounds, color::rgba(40, 40, 45, 255));

        // Draw committed text in white
        if !self.text.is_empty() {
            let text_pos = self.bounds.origin.offset(10.0, 25.0);
            ctx.draw_text(&self.text, text_pos, color::rgba(255, 255, 255, 255), 18.0);
        }

        // Draw preedit text in yellow (right after committed text)
        if !self.preedit.is_empty() {
            // Calculate width of committed text to position preedit after it
            let committed_width = if self.text.is_empty() {
                0.0
            } else {
                self.text.len() as f64 * 10.0 // Rough estimate
            };

            let preedit_pos = self.bounds.origin.offset(10.0 + committed_width, 25.0);
            ctx.draw_text(
                &self.preedit,
                preedit_pos,
                color::rgba(255, 255, 0, 255), // Yellow for preedit
                18.0,
            );

            // Draw underline under preedit text
            let underline_y = preedit_pos.y + 5.0;
            let underline_width = self.preedit.len() as f64 * 10.0;
            let underline_rect = Rect::new(
                crate::types::Point::new(preedit_pos.x, underline_y),
                Size::new(underline_width, 2.0),
            );
            ctx.draw_rect(underline_rect, color::rgba(255, 255, 0, 255));
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
            self.bounds.origin.offset(cursor_x, cursor_y),
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
        use crate::event::{InputEventEnum, Key, NamedKey};

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
