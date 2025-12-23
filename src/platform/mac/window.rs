//! macOS window implementation using NSWindow/NSView with objc2

use crate::platform::{
    Modifiers, MouseButton, PlatformInput, PlatformWindow, WindowCallbacks, WindowOptions,
};
use crate::types::{point, vector, Point, Rect, Size};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSBackingStoreType, NSEvent, NSEventModifierFlags, NSView, NSWindow,
    NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString};

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
            if let Some(input) = self.convert_mouse_event(event, MouseButton::Left, true) {
                self.invoke_input_callback(input);
            }
        }

        #[unsafe(method(mouseUp:))]
        fn mouse_up(&self, event: &NSEvent) {
            if let Some(input) = self.convert_mouse_event(event, MouseButton::Left, false) {
                self.invoke_input_callback(input);
            }
        }

        #[unsafe(method(rightMouseDown:))]
        fn right_mouse_down(&self, event: &NSEvent) {
            if let Some(input) = self.convert_mouse_event(event, MouseButton::Right, true) {
                self.invoke_input_callback(input);
            }
        }

        #[unsafe(method(rightMouseUp:))]
        fn right_mouse_up(&self, event: &NSEvent) {
            if let Some(input) = self.convert_mouse_event(event, MouseButton::Right, false) {
                self.invoke_input_callback(input);
            }
        }

        #[unsafe(method(mouseMoved:))]
        fn mouse_moved(&self, event: &NSEvent) {
            let position = self.get_mouse_position(event);
            let modifiers = Self::get_modifiers(event);
            let input = PlatformInput::MouseMove {
                position,
                modifiers,
            };
            self.invoke_input_callback(input);
        }

        #[unsafe(method(mouseDragged:))]
        fn mouse_dragged(&self, event: &NSEvent) {
            let position = self.get_mouse_position(event);
            let modifiers = Self::get_modifiers(event);
            let input = PlatformInput::MouseMove {
                position,
                modifiers,
            };
            self.invoke_input_callback(input);
        }

        #[unsafe(method(scrollWheel:))]
        fn scroll_wheel(&self, event: &NSEvent) {
            let delta_x = event.scrollingDeltaX();
            let delta_y = event.scrollingDeltaY();
            let modifiers = Self::get_modifiers(event);

            let input = PlatformInput::MouseWheel {
                delta: vector(delta_x, delta_y),
                modifiers,
            };
            self.invoke_input_callback(input);
        }

        // Keyboard events
        #[unsafe(method(keyDown:))]
        fn key_down(&self, event: &NSEvent) {
            if let Some(input) = Self::convert_key_event(event, true) {
                self.invoke_input_callback(input);
            }
        }

        #[unsafe(method(keyUp:))]
        fn key_up(&self, event: &NSEvent) {
            if let Some(input) = Self::convert_key_event(event, false) {
                self.invoke_input_callback(input);
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

    fn invoke_input_callback(&self, input: PlatformInput) {
        let mut state = self.ivars().state.borrow_mut();
        if let Some(callback) = state.callbacks.input.as_mut() {
            callback(input);
        }
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
}

// ============================================================================
// NSWindow Creation
// ============================================================================

unsafe fn create_window(options: &WindowOptions, mtm: MainThreadMarker) -> Retained<NSWindow> {
    let rect = rect_to_nsrect(options.bounds);

    let mut style_mask = NSWindowStyleMask::Closable
        | NSWindowStyleMask::Miniaturizable
        | NSWindowStyleMask::Resizable
        | NSWindowStyleMask::Titled;

    if let Some(titlebar) = &options.titlebar {
        if titlebar.appears_transparent {
            style_mask |= NSWindowStyleMask::FullSizeContentView;
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

    // Set title
    let ns_title = NSString::from_str(&options.title);
    window.setTitle(&ns_title);

    // Configure titlebar if needed
    if let Some(titlebar) = &options.titlebar {
        if titlebar.appears_transparent {
            window.setTitlebarAppearsTransparent(true);
        }
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
