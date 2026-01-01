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
mod checkbox;
mod radio_group;
mod switch;
mod spinner;
mod scrollable_container;
pub mod rich_text_label;
pub mod text_input;
pub mod text_area;

pub use label::{Label, WrapMode, Padding};
pub use button::{Button, ButtonContent, ButtonState, ButtonStyle};
pub use button_group::{ButtonGroup, ButtonGroupItem};
pub use scrollbar::{ScrollBar, Orientation, ScrollBarState};
pub use slider::Slider;
pub use progressbar::ProgressBar;
pub use checkbox::{Checkbox, CheckboxState, CheckboxStyle};
pub use radio_group::{RadioGroup, RadioGroupItem, RadioStyle};
pub use switch::{Switch, SwitchState};
pub use spinner::{Spinner, SpinnerMode};
pub use scrollable_container::{ScrollableContainer, ScrollMode};
pub use rich_text_label::RichTextLabel;
pub use text_input::{TextInput, InputState, InputStyle};
pub use text_area::{TextArea, TextAreaStyle};
