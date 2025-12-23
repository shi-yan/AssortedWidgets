mod container;
mod debug_rect;
mod clipped_container;
mod animated_rect;
mod text_demo;  // Phase 3.2: Clean API demo element
mod text_label;  // Phase 3.3: TextLabel with measure() integration
mod animated_text_label;  // Phase 3.3: Dynamic text truncation demo
mod clickable_rect;  // Phase 2: Event handling demo
mod simple_input_box;  // Phase 2.2: IME and focus testing
mod draggable_rect;  // Phase 2.2: Mouse capture and cross-window drag-drop
// mod text_test;  // Disabled - Phase 3.1 API is outdated

pub use container::Container;
pub use debug_rect::DebugRect;
pub use clipped_container::ClippedContainer;
pub use animated_rect::AnimatedRect;
pub use text_demo::TextDemoElement;
pub use text_label::TextLabel;
pub use animated_text_label::AnimatedTextLabel;
pub use clickable_rect::ClickableRect;
pub use simple_input_box::SimpleInputBox;
pub use draggable_rect::DraggableRect;
// pub use text_test::TextTestElement;
