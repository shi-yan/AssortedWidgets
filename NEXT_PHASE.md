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

#### Phase 3.2: Text Shaping & Two-Tier API (Week 3)

**Goal:** Implement the TextLayout object and both high-level/low-level APIs

**Why Second:** Now that rendering works, build the professional two-tier API architecture

**Architecture:** See [ARCHITECTURE.md § Two-Tier API Design](ARCHITECTURE.md#two-tier-api-design)

**Implementation Steps:**

1. **TextLayout Object** ([src/text/layout.rs](src/text/layout.rs)):
   ```rust
   pub struct TextLayout {
       buffer: cosmic_text::Buffer,
       size: Size<f32>,
       truncate: Option<Truncate>,
   }

   impl TextLayout {
       pub fn size(&self) -> Size<f32> { ... }
       pub fn hit_test(&self, position: Point) -> Option<usize> { ... }
       pub fn cursor_rect(&self, index: usize) -> Option<Rect> { ... }
       pub fn selection_rects(&self, start: usize, end: usize) -> Vec<Rect> { ... }
       pub fn buffer(&self) -> &cosmic_text::Buffer { ... }
   }
   ```

2. **TextEngine with Dual Caching** ([src/text/engine.rs](src/text/engine.rs)):
   ```rust
   pub struct TextEngine {
       font_system: FontSystem,
       swash_cache: SwashCache,

       // High-level: Global LRU cache
       managed_cache: HashMap<TextCacheKey, CachedTextLayout>,
       current_frame: u64,
   }

   #[derive(Hash, PartialEq, Eq)]
   struct TextCacheKey {
       text: String,
       font_size_bits: u32,
       max_width_bits: u32,
   }

   struct CachedTextLayout {
       layout: TextLayout,
       last_used_frame: u64,
   }

   impl TextEngine {
       // Low-level API: Return owned TextLayout
       pub fn create_layout(
           &mut self,
           text: &str,
           style: &TextStyle,
           max_width: Option<f32>,
           truncate: Option<Truncate>,
       ) -> TextLayout { ... }

       // High-level API: Transparently cached
       fn get_or_create_managed(
           &mut self,
           text: &str,
           style: &TextStyle,
           max_width: Option<f32>,
       ) -> &TextLayout { ... }

       pub fn begin_frame(&mut self) { ... }
   }
   ```

3. **PaintContext High-Level API** ([src/paint/context.rs](src/paint/context.rs)):
   ```rust
   impl PaintContext<'_> {
       /// High-level: Draw text with automatic caching
       pub fn draw_text(
           &mut self,
           text: &str,
           style: &TextStyle,
           position: Point,
           max_width: Option<f32>,
       ) {
           let layout = self.text_engine.get_or_create_managed(text, style, max_width);
           self.draw_layout(layout, position, style.color);
       }

       /// Low-level: Render a pre-shaped TextLayout
       pub fn draw_layout(
           &mut self,
           layout: &TextLayout,
           position: Point,
           color: Color,
       ) {
           // Iterate layout.buffer.layout_runs() and push glyphs
       }
   }
   ```

4. **Shaping Features to Test:**
   - **Ligatures:** "office" should render "ffi" as one glyph
   - **Kerning:** "AV" should be closer than "AA"
   - **Complex Scripts:** Arabic text joins correctly
   - **Bidirectional:** Mix "Hello שלום مرحبا" (LTR + RTL)
   - **Font Fallback:** "Hello 你好 👋" automatically uses 3+ fonts
   - **Truncation:** "Very long text..." with ellipsis

5. **Test Scenes:**

   **Scene A: High-Level API (Simple Widgets)**
   ```rust
   // Button label - uses managed cache
   ctx.draw_text("Save", &TextStyle::default(), Point::new(10, 10), None);
   ```

   **Scene B: Low-Level API (Manual Control)**
   ```rust
   // Editor line - widget owns the layout
   struct EditorLine {
       text: String,
       layout: TextLayout,  // Cached by widget
   }

   impl Element for EditorWidget {
       fn paint(&self, ctx: &mut PaintContext) {
           for (idx, line) in self.visible_lines() {
               ctx.draw_layout(&line.layout, Point::new(0, idx * 20), Color::BLACK);
           }
       }
   }
   ```

   **Scene C: Shaping Validation**
   - Draw shaped strings: "The office offers efficient service"
   - Test Arabic: "مرحبا بك" (right-to-left, joining)
   - Test mixed: "English עברית العربية 中文"
   - Verify ligatures appear correctly

**Deliverables:**
- ✅ `TextLayout` object with cosmic-text Buffer
- ✅ `TextEngine` with dual-mode caching (managed + manual)
- ✅ High-level API: `ctx.draw_text()` with transparent LRU cache
- ✅ Low-level API: `engine.create_layout()` + `ctx.draw_layout()`
- ✅ Generational cache eviction (frame-based)
- ✅ Shaped text with ligatures and kerning
- ✅ Complex script support (Arabic, Indic)
- ✅ Bidirectional text rendering
- ✅ Automatic font fallback for multi-language text
- ✅ **Text wrapping works automatically** (cosmic-text handles it via max_width)
- ✅ **Multi-line text rendering works** (cosmic-text Buffer supports it)
- ✅ Test scenes demonstrating both APIs
- ✅ Hit-testing APIs stubbed for Phase 4

**Phase 3.2 Status: ✅ COMPLETE**

**Known Limitations (To be addressed in Phase 3.3):**
- ⚠️ Ellipsis truncation stubbed (lines 223-230 in [src/text/engine.rs](src/text/engine.rs))
- ⚠️ API has too many parameters (needs bundle struct refactoring)
- ⚠️ No TextLabel element with full measure integration yet
- ⚠️ No performance benchmarking or optimization

---

#### Phase 3.3: Polish, Performance & Layout Integration (Week 4)

**Status:** 🚧 NEXT PHASE (Phase 3.2 Complete ✅)

**Goal:** Complete the text rendering system with ellipsis truncation, API cleanup, and full Taffy integration

**Why Now:** Phase 3.2 is complete! Text rendering with ligatures, bidirectional text, emoji, wrapping, and two-tier API all work. Now we need to polish the rough edges, clean up the API, and integrate with the layout system.

**Phase 3.2 Achievements:**
- ✅ Text wrapping works (cosmic-text handles it when you pass `max_width`)
- ✅ Multi-line text works (cosmic-text Buffer supports it natively)
- ✅ Ligatures and kerning work (demonstrated with "office" → "ffi" ligature)
- ✅ Bidirectional text works (mixed LTR/RTL rendering)
- ✅ Multi-language support works (automatic font fallback)
- ✅ Emoji rendering works (color glyphs)
- ✅ Two-tier API functional (high-level managed cache + low-level manual control)
- ✅ Glyph atlas with multi-page support and LRU eviction
- ✅ Retina/HiDPI display support

**Remaining Issues from Phase 3.2:**
- ⚠️ Demo is hacky (embedded in event_loop instead of using Element trait)
- ⚠️ API has too many parameters (needs bundle struct refactoring)
- ⚠️ Ellipsis truncation is stubbed but not implemented
- ⚠️ No TextLabel element with full measure integration yet
- ⚠️ No performance benchmarking or optimization yet

**Critical Architectural Issue: Hacky Demo Implementation**

The current Phase 3.2 demo is embedded directly in `event_loop.rs` via the `render_test_text()` method. This is **not** how the framework is intended to be used. The demo should use the public Element API just like a real GUI integrator would.

**Current Hacky State:**
```rust
// src/event_loop.rs
pub struct GuiEventLoop {
    // ❌ WRONG: Event loop owns rendering infrastructure
    rect_renderer: Option<RectRenderer>,
    text_renderer: Option<TextRenderer>,
    glyph_atlas: Option<GlyphAtlas>,
    font_system: FontSystemWrapper,
    text_engine: TextEngine,

    // ❌ WRONG: Demo-specific state in event loop
    demo_layouts: Option<DemoTextLayouts>,
    demo_start_time: std::time::Instant,
    demo_frame_count: u64,
    demo_atlas_dumped: bool,
}

// ❌ WRONG: Hardcoded demo rendering
fn render_test_text(&mut self, paint_ctx: &mut PaintContext) {
    let shaped_layout = self.text_engine.create_layout(...);
    paint_ctx.draw_layout(&shaped_layout, ...);
    // ... more hardcoded text rendering
}
```

**Why This Is Wrong:**
1. The event loop shouldn't own rendering infrastructure - it should just orchestrate
2. Demo code pollutes the core event loop with demo-specific fields
3. Doesn't demonstrate how real integrators would use the framework
4. Real users would implement Elements, not hack the event loop

**Proper Architecture (Phase 3.3 Goal):**
```rust
// src/elements/text_demo.rs - NEW FILE
pub struct TextDemoElement {
    id: WidgetId,
    bounds: Rect,
    // Element-owned state (like a real widget would have)
}

impl Element for TextDemoElement {
    fn paint(&self, ctx: &mut PaintContext) {
        // Use the public API like a real integrator
        ctx.draw_text("Hello World", &TextStyle::default(), Point::new(10, 10), None);
    }
}

// src/main.rs - Clean demo setup
let mut event_loop = GuiEventLoop::new().await?;
event_loop.create_window(WindowOptions::default())?;

// Add demo element through public API
let demo_id = event_loop.element_manager_mut().create_element(
    TextDemoElement::new()
);
event_loop.scene_graph_mut().set_root(demo_id);

event_loop.run();  // Clean!
```

**Migration Plan (Part of Phase 3.3):**
1. Move rendering infrastructure from `GuiEventLoop` to `PaintContext` (bundle struct)
2. Create proper demo elements using Element trait
3. Remove all demo-specific fields from event loop
4. Update `main.rs` to use public API for demos

This refactoring will:
- ✅ Demonstrate correct framework usage
- ✅ Validate that the public API is actually usable
- ✅ Remove pollution from core event loop
- ✅ Serve as reference implementation for integrators

**Implementation Steps:**

1. **Fix Demo Architecture** (NEW - Highest Priority):

   a. **Move Rendering Infrastructure to PaintContext Bundle:**
   ```rust
   // src/paint/context.rs
   pub struct RenderBundle<'a> {
       pub atlas: &'a mut GlyphAtlas,
       pub font_system: &'a mut FontSystemWrapper,
       pub text_engine: &'a mut TextEngine,
       pub queue: &'a wgpu::Queue,
       pub device: &'a wgpu::Device,
       pub scale_factor: f32,
   }

   pub struct PaintContext<'a> {
       rects: Vec<RectInstance>,
       text_instances: Vec<TextInstance>,
       window_size: Size,
       clip_stack: Vec<Rect>,

       // Bundle all rendering resources
       bundle: RenderBundle<'a>,
   }

   impl PaintContext<'_> {
       // Clean 3-parameter API!
       pub fn draw_layout(
           &mut self,
           layout: &TextLayout,
           position: Point,
           color: Color,
       ) {
           // Access everything from self.bundle
       }

       // High-level API
       pub fn draw_text(
           &mut self,
           text: &str,
           style: &TextStyle,
           position: Point,
           max_width: Option<f32>,
       ) {
           let layout = self.bundle.text_engine.get_or_create_managed(
               text, style, max_width
           );
           self.draw_layout(layout, position, style.color);
       }
   }
   ```

   b. **Create Proper Demo Elements:**
   ```rust
   // src/elements/text_demo.rs
   pub struct TextDemoElement {
       id: WidgetId,
       bounds: Rect,
   }

   impl Element for TextDemoElement {
       fn paint(&self, ctx: &mut PaintContext) {
           let mut y = 50.0;

           // Use high-level API like a real widget
           ctx.draw_text(
               "The office offers efficient service",
               &TextStyle::new().size(18.0),
               Point::new(40.0, y),
               None,
           );
           y += 50.0;

           ctx.draw_text(
               "Hello שלום مرحبا 你好 👋",
               &TextStyle::new().size(24.0),
               Point::new(40.0, y),
               None,
           );
           // ... etc
       }
   }
   ```

   c. **Clean Up Event Loop:**
   - Remove: `demo_layouts`, `demo_start_time`, `demo_frame_count`, `demo_atlas_dumped`
   - Keep: rendering infrastructure but pass to PaintContext as bundle
   - Remove: `render_test_text()` method

   d. **Update main.rs:**
   ```rust
   let mut event_loop = GuiEventLoop::new().await?;
   event_loop.create_window(WindowOptions::default())?;

   // Add demo element through public API
   let demo = TextDemoElement::new(WidgetId::new());
   let demo_id = demo.id();
   event_loop.element_manager_mut().add(Box::new(demo));
   event_loop.scene_graph_mut().set_root(demo_id);

   event_loop.run();
   ```

2. **Implement Ellipsis Truncation** ([src/text/engine.rs](src/text/engine.rs)):

   **Current State (lines 223-230):**
   ```rust
   // Apply truncation if requested
   if truncate == Truncate::End {
       if let Some(width) = max_width {
           // cosmic-text doesn't have built-in ellipsis truncation,
           // so we'll implement it manually in a later phase
           // For now, just wrap
           buffer.set_size(font_system, Some(width), None);
       }
   }
   ```

   **Implementation:**
   ```rust
   // Apply truncation with ellipsis
   if truncate == Truncate::End {
       if let Some(width) = max_width {
           // Strategy: Shape full text, measure width, truncate if needed
           let full_width = buffer.layout_runs()
               .map(|run| run.line_w)
               .max_by(|a, b| a.partial_cmp(b).unwrap())
               .unwrap_or(0.0);

           if full_width > width {
               // Binary search to find max chars that fit with "..."
               let ellipsis = "…";
               let ellipsis_width = measure_text(font_system, ellipsis, style);
               let available = width - ellipsis_width;

               // Find longest prefix that fits
               let truncated = find_truncation_point(
                   font_system, text, style, available
               );

               // Re-shape with ellipsis
               let truncated_text = format!("{}{}", truncated, ellipsis);
               buffer.set_text(font_system, &truncated_text, &attrs, Shaping::Advanced, None);
               buffer.shape_until_scroll(font_system, false);
           }
       }
   }
   ```

2. **Bundle Struct API Cleanup** ([src/paint/context.rs](src/paint/context.rs)):

   **Current Problem:**
   ```rust
   // Too many parameters! (6 parameters)
   pub fn draw_layout(
       &mut self,
       layout: &TextLayout,
       position: Point,
       color: Color,
       atlas: &mut GlyphAtlas,
       font_system: &mut FontSystemWrapper,
       queue: &wgpu::Queue,
   )
   ```

   **Proposed Solution:**
   ```rust
   // Bundle rendering resources
   pub struct RenderBundle<'a> {
       pub atlas: &'a mut GlyphAtlas,
       pub font_system: &'a mut FontSystemWrapper,
       pub queue: &'a wgpu::Queue,
       pub device: &'a wgpu::Device,
   }

   // Cleaner API with only 4 parameters
   pub fn draw_layout(
       &mut self,
       layout: &TextLayout,
       position: Point,
       color: Color,
       bundle: &mut RenderBundle,
   )

   // Or even better: embed bundle in PaintContext
   pub struct PaintContext<'a> {
       rects: Vec<RectInstance>,
       window_size: Size,
       clip_stack: Vec<Rect>,

       // Rendering resources (owned by PaintContext)
       render_bundle: RenderBundle<'a>,
   }

   // Ultimate goal: Just 3 parameters!
   impl PaintContext<'_> {
       pub fn draw_layout(
           &mut self,
           layout: &TextLayout,
           position: Point,
           color: Color,
       ) {
           // Access atlas/font_system/queue from self.render_bundle
       }
   }
   ```

3. **TextLabel Element with Measure Support** ([src/elements/text_label.rs](src/elements/text_label.rs)):
   ```rust
   pub struct TextLabel {
       id: WidgetId,
       bounds: Rect,
       text: String,
       style: TextStyle,
       color: Color,

       /// Cached layout (invalidated on text/width change)
       cached_layout: Option<TextLayout>,
       cached_width: Option<f32>,
   }

   impl TextLabel {
       pub fn set_text(&mut self, text: String) {
           if self.text != text {
               self.text = text;
               self.cached_layout = None;  // Invalidate
               // Will trigger layout recalculation via mark_dirty
           }
       }

       fn ensure_layout(&mut self, engine: &mut TextEngine, max_width: Option<f32>) {
           // Only re-shape if text or width changed
           let needs_reshape = self.cached_layout.is_none()
               || self.cached_width != max_width;

           if needs_reshape {
               self.cached_layout = Some(engine.create_layout(
                   &self.text,
                   &self.style,
                   max_width,
                   None,  // No truncation
               ));
               self.cached_width = max_width;
           }
       }
   }

   impl Element for TextLabel {
       fn measure(
           &mut self,
           engine: &mut TextEngine,
           known_dimensions: taffy::Size<Option<f32>>,
           available_space: taffy::Size<AvailableSpace>,
       ) -> Option<Size> {
           // Case 1: Width is known → wrap to that width
           if let Some(width) = known_dimensions.width {
               self.ensure_layout(engine, Some(width));
               return Some(Size::new(
                   width as f64,
                   self.cached_layout.as_ref()?.size().height as f64
               ));
           }

           // Case 2: Width is auto → return intrinsic size (no wrapping)
           self.ensure_layout(engine, None);
           let size = self.cached_layout.as_ref()?.size();
           Some(Size::new(size.width as f64, size.height as f64))
       }

       fn paint(&self, ctx: &mut PaintContext) {
           if let Some(layout) = &self.cached_layout {
               ctx.draw_layout(layout, self.bounds.origin, self.color);
           }
       }
   }
   ```

4. **Performance Benchmarking & Optimization:**

   **Metrics to Track:**
   - Glyph atlas utilization (pages used vs. wasted space)
   - Cache hit rate (managed API)
   - Frame time breakdown (shaping vs. rasterization vs. rendering)
   - Memory usage (cache size, atlas size)

   **Optimization Targets:**
   - Ensure <16ms frame time with 1000+ unique glyphs
   - Cache hit rate >95% for typical UI (buttons, labels, menus)
   - Atlas growth strategy (when to add pages vs. evict old glyphs)

   **Tools:**
   ```rust
   // Add performance tracking to TextEngine
   pub struct TextEngineStats {
       pub cache_hits: u64,
       pub cache_misses: u64,
       pub shapes_this_frame: u64,
       pub rasterizations_this_frame: u64,
       pub managed_cache_size: usize,
   }

   impl TextEngine {
       pub fn stats(&self) -> TextEngineStats { ... }
       pub fn reset_frame_stats(&mut self) { ... }
   }
   ```

5. **GuiEventLoop Integration with Measure:**
   ```rust
   impl GuiEventLoop {
       fn render_frame_internal(&mut self) {
           if self.needs_layout {
               // Compute layout with measure function
               self.layout_manager.compute_layout_with_measure(
                   self.window_size,
                   |known, available, _node_id, context, _style| {
                       if let Some(ctx) = context {
                           if ctx.needs_measure {
                               if let Some(element) = self.element_manager.get_mut(ctx.widget_id) {
                                   // Pass TextEngine for shaping during measurement
                                   if let Some(size) = element.measure(
                                       &mut self.text_engine,
                                       known,
                                       available
                                   ) {
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

               // Apply layout results
               for widget_id in self.element_manager.widget_ids() {
                   if let Some(bounds) = self.layout_manager.get_layout(widget_id) {
                       if let Some(element) = self.element_manager.get_mut(widget_id) {
                           element.set_bounds(bounds);
                       }
                   }
               }
           }
       }
   }
   ```

6. **Test Scenes for Phase 3.3:**

   **Scene A: Ellipsis Truncation**
   ```rust
   // Single-line label with ellipsis when text overflows
   // Container { width: 150px, height: 24px }
   //   Label { text: "This is very long text that needs truncation", truncate: End }
   //   →  Displays: "This is very long te…"
   ```

   **Scene B: TextLabel with Auto-Sizing**
   ```rust
   // Label with auto width - parent grows to fit
   // Container { width: auto, height: auto }
   //   TextLabel { text: "Short" }  →  Container: 50px wide
   //   TextLabel { text: "Very long text" }  →  Container: 150px wide
   ```

   **Scene C: TextLabel with Wrapping**
   ```rust
   // Label with constrained width - text wraps to multiple lines
   // Container { width: 200px, height: auto }
   //   TextLabel { text: "This is a long paragraph that will wrap" }
   //   →  TextLabel: 200px wide, 60px tall (3 lines)
   ```

   **Scene D: Performance Test**
   ```rust
   // 1000 unique labels (no cache hits)
   // Measure time to shape, rasterize, and render
   // Target: <16ms total frame time
   ```

**Phase 3.3 Deliverables:**
- ⬜ **Fix demo architecture** (Highest Priority):
  - Move rendering infrastructure to PaintContext bundle
  - Remove demo-specific fields from GuiEventLoop
  - Create TextDemoElement using Element trait
  - Update main.rs to use public API
  - Validate that framework is actually usable by integrators
- ⬜ **Bundle struct API cleanup** (reduce parameter count from 7 to 3)
  - `draw_layout()`: 7 params → 3 params
  - `draw_text()`: high-level API with clean interface
  - All rendering resources bundled in PaintContext
- ⬜ **Ellipsis truncation** fully implemented (not just stubbed)
  - Binary search for optimal truncation point
  - Character-level truncation with ellipsis ("…")
  - Proper handling of multi-byte characters
- ⬜ **TextLabel element** with `measure()` implementation
  - Cached layout invalidation on text/width change
  - Full Taffy integration via measure function
  - Bidirectional layout flows:
    - Window resize → text reflows (via measure)
    - Text change → parent resizes via `mark_dirty`
- ⬜ **Performance benchmarking and optimization**
  - `TextEngine` performance stats and monitoring
  - Test scenes covering all features
  - Performance validation: <16ms for 1000+ unique glyphs
  - Cache hit rate metrics (target >95%)
- ⬜ **Documentation update** with clean API examples
  - High-level API usage patterns
  - Low-level API usage patterns
  - When to use which API

**Phase 3 Complete When:**
- ✅ Can render text with shaping, wrapping, multi-language (Phase 3.2 ✅)
- ✅ Font fallback and multi-language support works (Phase 3.2 ✅)
- ✅ Emoji rendering works (Phase 3.2 ✅)
- ✅ Both high-level and low-level APIs functional (Phase 3.2 ✅)
- ✅ Frame-based cache eviction working (Phase 3.2 ✅)
- ⬜ Demo uses Element trait properly (Phase 3.3 target - CRITICAL)
- ⬜ Clean API with bundle struct (Phase 3.3 target)
- ⬜ Text elements integrate with Taffy layout (Phase 3.3 target)
- ⬜ Ellipsis truncation works (Phase 3.3 target)
- ⬜ Performance: <16ms frame time with 1000+ glyphs (Phase 3.3 target)
- ⬜ Documentation updated with API examples (Phase 3.3 target)

**Phase 3.2 Status: ✅ COMPLETE**
- All core text rendering features work
- Text shaping with ligatures and kerning ✅
- Bidirectional and multi-language text ✅
- Emoji rendering ✅
- Text wrapping and multi-line ✅
- Two-tier API (managed + manual) ✅
- Glyph atlas with LRU eviction ✅
- Retina/HiDPI support ✅

**Phase 3.3 Focus:**
- Fix hacky demo architecture (use Element trait)
- Clean up API (bundle struct to reduce parameters)
- Add ellipsis truncation
- Integrate with Taffy layout system
- Performance benchmarking and optimization

---

### Phase 3 Summary: Two-Tier Text Rendering Architecture

**Overview:** Phase 3 implements a professional-grade text rendering system with two complementary APIs designed to serve different use cases.

#### Architecture Highlights

**Two-Tier API:**
- **High-Level Managed API**: `ctx.draw_text()` with automatic LRU caching for simple widgets
- **Low-Level Manual API**: `engine.create_layout()` + `ctx.draw_layout()` for advanced widgets

**Key Components:**
- **TextLayout**: Wraps cosmic-text Buffer, provides hit-testing and geometric queries
- **TextEngine**: Manages FontSystem, SwashCache, and dual-mode caching
- **GlyphAtlas**: Multi-page RGBA8 texture array with etagere bin packing
- **PaintContext**: Provides both high-level and low-level rendering APIs

#### Implementation Order

```
Phase 3.1 (Week 1-2)
  ↓
  GlyphAtlas + TextRenderer + Basic glyph rendering
  ↓
Phase 3.2 (Week 3)
  ↓
  TextLayout object + Two-tier API + Shaping + Caching
  ↓
Phase 3.3 (Week 4)
  ↓
  Wrapping + Measurement + Taffy integration
```

#### API Quick Reference

**For Simple Widgets (Buttons, Labels):**
```rust
impl Element for Button {
    fn paint(&self, ctx: &mut PaintContext) {
        // Automatically cached, zero hassle
        ctx.draw_text(
            &self.label,
            &TextStyle::default().size(14.0),
            self.bounds.origin,
            Some(self.bounds.width())
        );
    }
}
```

**For Advanced Widgets (Editors, Terminals):**
```rust
pub struct EditorLine {
    text: String,
    layout: TextLayout,  // Widget owns the layout
}

impl EditorWidget {
    // Called ONLY when text changes
    fn update_line(&mut self, idx: usize, new_text: String, engine: &mut TextEngine) {
        self.lines[idx].layout = engine.create_layout(
            &new_text,
            &self.style,
            Some(self.viewport_width),
            None,
        );
    }
}

impl Element for EditorWidget {
    fn paint(&self, ctx: &mut PaintContext) {
        // Zero shaping cost - just render pre-computed layouts
        for (idx, line) in self.visible_lines() {
            ctx.draw_layout(&line.layout, Point::new(0, idx * 20), Color::BLACK);
        }
    }
}
```

#### Performance Characteristics

| Operation | High-Level API | Low-Level API |
|-----------|----------------|---------------|
| **First Draw** | Shape + Cache + Render | Shape + Render |
| **Subsequent Draws (Same Text)** | Cache Hit (~0.1μs) + Render | Direct Render |
| **Text Change** | Auto-invalidate + Re-shape | Widget re-shapes explicitly |
| **Memory** | Global LRU (deduplicates) | Widget-owned (isolated) |
| **Best For** | Static UI text (menus, buttons) | Dynamic content (editors, logs) |

#### Cache Eviction Strategy

**High-Level (Generational):**
- Frame counter tracks usage
- Stale entries (unused for 120 frames) purged every 60 frames
- Automatic, invisible to developer

**Low-Level (Widget-Owned):**
- Widget explicitly manages lifecycle
- Drop `TextLayout` when line goes off-screen
- Perfect for virtualized UIs

#### When to Use Which API

| Widget Type | Recommended API | Why |
|-------------|----------------|-----|
| Button, Label, Menu | High-Level Managed | Static text, benefits from deduplication |
| Editor, Terminal | Low-Level Manual | Thousands of unique lines, precise invalidation |
| Tooltip, Status Bar | High-Level Managed | Transient text, LRU handles cleanup |
| File Browser (1000+ items) | Low-Level Manual | Virtualization required |
| Log Viewer | Low-Level Manual | Ring buffer pattern |

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
