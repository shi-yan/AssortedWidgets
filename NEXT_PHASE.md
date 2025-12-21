# Phase 3: Text Rendering and Advanced Primitives

This document outlines the next phase of development for AssortedWidgets' rendering system. Phases 1-2 (completed) implemented basic layout, clipping, and the advanced layout system with measure function support. Phase 3 will add text rendering, additional primitives, and complete measure function integration.

## Current State (Phase 2 Complete ✅)

### Phase 1 Completed Features
- ✅ Taffy 0.9 integration for Flexbox/Grid layout
- ✅ PaintContext for collecting draw commands
- ✅ RectRenderer with GPU instanced rendering
- ✅ Basic Element trait with `layout()` and `paint()` methods
- ✅ Container and DebugRect elements
- ✅ **Scene graph traversal for correct rendering order**
- ✅ **Simplified ElementManager API (no lifetime hell)**
- ✅ **Layout invalidation on window resize**
- ✅ **Working red/green split test scene**

### Phase 2 Completed Features (NEW ✅)
- ✅ **Shader-based clipping** with clip stack for nested clipping
- ✅ **Taffy 0.9 upgrade** with context-based measure system
- ✅ **Measure function support** in Element trait
- ✅ **Layout system with measure functions** via `compute_layout_with_measure()`
- ✅ **Bidirectional layout flows** documented:
  - Root → Leaves (window resize)
  - Leaves → Root (content changes via mark_dirty)
- ✅ **MeasureContext** for storing measurement data per-node
- ✅ **macOS app delegate** to quit when all windows closed

### Architecture Improvements Made
1. **Rendering Order Fix**: Changed from arbitrary HashMap iteration to scene graph depth-first traversal
2. **ElementManager API**: Simplified from complex iterators to simple ID-based lookup
3. **Three-Tree Design**: ElementManager (storage) + SceneGraph (render order) + LayoutManager (positions)
4. **Layout Pipeline**: Proper invalidation → Taffy computation → bounds application → paint → render
5. **Clipping System**: Shader-based clipping with clip stack (push/pop API)
6. **Measure System**: Context-based measure functions following Taffy 0.9 patterns
7. **Layout Flows**: Clear bidirectional update paths for window resize and content changes

### Current Limitations
1. **No Text Rendering**: Critical for any real UI - needs glyph atlas and text shaping
2. **Limited Primitives**: Only rectangles implemented (no circles, rounded rects, lines)
3. **No Measure Implementation**: Measure functions defined but no real text/image elements using them
4. **Simple Z-Ordering**: Relies on tree traversal order (no explicit z-index)
5. **No Input Handling**: Mouse/keyboard events not routed to elements yet

---

## Phase 3 Features

### 1. Text Rendering System (Highest Priority)

**Goal:** Implement a production-quality text rendering system with glyph atlas and text shaping.

**Status:** Phase 2 laid the groundwork (measure functions, layout system), now we need actual text rendering.

#### Design: Shader-Based Clipping

Add clip rectangle to instance data and discard fragments in shader:

```rust
// src/paint/primitives.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
    pub rect: [f32; 4],        // x, y, width, height
    pub color: [f32; 4],       // r, g, b, a
    pub clip_rect: [f32; 4],   // NEW: clip x, y, width, height
}
```

**Shader Updates:**

```wgsl
// shaders/rect.wgsl
struct VertexInput {
    @builtin(vertex_index) vertex_idx: u32,
    @location(0) rect: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) clip_rect: vec4<f32>,  // NEW
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_pos: vec2<f32>,      // NEW: for clipping test
    @location(2) clip_rect: vec4<f32>,      // NEW: pass to fragment
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // ... existing code ...
    out.world_pos = world_pos;
    out.clip_rect = in.clip_rect;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Clip test: discard if outside clip rect
    let clip_min = in.clip_rect.xy;
    let clip_max = in.clip_rect.xy + in.clip_rect.zw;

    if (in.world_pos.x < clip_min.x || in.world_pos.x > clip_max.x ||
        in.world_pos.y < clip_min.y || in.world_pos.y > clip_max.y) {
        discard;
    }

    return in.color;
}
```

**PaintContext API:**

```rust
// src/paint/context.rs
pub struct PaintContext {
    rects: Vec<RectInstance>,
    window_size: Size,
    clip_stack: Vec<Rect>,  // NEW: stack of clip rects
}

impl PaintContext {
    /// Push a clip rect (intersection of current clip)
    pub fn push_clip(&mut self, rect: Rect) {
        let parent_clip = self.clip_stack.last().copied()
            .unwrap_or(Rect::new(Point::ZERO, self.window_size));

        // Intersect with parent clip
        let clipped = rect.intersection(parent_clip);
        self.clip_stack.push(clipped);
    }

    /// Pop the current clip rect
    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    /// Draw rect with current clip
    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        let clip = self.clip_stack.last().copied()
            .unwrap_or(Rect::new(Point::ZERO, self.window_size));

        self.rects.push(RectInstance {
            rect: [rect.origin.x as f32, rect.origin.y as f32,
                   rect.size.width as f32, rect.size.height as f32],
            color: [color.r, color.g, color.b, color.a],
            clip_rect: [clip.origin.x as f32, clip.origin.y as f32,
                       clip.size.width as f32, clip.size.height as f32],
        });
    }
}
```

**Usage in Scrollable Container:**

```rust
impl Element for ScrollArea {
    fn paint(&self, ctx: &mut PaintContext) {
        // Set clip to viewport
        ctx.push_clip(self.viewport_rect);

        // Draw children (automatically clipped)
        for child in &self.children {
            child.paint(ctx);
        }

        ctx.pop_clip();

        // Draw scrollbars (not clipped)
        ctx.draw_rect(self.scrollbar_rect, Color::rgba(0.5, 0.5, 0.5, 0.8));
    }
}
```

**Benefits:**
- ✅ Zero GPU state changes (perfect batching)
- ✅ Nested clipping works automatically (clip stack intersection)
- ✅ Pixel-perfect clipping in fragment shader
- ✅ Coexists with other rendering

---

### 2. Depth Buffer Integration (Medium Priority)

**Goal:** Use GPU hardware depth testing for correct occlusion instead of z-sorting.

#### Current Approach: Z-Sorting

```rust
// Simple but limited
fn paint_tree(&mut self, ctx: &mut PaintContext) {
    // Traverse in depth-first order
    for child in &children {
        paint_tree(child, ctx);  // Drawing order = traversal order
    }
}
```

**Problems:**
- Relies on tree traversal order
- No explicit z-ordering
- Can't render overlapping siblings correctly without careful tree structure

#### Phase 2 Approach: Depth Buffer

**Add z-value to instances:**

```rust
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub clip_rect: [f32; 4],
    pub z_index: f32,  // NEW: depth value (0.0 = back, 1.0 = front)
}
```

**Enable depth testing in pipeline:**

```rust
// src/paint/rect_renderer.rs
let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    // ... existing config ...
    depth_stencil: Some(wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth24Plus,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,  // Closer fragments win
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    }),
    // ...
});
```

**Shader updates:**

```wgsl
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // ... existing code ...

    // Map z_index (0..1) to clip space depth
    out.position = vec4(clip_pos, in.z_index, 1.0);  // Use z_index as depth
    return out;
}
```

**Z-value assignment during tree traversal:**

```rust
fn paint_tree_with_depth(&mut self, node: &SceneNode, ctx: &mut PaintContext, depth: f32) {
    // Paint current node
    ctx.set_depth(depth);
    node.element.paint(ctx);

    // Paint children with incremented depth
    for (i, child) in node.children.iter().enumerate() {
        let child_depth = depth + 0.001 * (i as f32);  // Increment per child
        paint_tree_with_depth(child, ctx, child_depth);
    }
}
```

**Benefits:**
- ✅ Automatic occlusion handling by GPU
- ✅ No need for manual sorting
- ✅ Explicit z-ordering when needed
- ✅ Better performance for overlapping elements

**Trade-offs:**
- Requires depth buffer allocation (small memory cost)
- Need to manage z-value range carefully (avoid z-fighting)

---

### 3. Advanced Z-Ordering (Low Priority)

**Goal:** Production-quality layering with explicit z-index support.

#### Layered Batching with 64-bit Sort Keys

This is the approach used by production UI frameworks (Chrome, Flutter):

```rust
pub struct DrawCommand {
    sort_key: u64,
    instance: RectInstance,
}

// Sort key encoding:
// Bits 63-48: Layer (16 bits) - explicit z-index
// Bits 47-32: Tree depth (16 bits)
// Bits 31-16: Material ID (16 bits) - for state batching
// Bits 15-0:  Instance ID (16 bits) - stable sort

fn encode_sort_key(layer: u16, depth: u16, material: u16, instance: u16) -> u64 {
    ((layer as u64) << 48) |
    ((depth as u64) << 32) |
    ((material as u64) << 16) |
    (instance as u64)
}
```

**Element API:**

```rust
pub trait Element {
    // ...
    fn z_index(&self) -> i32 { 0 }  // Explicit layer override
}

// Example: Modal dialog
impl Element for Modal {
    fn z_index(&self) -> i32 { 1000 }  // Always on top
}
```

**Rendering:**

```rust
fn render_sorted(commands: &mut Vec<DrawCommand>) {
    // Sort by 64-bit key (stable, fast)
    commands.sort_by_key(|cmd| cmd.sort_key);

    // Batch consecutive commands with same material
    let mut current_material = 0;
    let mut batch = Vec::new();

    for cmd in commands {
        let material = (cmd.sort_key >> 16) & 0xFFFF;
        if material != current_material {
            render_batch(&batch);
            batch.clear();
            current_material = material;
        }
        batch.push(cmd.instance);
    }

    render_batch(&batch);
}
```

**Benefits:**
- ✅ Explicit layer control (modals, tooltips, overlays)
- ✅ Automatic batching within layers
- ✅ Predictable rendering order
- ✅ Industry-proven approach

---

### 4. Additional Primitives (Medium Priority)

Extend beyond rectangles to support common UI needs:

#### Circle Renderer

```rust
// src/paint/circle_renderer.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CircleInstance {
    pub center: [f32; 2],
    pub radius: f32,
    pub color: [f32; 4],
    pub clip_rect: [f32; 4],
}
```

```wgsl
// shaders/circle.wgsl
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist = length(in.local_pos - vec2(0.5, 0.5));
    if dist > 0.5 {
        discard;  // Outside circle
    }

    // Smooth anti-aliasing
    let alpha = 1.0 - smoothstep(0.48, 0.5, dist);
    return vec4(in.color.rgb, in.color.a * alpha);
}
```

#### Rounded Rectangle Renderer

```rust
pub struct RoundedRectInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub corner_radius: f32,  // Single radius for all corners
    pub clip_rect: [f32; 4],
}
```

#### Line/Stroke Renderer

```rust
pub struct LineInstance {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub width: f32,
    pub color: [f32; 4],
    pub clip_rect: [f32; 4],
}
```

---

### 5. Text Rendering (High Priority - Future)

This is a major undertaking and will likely be its own phase. Documented here for completeness.

#### Glyph Atlas Approach

See [ARCHITECTURE.md § Text Rendering](#text-rendering) for detailed design.

**Key Components:**
1. Font loading (fontdue or cosmic-text)
2. Text shaping (harfbuzz via rustybuzz)
3. Glyph rasterization
4. Atlas packing (etagere)
5. Instanced quad rendering

**Estimated Complexity:** 2-3 weeks of focused development

---

## Implementation Plan

### Phase 2.1: Clipping (Essential for scrolling)

1. Update `RectInstance` with `clip_rect` field
2. Modify rect shader for fragment clipping
3. Add `PaintContext::push_clip()` / `pop_clip()`
4. Update vertex buffer layout in RectRenderer
5. Test with ScrollArea element

**Estimated time:** 1-2 days

### Phase 2.2: Depth Buffer (Better z-ordering)

1. Create depth texture in RenderContext
2. Enable depth testing in RectRenderer pipeline
3. Add `z_index` field to RectInstance
4. Implement depth assignment during tree traversal
5. Test with overlapping elements

**Estimated time:** 1 day

### Phase 2.3: Additional Primitives (UI completeness)

1. Implement CircleRenderer
2. Implement RoundedRectRenderer
3. Implement LineRenderer
4. Update PaintContext API
5. Create test elements using new primitives

**Estimated time:** 2-3 days

### Phase 2.4: Advanced Z-Ordering (Optional optimization)

1. Design 64-bit sort key encoding
2. Implement sort key generation during traversal
3. Sort commands before rendering
4. Batch consecutive commands with same state
5. Benchmark vs current approach

**Estimated time:** 2-3 days (low priority)

---

## Testing Strategy

### Clipping Tests
- Nested scroll areas
- Clip rect intersection
- Scroll position updates
- Edge cases (zero-size clips, negative offsets)

### Depth Buffer Tests
- Overlapping siblings
- Parent-child occlusion
- Explicit z-index ordering
- Transparent elements

### Performance Tests
- 10,000+ rectangles with varied z-ordering
- Deep nesting (20+ levels)
- Rapid clip rect changes (scrolling)
- Frame time budget compliance (<16ms)

---

## Open Questions

1. **Stencil Buffer:** Should we use stencil for complex clipping (rounded corners)?
   - Pros: Hardware accelerated, handles any shape
   - Cons: State changes break batching
   - Decision: Defer until needed (shader clipping sufficient for Phase 2)

2. **Clip Rect Optimization:** Can we cull instances entirely outside clip?
   - Would reduce GPU fragment workload
   - Requires CPU-side rect intersection test
   - Benchmark to determine if worthwhile

3. **Depth Precision:** What z-value range and increment?
   - 24-bit depth buffer gives ~16 million discrete values
   - Need to handle deep trees without z-fighting
   - Proposal: 0.001 increment = 1000 levels max

---

## Success Criteria

Phase 3 will be considered complete when:

- ✅ Text rendering with glyph atlas works
- ✅ TextLabel element with measure function implemented
- ✅ At least 3 primitive types (rect, circle, rounded rect)
- ✅ All tests pass with text elements
- ✅ Frame budget maintained (<16ms for 10k elements)
- ✅ Documentation updated with text rendering APIs

---

## Recommended Next Steps

Based on the current state (Phase 2 complete), here's the recommended order of implementation for Phase 3:

### 1. **Text Rendering System** (Highest Priority - Split into 3 Sub-Phases)

**Status:** ✅ Measure system ready, need actual text implementation

**Overall Approach:** cosmic-text (brain) + multi-page glyph atlas (memory)

See [ARCHITECTURE.md § Text Rendering](ARCHITECTURE.md#text-rendering) for detailed design decisions.

---

#### Phase 3.1: Character Sheet Foundation (Week 1-2)

**Goal:** Build the GPU texture atlas system and render individual characters

**Why First:** Establishes the rendering foundation before adding complexity

**Implementation Steps:**

1. **Dependencies:**
   ```toml
   [dependencies]
   cosmic-text = "0.15"  # Font discovery, rasterization
   etagere = "0.2"       # 2D bin packing
   ```

2. **GlyphAtlas Implementation** ([src/text/atlas.rs](src/text/atlas.rs)):
   - RGBA8 texture array (start with 1 page of 2048×2048)
   - `etagere::BucketedAtlasAllocator` for bin packing
   - `HashMap<GlyphKey, GlyphLocation>` for cache
   - Simple "no eviction" strategy (grow pages as needed)
   - Methods:
     - `insert(key, pixels, width, height, is_color) -> UvRect`
     - `get(key) -> Option<GlyphLocation>`
     - `begin_frame()` for frame counter

3. **cosmic-text Integration** ([src/text/font_system.rs](src/text/font_system.rs)):
   - Initialize `FontSystem` (discovers system fonts)
   - Initialize `SwashCache` for rasterization
   - Font discovery works automatically

4. **TextRenderer** ([src/text/renderer.rs](src/text/renderer.rs)):
   - GPU pipeline for instanced quads
   - Shader with glyph type flag (mono vs. color emoji)
   - Instance buffer management (like RectRenderer)
   - Methods:
     - `draw_glyph(char, x, y, font_size, color)`
     - `flush(render_pass)` to batch render

5. **Shader** ([shaders/text.wgsl](shaders/text.wgsl)):
   - Vertex shader: quad generation + UV mapping
   - Fragment shader: sample atlas, handle mono vs. color
   - Support clipping (reuse clip_rect pattern from RectRenderer)

6. **Test Scene:**
   - Draw individual characters: "H", "e", "l", "l", "o"
   - Each at manual positions (no shaping/layout yet)
   - Mix text and emoji: "Hello 👋"
   - Verify atlas caching (subsequent frames reuse glyphs)

**Deliverables:**
- ✅ RGBA8 texture atlas with etagere packing
- ✅ cosmic-text font system initialized
- ✅ Can render individual glyphs with font fallback
- ✅ Emoji support works
- ✅ Atlas grows automatically when full
- ✅ Simple test showing "Hello 👋" with manual positioning

**Known Limitations (Intentional):**
- ❌ No text shaping (ligatures, kerning don't work)
- ❌ No automatic layout (must position each glyph manually)
- ❌ No wrapping or measurement
- ❌ Complex scripts (Arabic, Indic) won't render correctly

---

#### Phase 3.2: Text Shaping (Week 3)

**Goal:** Use cosmic-text's Buffer/LayoutRun for proper text layout

**Why Second:** Now that rendering works, add correct positioning/shaping

**Implementation Steps:**

1. **cosmic-text Buffer Integration:**
   - Create `cosmic_text::Buffer` for text layout
   - Use `buffer.set_text()` to set content
   - Use `buffer.layout_runs()` to get shaped glyphs

2. **Update TextRenderer:**
   - New method: `draw_text(buffer: &Buffer, x, y, color)`
   - Iterate `layout_runs()` and `run.glyphs`
   - Use `glyph.x`, `glyph.y` for positions (cosmic-text provides these)
   - Handle `glyph.cache_key` for rasterization

3. **Shaping Features to Test:**
   - **Ligatures:** "office" should render "ffi" as one glyph
   - **Kerning:** "AV" should be closer than "AA"
   - **Complex Scripts:** Arabic text joins correctly
   - **Bidirectional:** Mix "Hello שלום مرحبا" (LTR + RTL)
   - **Font Fallback:** "Hello 你好 👋" automatically uses 3+ fonts

4. **Test Scene:**
   - Draw shaped strings: "The office offers efficient service"
   - Test Arabic: "مرحبا بك" (right-to-left, joining)
   - Test mixed: "English עברית العربية 中文"
   - Verify ligatures appear correctly

**Deliverables:**
- ✅ Shaped text with ligatures and kerning
- ✅ Complex script support (Arabic, Indic)
- ✅ Bidirectional text rendering
- ✅ Automatic font fallback for multi-language text
- ✅ Test scene with shaped text examples

**Still Missing (Intentional):**
- ❌ Text wrapping (all on one line)
- ❌ Measurement for layout system
- ❌ Multi-line support

---

#### Phase 3.3: Measurement & Wrapping (Week 4)

**Goal:** Integrate with Taffy layout system via measure functions

**Why Last:** Requires working rendering + shaping foundation

**Implementation Steps:**

1. **TextLabel Element** ([src/elements/text_label.rs](src/elements/text_label.rs)):
   ```rust
   pub struct TextLabel {
       id: WidgetId,
       bounds: Rect,
       text: String,
       font_size: f32,
       color: Color,
       buffer: cosmic_text::Buffer,  // Cached layout
       needs_reshape: bool,
   }
   ```

2. **Implement Element::measure():**
   ```rust
   fn measure(
       &self,
       known_dimensions: taffy::Size<Option<f32>>,
       available_space: taffy::Size<AvailableSpace>,
   ) -> Option<Size> {
       // If width is known, wrap text to that width
       if let Some(width) = known_dimensions.width {
           self.buffer.set_size(width, f32::MAX);
           self.buffer.shape_until_scroll();

           // Return measured height after wrapping
           let height = self.buffer.layout_runs()
               .map(|run| run.line_y)
               .max()
               .unwrap_or(self.font_size);

           return Some(Size::new(width, height));
       }

       // If width is auto, return intrinsic size (single line)
       let width = self.buffer.layout_runs()
           .map(|run| run.line_w)
           .max()
           .unwrap_or(0.0);

       Some(Size::new(width, self.font_size))
   }
   ```

3. **Text Wrapping:**
   - Use `buffer.set_size(width, f32::MAX)` to set wrap width
   - `buffer.shape_until_scroll()` performs wrapping
   - Iterate `layout_runs()` for multi-line rendering

4. **Text Truncation:**
   - Detect when text exceeds available height
   - Use `buffer.set_size(width, height)` to clip
   - Optionally append "..." ellipsis

5. **Integration with GuiEventLoop:**
   - Update event loop to call `compute_layout_with_measure()`
   - Dispatch to `element.measure()` for text elements
   - Test bidirectional layout:
     - Window resize → text wraps
     - Text content change → parent resizes

6. **Test Scenes:**
   - Auto-sizing label: text grows, parent container grows
   - Fixed-width label: text wraps to multiple lines
   - Scrollable area: text truncates with ellipsis
   - Mix with other elements: button with text label

**Deliverables:**
- ✅ TextLabel element with `measure()` implementation
- ✅ Text wrapping based on available width
- ✅ Multi-line text rendering
- ✅ Text truncation with ellipsis
- ✅ Bidirectional layout works:
  - Window resize → text reflows
  - Text change → parent resizes
- ✅ Integration test: button with auto-sizing label

**Phase 3 Complete When:**
- ✅ Can render text with shaping, wrapping, measurement
- ✅ Text elements integrate with Taffy layout
- ✅ Font fallback and multi-language support works
- ✅ Emoji rendering works
- ✅ Performance: <16ms frame time with 1000+ glyphs

---

### 2. **Complete Measure Function Integration** (High Priority)
**Status:** ✅ API ready, need real usage

**Approach:** Update GuiEventLoop to use compute_layout_with_measure()

**Implementation:**
```rust
// Update event loop to use measure functions
if self.needs_layout {
    self.layout_manager.compute_layout_with_measure(
        self.window_size,
        |known, available, _node_id, context, _style| {
            if let Some(ctx) = context {
                if ctx.needs_measure {
                    // Dispatch to element's measure method
                    if let Some(element) = self.element_manager.get(ctx.widget_id) {
                        if let Some(size) = element.measure(known, available) {
                            return taffy::Size {
                                width: size.width as f32,
                                height: size.height as f32,
                            };
                        }
                    }
                }
            }
            taffy::Size::ZERO
        },
    )?;
}
```

**Deliverable:** Fully working bidirectional layout with text elements

### 3. **Additional Primitives** (Medium Priority)
**Why Third:** Needed for realistic UI elements (buttons, badges, separators)

**Approach:** Implement CircleRenderer and RoundedRectRenderer
- Reuse instancing pattern from RectRenderer
- Smooth anti-aliasing in shaders using distance fields
- Add to PaintContext API

**Deliverable:** Circles, rounded rectangles, and lines

### 4. **Input Handling & Hit Testing** (Medium Priority)
**Status:** Platform input events exist, need routing to elements

**Approach:**
- Implement hit testing via scene graph traversal
- Add event propagation (bubbling/capturing like DOM)
- Route PlatformInput to elements via on_event()
- Handle focus management

**Deliverable:** Clickable buttons, hover states, keyboard focus

### 5. **Test Elements & Examples** (Medium Priority)
**Why:** Validate the entire system works end-to-end

**Approach:**
- Create TextLabel with dynamic sizing
- Create Button with hover/press states
- Create ScrollArea using clipping
- Create example app demonstrating all features

**Deliverable:** Working demo app with interactive UI

## Beyond Phase 3

Future phases will tackle:

- **Phase 4:** Advanced input (drag & drop, gestures, IME integration)
- **Phase 5:** Animation system (tweening, transitions, layout animations)
- **Phase 6:** Additional elements (TextInput, Checkbox, Slider, etc.)
- **Phase 7:** Theme system (uniform buffer, style propagation)
- **Phase 8:** Accessibility (screen readers, keyboard navigation)
- **Phase 9:** Developer tools (inspector, performance profiler)

Each phase builds on the previous, maintaining architectural consistency while adding capability.

---

## What Phase 1 Taught Us

### Technical Lessons

1. **Rust Lifetime Complexity**: Iterators over trait objects are hard. Simple ID-based lookup is often better.

2. **Separation of Concerns**: Three separate trees (storage, render order, layout) is cleaner than trying to make one tree do everything.

3. **Explicit Over Implicit**: Marking layout dirty explicitly is better than trying to auto-detect changes.

4. **Scene Graph Traversal**: Essential for correct rendering order—don't rely on HashMap iteration!

### Design Validation

✅ **Event Queue Architecture**: Clean separation, no RefCell needed
✅ **Taffy Integration**: Works great for layout, caching is fast
✅ **Instanced Rendering**: Excellent performance, easy to extend
✅ **Flat Storage**: Fast lookups, good cache locality

### Next Architecture Decisions

- **Clipping**: Shader-based vs stencil buffer? → Shader-based (better batching)
- **Text**: Glyph atlas vs path rendering? → Glyph atlas (industry standard)
- **Input**: Event bubbling vs capture? → Both (like DOM)
- **State**: Mutable elements vs immutable props? → Mutable (Rust-friendly)
