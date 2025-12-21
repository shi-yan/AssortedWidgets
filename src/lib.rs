//! # AssortedWidgets
//!
//! A GUI framework using a signal/slot architecture with flat element storage.
//!
//! ## Architecture
//!
//! - **ElementManager**: Flat hash table storage for all UI elements
//! - **SceneGraph**: Tree structure using IDs for hierarchical organization
//! - **Signal/Slot**: Qt-inspired deferred message system
//! - **Thread-safe**: Cross-thread messaging via `GuiHandle`
//!
//! ## Example
//!
//! ```no_run
//! use assorted_widgets::*;
//!
//! let mut event_loop = GuiEventLoop::new();
//! let handle = event_loop.get_handle();
//!
//! // Run event loop
//! event_loop.process_events();
//! ```

// Module declarations
pub mod connection;
pub mod element;
pub mod element_manager;
pub mod elements;
pub mod event;
pub mod event_loop;
pub mod handle;
pub mod layout;
pub mod paint;
pub mod platform;
pub mod render;
pub mod scene_graph;
pub mod text;
pub mod types;

// Re-export public API
pub use connection::{Connection, ConnectionTable};
pub use element::Element;
pub use element_manager::ElementManager;
pub use event::{GuiEvent, OsEvent};
pub use event_loop::GuiEventLoop;
pub use handle::GuiHandle;
pub use platform::{
    Modifiers, MouseButton, PlatformInput, PlatformWindow, TitlebarOptions, WindowCallbacks,
    WindowOptions,
};
pub use scene_graph::{SceneGraph, SceneNode};
pub use types::{
    point, rect, size, vector, DeferredCommand, GuiMessage, Point, Rect, ScreenPixels, Size,
    Vector, WidgetId,
};
