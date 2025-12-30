//! Path and stroke types for vector graphics
//!
//! Provides a builder API for creating custom vector paths (lines, bezier curves, etc.)
//! and stroke styling for line rendering.

use crate::types::Point;
use super::primitives::Color;

/// Stroke styling for lines and path outlines
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stroke {
    pub color: Color,
    pub width: f32,
    pub cap: LineCap,
    pub join: LineJoin,
}

impl Stroke {
    /// Create a new stroke with default cap and join
    pub fn new(color: Color, width: f32) -> Self {
        Self {
            color,
            width,
            cap: LineCap::Butt,
            join: LineJoin::Miter,
        }
    }

    /// Set the line cap style
    pub fn with_cap(mut self, cap: LineCap) -> Self {
        self.cap = cap;
        self
    }

    /// Set the line join style
    pub fn with_join(mut self, join: LineJoin) -> Self {
        self.join = join;
        self
    }
}

/// Line cap style (end of line)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    /// Square end at exact endpoint
    Butt,
    /// Rounded end extending past endpoint
    Round,
    /// Square end extending past endpoint
    Square,
}

impl LineCap {
    /// Convert to Lyon's LineCap
    pub fn to_lyon(&self) -> lyon::tessellation::LineCap {
        match self {
            LineCap::Butt => lyon::tessellation::LineCap::Butt,
            LineCap::Round => lyon::tessellation::LineCap::Round,
            LineCap::Square => lyon::tessellation::LineCap::Square,
        }
    }
}

/// Line join style (corners)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    /// Sharp corner (can extend far for acute angles)
    Miter,
    /// Rounded corner
    Round,
    /// Flat corner
    Bevel,
}

impl LineJoin {
    /// Convert to Lyon's LineJoin
    pub fn to_lyon(&self) -> lyon::tessellation::LineJoin {
        match self {
            LineJoin::Miter => lyon::tessellation::LineJoin::Miter,
            LineJoin::Round => lyon::tessellation::LineJoin::Round,
            LineJoin::Bevel => lyon::tessellation::LineJoin::Bevel,
        }
    }
}

/// Vector path command
#[derive(Debug, Clone, PartialEq)]
pub enum PathCommand {
    /// Move to a point (start a new subpath)
    MoveTo(Point),
    /// Draw a straight line to a point
    LineTo(Point),
    /// Draw a quadratic bezier curve
    QuadraticTo { control: Point, to: Point },
    /// Draw a cubic bezier curve
    CubicTo {
        control1: Point,
        control2: Point,
        to: Point,
    },
    /// Close the current subpath
    Close,
}

/// Vector path for custom shapes
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    commands: Vec<PathCommand>,
}

impl Path {
    /// Create a new empty path
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Create a path with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            commands: Vec::with_capacity(capacity),
        }
    }

    /// Move to a point (start a new subpath)
    pub fn move_to(&mut self, point: Point) -> &mut Self {
        self.commands.push(PathCommand::MoveTo(point));
        self
    }

    /// Draw a straight line to a point
    pub fn line_to(&mut self, point: Point) -> &mut Self {
        self.commands.push(PathCommand::LineTo(point));
        self
    }

    /// Draw a quadratic bezier curve
    pub fn quadratic_to(&mut self, control: Point, to: Point) -> &mut Self {
        self.commands
            .push(PathCommand::QuadraticTo { control, to });
        self
    }

    /// Draw a cubic bezier curve
    pub fn cubic_to(&mut self, control1: Point, control2: Point, to: Point) -> &mut Self {
        self.commands.push(PathCommand::CubicTo {
            control1,
            control2,
            to,
        });
        self
    }

    /// Close the current subpath
    pub fn close(&mut self) -> &mut Self {
        self.commands.push(PathCommand::Close);
        self
    }

    /// Get the path commands
    pub fn commands(&self) -> &[PathCommand] {
        &self.commands
    }

    /// Convert to Lyon path builder
    pub fn to_lyon_path(&self) -> lyon::path::Path {
        use lyon::geom::point;
        use lyon::path::Path as LyonPath;

        let mut builder = LyonPath::builder();
        let mut path_started = false;

        for cmd in &self.commands {
            match cmd {
                PathCommand::MoveTo(p) => {
                    // If a path was already started, end it before starting a new one
                    if path_started {
                        builder.end(false);
                    }
                    builder.begin(point(p.x as f32, p.y as f32));
                    path_started = true;
                }
                PathCommand::LineTo(p) => {
                    builder.line_to(point(p.x as f32, p.y as f32));
                }
                PathCommand::QuadraticTo { control, to } => {
                    builder.quadratic_bezier_to(
                        point(control.x as f32, control.y as f32),
                        point(to.x as f32, to.y as f32),
                    );
                }
                PathCommand::CubicTo {
                    control1,
                    control2,
                    to,
                } => {
                    builder.cubic_bezier_to(
                        point(control1.x as f32, control1.y as f32),
                        point(control2.x as f32, control2.y as f32),
                        point(to.x as f32, to.y as f32),
                    );
                }
                PathCommand::Close => {
                    builder.end(true);
                    path_started = false;
                }
            }
        }

        // If path was started but never closed, end it
        if path_started {
            builder.end(false);
        }

        builder.build()
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

/// Path builder with fluent API
impl Path {
    /// Create a rectangle path
    pub fn rect(rect: crate::types::Rect) -> Self {
        let mut path = Self::new();
        path.move_to(Point::new(rect.origin.x, rect.origin.y))
            .line_to(Point::new(
                rect.origin.x + rect.size.width,
                rect.origin.y,
            ))
            .line_to(Point::new(
                rect.origin.x + rect.size.width,
                rect.origin.y + rect.size.height,
            ))
            .line_to(Point::new(
                rect.origin.x,
                rect.origin.y + rect.size.height,
            ))
            .close();
        path
    }

    /// Create a circle path (approximated with bezier curves)
    pub fn circle(center: Point, radius: f64) -> Self {
        let mut path = Self::new();

        // Magic constant for approximating circle with cubic beziers
        let k = 0.5522847498;
        let offset = radius * k;

        // Start at right point
        path.move_to(Point::new(center.x + radius, center.y));

        // Top-right arc
        path.cubic_to(
            Point::new(center.x + radius, center.y - offset),
            Point::new(center.x + offset, center.y - radius),
            Point::new(center.x, center.y - radius),
        );

        // Top-left arc
        path.cubic_to(
            Point::new(center.x - offset, center.y - radius),
            Point::new(center.x - radius, center.y - offset),
            Point::new(center.x - radius, center.y),
        );

        // Bottom-left arc
        path.cubic_to(
            Point::new(center.x - radius, center.y + offset),
            Point::new(center.x - offset, center.y + radius),
            Point::new(center.x, center.y + radius),
        );

        // Bottom-right arc
        path.cubic_to(
            Point::new(center.x + offset, center.y + radius),
            Point::new(center.x + radius, center.y + offset),
            Point::new(center.x + radius, center.y),
        );

        path.close();
        path
    }
}
