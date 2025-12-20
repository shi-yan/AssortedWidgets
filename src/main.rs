use assorted_widgets::{GuiEventLoop, WindowOptions};
use assorted_widgets::elements::{Container, DebugRect, ClippedContainer};
use assorted_widgets::layout::{Style, FlexDirection, Dimension, Display, JustifyContent, AlignItems};
use assorted_widgets::paint::Color;
use assorted_widgets::scene_graph::SceneNode;
use assorted_widgets::types::WidgetId;

fn main() {
    println!("AssortedWidgets - Clipping Test Demo");
    println!("=====================================");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("Initializing WebGPU...");

        let mut event_loop = pollster::block_on(async {
            GuiEventLoop::new_with_window(WindowOptions {
                title: "AssortedWidgets - Clipping Test".to_string(),
                ..Default::default()
            })
            .await
        })
        .expect("Failed to initialize rendering");

        println!("WebGPU initialized successfully!");
        println!();

        // Create test scene: clipping demonstration
        println!("Creating test scene...");
        setup_test_scene(&mut event_loop);
        println!("Scene created!");
        println!();

        println!("Starting continuous rendering...");
        println!("You should see:");
        println!("  - Large red background (full window)");
        println!("  - Blue square clipped to center 300x300 region");
        println!("  - Green square extending beyond clip bounds (clipped)");
        println!("The scene demonstrates shader-based clipping.");
        println!("Try resizing the window - clipping will update!");
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
    let background_id = event_loop.element_manager_mut().next_id();
    let clipped_id = event_loop.element_manager_mut().next_id();

    // Create root container (centered layout)
    let root_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: Some(JustifyContent::Center),
        align_items: Some(AlignItems::Center),
        size: taffy::Size {
            width: Dimension::Percent(1.0),  // 100% width
            height: Dimension::Percent(1.0), // 100% height
        },
        ..Default::default()
    };

    let root_container = Container::new(root_id, root_style.clone());

    // Create background rect (red, full window)
    let background_rect = DebugRect::new(background_id, Color::RED);

    // Create clipped container (blue background with green overflow)
    // The overflow content will be clipped to the container's bounds
    let clipped_container = ClippedContainer::new(
        clipped_id,
        Color::rgba(0.0, 0.0, 1.0, 0.8),  // Blue background
        Color::rgba(0.0, 1.0, 0.0, 0.8),  // Green overflow (will be clipped)
    ).with_style(Style {
        size: taffy::Size {
            width: Dimension::Length(300.0),
            height: Dimension::Length(300.0),
        },
        ..Default::default()
    });

    // Add elements to element manager
    event_loop.element_manager_mut().add_element_with_id(root_id, Box::new(root_container));
    event_loop.element_manager_mut().add_element_with_id(background_id, Box::new(background_rect));
    event_loop.element_manager_mut().add_element_with_id(clipped_id, Box::new(clipped_container));

    // Build layout tree
    event_loop.layout_manager_mut().create_node(root_id, root_style).unwrap();
    event_loop.layout_manager_mut().create_node(background_id,
        Style {
            position: taffy::Position::Absolute,
            inset: taffy::Rect {
                left: taffy::LengthPercentageAuto::Length(0.0),
                right: taffy::LengthPercentageAuto::Length(0.0),
                top: taffy::LengthPercentageAuto::Length(0.0),
                bottom: taffy::LengthPercentageAuto::Length(0.0),
            },
            ..Default::default()
        }
    ).unwrap();
    event_loop.layout_manager_mut().create_node(clipped_id,
        Style {
            size: taffy::Size {
                width: Dimension::Length(300.0),
                height: Dimension::Length(300.0),
            },
            ..Default::default()
        }
    ).unwrap();

    // Set up layout hierarchy
    event_loop.layout_manager_mut().add_child(root_id, background_id).unwrap();
    event_loop.layout_manager_mut().add_child(root_id, clipped_id).unwrap();
    event_loop.layout_manager_mut().set_root(root_id).unwrap();

    // Build scene graph (for render order)
    // Background first, then clipped container on top
    let mut root_node = SceneNode::new(root_id);
    root_node.add_child(SceneNode::new(background_id));
    root_node.add_child(SceneNode::new(clipped_id));
    event_loop.scene_graph_mut().set_root(root_node);

    // Mark layout as dirty
    event_loop.mark_layout_dirty();
}
