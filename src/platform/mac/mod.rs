//! macOS platform implementation using Cocoa/AppKit

mod window;

pub use window::MacWindow;

use cocoa::appkit::NSApplication;
use cocoa::base::{id, nil, BOOL, YES};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize macOS application
/// Must be called before creating any windows
pub fn init() {
    INIT.call_once(|| {
        unsafe {
            // Initialize NSApplication
            let app = NSApplication::sharedApplication(nil);
            app.setActivationPolicy_(
                cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
            );

            // Set up application delegate to quit when last window closes
            let delegate = create_app_delegate();
            let _: () = msg_send![app, setDelegate: delegate];
        }
    });
}

/// Create NSApplicationDelegate that quits when all windows close
unsafe fn create_app_delegate() -> id {
    let delegate_class = get_app_delegate_class();
    let delegate: id = msg_send![delegate_class, new];
    delegate
}

fn get_app_delegate_class() -> &'static Class {
    static mut DELEGATE_CLASS: Option<&'static Class> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("AssortedWidgetsAppDelegate", superclass).unwrap();

        decl.add_method(
            sel!(applicationShouldTerminateAfterLastWindowClosed:),
            app_should_terminate as extern "C" fn(&Object, Sel, id) -> BOOL,
        );

        DELEGATE_CLASS = Some(decl.register());
    });

    unsafe { DELEGATE_CLASS.unwrap() }
}

extern "C" fn app_should_terminate(_this: &Object, _sel: Sel, _sender: id) -> BOOL {
    YES // Quit when all windows are closed
}
