use std::any::Any;
use std::time::Instant;

use crate::widget::Widget;
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::{Color, PaintContext};
use crate::types::{DeferredCommand, GuiMessage, Rect, Size, WidgetId};
use taffy::AvailableSpace;

/// An animated rectangle that changes its intrinsic width over time
///
/// This widget demonstrates measure functions and dynamic layout.
/// The width oscillates using a sine wave, causing the layout system
/// to recalculate from leaves to root.
pub struct AnimatedRect {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    color: Color,
    style: Style,

    // Animation state
    start_time: Instant,
    base_width: f64,      // Base width (center of oscillation)
    amplitude: f64,       // Oscillation amplitude
    frequency: f64,       // Oscillation frequency (Hz)
    fixed_height: f64,    // Fixed height
}

impl AnimatedRect {
    pub fn new(id: WidgetId, color: Color, base_width: f64, amplitude: f64) -> Self {
        AnimatedRect {
            id,
            bounds: Rect::default(),
            dirty: true,
            color,
            style: Style::default(),
            start_time: Instant::now(),
            base_width,
            amplitude,
            frequency: 0.5,  // 0.5 Hz = 2 second cycle
            fixed_height: 100.0,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Get current animated width based on elapsed time
    fn current_width(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let angle = 2.0 * std::f64::consts::PI * self.frequency * elapsed;
        self.base_width + self.amplitude * angle.sin()
    }

    /// Update animation (call this each frame)
    pub fn update(&mut self) {
        // Animation state changes, so mark layout as dirty
        // This triggers re-measurement and layout recalculation
        self.mark_needs_layout();
    }
}

impl Widget for AnimatedRect {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        println!("[AnimatedRect] set_bounds() - width: {:.1}px, height: {:.1}px",
                 bounds.size.width, bounds.size.height);
        self.bounds = bounds;
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout(&self) -> Style {
        self.style.clone()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw a filled rectangle with our color
        ctx.draw_rect(self.bounds, self.color);
    }

    /// Measure function: returns current animated width
    ///
    /// This demonstrates leaves → root layout flow:
    /// 1. Element's intrinsic size changes over time
    /// 2. Parent container adjusts to fit
    /// 3. Changes propagate up to root
    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<AvailableSpace>,
    ) -> Option<Size> {
        // Get current animated width
        let width = self.current_width();
        let elapsed = self.start_time.elapsed().as_secs_f64();

        println!("[AnimatedRect] measure() called - elapsed: {:.2}s, width: {:.1}px", elapsed, width);

        // Return current animated size
        Some(Size::new(width, self.fixed_height))
    }

    /// This element needs custom measurement
    fn needs_measure(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
