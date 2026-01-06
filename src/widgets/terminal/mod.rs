// Terminal Emulator Widget
//
// A terminal emulator widget with minimap support.
// Phase 1: Basic terminal rendering and interaction.
// Phase 2: Minimap infrastructure.

mod config;
mod minimap;

use crate::types::{Rect, Point, DirtyLevel, WidgetId};
use crate::widget::Widget;
use crate::event::GuiEvent;
use crate::paint::{PaintContext, Color};
use crate::text::TextStyle;
use crate::WidgetState;
use std::any::Any;

use alacritty_terminal::term::Term;
use alacritty_terminal::event::{Event as TermEvent, EventListener};
use alacritty_terminal::tty::Pty;
use alacritty_terminal::event_loop::Notifier;
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Point as TermPoint, Line, Column};
use alacritty_terminal::term::cell::{Cell, Flags};
use alacritty_terminal::term::RenderableContent;
use alacritty_terminal::vte::ansi::{Color as AnsiColor, NamedColor, Processor};

use std::sync::Arc;
use std::time::{Duration, Instant};

pub use config::*;
use minimap::TerminalMinimapManager;
use crate::widgets::code_editor::CharSheet;

/// Terminal emulator widget
///
/// Phase 1: Basic terminal with text rendering, keyboard/mouse input, and scrollback.
/// Phase 2+: Minimap integration.
pub struct TerminalEmulator {
    /// Widget state (ID, dirty tracking)
    state: WidgetState,

    /// Widget bounds
    bounds: Rect,

    /// Terminal state (managed by alacritty_terminal)
    term: Term<EventListenerImpl>,

    /// VTE parser (processes input bytes)
    parser: Processor,

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

    /// Resize debouncing: Last resize time (Phase 4)
    last_resize_time: Option<Instant>,

    /// Resize debouncing: Minimum time between resizes (milliseconds)
    resize_debounce_ms: u64,

    /// Pending resize dimensions (cols, rows)
    pending_resize: Option<(usize, usize)>,
}

/// Event listener for terminal events
///
/// This receives notifications from alacritty_terminal when the grid changes.
struct EventListenerImpl {
    dirty: bool,
}

/// Simple size implementation for Dimensions trait
struct SimpleSize {
    cols: usize,
    rows: usize,
}

impl Dimensions for SimpleSize {
    fn total_lines(&self) -> usize {
        self.rows
    }

    fn screen_lines(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.cols
    }
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
            NamedColor::Black => Color::rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0),
            NamedColor::Red => Color::rgb(205.0 / 255.0, 49.0 / 255.0, 49.0 / 255.0),
            NamedColor::Green => Color::rgb(13.0 / 255.0, 188.0 / 255.0, 121.0 / 255.0),
            NamedColor::Yellow => Color::rgb(229.0 / 255.0, 229.0 / 255.0, 16.0 / 255.0),
            NamedColor::Blue => Color::rgb(36.0 / 255.0, 114.0 / 255.0, 200.0 / 255.0),
            NamedColor::Magenta => Color::rgb(188.0 / 255.0, 63.0 / 255.0, 188.0 / 255.0),
            NamedColor::Cyan => Color::rgb(17.0 / 255.0, 168.0 / 255.0, 205.0 / 255.0),
            NamedColor::White => Color::rgb(229.0 / 255.0, 229.0 / 255.0, 229.0 / 255.0),
            NamedColor::BrightBlack => Color::rgb(102.0 / 255.0, 102.0 / 255.0, 102.0 / 255.0),
            NamedColor::BrightRed => Color::rgb(241.0 / 255.0, 76.0 / 255.0, 76.0 / 255.0),
            NamedColor::BrightGreen => Color::rgb(35.0 / 255.0, 209.0 / 255.0, 139.0 / 255.0),
            NamedColor::BrightYellow => Color::rgb(245.0 / 255.0, 245.0 / 255.0, 67.0 / 255.0),
            NamedColor::BrightBlue => Color::rgb(59.0 / 255.0, 142.0 / 255.0, 234.0 / 255.0),
            NamedColor::BrightMagenta => Color::rgb(214.0 / 255.0, 112.0 / 255.0, 214.0 / 255.0),
            NamedColor::BrightCyan => Color::rgb(41.0 / 255.0, 184.0 / 255.0, 219.0 / 255.0),
            NamedColor::BrightWhite => Color::rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0),
            NamedColor::Foreground => Color::rgb(229.0 / 255.0, 229.0 / 255.0, 229.0 / 255.0),
            NamedColor::Background => Color::rgb(15.0 / 255.0, 15.0 / 255.0, 20.0 / 255.0),
            _ => Color::rgb(229.0 / 255.0, 229.0 / 255.0, 229.0 / 255.0),
        },
        AnsiColor::Spec(rgb) => Color::rgb(rgb.r as f32 / 255.0, rgb.g as f32 / 255.0, rgb.b as f32 / 255.0),
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
                let gray = ((idx - 16) * 10).min(255) as f32 / 255.0;
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
        let dims = SimpleSize { cols, rows };
        let term = Term::new(
            alacritty_terminal::term::Config::default(),
            &dims,
            event_listener,
        );

        // Create minimap manager with shared CharSheet
        let char_sheet = Arc::new(CharSheet::with_defaults());
        let minimap_manager = Some(TerminalMinimapManager::new(
            MINIMAP_WIDTH_PIXELS,
            char_sheet,
        ));

        Self {
            state: WidgetState::new(),
            bounds: Rect::zero(),
            term,
            parser: Processor::new(),
            cols,
            rows,
            cell_width: 0.0,
            cell_height: 0.0,
            scroll_offset: 0,
            config: TerminalConfig::default(),
            scale_factor: 1.0,
            minimap_manager,
            dirty: true,
            last_resize_time: None,
            resize_debounce_ms: 150,  // 150ms debounce for resize
            pending_resize: None,
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

    /// Check if terminal is in alternate screen mode (Phase 6)
    ///
    /// Alternate screen is used by full-screen applications like vim, less, htop.
    /// Returns true if in alternate screen mode.
    pub fn is_alternate_screen(&self) -> bool {
        // alacritty_terminal provides mode() method
        self.term.mode().contains(alacritty_terminal::term::TermMode::ALT_SCREEN)
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

    /// Write bytes to the terminal (processes VT sequences)
    ///
    /// For Phase 2 demo purposes. In Phase 8, this would come from PTY.
    pub fn write(&mut self, data: &[u8]) {
        // Process bytes through VTE parser
        // This updates the terminal grid with the parsed content
        self.parser.advance(&mut self.term, data);
        self.dirty = true;
    }

    /// Check and handle pending resize (Phase 4)
    ///
    /// Called during paint() to apply debounced resizes.
    fn check_pending_resize(&mut self) {
        if let Some((new_cols, new_rows)) = self.pending_resize {
            let should_resize = if let Some(last_time) = self.last_resize_time {
                let elapsed = last_time.elapsed();
                elapsed >= Duration::from_millis(self.resize_debounce_ms)
            } else {
                true  // First resize, always proceed
            };

            if should_resize {
                self.apply_resize(new_cols, new_rows);
                self.pending_resize = None;
            }
        }
    }

    /// Apply terminal resize (Phase 4)
    ///
    /// Resizes the terminal grid and invalidates all minimap pages.
    fn apply_resize(&mut self, new_cols: usize, new_rows: usize) {
        if new_cols == self.cols && new_rows == self.rows {
            return;  // No change
        }

        // Resize the terminal grid
        let dims = SimpleSize { cols: new_cols, rows: new_rows };
        self.term.resize(dims);

        self.cols = new_cols;
        self.rows = new_rows;

        // Update cell dimensions
        self.update_cell_dimensions();

        // Invalidate all minimap pages (reflow occurred)
        if let Some(ref mut minimap) = self.minimap_manager {
            minimap.invalidate_all();
        }

        self.last_resize_time = Some(Instant::now());
        self.dirty = true;
    }

    /// Calculate cell dimensions based on current bounds
    fn update_cell_dimensions(&mut self) {
        if self.cols > 0 && self.rows > 0 {
            self.cell_width = self.bounds.width() as f32 / self.cols as f32;
            self.cell_height = self.config.font_size * DEFAULT_LINE_HEIGHT * self.scale_factor;
        }
    }

    /// Calculate terminal dimensions from pixel bounds (Phase 4)
    ///
    /// Returns (cols, rows) that would fit in the given bounds.
    fn calculate_dimensions_from_bounds(&self) -> (usize, usize) {
        let cell_height = self.config.font_size * DEFAULT_LINE_HEIGHT * self.scale_factor;
        let assumed_cell_width = cell_height * 0.6;  // Approximate monospace ratio

        let cols = ((self.bounds.width() as f32 - MINIMAP_WIDTH_PIXELS as f32 - 20.0) / assumed_cell_width).max(1.0) as usize;
        let rows = (self.bounds.height() as f32 / cell_height).max(1.0) as usize;

        (cols.max(10), rows.max(3))  // Minimum viable dimensions
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
        let minimap_x = self.bounds.max().x as f32 - MINIMAP_WIDTH_PIXELS as f32 - 10.0;
        let minimap_y = self.bounds.min().y as f32 + 10.0;
        let minimap_width = MINIMAP_WIDTH_PIXELS as f32;
        let minimap_height = self.bounds.height() as f32 - 20.0;

        // Draw minimap background
        let minimap_rect = Rect::new(
            Point::new(minimap_x as f64, minimap_y as f64),
            crate::types::Size::new(minimap_width as f64, minimap_height as f64),
        );
        ctx.draw_rect(minimap_rect, Color::rgb(25.0 / 255.0, 25.0 / 255.0, 30.0 / 255.0));

        // Draw page indicators (placeholder visualization)
        let page_count = minimap.page_count();
        if page_count > 0 {
            let page_height = minimap_height / page_count.max(1) as f32;

            for page_num in 0..page_count {
                if let Some(page) = minimap.get_page(page_num) {
                    let y = minimap_y + (page_num as f32 * page_height);

                    // Color based on page status
                    let color = match page.status {
                        minimap::PageStatus::Clean => Color::rgb(50.0 / 255.0, 100.0 / 255.0, 50.0 / 255.0),      // Green
                        minimap::PageStatus::Dirty => Color::rgb(100.0 / 255.0, 100.0 / 255.0, 50.0 / 255.0),     // Yellow
                        minimap::PageStatus::Rasterizing => Color::rgb(50.0 / 255.0, 50.0 / 255.0, 100.0 / 255.0), // Blue
                        minimap::PageStatus::Stale => Color::rgb(100.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0),      // Red
                    };

                    let page_rect = Rect::new(
                        Point::new((minimap_x + 2.0) as f64, (y + 1.0) as f64),
                        crate::types::Size::new((minimap_width - 4.0) as f64, (page_height - 2.0) as f64),
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
                Point::new(minimap_x as f64, indicator_y as f64),
                crate::types::Size::new(minimap_width as f64, indicator_height as f64),
            );
            ctx.draw_rect(indicator_rect, Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 128.0 / 255.0));
        }

        // Phase 4: Visual feedback for pending resize
        if self.pending_resize.is_some() {
            // Draw a subtle overlay to indicate pending resize
            let overlay_rect = Rect::new(
                Point::new(minimap_x as f64, minimap_y as f64),
                crate::types::Size::new(minimap_width as f64, 20.0),
            );
            ctx.draw_rect(overlay_rect, Color::rgba(255.0 / 255.0, 200.0 / 255.0, 100.0 / 255.0, 180.0 / 255.0));
        }

        // Phase 4: Count and show stale pages (regenerating after reflow)
        let stale_count = (0..page_count)
            .filter_map(|i| minimap.get_page(i))
            .filter(|p| p.status == minimap::PageStatus::Stale)
            .count();

        if stale_count > 0 {
            // Draw regeneration indicator
            let regen_rect = Rect::new(
                Point::new(minimap_x as f64, (minimap_y + minimap_height - 20.0) as f64),
                crate::types::Size::new(minimap_width as f64, 20.0),
            );
            ctx.draw_rect(regen_rect, Color::rgba(200.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 180.0 / 255.0));
        }
    }

    /// Render the terminal grid
    fn render_grid(&self, ctx: &mut PaintContext) {
        // Get renderable content from terminal
        let content = self.term.renderable_content();

        // Create default text style for terminal
        let mut text_style = TextStyle::default();
        text_style.font_size = self.config.font_size * self.scale_factor;
        text_style.line_height = DEFAULT_LINE_HEIGHT;
        text_style.font_family = self.config.font_family.clone();

        // Iterate over all renderable cells
        for indexed in content.display_iter {
            let pos = indexed.point;
            let cell = indexed.cell;

            // Get row and column indices
            let row_idx = pos.line.0 as usize;
            let col_idx = pos.column.0;

            // Skip if out of visible bounds
            if row_idx >= self.rows || col_idx >= self.cols {
                continue;
            }

            // Calculate pixel position
            let x = self.bounds.min().x as f32 + (col_idx as f32 * self.cell_width);
            let y = self.bounds.min().y as f32 + (row_idx as f32 * self.cell_height);

            // Create cell rect
            let cell_rect = Rect::new(
                Point::new(x as f64, y as f64),
                crate::types::Size::new(self.cell_width as f64, self.cell_height as f64),
            );

            // Render background color (if not default)
            let bg_color = ansi_to_color(&cell.bg);
            if bg_color != Color::rgb(15.0 / 255.0, 15.0 / 255.0, 20.0 / 255.0) {
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
                    Point::new(x as f64, y as f64),
                    Some(self.cell_width),
                );
            }
        }
    }
}

impl Widget for TerminalEmulator {
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
        self.bounds = bounds;
        self.state.bounds = bounds;

        // Phase 4: Detect if resize is needed
        let (new_cols, new_rows) = self.calculate_dimensions_from_bounds();
        if new_cols != self.cols || new_rows != self.rows {
            // Dimension change detected, schedule debounced resize
            self.pending_resize = Some((new_cols, new_rows));
        }

        self.update_cell_dimensions();
        self.dirty = true;
        self.state.dirty = DirtyLevel::Visual;
    }

    fn dirty_level(&self) -> DirtyLevel {
        self.state.dirty
    }

    fn set_dirty_level(&mut self, level: DirtyLevel) {
        self.state.dirty = level;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn paint(&self, ctx: &mut PaintContext) {
        // Draw background
        ctx.draw_rect(self.bounds, Color::rgb(15.0 / 255.0, 15.0 / 255.0, 20.0 / 255.0));

        // Phase 6: Check if in alternate screen mode
        let is_alt_screen = self.is_alternate_screen();

        // Render grid
        self.render_grid(ctx);

        // Render minimap (Phase 2+ - placeholder for GPU texture upload)
        // Phase 6: Hide minimap completely in alternate screen mode
        if !is_alt_screen {
            if let Some(ref minimap) = self.minimap_manager {
                self.render_minimap(ctx, minimap);
            }
        }
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn is_interactive(&self) -> bool {
        true
    }
}
