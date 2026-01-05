// Terminal Emulator Demo
//
// Phase 1: Basic terminal rendering
// - Grid display with ANSI colors
// - Scrollback navigation
// - Mouse and keyboard input (structure in place)
//
// Note: PTY integration will come in Phase 8.
// For now, this demonstrates the terminal widget structure.

use assorted_widgets::Application;
use assorted_widgets::widgets::TerminalEmulator;

fn main() {
    Application::launch(|app| {
        app.spawn_window("Terminal Emulator Demo", 1000.0, 600.0, |window| {
            // Create terminal emulator
            let terminal = TerminalEmulator::with_defaults();

            // Note: In Phase 1, the terminal displays an empty grid.
            // Future phases will add:
            // - Phase 2: Minimap
            // - Phase 3-5: Incremental updates, reflow, multi-threading
            // - Phase 6: Alternate screen buffer handling
            // - Phase 8: PTY integration for actual shell interaction

            window.set_main_widget(terminal);
        });
    });
}
