//! macOS platform implementation using Cocoa/AppKit

mod window;

pub use window::MacWindow;

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize macOS application
/// Must be called before creating any windows
pub fn init() {
    INIT.call_once(|| {
        unsafe {
            // Initialize NSApplication
            use cocoa::appkit::NSApplication;
            use cocoa::base::nil;

            let app = NSApplication::sharedApplication(nil);
            app.setActivationPolicy_(
                cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
            );
        }
    });
}
