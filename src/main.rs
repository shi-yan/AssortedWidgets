use assorted_widgets::{Application, Element, WindowOptions};
use assorted_widgets::elements::{ClickableRect, SimpleInputBox};
use assorted_widgets::scene_graph::SceneNode;
use assorted_widgets::types::{Point, Rect, Size, WidgetId};
use assorted_widgets::paint::Color;

fn main() {
    println!("AssortedWidgets - Phase 2.2: Focus & IME Demo");
    println!("==============================================");
    println!();
    println!("This demo tests:");
    println!("  ✓ Focus management (click input box to focus)");
    println!("  ✓ Tab navigation between focusable elements");
    println!("  ✓ Keyboard input to focused element");
    println!("  ✓ IME cursor positioning");
    println!("  ✓ Mouse capture (for future drag operations)");
    println!();
    println!("Instructions:");
    println!("  1. Click the input box to focus it");
    println!("  2. Type characters to see them appear");
    println!("  3. Press Backspace to delete");
    println!("  4. Use Tab to cycle focus (when multiple focusable elements)");
    println!();
    println!("Note: Full IME support (Chinese/Japanese/Korean) requires");
    println!("      NSTextInputClient implementation (future enhancement)");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        let mut app = pollster::block_on(async {
            Application::new().await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!");
        println!();

        // Create window
        let window_id = app.create_window(WindowOptions {
            bounds: Rect::new(Point::new(100.0, 100.0), Size::new(800.0, 600.0)),
            title: "Phase 2: Hit Testing Demo".to_string(),
            titlebar: None,
            borderless: false,
            transparent: false,
            always_on_top: false,
            utility: false,
        })
        .expect("Failed to create window");

        println!("Window created!");
        println!();

        // ================================================================
        // Create test widgets for focus and IME
        // ================================================================

        // Input box 1: Primary input
        let input1 = SimpleInputBox::new(WidgetId::new(1));
        let input1_id = input1.id();

        // Input box 2: Secondary input (for Tab testing)
        let input2 = SimpleInputBox::new(WidgetId::new(2));
        let input2_id = input2.id();

        // Get mutable reference to the window to set up UI
        let window = app.window_mut(window_id).expect("Window not found");

        // Add input boxes to element manager
        window.element_manager_mut().add_element(Box::new(input1));
        window.element_manager_mut().add_element(Box::new(input2));

        // Create layout nodes with vertical stacking
        let layout_style = taffy::Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::Size {
                width: taffy::LengthPercentage::length(20.0),
                height: taffy::LengthPercentage::length(20.0),
            },
            padding: taffy::Rect {
                left: taffy::LengthPercentage::length(50.0),
                right: taffy::LengthPercentage::length(50.0),
                top: taffy::LengthPercentage::length(50.0),
                bottom: taffy::LengthPercentage::length(50.0),
            },
            ..Default::default()
        };

        window.layout_manager_mut().create_node(input1_id, layout_style.clone())
            .expect("Failed to create layout node");
        window.layout_manager_mut().create_node(input2_id, layout_style.clone())
            .expect("Failed to create layout node");

        // Create scene graph with input boxes
        let mut root = SceneNode::new(input1_id);
        root.add_child(SceneNode::new(input2_id));

        window.scene_graph_mut().set_root(root);
        window.layout_manager_mut().set_root(input1_id)
            .expect("Failed to set layout root");

        println!("✅ Demo setup complete!");
        println!();
        println!("═══════════════════════════════════════════════════════");
        println!("  TEST INSTRUCTIONS");
        println!("═══════════════════════════════════════════════════════");
        println!();
        println!("✓ Two input boxes are displayed vertically");
        println!("✓ Click an input box to focus it (watch terminal log)");
        println!("✓ Type characters to see them appear in white");
        println!("✓ Press Backspace to delete characters");
        println!("✓ Press Tab to switch focus between input boxes");
        println!();
        println!("Future enhancements:");
        println!("  - Full IME support requires NSTextInputClient");
        println!("  - Preedit text shown in yellow with underline");
        println!("  - Committed text shown in white");
        println!("═══════════════════════════════════════════════════════");
        println!();

        // Run application event loop
        app.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo currently only runs on macOS.");
        println!("Windows and Linux support coming soon!");
    }
}
