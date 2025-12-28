//! ProgressBar widget - displays progress visually
//!
//! Features:
//! - Horizontal and vertical orientation
//! - Configurable progress value (0.0 to 1.0 or custom range)
//! - Optional label above the progress bar
//! - Optional progress text/percentage display
//! - Customizable colors and appearance
//! - Indeterminate mode (future enhancement)
//!
//! # Example
//! ```rust,ignore
//! // Basic progress bar
//! let progress = ProgressBar::horizontal()
//!     .progress(0.65)
//!     .show_percentage(true);
//!
//! // Progress bar with label and custom text
//! let download = ProgressBar::horizontal()
//!     .label("Downloading...")
//!     .progress(0.45)
//!     .progress_text(|p| format!("{:.0} MB / 100 MB", p * 100.0))
//!     .fill_color(Color::rgb(0.3, 0.7, 0.4));
//! ```

use std::any::Any;

use crate::event::input::{EventResponse, InputEventEnum, MouseEvent};
use crate::event::OsEvent;
use crate::layout::Style;
use crate::paint::primitives::Color;
use crate::paint::types::{Brush, CornerRadius, ShapeStyle};
use crate::paint::PaintContext;
use crate::text::TextStyle;
use crate::types::{DeferredCommand, GuiMessage, Point, Rect, Size, WidgetId};
use crate::widget::Widget;

/// Progress bar orientation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// ProgressBar widget - non-interactive progress indicator
pub struct ProgressBar {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Orientation
    orientation: Orientation,

    // Progress value (0.0 to 1.0)
    progress: f64,

    // Label and text display
    label: Option<String>,
    show_percentage: bool,
    progress_text: Option<Box<dyn Fn(f64) -> String>>,

    // Styling
    bar_height: f32,             // Height of the progress bar
    track_color: Color,          // Background color
    fill_color: Color,           // Progress fill color
    label_color: Color,          // Label text color
    text_color: Color,           // Progress text color
    corner_radius: f32,          // Corner radius for rounded appearance

    // Pending deferred commands
    pending_commands: Vec<DeferredCommand>,
}

impl ProgressBar {
    /// Create a new progress bar with the given orientation
    pub fn new(orientation: Orientation) -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            orientation,
            progress: 0.0,
            label: None,
            show_percentage: false,
            progress_text: None,
            bar_height: 8.0,
            track_color: Color::rgba(0.3, 0.3, 0.3, 0.5),
            fill_color: Color::rgba(0.4, 0.6, 0.8, 0.9),
            label_color: Color::rgba(0.9, 0.9, 0.9, 1.0),
            text_color: Color::rgba(0.9, 0.9, 0.9, 1.0),
            corner_radius: 4.0,
            pending_commands: Vec::new(),
        }
    }

    /// Create a horizontal progress bar
    pub fn horizontal() -> Self {
        Self::new(Orientation::Horizontal)
    }

    /// Create a vertical progress bar
    pub fn vertical() -> Self {
        Self::new(Orientation::Vertical)
    }

    // ========================================================================
    // Builder Pattern API
    // ========================================================================

    /// Set the progress value (0.0 to 1.0)
    pub fn progress(mut self, progress: f64) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set the label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show/hide percentage text
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Set a custom progress text formatter
    pub fn progress_text<F>(mut self, formatter: F) -> Self
    where
        F: Fn(f64) -> String + 'static,
    {
        self.progress_text = Some(Box::new(formatter));
        self
    }

    /// Set the progress bar height
    pub fn bar_height(mut self, height: f32) -> Self {
        self.bar_height = height.max(2.0);
        self
    }

    /// Set the track (background) color
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    /// Set the fill (progress) color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Set the label text color
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    /// Set the progress text color
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius.max(0.0);
        self
    }

    /// Set layout style
    pub fn layout_style(mut self, style: Style) -> Self {
        self.layout_style = style;
        self
    }

    // ========================================================================
    // Runtime Mutation API
    // ========================================================================

    /// Set the progress value (0.0 to 1.0)
    pub fn set_progress(&mut self, progress: f64) {
        let clamped = progress.clamp(0.0, 1.0);
        if (self.progress - clamped).abs() > f64::EPSILON {
            self.progress = clamped;
            self.dirty = true;

            // Emit progress changed signal
            self.pending_commands.push(DeferredCommand {
                target: self.id,
                message: GuiMessage::Custom {
                    source: self.id,
                    signal_type: "progress_changed".to_string(),
                    data: Box::new(clamped),
                },
            });
        }
    }

    /// Get the current progress value
    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    // ========================================================================
    // Internal Helpers
    // ========================================================================

    /// Get the progress bar area (excludes label/text areas)
    fn get_bar_bounds(&self) -> Rect {
        let mut bounds = self.bounds;

        // Reserve space for label at the top
        if self.label.is_some() {
            bounds.origin.y += 24.0;
            bounds.size.height -= 24.0;
        }

        // Reserve space for percentage/text display
        if self.show_percentage || self.progress_text.is_some() {
            match self.orientation {
                Orientation::Horizontal => {
                    bounds.size.width -= 60.0; // Space for text on the right
                }
                Orientation::Vertical => {
                    bounds.size.height -= 24.0; // Space for text at bottom
                }
            }
        }

        bounds
    }

    /// Format the progress text for display
    fn format_progress(&self) -> String {
        if let Some(ref formatter) = self.progress_text {
            formatter(self.progress)
        } else if self.show_percentage {
            format!("{:.0}%", self.progress * 100.0)
        } else {
            String::new()
        }
    }
}

// ========================================================================
// Widget Trait Implementation
// ========================================================================

impl Widget for ProgressBar {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn on_message(&mut self, _message: &GuiMessage) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn on_event(&mut self, _event: &OsEvent) -> Vec<DeferredCommand> {
        Vec::new()
    }

    fn drain_deferred_commands(&mut self) -> Vec<DeferredCommand> {
        std::mem::take(&mut self.pending_commands)
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout(&self) -> Style {
        self.layout_style.clone()
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bar_bounds = self.get_bar_bounds();

        // Draw label if present
        if let Some(ref label) = self.label {
            let label_style = TextStyle::new()
                .size(14.0)
                .color(self.label_color);

            let label_pos = Point::new(self.bounds.origin.x, self.bounds.origin.y + 2.0);
            ctx.draw_text(label, &label_style, label_pos, None);
        }

        // Calculate bar rectangle (centered in bar_bounds)
        let bar_rect = match self.orientation {
            Orientation::Horizontal => Rect::new(
                Point::new(
                    bar_bounds.origin.x,
                    bar_bounds.origin.y + (bar_bounds.size.height - self.bar_height as f64) / 2.0,
                ),
                Size::new(bar_bounds.size.width, self.bar_height as f64),
            ),
            Orientation::Vertical => Rect::new(
                Point::new(
                    bar_bounds.origin.x + (bar_bounds.size.width - self.bar_height as f64) / 2.0,
                    bar_bounds.origin.y,
                ),
                Size::new(self.bar_height as f64, bar_bounds.size.height),
            ),
        };

        // Draw track background
        ctx.draw_styled_rect(
            bar_rect,
            ShapeStyle {
                fill: Brush::Solid(self.track_color),
                corner_radius: CornerRadius::uniform(self.corner_radius),
                border: None,
                shadow: None,
            },
        );

        // Draw progress fill
        let fill_rect = match self.orientation {
            Orientation::Horizontal => Rect::new(
                bar_rect.origin,
                Size::new(bar_rect.size.width * self.progress, bar_rect.size.height),
            ),
            Orientation::Vertical => {
                // Vertical fills from bottom to top
                let fill_height = bar_rect.size.height * self.progress;
                Rect::new(
                    Point::new(
                        bar_rect.origin.x,
                        bar_rect.origin.y + bar_rect.size.height - fill_height,
                    ),
                    Size::new(bar_rect.size.width, fill_height),
                )
            }
        };

        // Only draw fill if progress > 0
        if self.progress > 0.0 {
            ctx.draw_styled_rect(
                fill_rect,
                ShapeStyle {
                    fill: Brush::Solid(self.fill_color),
                    corner_radius: CornerRadius::uniform(self.corner_radius),
                    border: None,
                    shadow: None,
                },
            );
        }

        // Draw progress text if enabled
        let text = self.format_progress();
        if !text.is_empty() {
            let text_style = TextStyle::new()
                .size(14.0)
                .color(self.text_color);

            let text_pos = match self.orientation {
                Orientation::Horizontal => Point::new(
                    bar_bounds.origin.x + bar_bounds.size.width + 10.0,
                    bar_bounds.origin.y + (bar_bounds.size.height - 16.0) / 2.0,
                ),
                Orientation::Vertical => Point::new(
                    bar_bounds.origin.x,
                    bar_bounds.origin.y + bar_bounds.size.height + 8.0,
                ),
            };

            ctx.draw_text(&text, &text_style, text_pos, None);
        }
    }

    fn needs_measure(&self) -> bool {
        true
    }

    fn measure(
        &self,
        _known_dimensions: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Option<Size> {
        None
    }

    fn is_interactive(&self) -> bool {
        false // Progress bars are not interactive
    }

    fn is_focusable(&self) -> bool {
        false
    }

    fn preferred_cursor(&self) -> Option<crate::types::CursorType> {
        None
    }

    fn on_mouse_enter(&mut self, _event: &mut MouseEvent) -> EventResponse {
        EventResponse::PassThrough
    }

    fn on_mouse_leave(&mut self, _event: &mut MouseEvent) -> EventResponse {
        EventResponse::PassThrough
    }

    fn dispatch_mouse_event(&mut self, _event: &mut InputEventEnum) -> EventResponse {
        EventResponse::Ignored
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
