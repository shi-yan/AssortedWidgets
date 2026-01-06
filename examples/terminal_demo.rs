// Terminal Emulator Demo
//
// Phase 2-6: Terminal with minimap visualization
// - Grid display with ANSI colors
// - Minimap showing page status
// - Scrollback navigation
// - Alternate screen buffer testing
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

            // Set minimap scale factor for high-DPI displays (2.0 = Retina/2x)
            // This makes ONLY the minimap render at 2x resolution for sharper display
            // Terminal text rendering uses system DPI automatically
            terminal.set_minimap_scale_factor(2.0);

            // Feed sample text to the terminal to demonstrate minimap
            // We can write directly to the terminal's parser (no PTY needed)
            populate_terminal_with_demo_content(&mut terminal);

            // Debug: Dump minimap to PNG files for inspection
            if let Err(e) = terminal.dump_minimap_to_png() {
                eprintln!("⚠️  Failed to dump minimap: {}", e);
            }

            // Note: Current status
            // ✅ Phase 1: Grid rendering with ANSI colors
            // ✅ Phase 2: Minimap infrastructure (complete)
            // ✅ Phase 3: Incremental updates with dirty tracking
            // ✅ Phase 4: Reflow handling with debouncing
            // ✅ Phase 5: Multi-threaded background rasterization
            // ✅ Phase 6: Alternate screen buffer support
            //    - Minimap hidden in alternate screen (vim, less, htop)
            //    - Demo includes mode switching test
            // Future phases:
            // - Phase 7: Polish & optimization
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
    text.push_str("=== Terminal Emulator Demo ===\x1b[0m\r\n\r\n");

    // Explanation
    text.push_str("\x1b[1mPhases 2-6: Complete Minimap System\x1b[0m\r\n");
    text.push_str("Look at the right side to see the minimap!\r\n\r\n");

    // Color demonstrations
    text.push_str("\x1b[31mRed text\x1b[0m - \x1b[32mGreen text\x1b[0m - ");
    text.push_str("\x1b[33mYellow text\x1b[0m - \x1b[34mBlue text\x1b[0m\r\n");
    text.push_str("\x1b[35mMagenta text\x1b[0m - \x1b[36mCyan text\x1b[0m\r\n\r\n");

    // Add many lines to create scrollback
    for i in 1..=100 {
        let color_code = 31 + (i % 7); // Cycle through colors
        text.push_str(&format!(
            "\x1b[{}mLine {}: This is sample content to fill the terminal and demonstrate minimap pages.\x1b[0m\r\n",
            color_code, i
        ));
    }

    // Final message
    text.push_str("\r\n\x1b[1;36m=== End of Demo Content ===\x1b[0m\r\n");
    text.push_str("Scroll up/down to see the minimap viewport indicator move!\r\n\r\n");

    // Phase 6: Alternate screen buffer test
    text.push_str("\x1b[1;33m=== Phase 6: Alternate Screen Test ===\x1b[0m\r\n");
    text.push_str("Entering alternate screen mode in 3... 2... 1...\r\n\r\n");

    // Enter alternate screen mode
    text.push_str("\x1b[?1049h"); // Enter alternate screen

    // Content in alternate screen (like vim, less, etc.)
    text.push_str("\x1b[2J\x1b[H"); // Clear screen and move to home

    text.push_str("\x1b[1;36m╔══════════════════════════════════════════════════╗\x1b[0m\r\n");
    text.push_str("\x1b[1;36m║      ALTERNATE SCREEN MODE (Like vim/less)       ║\x1b[0m\r\n");
    text.push_str("\x1b[1;36m╚══════════════════════════════════════════════════╝\x1b[0m\r\n\r\n");

    text.push_str("\x1b[1;33mNotice:\x1b[0m The minimap is now \x1b[1;31mHIDDEN\x1b[0m!\r\n\r\n");

    text.push_str("This is the alternate screen buffer, used by:\r\n");
    text.push_str("  • vim/nvim - Text editors\r\n");
    text.push_str("  • less/more - Pagers\r\n");
    text.push_str("  • htop/top - System monitors\r\n");
    text.push_str("  • tmux/screen - Terminal multiplexers\r\n\r\n");

    text.push_str("The minimap is automatically hidden because:\r\n");
    text.push_str("  1. No scrollback in alternate screen\r\n");
    text.push_str("  2. Full-screen apps manage their own display\r\n");
    text.push_str("  3. Minimap would be distracting/irrelevant\r\n\r\n");

    text.push_str("\x1b[1;32mExiting alternate screen in 2 seconds...\x1b[0m\r\n");

    // Exit alternate screen mode
    text.push_str("\x1b[?1049l"); // Exit alternate screen

    // Back to normal screen
    text.push_str("\r\n\x1b[1;32m✓ Returned to normal screen!\x1b[0m\r\n");
    text.push_str("The minimap should be \x1b[1;32mVISIBLE\x1b[0m again.\r\n\r\n");

    text.push_str("\x1b[1;36mPhase 6 Complete:\x1b[0m Alternate screen buffer handled correctly!\r\n");

    text
}
