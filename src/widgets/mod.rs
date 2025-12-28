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
mod button_group;
mod scrollbar;
mod slider;
mod progressbar;
pub mod rich_text_label;

pub use label::{Label, WrapMode, Padding};
pub use button::{Button, ButtonContent, ButtonState, ButtonStyle};
pub use button_group::{ButtonGroup, ButtonGroupItem};
pub use scrollbar::{ScrollBar, Orientation, ScrollBarState};
pub use slider::Slider;
pub use progressbar::ProgressBar;
pub use rich_text_label::RichTextLabel;
