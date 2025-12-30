use crate::impl_widget_essentials;
use crate::paint::context::PaintContext;
use crate::paint::primitives::Color;
use crate::paint::types::Stroke;
use crate::types::{FrameInfo, Point, Rect, WidgetId};
use crate::widget::Widget;
use std::f32::consts::PI;
use taffy::prelude::*;

/// Spinner mode determines the behavior and appearance
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerMode {
    /// Indeterminate mode: spinning animation for loading (no progress value shown)
    Indeterminate,
    /// Determinate mode: shows progress value (0.0 to 1.0) as a filling circle
    Determinate,
}

/// Spinner widget for displaying loading states or progress
///
/// # Modes
/// - **Indeterminate**: Continuous spinning animation, useful for unknown duration operations
/// - **Determinate**: Static progress circle that fills from 0% to 100%
///
/// # Example
/// ```no_run
/// // Indeterminate spinner
/// let spinner = Spinner::indeterminate()
///     .size(40.0)
///     .color(Color::rgb(0.3, 0.6, 1.0));
///
/// // Determinate spinner showing 60% progress
/// let spinner = Spinner::determinate(0.6)
///     .size(60.0)
///     .show_percentage(true);
/// ```
pub struct Spinner {
    id: WidgetId,
    bounds: Rect,
    dirty: bool,
    layout_style: Style,

    // Spinner configuration
    mode: SpinnerMode,
    progress: f64, // 0.0 to 1.0, only used in Determinate mode
    size: f32,     // Diameter of the spinner
    stroke_width: f32,

    // Visual styling
    color: Color,
    background_color: Option<Color>, // Optional track color
    show_percentage: bool,            // Show percentage text in determinate mode
    font_size: f32,

    // Animation state (for indeterminate mode)
    rotation: f32, // Current rotation angle in radians
    speed: f32,    // Rotation speed in radians per second
}

impl Spinner {
    /// Create an indeterminate spinner (spinning animation)
    pub fn indeterminate() -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            mode: SpinnerMode::Indeterminate,
            progress: 0.0,
            size: 32.0,
            stroke_width: 3.0,
            color: Color::rgb(0.3, 0.6, 1.0),
            background_color: Some(Color::rgba(0.3, 0.6, 1.0, 0.2)),
            show_percentage: false,
            font_size: 14.0,
            rotation: 0.0,
            speed: 3.0 * PI, // 1.5 rotations per second
        }
    }

    /// Create a determinate spinner (progress circle)
    pub fn determinate(progress: f64) -> Self {
        Self {
            id: WidgetId::new(0),
            bounds: Rect::default(),
            dirty: true,
            layout_style: Style::default(),
            mode: SpinnerMode::Determinate,
            progress: progress.clamp(0.0, 1.0),
            size: 48.0,
            stroke_width: 4.0,
            color: Color::rgb(0.3, 0.8, 0.3),
            background_color: Some(Color::rgba(0.5, 0.5, 0.5, 0.2)),
            show_percentage: true,
            font_size: 14.0,
            rotation: 0.0,
            speed: 0.0,
        }
    }

    /// Set the spinner size (diameter)
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self.stroke_width = (size / 12.0).max(2.0); // Auto-scale stroke
        self.font_size = (size / 3.0).max(10.0); // Auto-scale font
        self
    }

    /// Set the stroke width
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    /// Set the spinner color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the background track color (None for no background)
    pub fn background_color(mut self, color: Option<Color>) -> Self {
        self.background_color = color;
        self
    }

    /// Set whether to show percentage text in determinate mode
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Set the font size for percentage text
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set rotation speed for indeterminate mode (radians per second)
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the progress value (0.0 to 1.0) for determinate mode
    pub fn set_progress(&mut self, progress: f64) {
        let clamped = progress.clamp(0.0, 1.0);
        if (self.progress - clamped).abs() > 0.001 {
            self.progress = clamped;
            self.dirty = true;
        }
    }

    /// Get the current progress value
    pub fn progress(&self) -> f64 {
        self.progress
    }

    /// Switch to indeterminate mode
    pub fn set_indeterminate(&mut self) {
        if self.mode != SpinnerMode::Indeterminate {
            self.mode = SpinnerMode::Indeterminate;
            self.rotation = 0.0;
            self.dirty = true;
        }
    }

    /// Switch to determinate mode with given progress
    pub fn set_determinate(&mut self, progress: f64) {
        let clamped = progress.clamp(0.0, 1.0);
        if self.mode != SpinnerMode::Determinate || (self.progress - clamped).abs() > 0.001 {
            self.mode = SpinnerMode::Determinate;
            self.progress = clamped;
            self.dirty = true;
        }
    }

    /// Draw an arc from start_angle to end_angle
    fn draw_arc(
        &self,
        ctx: &mut PaintContext,
        center: Point,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        color: Color,
        stroke_width: f32,
    ) {
        // Number of segments for smooth arc (more for larger arcs)
        let angle_span = (end_angle - start_angle).abs();
        let segments = (angle_span / (PI / 16.0)).ceil().max(4.0) as usize;

        let mut path = crate::paint::types::Path::new();
        let mut first = true;

        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = start_angle + (end_angle - start_angle) * t;
            let x = center.x + (radius * angle.cos()) as f64;
            let y = center.y + (radius * angle.sin()) as f64;
            let point = Point::new(x, y);

            if first {
                path.move_to(point);
                first = false;
            } else {
                path.line_to(point);
            }
        }

        ctx.stroke_path(path, Stroke::new(color, stroke_width));
    }
}

impl Widget for Spinner {
    impl_widget_essentials!();

    fn layout(&self) -> Style {
        let size_value = self.size + self.stroke_width;
        Style {
            size: taffy::Size {
                width: Dimension::length(size_value),
                height: Dimension::length(size_value),
            },
            ..self.layout_style.clone()
        }
    }

    fn needs_continuous_updates(&self) -> bool {
        // Only need continuous updates in indeterminate mode for animation
        self.mode == SpinnerMode::Indeterminate
    }

    fn update(&mut self, frame: &FrameInfo) {
        if self.mode == SpinnerMode::Indeterminate {
            // Update rotation for spinning animation
            self.rotation += self.speed * frame.dt as f32;
            // Normalize to 0..2π
            if self.rotation > 2.0 * PI {
                self.rotation -= 2.0 * PI;
            }
            self.dirty = true;
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let center = Point::new(
            self.bounds.origin.x + self.bounds.width() / 2.0,
            self.bounds.origin.y + self.bounds.height() / 2.0,
        );
        let radius = (self.size - self.stroke_width) / 2.0;

        match self.mode {
            SpinnerMode::Indeterminate => {
                // Draw background track (full circle)
                if let Some(bg_color) = self.background_color {
                    self.draw_arc(ctx, center, radius, 0.0, 2.0 * PI, bg_color, self.stroke_width);
                }

                // Draw spinning arc (3/4 circle that rotates)
                let arc_length = 1.5 * PI; // 270 degrees
                let start_angle = self.rotation - PI / 2.0; // Start at top
                let end_angle = start_angle + arc_length;

                self.draw_arc(
                    ctx,
                    center,
                    radius,
                    start_angle,
                    end_angle,
                    self.color,
                    self.stroke_width,
                );
            }
            SpinnerMode::Determinate => {
                // Draw background track (full circle)
                if let Some(bg_color) = self.background_color {
                    self.draw_arc(ctx, center, radius, 0.0, 2.0 * PI, bg_color, self.stroke_width);
                }

                // Draw progress arc (from top, clockwise)
                if self.progress > 0.0 {
                    let start_angle = -PI / 2.0; // Start at top (12 o'clock)
                    let end_angle = start_angle + (2.0 * PI * self.progress as f32);

                    self.draw_arc(
                        ctx,
                        center,
                        radius,
                        start_angle,
                        end_angle,
                        self.color,
                        self.stroke_width,
                    );
                }

                // Draw percentage text in the center
                if self.show_percentage {
                    let percentage = (self.progress * 100.0).round() as i32;
                    let text = format!("{}%", percentage);

                    let text_style = crate::text::TextStyle {
                        font_size: self.font_size,
                        text_color: self.color,
                        ..Default::default()
                    };

                    // Create layout and measure text to center it
                    let layout = ctx.with_text_engine(|engine| {
                        engine.create_layout(&text, &text_style, None, crate::text::Truncate::End)
                    });

                    let text_width = layout.width();
                    let text_height = layout.height();

                    let text_x = center.x - text_width / 2.0;
                    let text_y = center.y - text_height / 2.0;

                    ctx.draw_layout(&layout, Point::new(text_x, text_y), self.color);
                }
            }
        }
    }
}
