//! macOS window implementation using NSWindow/NSView with objc2

use crate::event::{InputEventEnum, Key, KeyEvent, Modifiers, MouseButton, MouseEvent, NamedKey, WheelEvent, WheelPhase};
use crate::platform::{
    PlatformInput, PlatformWindow, WindowCallbacks, WindowOptions,
};
use crate::types::{point, vector, Point, Rect, Size};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSBackingStoreType, NSEvent, NSEventModifierFlags, NSTextInputClient,
    NSView, NSWindow, NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString, NSRange};

use std::cell::RefCell;
use std::rc::Rc;

use raw_window_handle::{
    AppKitDisplayHandle, AppKitWindowHandle, HasDisplayHandle, HasWindowHandle,
    RawDisplayHandle, RawWindowHandle,
};

// ============================================================================
// Window State
// ============================================================================

/// Shared state between Objective-C callbacks and Rust
#[derive(Debug)]
struct WindowState {
    callbacks: WindowCallbacks,
    scale_factor: f64,
    bounds: Rect,
}

impl WindowState {
    fn new() -> Self {
        Self {
            callbacks: WindowCallbacks::default(),
            scale_factor: 1.0,
            bounds: Rect::default(),
        }
    }
}

// ============================================================================
// Custom NSView
// ============================================================================

#[derive(Debug)]
struct ViewIvars {
    state: Rc<RefCell<WindowState>>,
}

define_class!(
    // SAFETY:
    // - The superclass NSView does not have any subclassing requirements.
    // - `CustomView` does not implement `Drop`.
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[name = "AssortedWidgetsView"]
    #[ivars = ViewIvars]
    struct CustomView;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for CustomView {}

    impl CustomView {
        // Mouse events
        #[unsafe(method(mouseDown:))]
        fn mouse_down(&self, event: &NSEvent) {
            let mouse_event = self.convert_to_mouse_event(event, MouseButton::Left);
            println!("[NATIVE] mouseDown detected at ({:.1}, {:.1})",
                     mouse_event.position.x, mouse_event.position.y);
            self.invoke_input_event_callback(InputEventEnum::MouseDown(mouse_event));
        }

        #[unsafe(method(mouseUp:))]
        fn mouse_up(&self, event: &NSEvent) {
            let mouse_event = self.convert_to_mouse_event(event, MouseButton::Left);
            println!("[NATIVE] mouseUp detected at ({:.1}, {:.1})",
                     mouse_event.position.x, mouse_event.position.y);
            self.invoke_input_event_callback(InputEventEnum::MouseUp(mouse_event));
        }

        #[unsafe(method(rightMouseDown:))]
        fn right_mouse_down(&self, event: &NSEvent) {
            let mouse_event = self.convert_to_mouse_event(event, MouseButton::Right);
            self.invoke_input_event_callback(InputEventEnum::MouseDown(mouse_event));
        }

        #[unsafe(method(rightMouseUp:))]
        fn right_mouse_up(&self, event: &NSEvent) {
            let mouse_event = self.convert_to_mouse_event(event, MouseButton::Right);
            self.invoke_input_event_callback(InputEventEnum::MouseUp(mouse_event));
        }

        #[unsafe(method(mouseMoved:))]
        fn mouse_moved(&self, event: &NSEvent) {
            let mouse_event = self.convert_to_mouse_event(event, MouseButton::Left);
            self.invoke_input_event_callback(InputEventEnum::MouseMove(mouse_event));
        }

        #[unsafe(method(mouseDragged:))]
        fn mouse_dragged(&self, event: &NSEvent) {
            let mouse_event = self.convert_to_mouse_event(event, MouseButton::Left);
            println!("[NATIVE] mouseDragged at ({:.1}, {:.1})",
                     mouse_event.position.x, mouse_event.position.y);
            self.invoke_input_event_callback(InputEventEnum::MouseMove(mouse_event));
        }

        #[unsafe(method(scrollWheel:))]
        fn scroll_wheel(&self, event: &NSEvent) {
            let wheel_event = self.convert_to_wheel_event(event);
            self.invoke_input_event_callback(InputEventEnum::Wheel(wheel_event));
        }

        // Keyboard events
        #[unsafe(method(keyDown:))]
        fn key_down(&self, event: &NSEvent) {
            println!("[IME] keyDown called, calling interpretKeyEvents...");

            // CRITICAL: Call interpretKeyEvents to trigger IME processing
            // This will call insertText: for committed text or setMarkedText: for preedit
            unsafe {
                use objc2_foundation::NSArray;

                // Create array with single event
                let events = NSArray::from_slice(&[event]);
                let _: () = msg_send![self, interpretKeyEvents: &*events];
            }

            println!("[IME] interpretKeyEvents finished");
            // Note: Don't convert/dispatch here - let interpretKeyEvents call insertText/setMarkedText
        }

        #[unsafe(method(keyUp:))]
        fn key_up(&self, event: &NSEvent) {
            if let Some(key_event) = Self::convert_to_key_event(event) {
                self.invoke_input_event_callback(InputEventEnum::KeyUp(key_event));
            }
        }

        // View properties
        #[unsafe(method(acceptsFirstResponder))]
        fn accepts_first_responder(&self) -> bool {
            true
        }

        #[unsafe(method(isFlipped))]
        fn is_flipped(&self) -> bool {
            true // Use top-left origin
        }
    }

    // SAFETY: NSTextInputClient protocol implementation for IME support
    unsafe impl NSTextInputClient for CustomView {
        #[unsafe(method(insertText:replacementRange:))]
        fn insert_text(&self, string: &NSObject, _replacement_range: NSRange) {
            println!("[IME] ✅ insertText called!");

            // Extract string - can be NSString or NSAttributedString
            let text_str = unsafe {
                // Check class name to determine type
                let class_name: *const NSString = msg_send![string, className];
                let class_name_str = (*class_name).to_string();

                if class_name_str.contains("AttributedString") {
                    // It's an NSAttributedString - extract the string property
                    let ns_str: *const NSString = msg_send![string, string];
                    (*ns_str).to_string()
                } else {
                    // It's a plain NSString - cast directly
                    let ns_str: *const NSString = std::mem::transmute(string);
                    (*ns_str).to_string()
                }
            };

            println!("[IME] insertText (commit): '{}'", text_str);

            // Send IME commit event to application
            // For now, just send as KeyDown events
            for ch in text_str.chars() {
                let key_event = KeyEvent::new(Key::Character(ch), Modifiers::default());
                self.invoke_input_event_callback(InputEventEnum::KeyDown(key_event));
            }
        }

        #[unsafe(method(doCommandBySelector:))]
        fn do_command_by_selector(&self, selector: objc2::runtime::Sel) {
            println!("[IME] doCommandBySelector: {:?}", selector);
            // We don't handle any special commands, so just log them
            // The system will handle special keys (Enter, Tab, etc.)
        }

        #[unsafe(method(setMarkedText:selectedRange:replacementRange:))]
        fn set_marked_text(&self, string: &NSObject, _selected_range: NSRange, _replacement_range: NSRange) {
            println!("[IME] ✅ setMarkedText called!");

            // Extract string - can be NSString or NSAttributedString
            let text_str = unsafe {
                // Check class name to determine type
                let class_name: *const NSString = msg_send![string, className];
                let class_name_str = (*class_name).to_string();

                if class_name_str.contains("AttributedString") {
                    // It's an NSAttributedString - extract the string property
                    let ns_str: *const NSString = msg_send![string, string];
                    (*ns_str).to_string()
                } else {
                    // It's a plain NSString - cast directly
                    let ns_str: *const NSString = std::mem::transmute(string);
                    (*ns_str).to_string()
                }
            };

            println!("[IME] setMarkedText (preedit): '{}'", text_str);

            // TODO: Send IME preedit event to application
        }

        #[unsafe(method(unmarkText))]
        fn unmark_text(&self) {
            println!("[IME] unmarkText");
            // TODO: Clear preedit text
        }

        #[unsafe(method(hasMarkedText))]
        fn has_marked_text(&self) -> bool {
            // TODO: Track if we have preedit text
            false
        }

        #[unsafe(method(markedRange))]
        fn marked_range(&self) -> NSRange {
            // TODO: Return range of preedit text
            NSRange {location: usize::MAX, length: 0} // NSNotFound
        }

        #[unsafe(method(selectedRange))]
        fn selected_range(&self) -> NSRange {
            // TODO: Return cursor position
            NSRange {location: 0, length: 0}
        }

        #[unsafe(method(validAttributesForMarkedText))]
        fn valid_attributes_for_marked_text(&self) -> *const NSObject {
            //println!("[IME] validAttributesForMarkedText called");
            // Return empty array
            unsafe {
                use objc2_foundation::NSArray;
                let empty_array: Retained<NSArray<NSObject>> = NSArray::new();
                Retained::into_raw(empty_array) as *const NSObject
            }
        }

        #[unsafe(method(attributedSubstringForProposedRange:actualRange:))]
        fn attributed_substring_for_proposed_range(
            &self,
            _range: NSRange,
            _actual_range: *mut NSRange,
        ) -> *const NSObject {
            println!("[IME] attributedSubstringForProposedRange called");
            // Return nil - we don't support attributed strings
            std::ptr::null()
        }

        #[unsafe(method(firstRectForCharacterRange:actualRange:))]
        fn first_rect_for_character_range(&self, _range: NSRange, _actual_range: *mut NSRange) -> NSRect {
            println!("[IME] firstRectForCharacterRange called");
            // Return position where IME candidate window should appear
            // For now, return view bounds
            unsafe { msg_send![self, bounds] }
        }

        #[unsafe(method(characterIndexForPoint:))]
        fn character_index_for_point(&self, _point: NSPoint) -> usize {
            println!("[IME] characterIndexForPoint called");
            // Return NSNotFound
            usize::MAX
        }
    }
);

impl CustomView {
    fn new(state: Rc<RefCell<WindowState>>, mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(ViewIvars { state });
        // SAFETY: The signature of `NSView`'s `init` method is correct.
        unsafe { msg_send![super(this), init] }
    }

    fn convert_mouse_event(
        &self,
        event: &NSEvent,
        button: MouseButton,
        is_down: bool,
    ) -> Option<PlatformInput> {
        let position = self.get_mouse_position(event);
        let modifiers = Self::get_modifiers(event);

        Some(if is_down {
            PlatformInput::MouseDown {
                position,
                button,
                modifiers,
            }
        } else {
            PlatformInput::MouseUp {
                position,
                button,
                modifiers,
            }
        })
    }

    fn get_mouse_position(&self, event: &NSEvent) -> Point {
        let ns_point = event.locationInWindow();
        let view_point: NSPoint = unsafe { msg_send![self, convertPoint:ns_point, fromView:None::<&NSView>] };
        point(view_point.x, view_point.y)
    }

    fn get_modifiers(event: &NSEvent) -> Modifiers {
        let flags = event.modifierFlags();

        Modifiers {
            shift: flags.contains(NSEventModifierFlags::Shift),
            control: flags.contains(NSEventModifierFlags::Control),
            alt: flags.contains(NSEventModifierFlags::Option),
            command: flags.contains(NSEventModifierFlags::Command),
        }
    }

    fn convert_key_event(event: &NSEvent, is_down: bool) -> Option<PlatformInput> {
        let characters = event.characters()?;
        let key = characters.to_string();
        let modifiers = Self::get_modifiers(event);

        Some(if is_down {
            PlatformInput::KeyDown { key, modifiers }
        } else {
            PlatformInput::KeyUp { key, modifiers }
        })
    }

    fn invoke_input_event_callback(&self, event: InputEventEnum) {
        let mut state = self.ivars().state.borrow_mut();
        if let Some(callback) = state.callbacks.input_event.as_mut() {
            callback(event);
        }
    }

    // ========================================
    // New Event System Conversions
    // ========================================

    /// Convert NSEvent modifiers to our Modifiers struct
    fn convert_modifiers(event: &NSEvent) -> Modifiers {
        let flags = event.modifierFlags();
        Modifiers {
            shift: flags.contains(NSEventModifierFlags::Shift),
            control: flags.contains(NSEventModifierFlags::Control),
            alt: flags.contains(NSEventModifierFlags::Option),
            command: flags.contains(NSEventModifierFlags::Command),
        }
    }

    /// Convert NSEvent to MouseEvent (for down/up/move)
    fn convert_to_mouse_event(&self, event: &NSEvent, button: MouseButton) -> MouseEvent {
        let position = self.get_mouse_position(event);
        let modifiers = Self::convert_modifiers(event);
        let click_count = event.clickCount() as u8;

        MouseEvent::new(position, button, modifiers)
            .with_click_count(click_count)
    }

    /// Convert NSEvent to WheelEvent
    fn convert_to_wheel_event(&self, event: &NSEvent) -> WheelEvent {
        let delta_x = event.scrollingDeltaX();
        let delta_y = event.scrollingDeltaY();
        let modifiers = Self::convert_modifiers(event);

        // Detect wheel phase (for trackpad momentum)
        let phase = match event.phase() {
            p if p.contains(objc2_app_kit::NSEventPhase::Began) => WheelPhase::Begin,
            p if p.contains(objc2_app_kit::NSEventPhase::Ended) => WheelPhase::End,
            p if p.contains(objc2_app_kit::NSEventPhase::MayBegin) => WheelPhase::Begin,
            _ => {
                if event.momentumPhase().contains(objc2_app_kit::NSEventPhase::Changed) {
                    WheelPhase::Momentum
                } else {
                    WheelPhase::Update
                }
            }
        };

        WheelEvent::new(vector(delta_x, delta_y), modifiers)
            .with_phase(phase)
    }

    /// Convert NSEvent key to our Key enum
    fn convert_key(event: &NSEvent) -> Option<Key> {
        let characters = event.characters()?;
        let key_str = characters.to_string();

        // Check for named keys first
        match key_str.as_str() {
            "\r" | "\n" => Some(Key::Named(NamedKey::Enter)),
            "\t" => Some(Key::Named(NamedKey::Tab)),
            "\u{1b}" => Some(Key::Named(NamedKey::Escape)),
            "\u{7f}" => Some(Key::Named(NamedKey::Backspace)),
            " " => Some(Key::Named(NamedKey::Space)),
            _ => {
                // Check virtual key code for special keys
                let key_code = event.keyCode();
                match key_code {
                    123 => Some(Key::Named(NamedKey::ArrowLeft)),
                    124 => Some(Key::Named(NamedKey::ArrowRight)),
                    125 => Some(Key::Named(NamedKey::ArrowDown)),
                    126 => Some(Key::Named(NamedKey::ArrowUp)),
                    117 => Some(Key::Named(NamedKey::Delete)),
                    115 => Some(Key::Named(NamedKey::Home)),
                    119 => Some(Key::Named(NamedKey::End)),
                    116 => Some(Key::Named(NamedKey::PageUp)),
                    121 => Some(Key::Named(NamedKey::PageDown)),
                    122 => Some(Key::Named(NamedKey::F1)),
                    120 => Some(Key::Named(NamedKey::F2)),
                    99 => Some(Key::Named(NamedKey::F3)),
                    118 => Some(Key::Named(NamedKey::F4)),
                    96 => Some(Key::Named(NamedKey::F5)),
                    97 => Some(Key::Named(NamedKey::F6)),
                    98 => Some(Key::Named(NamedKey::F7)),
                    100 => Some(Key::Named(NamedKey::F8)),
                    101 => Some(Key::Named(NamedKey::F9)),
                    109 => Some(Key::Named(NamedKey::F10)),
                    103 => Some(Key::Named(NamedKey::F11)),
                    111 => Some(Key::Named(NamedKey::F12)),
                    _ => {
                        // Regular character key
                        key_str.chars().next().map(Key::Character)
                    }
                }
            }
        }
    }

    /// Convert NSEvent to KeyEvent
    fn convert_to_key_event(event: &NSEvent) -> Option<KeyEvent> {
        let key = Self::convert_key(event)?;
        let modifiers = Self::convert_modifiers(event);
        let is_repeat = event.isARepeat();

        Some(KeyEvent::new(key, modifiers).with_repeat(is_repeat))
    }
}

// ============================================================================
// Custom NSWindowDelegate
// ============================================================================

#[derive(Debug)]
struct WindowDelegateIvars {
    state: Rc<RefCell<WindowState>>,
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `CustomWindowDelegate` does not implement `Drop`.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "AssortedWidgetsWindowDelegate"]
    #[ivars = WindowDelegateIvars]
    struct CustomWindowDelegate;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for CustomWindowDelegate {}

    // SAFETY: `NSWindowDelegate` has no safety requirements.
    unsafe impl NSWindowDelegate for CustomWindowDelegate {
        #[unsafe(method(windowShouldClose:))]
        fn window_should_close(&self, _sender: &NSWindow) -> bool {
            // Invoke close callback if set
            let mut state = self.ivars().state.borrow_mut();
            if let Some(ref mut close_callback) = state.callbacks.close {
                close_callback();
            }
            true
        }

        #[unsafe(method(windowDidResize:))]
        fn window_did_resize(&self, notification: &NSNotification) {
            // Get the window from the notification
            let window: *const NSWindow = unsafe { msg_send![notification, object] };
            let window = unsafe { &*window };

            // Get content bounds (excludes titlebar)
            let frame = window.frame();
            let content_rect: NSRect = unsafe { msg_send![window, contentRectForFrameRect:frame] };
            let bounds = nsrect_to_rect(content_rect);

            // Invoke resize callback if set
            let mut state = self.ivars().state.borrow_mut();
            if let Some(ref mut resize_callback) = state.callbacks.resize {
                resize_callback(bounds);
            }
        }

        #[unsafe(method(windowDidChangeBackingProperties:))]
        fn window_did_change_backing_properties(&self, notification: &NSNotification) {
            // Get the window from the notification
            let window: *const NSWindow = unsafe { msg_send![notification, object] };
            let window = unsafe { &*window };

            // Get new scale factor
            let new_scale_factor = window.backingScaleFactor();

            // Update cached scale factor and invoke callback
            let mut state = self.ivars().state.borrow_mut();
            let old_scale_factor = state.scale_factor;

            // Only invoke callback if scale factor actually changed
            if (new_scale_factor - old_scale_factor).abs() > 0.01 {
                state.scale_factor = new_scale_factor;

                if let Some(ref mut callback) = state.callbacks.scale_factor_changed {
                    callback(new_scale_factor);
                }
            }
        }
    }
);

impl CustomWindowDelegate {
    fn new(state: Rc<RefCell<WindowState>>, mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(WindowDelegateIvars { state });
        // SAFETY: The signature of `NSObject`'s `init` method is correct.
        unsafe { msg_send![super(this), init] }
    }
}

// ============================================================================
// MacWindow
// ============================================================================

pub struct MacWindow {
    native_window: Retained<NSWindow>,
    native_view: Retained<CustomView>,
    state: Rc<RefCell<WindowState>>,
    #[allow(dead_code)]
    delegate: Retained<CustomWindowDelegate>,
}

impl MacWindow {
    pub fn new(options: WindowOptions) -> Self {
        unsafe {
            super::init();

            let mtm = MainThreadMarker::new().expect("Must be called on main thread");
            let state = Rc::new(RefCell::new(WindowState::new()));

            // Create NSWindow
            let native_window = create_window(&options, mtm);

            // Create window delegate
            let delegate = CustomWindowDelegate::new(Rc::clone(&state), mtm);
            let delegate_obj = ProtocolObject::from_ref(&*delegate);
            native_window.setDelegate(Some(delegate_obj));

            // Create custom NSView
            let native_view = CustomView::new(Rc::clone(&state), mtm);

            // Set view as window's content view
            native_window.setContentView(Some(&native_view));

            // Update initial state
            let ns_bounds = native_window.frame();
            let backing_scale_factor = native_window.backingScaleFactor();

            state.borrow_mut().scale_factor = backing_scale_factor;
            state.borrow_mut().bounds = nsrect_to_rect(ns_bounds);

            // Make window visible and activate
            native_window.makeKeyAndOrderFront(None);

            MacWindow {
                native_window,
                native_view,
                state,
                delegate,
            }
        }
    }
}

impl PlatformWindow for MacWindow {
    fn bounds(&self) -> Rect {
        let frame = self.native_window.frame();
        nsrect_to_rect(frame)
    }

    fn content_bounds(&self) -> Rect {
        let frame = self.native_window.frame();
        let content_rect: NSRect = unsafe { msg_send![&self.native_window, contentRectForFrameRect:frame] };
        nsrect_to_rect(content_rect)
    }

    fn scale_factor(&self) -> f64 {
        self.state.borrow().scale_factor
    }

    fn set_title(&mut self, title: &str) {
        let ns_title = NSString::from_str(title);
        self.native_window.setTitle(&ns_title);
    }

    fn set_visible(&mut self, visible: bool) {
        if visible {
            self.native_window.makeKeyAndOrderFront(None);
        } else {
            self.native_window.orderOut(None);
        }
    }

    fn minimize(&mut self) {
        self.native_window.miniaturize(None);
    }

    fn zoom(&mut self) {
        self.native_window.zoom(None);
    }

    fn activate(&mut self) {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");
        self.native_window.makeKeyAndOrderFront(None);
        let app = NSApplication::sharedApplication(mtm);
        #[allow(deprecated)]
        app.activateIgnoringOtherApps(true);
    }

    fn close(&mut self) {
        self.native_window.close();
    }

    fn invalidate(&mut self) {
        self.native_view.setNeedsDisplay(true);
    }

    fn set_callbacks(&mut self, callbacks: WindowCallbacks) {
        self.state.borrow_mut().callbacks = callbacks;
    }

    fn set_position(&mut self, position: Point) {
        // Convert to NSPoint
        let ns_point = NSPoint {
            x: position.x,
            y: position.y,
        };

        // Set the window's frame origin (bottom-left corner in screen coordinates)
        unsafe {
            self.native_window.setFrameOrigin(ns_point);
        }
    }

    fn set_ime_cursor_area(&mut self, x: f64, y: f64, width: f64, height: f64) {
        // Store IME cursor area in window state
        let mut state = self.state.borrow_mut();
        // For now, just log the IME cursor area
        // Full IME support requires NSTextInputClient implementation
        println!("[IME] Cursor area set: ({}, {}) {}x{}", x, y, width, height);

        // TODO: Implement NSTextInputClient protocol in CustomView
        // This requires implementing methods like:
        // - setMarkedText:selectedRange:replacementRange:
        // - insertText:replacementRange:
        // - firstRectForCharacterRange:actualRange:
        // etc.
    }

    fn window_screen_origin(&self) -> Point {
        // Get window frame in screen coordinates
        let frame = self.native_window.frame();

        // macOS uses bottom-left origin for screen coordinates
        // The frame.origin gives us the bottom-left corner directly
        point(frame.origin.x, frame.origin.y)
    }

    fn window_to_screen(&self, window_pos: Point) -> Point {
        // Get window frame in screen coordinates
        let frame = self.native_window.frame();

        // window_pos is in top-left origin (because view is flipped)
        // screen coordinates are in bottom-left origin (macOS standard)
        //
        // To convert:
        // 1. frame.origin is the bottom-left corner of the window in screen coords
        // 2. window_pos.y is from the top of the window
        // 3. So screen_y = frame.origin.y + (frame.size.height - window_pos.y)

        let screen_x = frame.origin.x + window_pos.x;
        let screen_y = frame.origin.y + (frame.size.height - window_pos.y);

        point(screen_x, screen_y)
    }

    fn screen_to_window(&self, screen_pos: Point) -> Point {
        // Get window frame in screen coordinates
        let frame = self.native_window.frame();

        // screen_pos is in bottom-left origin (macOS standard)
        // window coordinates are in top-left origin (because view is flipped)
        //
        // To convert:
        // 1. frame.origin is the bottom-left corner in screen coords
        // 2. Calculate offset from bottom-left: offset_y = screen_pos.y - frame.origin.y
        // 3. Flip to top-left: window_y = frame.size.height - offset_y

        let window_x = screen_pos.x - frame.origin.x;
        let offset_y = screen_pos.y - frame.origin.y;
        let window_y = frame.size.height - offset_y;

        point(window_x, window_y)
    }
}

// ============================================================================
// NSWindow Creation
// ============================================================================

unsafe fn create_window(options: &WindowOptions, mtm: MainThreadMarker) -> Retained<NSWindow> {
    let rect = rect_to_nsrect(options.bounds);

    // Build style mask based on options
    let mut style_mask = if options.borderless {
        // Borderless window: no title bar, no resize controls
        NSWindowStyleMask::Borderless
    } else {
        // Normal window
        NSWindowStyleMask::Closable
            | NSWindowStyleMask::Miniaturizable
            | NSWindowStyleMask::Resizable
            | NSWindowStyleMask::Titled
    };

    // Utility windows (don't appear in Dock/taskbar)
    if options.utility {
        style_mask |= NSWindowStyleMask::UtilityWindow;
    }

    // Transparent titlebar (for custom chrome)
    if !options.borderless {
        if let Some(titlebar) = &options.titlebar {
            if titlebar.appears_transparent {
                style_mask |= NSWindowStyleMask::FullSizeContentView;
            }
        }
    }

    let window = NSWindow::initWithContentRect_styleMask_backing_defer(
        NSWindow::alloc(mtm),
        rect,
        style_mask,
        NSBackingStoreType::Buffered,
        false,
    );

    // SAFETY: Disable auto-release when closing windows.
    // This is required when creating `NSWindow` outside a window controller.
    window.setReleasedWhenClosed(false);

    // Set title (even for borderless - useful for debugging)
    let ns_title = NSString::from_str(&options.title);
    window.setTitle(&ns_title);

    // Configure titlebar if needed
    if !options.borderless {
        if let Some(titlebar) = &options.titlebar {
            if titlebar.appears_transparent {
                window.setTitlebarAppearsTransparent(true);
            }
        }
    }

    // Transparent window (for drag proxies)
    if options.transparent {
        window.setOpaque(false);
        window.setBackgroundColor(Some(&objc2_app_kit::NSColor::clearColor()));
        // Allow window to accept mouse events even though it's transparent
        window.setIgnoresMouseEvents(false);
    }

    // Always on top
    if options.always_on_top {
        const NSFLOATING_WINDOW_LEVEL: isize = 3;
        window.setLevel(NSFLOATING_WINDOW_LEVEL);
    }

    window
}

// ============================================================================
// Coordinate Conversion
// ============================================================================

fn rect_to_nsrect(rect: Rect) -> NSRect {
    NSRect::new(
        NSPoint::new(rect.origin.x, rect.origin.y),
        NSSize::new(rect.size.width, rect.size.height),
    )
}

fn nsrect_to_rect(nsrect: NSRect) -> Rect {
    Rect::new(
        point(nsrect.origin.x, nsrect.origin.y),
        Size::new(nsrect.size.width, nsrect.size.height),
    )
}

// ============================================================================
// raw-window-handle implementation for wgpu
// ============================================================================

impl HasWindowHandle for MacWindow {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let ptr = Retained::as_ptr(&self.native_view) as *mut _;
        let handle = AppKitWindowHandle::new(std::ptr::NonNull::new(ptr).unwrap());

        let raw_handle = RawWindowHandle::AppKit(handle);
        unsafe { Ok(raw_window_handle::WindowHandle::borrow_raw(raw_handle)) }
    }
}

impl HasDisplayHandle for MacWindow {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let handle = AppKitDisplayHandle::new();
        let raw_handle = RawDisplayHandle::AppKit(handle);
        unsafe { Ok(raw_window_handle::DisplayHandle::borrow_raw(raw_handle)) }
    }
}
