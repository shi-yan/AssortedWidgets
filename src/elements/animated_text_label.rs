//! Animated Text Label - Demonstrates Dynamic Text Truncation
//!
//! This element showcases:
//! - Dynamic width animation (simulates sidebar resize)
//! - Automatic text truncation with ellipsis ("...")
//! - Real-time text re-layout as container changes
//! - Performance of text shaping during continuous resizing
//!
//! Use case: Menu items in a collapsible sidebar that need to truncate
//! gracefully as the sidebar width changes.

use std::any::Any;
use std::time::Instant;

use crate::widget::Widget;
use crate::WidgetState;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::text::{TextStyle, TextAlign, Truncate};
use crate::types::{DirtyLevel, DeferredCommand, FrameInfo, GuiMessage, Point, Rect, Size, WidgetId};
use taffy::AvailableSpace;

/// An animated label that demonstrates dynamic text truncation
///
/// The container width oscillates, causing text to:
/// - Truncate with "..." when too narrow
/// - Expand to full text when wide enough
/// - Re-shape on every frame to test performance
pub struct AnimatedTextLabel {
    state: WidgetState,
    layout_style: Style,
    style: Style,

    // Text content
    text: String,
    text_style: TextStyle,
    bg_color: Color,

    // Animation state
    _start_time: Instant,
    _min_width: f64,       // Minimum width (text truncated heavily)
    _max_width: f64,       // Maximum width (text fully visible)
    _frequency: f64,       // Oscillation frequency (Hz)
    fixed_height: f64,    // Fixed height
}

impl AnimatedTextLabel {
    /// Create a new animated text label
    ///
    /// # Arguments
    /// * `id` - Widget identifier
    /// * `text` - The text to display (should be long enough to truncate)
    /// * `min_width` - Minimum width of the animation
    /// * `max_width` - Maximum width of the animation
    pub fn new(id: WidgetId, text: impl Into<String>, min_width: f64, max_width: f64) -> Self {
        AnimatedTextLabel {
            state: WidgetState::with_id(id),
            layout_style: Style::default(),
            style: Style::default(),
            text: text.into(),
            text_style: TextStyle::new()
                .size(18.0)
                .color(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })
                .align(TextAlign::Left),
            bg_color: Color { r: 1.0, g: 0.3, b: 0.4, a: 1.0 },
            _start_time: Instant::now(),
            _min_width: min_width,
            _max_width: max_width,
            _frequency: 0.3,  // 0.3 Hz = ~3.3 second cycle
            fixed_height: 50.0,
        }
    }

    /// Set background color
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Set text style
    pub fn with_text_style(mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self
    }

    /// Set layout style
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Get current animated width based on elapsed time
    fn current_width(&self) -> f64 {
        // Todo: global timer
        /* 
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let angle = 2.0 * std::f64::consts::PI * self.frequency * elapsed;

        // Oscillate between min_width and max_width
        let range = (self.max_width - self.min_width) / 2.0;
        let center = (self.max_width + self.min_width) / 2.0;
        center + range * angle.sin()
        */

        return 300.0;
    }
}

impl Widget for AnimatedTextLabel {
    fn id(&self) -> WidgetId {
        self.state.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.state.id = id;
    }

    fn bounds(&self) -> Rect {
        self.state.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.state.bounds = bounds;
    }

    fn dirty_level(&self) -> crate::types::DirtyLevel {
        self.state.dirty
    }

    fn set_dirty_level(&mut self, level: crate::types::DirtyLevel) {
        self.state.dirty = level;
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.set_dirty_level(crate::types::DirtyLevel::from(dirty));
    }

    fn is_dirty(&self) -> bool {
        self.dirty_level().needs_repaint()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }




    fn layout(&self) -> Style {
        self.style.clone()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw background
        ctx.draw_rect(self.state.bounds, self.bg_color);
        println!("[AnimatedTextLabel] paint() - bounds: x={:.1}, y={:.1}, w={:.1}, h={:.1}",
            self.state.bounds.origin.x, self.state.bounds.origin.y,
            self.state.bounds.size.width, self.state.bounds.size.height);
        // Use bounds width (not current_width!) to ensure text fits within layout-determined bounds
        // The bounds were set by measure() which called current_width() at layout time
        let padding = 10.0;
        let text_width = (self.state.bounds.size.width - 2.0 * padding).max(0.0) as f32;

        // Create text layout with ellipsis truncation
        // As width shrinks → text truncates with "..."
        // As width grows → more text becomes visible
        let layout = ctx.create_text_layout(
            &self.text,
            &self.text_style,
            Some(text_width),
            Truncate::End,  // Enable "..." truncation
        );

        // Calculate text position (centered vertically with padding)
        let text_y = self.state.bounds.origin.y + (self.fixed_height - layout.height()).max(0.0) / 2.0;
        let text_pos = Point::new(
            self.state.bounds.origin.x + padding,
            text_y,
        );

        // Draw the truncated text
        ctx.draw_layout(&layout, text_pos, self.text_style.text_color);
    }

    /// Measure function: returns current animated width
    ///
    /// This demonstrates how text truncation adapts to dynamic container sizes.
    /// Similar to a sidebar menu that animates between collapsed and expanded states.
    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<AvailableSpace>,
    ) -> Option<Size> {
        // Get current animated width
        let width = self.current_width();

        // Return current animated size
        Some(Size::new(width, self.fixed_height))
    }

    /// This element needs custom measurement
    fn needs_measure(&self) -> bool {
        true
    }

    /// Update animation state each frame
    fn update(&mut self, _frame: &FrameInfo) {
        // Animation changes intrinsic size, so mark layout as dirty
        // This triggers re-measurement and layout recalculation
        self.mark_needs_layout();
    }

    /// This element needs continuous updates for animation
    fn needs_continuous_updates(&self) -> bool {
        true  // Always animating
    }
}
