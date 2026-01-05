// Terminal Emulator Widget
//
// A terminal emulator widget with minimap support.
// Phase 1: Basic terminal rendering and interaction.
// Phase 2: Minimap infrastructure.

mod config;
mod minimap;

use crate::types::{Rect, Point};
use crate::widget::{Widget, EventResult};
use crate::event::GuiEvent;
use crate::paint::{PaintContext, Color};
use crate::text::TextStyle;

use alacritty_terminal::term::Term;
use alacritty_terminal::event::{Event as TermEvent, EventListener};
use alacritty_terminal::tty::Pty;
use alacritty_terminal::event_loop::Notifier;
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Point as TermPoint, Line, Column};
use alacritty_terminal::term::cell::{Cell, Flags};
use alacritty_terminal::vte::ansi::{Color as AnsiColor, NamedColor};

use std::sync::Arc;

pub use config::*;
use minimap::TerminalMinimapManager;
use crate::widgets::code_editor::CharSheet;

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

    /// Terminal configuration (fonts, etc.)
    config: TerminalConfig,

    /// DPI scale factor
    scale_factor: f32,

    /// Minimap manager (Phase 2+)
    minimap_manager: Option<TerminalMinimapManager>,

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

/// Convert ANSI color to our Color type
fn ansi_to_color(ansi_color: &AnsiColor) -> Color {
    match ansi_color {
        AnsiColor::Named(named) => match named {
            NamedColor::Black => Color::rgb(0, 0, 0),
            NamedColor::Red => Color::rgb(205, 49, 49),
            NamedColor::Green => Color::rgb(13, 188, 121),
            NamedColor::Yellow => Color::rgb(229, 229, 16),
            NamedColor::Blue => Color::rgb(36, 114, 200),
            NamedColor::Magenta => Color::rgb(188, 63, 188),
            NamedColor::Cyan => Color::rgb(17, 168, 205),
            NamedColor::White => Color::rgb(229, 229, 229),
            NamedColor::BrightBlack => Color::rgb(102, 102, 102),
            NamedColor::BrightRed => Color::rgb(241, 76, 76),
            NamedColor::BrightGreen => Color::rgb(35, 209, 139),
            NamedColor::BrightYellow => Color::rgb(245, 245, 67),
            NamedColor::BrightBlue => Color::rgb(59, 142, 234),
            NamedColor::BrightMagenta => Color::rgb(214, 112, 214),
            NamedColor::BrightCyan => Color::rgb(41, 184, 219),
            NamedColor::BrightWhite => Color::rgb(255, 255, 255),
            NamedColor::Foreground => Color::rgb(229, 229, 229),
            NamedColor::Background => Color::rgb(15, 15, 20),
            _ => Color::rgb(229, 229, 229),
        },
        AnsiColor::Spec(rgb) => Color::rgb(rgb.r, rgb.g, rgb.b),
        AnsiColor::Indexed(idx) => {
            // 256 color palette (simplified)
            // Full implementation would have complete 256 color table
            if *idx < 16 {
                // Use named colors for 0-15
                let named = match idx {
                    0 => NamedColor::Black,
                    1 => NamedColor::Red,
                    2 => NamedColor::Green,
                    3 => NamedColor::Yellow,
                    4 => NamedColor::Blue,
                    5 => NamedColor::Magenta,
                    6 => NamedColor::Cyan,
                    7 => NamedColor::White,
                    8 => NamedColor::BrightBlack,
                    9 => NamedColor::BrightRed,
                    10 => NamedColor::BrightGreen,
                    11 => NamedColor::BrightYellow,
                    12 => NamedColor::BrightBlue,
                    13 => NamedColor::BrightMagenta,
                    14 => NamedColor::BrightCyan,
                    _ => NamedColor::BrightWhite,
                };
                ansi_to_color(&AnsiColor::Named(named))
            } else {
                // Grayscale or 256 color (placeholder)
                let gray = ((idx - 16) * 10).min(255) as u8;
                Color::rgb(gray, gray, gray)
            }
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

        // Create minimap manager with shared CharSheet
        let char_sheet = Arc::new(CharSheet::with_defaults());
        let minimap_manager = Some(TerminalMinimapManager::new(
            MINIMAP_WIDTH_PIXELS,
            char_sheet,
        ));

        Self {
            bounds: Rect::zero(),
            term,
            cols,
            rows,
            cell_width: 0.0,
            cell_height: 0.0,
            scroll_offset: 0,
            config: TerminalConfig::default(),
            scale_factor: 1.0,
            minimap_manager,
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
            self.cell_height = self.config.font_size * DEFAULT_LINE_HEIGHT * self.scale_factor;
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

    /// Render minimap (Phase 2 placeholder - GPU texture upload to come)
    ///
    /// For now, this draws a simple visual representation of minimap pages.
    /// Full GPU texture implementation would upload page.intensity and page.color_rgb565.
    fn render_minimap(&self, ctx: &mut PaintContext, minimap: &TerminalMinimapManager) {
        // Calculate minimap position (right side of terminal)
        let minimap_x = self.bounds.max.x - MINIMAP_WIDTH_PIXELS as f32 - 10.0;
        let minimap_y = self.bounds.min.y + 10.0;
        let minimap_width = MINIMAP_WIDTH_PIXELS as f32;
        let minimap_height = self.bounds.height() - 20.0;

        // Draw minimap background
        let minimap_rect = Rect::new(
            Point::new(minimap_x, minimap_y),
            Point::new(minimap_x + minimap_width, minimap_y + minimap_height),
        );
        ctx.draw_rect(minimap_rect, Color::rgb(25, 25, 30));

        // Draw page indicators (placeholder visualization)
        let page_count = minimap.page_count();
        if page_count > 0 {
            let page_height = minimap_height / page_count.max(1) as f32;

            for page_num in 0..page_count {
                if let Some(page) = minimap.get_page(page_num) {
                    let y = minimap_y + (page_num as f32 * page_height);

                    // Color based on page status
                    let color = match page.status {
                        minimap::PageStatus::Clean => Color::rgb(50, 100, 50),      // Green
                        minimap::PageStatus::Dirty => Color::rgb(100, 100, 50),     // Yellow
                        minimap::PageStatus::Rasterizing => Color::rgb(50, 50, 100), // Blue
                        minimap::PageStatus::Stale => Color::rgb(100, 50, 50),      // Red
                    };

                    let page_rect = Rect::new(
                        Point::new(minimap_x + 2.0, y + 1.0),
                        Point::new(minimap_x + minimap_width - 2.0, y + page_height - 1.0),
                    );
                    ctx.draw_rect(page_rect, color);
                }
            }
        }

        // Draw viewport indicator (where user is currently viewing)
        let total_lines = self.term.grid().history_size() + self.rows;
        if total_lines > 0 {
            let viewport_start = self.scroll_offset as f32 / total_lines as f32;
            let viewport_height = self.rows as f32 / total_lines as f32;

            let indicator_y = minimap_y + (viewport_start * minimap_height);
            let indicator_height = (viewport_height * minimap_height).max(4.0);

            let indicator_rect = Rect::new(
                Point::new(minimap_x, indicator_y),
                Point::new(minimap_x + minimap_width, indicator_y + indicator_height),
            );
            ctx.draw_rect(indicator_rect, Color::rgba(255, 255, 255, 128));
        }
    }

    /// Render the terminal grid
    fn render_grid(&self, ctx: &mut PaintContext) {
        // Get grid reference
        let grid = self.term.grid();
        let display_offset = grid.display_offset();

        // Create default text style for terminal
        let mut text_style = TextStyle::default();
        text_style.font_size = self.config.font_size * self.scale_factor;
        text_style.line_height = DEFAULT_LINE_HEIGHT;
        text_style.font_family = self.config.font_family.clone();

        // Calculate visible range (in terms of grid lines)
        // display_offset is the number of lines scrolled back
        let visible_top_line = Line(-(display_offset as i32));

        // Render each visible row
        for row_idx in 0..self.rows {
            let line = Line(visible_top_line.0 + row_idx as i32);

            // Calculate Y position for this row
            let y = self.bounds.min.y + (row_idx as f32 * self.cell_height);

            // Render each column in this row
            for col_idx in 0..self.cols {
                let column = Column(col_idx);
                let point = TermPoint::new(line, column);

                // Try to get cell - skip if out of bounds
                let cell = match grid.get(line, column) {
                    Some(cell) => cell,
                    None => continue,
                };

                // Calculate X position for this column
                let x = self.bounds.min.x + (col_idx as f32 * self.cell_width);

                // Create cell rect
                let cell_rect = Rect::new(
                    Point::new(x, y),
                    Point::new(x + self.cell_width, y + self.cell_height),
                );

                // Render background color (if not default)
                let bg_color = ansi_to_color(&cell.bg);
                if bg_color != Color::rgb(15, 15, 20) {
                    ctx.draw_rect(cell_rect, bg_color);
                }

                // Get character to render
                let c = cell.c;
                if c != ' ' && c != '\0' {
                    // Get foreground color
                    let fg_color = ansi_to_color(&cell.fg);

                    // Update text style with cell's foreground color
                    text_style.text_color = fg_color;

                    // Handle bold/italic flags
                    if cell.flags.contains(Flags::BOLD) {
                        text_style.font_weight = cosmic_text::Weight::BOLD;
                    } else {
                        text_style.font_weight = cosmic_text::Weight::NORMAL;
                    }

                    if cell.flags.contains(Flags::ITALIC) {
                        text_style.font_style = cosmic_text::Style::Italic;
                    } else {
                        text_style.font_style = cosmic_text::Style::Normal;
                    }

                    // Render the character
                    let char_str = c.to_string();
                    ctx.draw_text(
                        &char_str,
                        &text_style,
                        Point::new(x, y),
                        Some(self.cell_width),
                    );
                }
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
        ctx.draw_rect(self.bounds, Color::rgb(15, 15, 20));

        // Update minimap (Phase 2)
        if let Some(ref mut minimap) = self.minimap_manager {
            let visible_line = self.scroll_offset;
            minimap.update(&self.term, visible_line);
        }

        // Render grid
        self.render_grid(ctx);

        // Render minimap (Phase 2 - placeholder for GPU texture upload)
        if let Some(ref minimap) = self.minimap_manager {
            self.render_minimap(ctx, minimap);
        }

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
