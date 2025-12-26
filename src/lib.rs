//! # AssortedWidgets
//!
//! A GUI framework using a signal/slot architecture with flat widget storage.
//!
//! ## Architecture
//!
//! - **WidgetManager**: Flat hash table storage for all UI widgets
//! - **WidgetTree**: Tree structure using IDs for hierarchical organization
//! - **Signal/Slot**: Qt-inspired deferred message system
//! - **Thread-safe**: Cross-thread messaging via `GuiHandle`
//!
//! ## Example
//!
//! ```no_run
//! use assorted_widgets::*;
//!
//! let mut app = pollster::block_on(Application::new()).unwrap();
//! let window_id = app.create_window(WindowOptions::default()).unwrap();
//!
//! // Run event loop (never returns)
//! app.run();
//! ```

// Module declarations
pub mod application;
pub mod connection;
pub mod widget;
pub mod widget_manager;
pub mod elements;
pub mod event;
pub mod handle;
pub mod icon;
pub mod image;
pub mod layout;
pub mod paint;
pub mod platform;
pub mod raw_surface;  // RawSurface for custom GPU rendering
pub mod render;
pub mod widget_tree;
pub mod text;
pub mod types;
pub mod widgets;  // Official reusable UI widgets
pub mod window;

// Re-export public API
pub use application::Application;
pub use connection::{Connection, ConnectionTable};
pub use widget::Widget;
pub use widget_manager::WidgetManager;
pub use event::{GuiEvent, OsEvent};
pub use handle::GuiHandle;
pub use platform::{
    Modifiers, MouseButton, PlatformInput, PlatformWindow, TitlebarOptions, WindowCallbacks,
    WindowOptions,
};
pub use widget_tree::{WidgetTree, TreeNode};
pub use types::{
    point, rect, size, vector, DeferredCommand, GuiMessage, Point, Rect, ScreenPixels, Size,
    Vector, WidgetId, WindowId,
};
pub use window::Window;
