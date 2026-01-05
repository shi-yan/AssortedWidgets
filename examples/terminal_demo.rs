// Terminal Emulator Demo
//
// Phase 2: Terminal with minimap visualization
// - Grid display with ANSI colors
// - Minimap showing page status
// - Scrollback navigation
//
// This demo feeds sample text to the terminal to demonstrate
// the minimap functionality without requiring PTY integration.

use assorted_widgets::Application;
use assorted_widgets::widgets::TerminalEmulator;

fn main() {
    Application::launch(|app| {
        app.spawn_window("Terminal Emulator Demo - Phase 2", 1200.0, 700.0, |window| {
            // Create terminal emulator
            let mut terminal = TerminalEmulator::with_defaults();

            // Feed sample text to the terminal to demonstrate minimap
            // We can write directly to the terminal's parser (no PTY needed)
            populate_terminal_with_demo_content(&mut terminal);

            // Note: Current status
            // ✅ Phase 1: Grid rendering with ANSI colors
            // ✅ Phase 2: Minimap infrastructure (90% - placeholder visualization)
            //    - Minimap shows page status (green=clean, yellow=dirty)
            //    - White indicator shows current viewport
            //    - GPU texture upload to come later
            // Future phases:
            // - Phase 3: Incremental updates
            // - Phase 4: Reflow handling
            // - Phase 5: Multi-threading
            // - Phase 8: PTY integration for shell interaction

            window.set_main_widget(terminal);
        });
    });
}

/// Populate terminal with demo content
///
/// Writes colored text and multiple lines to demonstrate:
/// - ANSI color rendering
/// - Scrollback buffer
/// - Minimap page creation
fn populate_terminal_with_demo_content(terminal: &mut TerminalEmulator) {
    // Generate demo text with ANSI escape sequences
    let demo_text = generate_demo_text();

    // Write to terminal (processes VT sequences)
    // Note: In Phase 8, this would come from a PTY
    terminal.write(demo_text.as_bytes());
}

/// Generate colored demo text with ANSI escape codes
fn generate_demo_text() -> String {
    let mut text = String::new();

    // Welcome message with colors
    text.push_str("\x1b[1;32m"); // Bold green
    text.push_str("=== Terminal Emulator Demo ===\x1b[0m\n\n");

    // Explanation
    text.push_str("\x1b[1mPhase 2: Minimap Visualization\x1b[0m\n");
    text.push_str("Look at the right side to see the minimap!\n\n");

    // Color demonstrations
    text.push_str("\x1b[31mRed text\x1b[0m - \x1b[32mGreen text\x1b[0m - ");
    text.push_str("\x1b[33mYellow text\x1b[0m - \x1b[34mBlue text\x1b[0m\n");
    text.push_str("\x1b[35mMagenta text\x1b[0m - \x1b[36mCyan text\x1b[0m\n\n");

    // Add many lines to create scrollback
    for i in 1..=100 {
        let color_code = 31 + (i % 7); // Cycle through colors
        text.push_str(&format!(
            "\x1b[{}mLine {}: This is sample content to fill the terminal and demonstrate minimap pages.\x1b[0m\n",
            color_code, i
        ));
    }

    // Final message
    text.push_str("\n\x1b[1;36m=== End of Demo Content ===\x1b[0m\n");
    text.push_str("Scroll up/down to see the minimap viewport indicator move!\n");

    text
}
