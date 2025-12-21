//! macOS platform implementation using objc2-app-kit

mod window;

pub use window::MacWindow;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, MainThreadOnly};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol};
use std::sync::Once;

static INIT: Once = Once::new();

// ============================================================================
// Application Delegate
// ============================================================================

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `AppDelegate` does not implement `Drop`.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "AssortedWidgetsAppDelegate"]
    struct AppDelegate;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for AppDelegate {}

    // SAFETY: `NSApplicationDelegate` has no safety requirements.
    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(applicationShouldTerminateAfterLastWindowClosed:))]
        fn application_should_terminate_after_last_window_closed(
            &self,
            _sender: &NSApplication,
        ) -> bool {
            true
        }
    }
);

impl AppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        // SAFETY: The signature of `NSObject`'s `init` method is correct.
        unsafe { msg_send![Self::alloc(mtm), init] }
    }
}

/// Initialize macOS application
/// Must be called before creating any windows
pub fn init() {
    INIT.call_once(|| {
        let mtm = MainThreadMarker::new().expect("Must be called on main thread");

        // Initialize NSApplication
        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

        // Set up application delegate to quit when last window closes
        let delegate = AppDelegate::new(mtm);
        let delegate_obj = ProtocolObject::from_ref(&*delegate);
        app.setDelegate(Some(delegate_obj));

        // Keep delegate alive
        std::mem::forget(delegate);
    });
}
