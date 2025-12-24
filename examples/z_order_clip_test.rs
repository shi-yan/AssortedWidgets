// Z-Ordering and Clipping Test Example (Phase 0)
//
// This demonstrates the Phase 0 API implementation:
// 1. Z-ordering with explicit layers (SHADOW, NORMAL, OVERLAY)
// 2. Clipping API (push_clip_rounded, pop_clip)
// 3. Command sorting by z-index
//
// This is a unit test / API demonstration. Phase 1 will integrate with rendering pipeline.

use assorted_widgets::paint::{
    PrimitiveBatcher, Color, ShapeStyle, CornerRadius,
    layers,
};
use assorted_widgets::types::Rect;

fn main() {
    println!("Z-Order and Clipping Test (Phase 0)");
    println!("=====================================\n");

    let mut batcher = PrimitiveBatcher::new();

    // Helper to create a Rect (euclid requires origin + size)
    let rect = |x: f64, y: f64, w: f64, h: f64| {
        Rect::new(
            euclid::Point2D::new(x, y),
            euclid::Size2D::new(w, h),
        )
    };

    // Test 1: Z-Ordering with explicit layers
    println!("Test 1: Z-Ordering with explicit layers");
    println!("----------------------------------------");

    // Red rectangle at SHADOW layer (z=-100) - renders first (behind)
    batcher.draw_rect_z(
        rect(50.0, 50.0, 200.0, 200.0),
        ShapeStyle::solid(Color::RED),
        layers::SHADOW,
    );
    println!("  ✓ Added red rectangle at SHADOW layer (z=-100)");

    // Blue rectangle at NORMAL layer (z=0) - renders second (middle)
    batcher.draw_rect_z(
        rect(100.0, 100.0, 200.0, 200.0),
        ShapeStyle::solid(Color::BLUE),
        layers::NORMAL,
    );
    println!("  ✓ Added blue rectangle at NORMAL layer (z=0)");

    // Green rectangle at OVERLAY layer (z=1000) - renders last (on top)
    batcher.draw_rect_z(
        rect(150.0, 150.0, 200.0, 200.0),
        ShapeStyle::solid(Color::GREEN),
        layers::OVERLAY,
    );
    println!("  ✓ Added green rectangle at OVERLAY layer (z=1000)");

    // Test 2: Rounded Rectangle Clipping
    println!("\nTest 2: Rounded rectangle clipping");
    println!("-----------------------------------");

    // White background
    batcher.draw_rect_z(
        rect(400.0, 50.0, 250.0, 300.0),
        ShapeStyle::solid(Color::WHITE),
        layers::NORMAL,
    );
    println!("  ✓ Added white background");

    // Push rounded clip region (20px corner radius)
    batcher.push_clip_rounded(
        rect(410.0, 60.0, 230.0, 280.0),
        CornerRadius::uniform(20.0),
    );
    println!("  ✓ Pushed rounded clip (20px radius)");

    // Yellow rectangle (should be clipped)
    let yellow = Color::rgb(1.0, 1.0, 0.0);
    batcher.draw_rect_z(
        rect(400.0, 50.0, 250.0, 300.0),
        ShapeStyle::solid(yellow),
        layers::NORMAL,
    );
    println!("  ✓ Added yellow rectangle (will be clipped)");

    // Pop clip
    batcher.pop_clip();
    println!("  ✓ Popped clip region");

    // Test 3: Nested Clipping
    println!("\nTest 3: Nested clipping (intersection)");
    println!("---------------------------------------");

    // Outer clip (50px radius)
    batcher.push_clip_rounded(
        rect(50.0, 400.0, 300.0, 200.0),
        CornerRadius::uniform(50.0),
    );
    println!("  ✓ Pushed outer clip (50px radius)");

    // Magenta background
    let magenta = Color::rgb(1.0, 0.0, 1.0);
    batcher.draw_rect_z(
        rect(50.0, 400.0, 300.0, 200.0),
        ShapeStyle::solid(magenta),
        layers::NORMAL,
    );
    println!("  ✓ Added magenta background");

    // Inner clip (30px radius, offset)
    batcher.push_clip_rounded(
        rect(100.0, 450.0, 200.0, 100.0),
        CornerRadius::uniform(30.0),
    );
    println!("  ✓ Pushed inner clip (30px radius)");

    // Cyan rectangle (clipped to intersection)
    let cyan = Color::rgb(0.0, 1.0, 1.0);
    batcher.draw_rect_z(
        rect(50.0, 400.0, 300.0, 200.0),
        ShapeStyle::solid(cyan),
        layers::NORMAL,
    );
    println!("  ✓ Added cyan rectangle (clipped to intersection)");

    // Pop both clips
    batcher.pop_clip();
    batcher.pop_clip();
    println!("  ✓ Popped both clip regions");

    // Sort commands by z-index (Phase 0 implementation)
    println!("\nSorting commands by z-index...");
    batcher.sort_commands();
    println!("  ✓ Commands sorted (stable sort by z-index, then batch_key)");

    // Report results
    println!("\n=== Phase 0 Implementation Summary ===");
    println!("Total draw commands: {}", batcher.len());
    println!();
    println!("✓ Implemented features:");
    println!("  [✓] Z-index added to DrawCommand::Rect");
    println!("  [✓] Layer constants (SHADOW=-100, NORMAL=0, OVERLAY=1000, etc.)");
    println!("  [✓] PrimitiveBatcher::draw_rect_z()");
    println!("  [✓] PrimitiveBatcher::push_clip_rounded()");
    println!("  [✓] PrimitiveBatcher::pop_clip()");
    println!("  [✓] PrimitiveBatcher::sort_commands()");
    println!("  [✓] ClipStack with max 8 nested regions");
    println!("  [✓] ClipRegion with GPU uniform conversion");
    println!();
    println!("⏳ Pending (Phase 1):");
    println!("  [ ] Integrate with rendering pipeline");
    println!("  [ ] Implement SDF shader with clipping support");
    println!("  [ ] Create actual visual test window");
    println!();
    println!("Phase 0 complete! Ready for Phase 1 (Rounded Rectangles + SDF rendering).");
}
