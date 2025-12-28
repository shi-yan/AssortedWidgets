//! Official reusable UI widgets for AssortedWidgets
//!
//! This module contains production-ready UI widgets with ergonomic APIs
//! and sensible defaults. All widgets follow these principles:
//! - Builder pattern for initialization
//! - Runtime mutation methods where appropriate
//! - Proper layout integration via measure()
//! - Clean separation of concerns

mod label;
mod button;
mod scrollbar;

pub use label::{Label, WrapMode, Padding};
pub use button::{Button, ButtonContent, ButtonState, ButtonStyle};
pub use scrollbar::{ScrollBar, Orientation, ScrollBarState};
