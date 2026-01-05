// Terminal Emulator Widget
//
// A terminal emulator widget with minimap support.
// Phase 1: Basic terminal rendering and interaction.

mod config;

use crate::types::{Rect, Point};
use crate::widget::{Widget, EventResult};
use crate::event::GuiEvent;
use crate::paint::PaintContext;

use alacritty_terminal::term::Term;
use alacritty_terminal::event::{Event as TermEvent, EventListener};
use alacritty_terminal::tty::Pty;
use alacritty_terminal::event_loop::Notifier;
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Point as TermPoint, Line, Column};
use alacritty_terminal::term::cell::Cell;

pub use config::*;

/// Terminal emulator widget
///
/// Phase 1: Basic terminal with text rendering, keyboard/mouse input, and scrollback.
/// Phase 2+: Minimap integration.
pub struct TerminalEmulator {
    /// Widget bounds
    bounds: Rect,

    /// Terminal state (managed by alacritty_terminal)
    term: Term<EventListenerImpl>,

    /// Terminal dimensions
    cols: usize,
    rows: usize,

    /// Cell dimensions in pixels
    cell_width: f32,
    cell_height: f32,

    /// Current scroll offset (lines from bottom)
    scroll_offset: usize,

    /// Font size
    font_size: f32,

    /// DPI scale factor
    scale_factor: f32,

    /// Whether the terminal needs redraw
    dirty: bool,
}

/// Event listener for terminal events
///
/// This receives notifications from alacritty_terminal when the grid changes.
struct EventListenerImpl {
    dirty: bool,
}

impl EventListener for EventListenerImpl {
    fn send_event(&self, event: TermEvent) {
        // For Phase 1, we'll just mark dirty on any event
        // In later phases, this could trigger more granular updates
        match event {
            TermEvent::PtyWrite(_) => {
                // Writing to PTY (Phase 8+)
            }
            TermEvent::Title(_) => {
                // Window title change
            }
            TermEvent::ResetTitle => {
                // Reset title
            }
            TermEvent::CursorBlinkingChange => {
                // Cursor blink state changed
            }
            TermEvent::Bell => {
                // Bell/beep
            }
            _ => {}
        }
    }
}

impl TerminalEmulator {
    /// Create a new terminal emulator
    ///
    /// # Arguments
    /// * `cols` - Terminal width in columns
    /// * `rows` - Terminal height in rows
    pub fn new(cols: usize, rows: usize) -> Self {
        let event_listener = EventListenerImpl { dirty: false };

        // Create terminal with configuration
        let term = Term::new(
            alacritty_terminal::term::Config::default(),
            &alacritty_terminal::grid::Dimensions {
                columns: cols,
                lines: rows,
            },
            event_listener,
        );

        Self {
            bounds: Rect::zero(),
            term,
            cols,
            rows,
            cell_width: 0.0,
            cell_height: 0.0,
            scroll_offset: 0,
            font_size: DEFAULT_FONT_SIZE,
            scale_factor: 1.0,
            dirty: true,
        }
    }

    /// Create with default dimensions
    pub fn with_defaults() -> Self {
        Self::new(DEFAULT_COLS, DEFAULT_ROWS)
    }

    /// Get total lines (scrollback + visible)
    pub fn total_lines(&self) -> usize {
        self.term.grid().history_size() + self.rows
    }

    /// Set scroll offset (lines from bottom)
    pub fn set_scroll_offset(&mut self, offset: usize) {
        let max_offset = self.term.grid().history_size();
        self.scroll_offset = offset.min(max_offset);
        self.dirty = true;
    }

    /// Scroll by delta (positive = scroll up, negative = scroll down)
    pub fn scroll(&mut self, delta: i32) {
        if delta > 0 {
            // Scroll up
            let new_offset = self.scroll_offset.saturating_add(delta as usize);
            self.set_scroll_offset(new_offset);
        } else {
            // Scroll down
            let delta_abs = delta.abs() as usize;
            self.set_scroll_offset(self.scroll_offset.saturating_sub(delta_abs));
        }
    }

    /// Calculate cell dimensions based on current bounds
    fn update_cell_dimensions(&mut self) {
        if self.cols > 0 && self.rows > 0 {
            self.cell_width = self.bounds.width() / self.cols as f32;
            self.cell_height = self.font_size * DEFAULT_LINE_HEIGHT * self.scale_factor;
        }
    }

    /// Process keyboard input
    ///
    /// Converts GUI key events to terminal input.
    fn handle_keyboard(&mut self, _key: &str, _mods: u32) {
        // Phase 1: Placeholder for keyboard handling
        // In full implementation, this would:
        // 1. Convert GUI key events to VT sequences
        // 2. Write to PTY (Phase 8+)
        // 3. Handle special keys (arrows, function keys, etc.)
        self.dirty = true;
    }

    /// Handle mouse click
    fn handle_mouse_click(&mut self, _x: f32, _y: f32) {
        // Phase 1: Basic click handling
        // Later: Selection, text copy, etc.
        self.dirty = true;
    }

    /// Render the terminal grid
    fn render_grid(&self, ctx: &mut PaintContext) {
        // Get grid reference
        let grid = self.term.grid();
        let display_offset = self.term.grid().display_offset();

        // Calculate visible range
        let start_line = display_offset + self.scroll_offset;
        let end_line = (start_line + self.rows).min(self.total_lines());

        // Render each visible line
        for row_idx in 0..self.rows {
            let grid_line = start_line + row_idx;
            if grid_line >= end_line {
                break;
            }

            // Calculate Y position
            let y = self.bounds.min.y + (row_idx as f32 * self.cell_height);

            // Get line from grid
            // Note: This is simplified - actual implementation needs proper Line/Column indexing
            let line = Line(grid_line as i32);

            // Render cells in this line
            for col_idx in 0..self.cols {
                let column = Column(col_idx);
                let point = TermPoint::new(line, column);

                // Get cell (this will need proper bounds checking)
                // let cell = &grid[point];

                // Calculate X position
                let x = self.bounds.min.x + (col_idx as f32 * self.cell_width);

                // Render cell (Phase 1: placeholder)
                // In full implementation:
                // 1. Extract character, colors, and flags from cell
                // 2. Render background color
                // 3. Render character with foreground color
                // 4. Handle bold, italic, underline flags

                // For now, just draw a placeholder
                let cell_rect = Rect::new(
                    Point::new(x, y),
                    Point::new(x + self.cell_width, y + self.cell_height),
                );

                // Placeholder: Draw cell background
                ctx.draw_rect(cell_rect, crate::paint::Color::rgb(20, 20, 25));
            }
        }
    }
}

impl Widget for TerminalEmulator {
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.update_cell_dimensions();
        self.dirty = true;
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn paint(&mut self, ctx: &mut PaintContext) {
        // Draw background
        ctx.draw_rect(self.bounds, crate::paint::Color::rgb(15, 15, 20));

        // Render grid
        self.render_grid(ctx);

        self.dirty = false;
    }

    fn handle_event(&mut self, event: &GuiEvent) -> EventResult {
        match event {
            GuiEvent::MouseDown { x, y, .. } => {
                if self.bounds.contains(Point::new(*x, *y)) {
                    self.handle_mouse_click(*x, *y);
                    return EventResult::Consumed;
                }
            }
            GuiEvent::Scroll { delta_y, .. } => {
                // Convert scroll delta to lines
                let lines = (*delta_y / self.cell_height) as i32;
                self.scroll(lines);
                return EventResult::Consumed;
            }
            GuiEvent::KeyPress { key, modifiers } => {
                self.handle_keyboard(key, *modifiers);
                return EventResult::Consumed;
            }
            _ => {}
        }

        EventResult::Ignored
    }

    fn wants_focus(&self) -> bool {
        true
    }

    fn set_scale_factor(&mut self, scale: f32) {
        self.scale_factor = scale;
        self.update_cell_dimensions();
        self.dirty = true;
    }
}
