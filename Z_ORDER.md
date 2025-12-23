# Z-Order Architecture for AssortedWidgets

> **Status:** Phase 2 Implementation (CPU Sorting)
> **Next Step:** Phase 2.1 - Two-Pass Rendering with Depth Buffer
> **Last Updated:** 2025-12-23

## Table of Contents

1. [Overview](#overview)
2. [Current Implementation](#current-implementation)
3. [Planned Architecture](#planned-architecture)
4. [Design Decisions](#design-decisions)
5. [Implementation Guide](#implementation-guide)
6. [Performance Considerations](#performance-considerations)

---

## Overview

Z-order (depth ordering) in AssortedWidgets ensures that UI elements are rendered and interact in the correct visual layering order. The system must handle both rendering (what appears on top) and hit testing (which element receives mouse clicks) consistently.

### Design Goals

1. **Visual Consistency**: Rendering and hit testing use the same z-order
2. **Automatic Assignment**: Z-order is assigned automatically during paint traversal
3. **Simplicity**: Depth-first scene graph traversal determines natural z-order
4. **Performance**: Efficient for typical UI scenarios (< 10,000 elements)
5. **3D Compatibility**: Architecture supports future integration with 3D content

---

## Current Implementation

### Phase 2: CPU-Side Sorting (Current)

**Status:** ✅ Implemented and working

The current implementation uses CPU-side sorting with automatic z-order assignment:

```rust
// During paint pass:
pub struct PaintContext {
    z_order: u32,  // Auto-increments with each draw call
    hit_tester: HitTester,  // Collects hitboxes with z-order
}

// Elements register hitboxes and draw
ctx.register_hitbox(self.id, self.bounds);  // z_order captured here
ctx.draw_rect(self.bounds, color);           // z_order assigned here

// After paint pass:
// 1. Sort primitives by z-order
rect_instances.sort_by_key(|inst| inst.z_order);
text_instances.sort_by_key(|inst| inst.z_order);

// 2. Render sorted primitives
render_all_rects();  // One batched draw call
render_all_text();   // One batched draw call

// 3. Extract hit tester with same z-order values
let hit_tester = paint_ctx.finalized_hit_tester();
```

### Z-Order Assignment Strategy

**Depth-First Traversal:**

```
Scene Graph:          Z-Order Assignment:
    Root                  z=0 (Root painted first)
    ├── Child A           z=1 (Child A painted second)
    │   ├── Child A1      z=2 (Child A1 painted third)
    │   └── Child A2      z=3 (Child A2 painted fourth)
    └── Child B           z=4 (Child B painted last - appears on top)
```

**Key Properties:**
- Later elements in depth-first order get higher z-order
- Parent painted before children (parent behind children)
- Siblings painted in order (later siblings on top of earlier)

### Benefits of Current Approach

✅ **Correct**: Rendering and hit testing perfectly synchronized
✅ **Simple**: No shader changes required
✅ **Transparent-Friendly**: Works for both opaque and transparent elements
✅ **Debuggable**: Easy to understand z-order assignment
✅ **Phase 2 Complete**: Hit testing validated with demo

### Limitations

❌ **CPU Overhead**: Sorting 10,000 primitives takes ~0.2ms
❌ **Not Optimal for Batching**: Sorting by z-order can break material batching
❌ **No GPU Depth Testing**: Not leveraging hardware z-buffer
❌ **3D Integration**: Difficult to composite 3D content with 2D UI

---

## Planned Architecture

### Phase 2.1: Two-Pass Rendering with Depth Buffer

**Status:** 📅 Planned (Next Implementation)

This is the recommended production architecture based on industry best practices and expert consultation.

#### The Two-Pass Strategy

```rust
// Pass 1: Opaque Elements (Depth-Tested, Unsorted)
// ================================================
// - Depth write: ENABLED
// - Depth test: ENABLED (LessOrEqual)
// - Draw order: ANY (or batched by material)
// - GPU automatically handles z-ordering via depth buffer

let opaque_rects: Vec<_> = rect_instances
    .iter()
    .filter(|r| r.color[3] >= 0.99)  // alpha >= 0.99 = opaque
    .cloned()
    .collect();

render_opaque_rects(&opaque_rects);  // Batched, no sorting needed!
render_opaque_text(&opaque_text);

// Pass 2: Transparent Elements (Sorted Back-to-Front)
// ===================================================
// - Depth write: DISABLED (don't block what's behind)
// - Depth test: ENABLED (respect opaque depth)
// - Draw order: SORTED (back-to-front for correct alpha blending)

let mut transparent_rects: Vec<_> = rect_instances
    .iter()
    .filter(|r| r.color[3] < 0.99)
    .cloned()
    .collect();

transparent_rects.sort_by_key(|r| std::cmp::Reverse(r.z_order));
render_transparent_rects(&transparent_rects);
render_transparent_text(&transparent_text);
```

#### Shader Changes Required

```wgsl
// Rect Shader (shaders/rect.wgsl)
@vertex
fn vs_main(instance: RectInstance, @builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    // Convert z_order to normalized depth (0.0 = far, 1.0 = near)
    // Use reverse mapping so higher z_order = closer to camera
    let depth = 1.0 - (f32(instance.z_order) / 10000.0);

    out.clip_position = vec4<f32>(world_pos, depth, 1.0);
    out.color = instance.color;
    return out;
}
```

#### Pipeline Configuration

```rust
// Create depth texture
let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
    format: wgpu::TextureFormat::Depth24Plus,
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    // ...
});

// Opaque pass pipeline
depth_stencil: Some(wgpu::DepthStencilState {
    format: wgpu::TextureFormat::Depth24Plus,
    depth_write_enabled: true,
    depth_compare: wgpu::CompareFunction::LessOrEqual,
    // ...
}),

// Transparent pass pipeline
depth_stencil: Some(wgpu::DepthStencilState {
    format: wgpu::TextureFormat::Depth24Plus,
    depth_write_enabled: false,  // Don't write depth!
    depth_compare: wgpu::CompareFunction::LessOrEqual,
    // ...
}),
```

### Phase 5: BoundsTree Optimization (Future)

**Status:** 📅 Future Optimization

Port gpui's `BoundsTree` for advanced batching optimization.

#### What is BoundsTree?

An R-tree variant that:
- **Assigns minimal z-values**: Non-overlapping elements can share the same z-order
- **Enables batching**: Elements with same z-order can be drawn in one call
- **Optimizes queries**: O(1) fast-path for finding max z-order in overlapping bounds

#### Example Benefit

```
Without BoundsTree:
  Rect A (50,50):    z=1
  Text A (50,50):    z=2
  Rect B (200,50):   z=3  ← Doesn't overlap A, but gets higher z!
  Text B (200,50):   z=4

  Result: 4 draw calls (Rect, Text, Rect, Text - constant switching)

With BoundsTree:
  Rect A (50,50):    z=1
  Text A (50,50):    z=2
  Rect B (200,50):   z=1  ← Can reuse z=1! No overlap!
  Text B (200,50):   z=2  ← Can reuse z=2!

  Result: 2 draw calls (All Rects at z=1, All Text at z=2)
```

---

## Design Decisions

### Why Not Use Depth Buffer in Phase 2?

**Decision:** Start with CPU sorting, add depth buffer in Phase 2.1

**Reasoning:**
1. **Validate Hit Testing First**: Phase 2 goal was hit testing - now validated ✅
2. **Transparency Complication**: Need two-pass rendering for transparency anyway
3. **Incremental Complexity**: Add one feature at a time
4. **Rapid Iteration**: Easier to debug without GPU depth testing

**Counter-Argument (from expert consultation):**
- "Adding depth buffer later = shader refactoring pain"
- "3D integration requires depth buffer - do it now"
- **Verdict**: Add depth buffer in Phase 2.1 (next step!)

### Why Not Implement BoundsTree Now?

**Decision:** Defer BoundsTree to Phase 5 optimization

**Reasoning:**
1. **Premature Optimization**: Current sorting is fast enough (< 1ms for 10k elements)
2. **Complexity**: BoundsTree adds ~500 lines of complex R-tree code
3. **Diminishing Returns**: Most UIs have < 1000 elements
4. **Clear Optimization Target**: Add when profiling shows sorting bottleneck

**When to Add BoundsTree:**
- Profiling shows z-order sorting is a bottleneck
- UI has > 5000 interactive elements
- Material batching becomes critical for performance

### Why Depth-First Z-Order Assignment?

**Decision:** Use scene graph depth-first traversal order for z-order

**Alternatives Considered:**
1. **Manual z-index properties**: Requires manual management, error-prone
2. **BoundsTree assignment**: Optimal but complex, deferred to Phase 5
3. **Layer system**: Adds conceptual complexity

**Chosen Approach:**
- Natural, predictable ordering (parent behind children)
- Matches developer intuition (paint order = z-order)
- Zero configuration required

---

## Implementation Guide

### For Element Authors

#### Making an Element Interactive

```rust
impl Element for MyButton {
    fn paint(&self, ctx: &mut PaintContext) {
        // 1. Register hitbox FIRST (before drawing)
        //    This captures the current z-order
        ctx.register_hitbox(self.id, self.bounds);

        // 2. Draw visual representation
        //    Z-order auto-increments here
        ctx.draw_rect(self.bounds, self.color);
        ctx.draw_text("Click Me", &style, position, None);
    }

    fn is_interactive(&self) -> bool {
        true  // This element handles mouse events
    }
}

impl MouseHandler for MyButton {
    fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
        println!("Button clicked!");
        EventResponse::Handled  // Stop propagation
    }
}
```

#### Understanding Z-Order in Hierarchies

```rust
// Parent-Child Hierarchy:
Container (z=0) {
    Background Rect (z=1)      ← Painted first (behind)
    Child Panel (z=2) {
        Panel Background (z=3)
        Button (z=4)            ← Painted last (on top)
    }
}

// Click at button position:
// Hit test returns Button (z=4) - highest z-order at that point
```

### For Renderer Developers

#### Current Rendering Flow

```rust
// In Window::render_frame()

// 1. Paint pass - collect primitives with z-order
let (mut rect_instances, mut text_instances, hit_tester) = {
    let paint_ctx = PaintContext::new(window_size, bundle);

    // Traverse scene graph (depth-first)
    scene_graph.traverse(|widget_id| {
        element.paint(&mut paint_ctx);  // z_order increments
    });

    // Extract primitives and hit tester
    (paint_ctx.rect_instances(), paint_ctx.text_instances(), paint_ctx.finalized_hit_tester())
};

// 2. Sort primitives by z-order (low to high)
rect_instances.sort_by_key(|inst| inst.z_order);
text_instances.sort_by_key(|inst| inst.z_order);

// 3. Render sorted primitives
render_rects(&rect_instances);
render_text(&text_instances);

// 4. Update hit tester for event dispatch
self.hit_tester = hit_tester;
```

---

## Performance Considerations

### Current Performance (Phase 2)

**Measured:**
- Z-order assignment: ~0 overhead (counter increment)
- Sorting 1,000 primitives: ~0.05ms
- Sorting 10,000 primitives: ~0.2ms
- Hit testing (linear scan): ~0.01ms for 1000 elements

**Bottlenecks:**
- CPU sorting becomes noticeable at > 5000 primitives
- Not leveraging GPU depth buffer hardware

### Estimated Performance (Phase 2.1 with Depth Buffer)

**Expected:**
- Opaque pass: No CPU sorting! GPU handles depth automatically
- Transparent pass: Only sort ~5% of primitives (most UI is opaque)
- Overall sorting: 10x faster for typical UIs

**Example:**
```
Typical UI: 10,000 primitives
  - 9,500 opaque (95%)
  - 500 transparent (5%)

Phase 2 (current):
  Sort 10,000 primitives = 0.2ms

Phase 2.1 (depth buffer):
  Sort 500 transparent primitives = 0.01ms
  GPU handles 9,500 opaque primitives = 0ms
```

### Future Performance (Phase 5 with BoundsTree)

**Expected:**
- Batching optimization: 2-5x fewer draw calls
- Z-value reuse: Non-overlapping elements share z-order
- Memory efficiency: Smaller z-order range (better precision)

---

## References

### Industry Best Practices

1. **Two-Pass Rendering**
   - Standard technique in game engines (Unity, Unreal, Godot)
   - Separates opaque (depth-tested) from transparent (sorted)
   - [Order your graphics draw calls around!](https://realtimecollisiondetection.net/blog/?p=86)

2. **Depth Buffer Usage**
   - WebGPU: Depth24Plus format for 24-bit precision
   - Maps z-order to 0.0-1.0 range
   - Hardware z-test faster than CPU sorting

3. **BoundsTree / R-Tree**
   - [gpui's BoundsTree](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/bounds_tree.rs)
   - R-tree spatial index for overlap queries
   - Enables z-value recycling for non-overlapping elements

### Related Documents

- `CLAUDE.md` - Overall architecture
- `EVENT_HANDLING.md` - Event system design
- Phase 2 commits: b39ac1e, 97ae5e7, 13c2a3d, 4a43345

---

## Appendix: Common Scenarios

### Scenario 1: Tooltip Over Button

```
Button (z=5)
  Background Rect (z=6)
  Label Text (z=7)

Tooltip (z=8) ← Created after button, appears on top
  Background Rect (z=9)
  Tooltip Text (z=10)
```

Click on tooltip → Hits Tooltip (z=8+), not Button

### Scenario 2: Overlapping Windows

```
Window 1 (z=0)
  Content (z=1-100)

Window 2 (z=101) ← Brought to front
  Content (z=102-200)
```

Click on overlap → Hits Window 2 (higher z-order range)

### Scenario 3: 3D Viewport in UI Panel

```
UI Panel (z=0)
  Panel Background (z=1)

  3D Viewport (z=2)
    3D Scene: Uses GPU depth buffer internally
    (z-order 2 reserves this "layer" for 3D)

  Close Button (z=3) ← Drawn on top of viewport
```

This is why depth buffer is important - it allows 3D content to use GPU depth testing while still respecting UI z-order!

---

**Next Steps:**
1. ✅ Phase 2 Complete: Hit testing with z-order validated
2. 🎯 Phase 2.1: Implement two-pass rendering with depth buffer
3. 📅 Phase 5: Add BoundsTree optimization if profiling shows need
