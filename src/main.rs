use assorted_widgets::{GuiEventLoop, WindowOptions};
use assorted_widgets::elements::{Container, DebugRect, AnimatedRect};
use assorted_widgets::layout::{Style, FlexDirection, Dimension, Display};
use assorted_widgets::paint::Color;
use assorted_widgets::scene_graph::SceneNode;

fn main() {
    println!("AssortedWidgets - Layout Animation Demo");
    println!("========================================");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        let mut event_loop = pollster::block_on(async {
            GuiEventLoop::new_with_window(WindowOptions {
                title: "AssortedWidgets - Layout Animation".to_string(),
                ..Default::default()
            })
            .await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!");
        println!();

        // Create test scene: animated layout demonstration
        println!("Creating animated layout test scene...");
        setup_test_scene(&mut event_loop);
        println!("Scene created!");
        println!();

        println!("Starting continuous rendering...");
        println!("You should see:");
        println!("  - Red rectangle on the left (animated width using sin wave)");
        println!("  - Green rectangle on the right (fills remaining space)");
        println!();
        println!("This demonstrates:");
        println!("  - Leaves → Root: Red rect's intrinsic size changes trigger layout recalculation");
        println!("  - Root → Leaves: Window resize redistributes space between both rects");
        println!();
        println!("Try resizing the window to see the layout system in action!");
        println!("Press Cmd+Q to quit.");
        println!();

        // Run event loop (never returns)
        // Rendering is handled internally via render_frame_internal()
        event_loop.run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("This demo only runs on macOS currently.");
    }
}

#[cfg(target_os = "macos")]
fn setup_test_scene(event_loop: &mut GuiEventLoop) {
    // Generate IDs
    let root_id = event_loop.element_manager_mut().next_id();
    let red_rect_id = event_loop.element_manager_mut().next_id();
    let green_rect_id = event_loop.element_manager_mut().next_id();

    // Create root container with horizontal layout
    let root_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,  // Horizontal layout
        size: taffy::Size {
            width: Dimension::percent(1.0),  // 100% width
            height: Dimension::percent(1.0), // 100% height
        },
        ..Default::default()
    };

    let root_container = Container::new(root_id, root_style.clone());

    // Create animated red rectangle (left side)
    // Width oscillates via sin wave: 200 ± 100px (range: 100-300px)
    // This demonstrates measure functions and leaves → root layout flow
    let red_rect = AnimatedRect::new(
        red_rect_id,
        Color::RED,
        200.0,  // base_width (center of oscillation)
        100.0,  // amplitude (oscillation range)
    );

    // Create static green rectangle (right side)
    // Uses flex_grow to fill remaining space
    // This demonstrates root → leaves layout flow (responds to window resize and red rect changes)
    let green_rect = DebugRect::new(green_rect_id, Color::GREEN);

    // Green rect style: fills remaining space
    let green_style = Style {
        flex_grow: 1.0,  // Fill remaining horizontal space
        ..Default::default()
    };

    // Add elements to element manager
    event_loop.element_manager_mut().add_element_with_id(root_id, Box::new(root_container));
    event_loop.element_manager_mut().add_element_with_id(red_rect_id, Box::new(red_rect));
    event_loop.element_manager_mut().add_element_with_id(green_rect_id, Box::new(green_rect));

    // Build layout tree
    event_loop.layout_manager_mut().create_node(root_id, root_style).unwrap();

    // Red rect uses measurable node (has dynamic intrinsic size via measure function)
    event_loop.layout_manager_mut().create_measurable_node(
        red_rect_id,
        Style::default(),  // No fixed size - measure function provides it
    ).unwrap();

    // Green rect uses regular node with flex_grow
    event_loop.layout_manager_mut().create_node(green_rect_id, green_style).unwrap();

    // Set up layout hierarchy
    event_loop.layout_manager_mut().add_child(root_id, red_rect_id).unwrap();
    event_loop.layout_manager_mut().add_child(root_id, green_rect_id).unwrap();
    event_loop.layout_manager_mut().set_root(root_id).unwrap();

    // Build scene graph (for render order)
    let mut root_node = SceneNode::new(root_id);
    root_node.add_child(SceneNode::new(red_rect_id));
    root_node.add_child(SceneNode::new(green_rect_id));
    event_loop.scene_graph_mut().set_root(root_node);

    // Mark layout as dirty for initial computation
    event_loop.mark_layout_dirty();
}
