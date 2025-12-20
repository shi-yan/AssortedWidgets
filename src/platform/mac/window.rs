//! macOS window implementation using NSWindow/NSView

use crate::platform::{
    Modifiers, MouseButton, PlatformInput, PlatformWindow, WindowCallbacks, WindowOptions,
};
use crate::types::{point, vector, Point, Rect, Size};

use cocoa::appkit::{NSApplication, NSBackingStoreType, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil, BOOL, NO, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use raw_window_handle::{
    AppKitDisplayHandle, AppKitWindowHandle, HasDisplayHandle, HasWindowHandle,
    RawDisplayHandle, RawWindowHandle,
};

// ============================================================================
// Window State
// ============================================================================

/// Shared state between Objective-C callbacks and Rust
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
// MacWindow
// ============================================================================

pub struct MacWindow {
    native_window: id,
    native_view: id,
    state: Rc<RefCell<WindowState>>,
}

impl MacWindow {
    pub fn new(options: WindowOptions) -> Self {
        unsafe {
            super::init();

            let state = Rc::new(RefCell::new(WindowState::new()));

            // Create NSWindow
            let native_window = create_window(&options);

            // Create custom NSView
            let native_view = create_view(Rc::clone(&state));

            // Set view as window's content view
            native_window.setContentView_(native_view);

            // Update initial state
            let ns_bounds = NSWindow::frame(native_window);
            let backing_scale_factor: f64 = msg_send![native_window, backingScaleFactor];

            state.borrow_mut().scale_factor = backing_scale_factor;
            state.borrow_mut().bounds = ns_rect_to_rect(ns_bounds);

            // Make window visible and activate
            native_window.makeKeyAndOrderFront_(nil);

            MacWindow {
                native_window,
                native_view,
                state,
            }
        }
    }
}

impl PlatformWindow for MacWindow {
    fn bounds(&self) -> Rect {
        unsafe {
            let frame = NSWindow::frame(self.native_window);
            ns_rect_to_rect(frame)
        }
    }

    fn content_bounds(&self) -> Rect {
        unsafe {
            let content_rect: NSRect = msg_send![self.native_window, contentRectForFrameRect: NSWindow::frame(self.native_window)];
            ns_rect_to_rect(content_rect)
        }
    }

    fn scale_factor(&self) -> f64 {
        self.state.borrow().scale_factor
    }

    fn set_title(&mut self, title: &str) {
        unsafe {
            let ns_title = NSString::alloc(nil).init_str(title);
            self.native_window.setTitle_(ns_title);
        }
    }

    fn set_visible(&mut self, visible: bool) {
        unsafe {
            if visible {
                self.native_window.makeKeyAndOrderFront_(nil);
            } else {
                self.native_window.orderOut_(nil);
            }
        }
    }

    fn minimize(&mut self) {
        unsafe {
            self.native_window.miniaturize_(nil);
        }
    }

    fn zoom(&mut self) {
        unsafe {
            let _: () = msg_send![self.native_window, zoom: nil];
        }
    }

    fn activate(&mut self) {
        unsafe {
            self.native_window.makeKeyAndOrderFront_(nil);
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, activateIgnoringOtherApps: YES];
        }
    }

    fn close(&mut self) {
        unsafe {
            self.native_window.close();
        }
    }

    fn invalidate(&mut self) {
        unsafe {
            let _: () = msg_send![self.native_view, setNeedsDisplay: YES];
        }
    }

    fn set_callbacks(&mut self, callbacks: WindowCallbacks) {
        self.state.borrow_mut().callbacks = callbacks;
    }
}

// ============================================================================
// NSWindow Creation
// ============================================================================

unsafe fn create_window(options: &WindowOptions) -> id {
    let rect = rect_to_ns_rect(options.bounds);

    let mut style_mask = NSWindowStyleMask::NSClosableWindowMask
        | NSWindowStyleMask::NSMiniaturizableWindowMask
        | NSWindowStyleMask::NSResizableWindowMask
        | NSWindowStyleMask::NSTitledWindowMask;

    if let Some(titlebar) = &options.titlebar {
        if titlebar.appears_transparent {
            style_mask |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
        }
    }

    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
        rect,
        style_mask,
        NSBackingStoreType::NSBackingStoreBuffered,
        NO,
    );

    // Set title
    let ns_title = NSString::alloc(nil).init_str(&options.title);
    window.setTitle_(ns_title);

    // Configure titlebar if needed
    if let Some(titlebar) = &options.titlebar {
        if titlebar.appears_transparent {
            let _: () = msg_send![window, setTitlebarAppearsTransparent: YES];
        }
    }

    window
}

// ============================================================================
// Custom NSView with Event Handling
// ============================================================================

unsafe fn create_view(state: Rc<RefCell<WindowState>>) -> id {
    let view_class = get_view_class();
    let view: id = msg_send![view_class, alloc];
    let view: id = msg_send![view, init];

    // Store state pointer in the view's associated object
    let state_ptr = Rc::into_raw(state) as *const c_void;
    (*view).set_ivar("_window_state", state_ptr as usize);

    view
}

fn get_view_class() -> &'static Class {
    static mut VIEW_CLASS: Option<&'static Class> = None;
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSView);
        let mut decl = ClassDecl::new("AssortedWidgetsView", superclass).unwrap();

        // Add ivar for storing window state pointer
        decl.add_ivar::<usize>("_window_state");

        // Mouse events
        decl.add_method(
            sel!(mouseDown:),
            mouse_down as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(sel!(mouseUp:), mouse_up as extern "C" fn(&Object, Sel, id));
        decl.add_method(
            sel!(mouseMoved:),
            mouse_moved as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(mouseDragged:),
            mouse_moved as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(rightMouseDown:),
            right_mouse_down as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(rightMouseUp:),
            right_mouse_up as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(scrollWheel:),
            scroll_wheel as extern "C" fn(&Object, Sel, id),
        );

        // Keyboard events
        decl.add_method(sel!(keyDown:), key_down as extern "C" fn(&Object, Sel, id));
        decl.add_method(sel!(keyUp:), key_up as extern "C" fn(&Object, Sel, id));

        // View properties
        decl.add_method(
            sel!(acceptsFirstResponder),
            accepts_first_responder as extern "C" fn(&Object, Sel) -> BOOL,
        );
        decl.add_method(
            sel!(isFlipped),
            is_flipped as extern "C" fn(&Object, Sel) -> BOOL,
        );

        VIEW_CLASS = Some(decl.register());
    });

    unsafe { VIEW_CLASS.unwrap() }
}

// ============================================================================
// Event Handlers
// ============================================================================

extern "C" fn mouse_down(this: &Object, _sel: Sel, event: id) {
    unsafe {
        if let Some(input) = convert_mouse_event(this, event, MouseButton::Left, true) {
            invoke_input_callback(this, input);
        }
    }
}

extern "C" fn mouse_up(this: &Object, _sel: Sel, event: id) {
    unsafe {
        if let Some(input) = convert_mouse_event(this, event, MouseButton::Left, false) {
            invoke_input_callback(this, input);
        }
    }
}

extern "C" fn right_mouse_down(this: &Object, _sel: Sel, event: id) {
    unsafe {
        if let Some(input) = convert_mouse_event(this, event, MouseButton::Right, true) {
            invoke_input_callback(this, input);
        }
    }
}

extern "C" fn right_mouse_up(this: &Object, _sel: Sel, event: id) {
    unsafe {
        if let Some(input) = convert_mouse_event(this, event, MouseButton::Right, false) {
            invoke_input_callback(this, input);
        }
    }
}

extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: id) {
    unsafe {
        let position = get_mouse_position(this, event);
        let modifiers = get_modifiers(event);
        let input = PlatformInput::MouseMove {
            position,
            modifiers,
        };
        invoke_input_callback(this, input);
    }
}

extern "C" fn scroll_wheel(this: &Object, _sel: Sel, event: id) {
    unsafe {
        let delta_x: f64 = msg_send![event, scrollingDeltaX];
        let delta_y: f64 = msg_send![event, scrollingDeltaY];
        let modifiers = get_modifiers(event);

        let input = PlatformInput::MouseWheel {
            delta: vector(delta_x, delta_y),
            modifiers,
        };
        invoke_input_callback(this, input);
    }
}

extern "C" fn key_down(this: &Object, _sel: Sel, event: id) {
    unsafe {
        if let Some(input) = convert_key_event(event, true) {
            invoke_input_callback(this, input);
        }
    }
}

extern "C" fn key_up(this: &Object, _sel: Sel, event: id) {
    unsafe {
        if let Some(input) = convert_key_event(event, false) {
            invoke_input_callback(this, input);
        }
    }
}

extern "C" fn accepts_first_responder(_this: &Object, _sel: Sel) -> BOOL {
    YES
}

extern "C" fn is_flipped(_this: &Object, _sel: Sel) -> BOOL {
    YES // Use top-left origin
}

// ============================================================================
// Event Conversion Helpers
// ============================================================================

unsafe fn convert_mouse_event(
    this: &Object,
    event: id,
    button: MouseButton,
    is_down: bool,
) -> Option<PlatformInput> {
    let position = get_mouse_position(this, event);
    let modifiers = get_modifiers(event);

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

unsafe fn get_mouse_position(this: &Object, event: id) -> Point {
    let ns_point: NSPoint = msg_send![event, locationInWindow];
    let view_point: NSPoint = msg_send![this as *const _ as id, convertPoint:ns_point fromView:nil];
    point(view_point.x, view_point.y)
}

unsafe fn get_modifiers(event: id) -> Modifiers {
    use cocoa::appkit::NSEventModifierFlags;
    let flags: u64 = msg_send![event, modifierFlags];

    Modifiers {
        shift: flags & NSEventModifierFlags::NSShiftKeyMask.bits() != 0,
        control: flags & NSEventModifierFlags::NSControlKeyMask.bits() != 0,
        alt: flags & NSEventModifierFlags::NSAlternateKeyMask.bits() != 0,
        command: flags & NSEventModifierFlags::NSCommandKeyMask.bits() != 0,
    }
}

unsafe fn convert_key_event(event: id, is_down: bool) -> Option<PlatformInput> {
    let characters: id = msg_send![event, characters];
    let c_str: *const i8 = msg_send![characters, UTF8String];
    if c_str.is_null() {
        return None;
    }

    let key = std::ffi::CStr::from_ptr(c_str)
        .to_string_lossy()
        .to_string();
    let modifiers = get_modifiers(event);

    Some(if is_down {
        PlatformInput::KeyDown { key, modifiers }
    } else {
        PlatformInput::KeyUp { key, modifiers }
    })
}

unsafe fn invoke_input_callback(this: &Object, input: PlatformInput) {
    if let Some(state) = get_window_state(this) {
        let mut state = state.borrow_mut();
        if let Some(callback) = state.callbacks.input.as_mut() {
            callback(input);
        }
    }
}

unsafe fn get_window_state(this: &Object) -> Option<Rc<RefCell<WindowState>>> {
    let state_ptr: usize = *this.get_ivar("_window_state");
    if state_ptr == 0 {
        return None;
    }

    let state = Rc::from_raw(state_ptr as *const RefCell<WindowState>);
    let cloned = Rc::clone(&state);
    // Don't drop the original Rc
    std::mem::forget(state);

    Some(cloned)
}

// ============================================================================
// Coordinate Conversion
// ============================================================================

fn rect_to_ns_rect(rect: Rect) -> NSRect {
    NSRect::new(
        NSPoint::new(rect.origin.x, rect.origin.y),
        NSSize::new(rect.size.width, rect.size.height),
    )
}

fn ns_rect_to_rect(ns_rect: NSRect) -> Rect {
    Rect::new(
        point(ns_rect.origin.x, ns_rect.origin.y),
        Size::new(ns_rect.size.width, ns_rect.size.height),
    )
}

// ============================================================================
// raw-window-handle implementation for wgpu
// ============================================================================

impl HasWindowHandle for MacWindow {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let handle = AppKitWindowHandle::new(std::ptr::NonNull::new(self.native_view as *mut _).unwrap());

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
