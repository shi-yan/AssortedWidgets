# Multi-Threaded RawSurface Rendering

**Status:** Design Document (Not Implemented)
**Target:** Future optimization for expensive custom rendering
**Author:** Architecture discussion
**Date:** 2024-12-25

## Overview

This document describes the architecture for multi-threaded rendering of RawSurface widgets, allowing complex 3D scenes to be rendered on background threads while the main thread handles UI compositing.

## Motivation

**When is this needed?**
- Heavy CPU-bound computation (physics simulation, complex mesh generation)
- Want to start rendering frame N+1 while GPU processes frame N
- Complex 3D scenes that take >16ms to encode commands

**When is this NOT needed?**
- Simple 3D scenes (single cube, basic geometry)
- GPU-bound rendering (most cases - GPU is already async!)
- Pure shader-based rendering (compute shaders are better)

## WebGPU Threading Model

### What's Thread-Safe:

✅ **`wgpu::Device`** - `Send + Sync`, can be shared across threads
✅ **`wgpu::Queue`** - `Send + Sync`, thread-safe submission
✅ **Command Encoding** - Each thread can create its own `CommandEncoder`
✅ **Resources** - `Texture`, `Buffer`, etc. are `Send + Sync`

### Key Properties:

1. **Command encoding** can happen in parallel on multiple threads
2. **Queue submission** is serialized but thread-safe
3. **GPU execution** is asynchronous regardless of CPU threading
4. **Resources** can be safely shared with `Arc<T>`

## Architecture Design

### Option 1: Dedicated Render Thread (Recommended)

Each RawSurface widget with expensive rendering gets its own thread.

```rust
pub struct AsyncRawSurface {
    // Widget identity
    id: WidgetId,
    bounds: Rect,

    // Shared GPU resources
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,

    // Double-buffered textures
    front_buffer: Arc<RwLock<wgpu::Texture>>,  // Currently displayed
    back_buffer: Arc<RwLock<wgpu::Texture>>,   // Being rendered to

    // Thread management
    render_thread: Option<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
    frame_ready: Arc<AtomicBool>,

    // Synchronization
    render_complete_rx: Receiver<()>,
    render_start_tx: Sender<RenderParams>,
}

struct RenderParams {
    rotation: f32,
    camera_pos: Vec3,
    // ... other per-frame params
}
```

**Flow:**
```
Main Thread:                    Render Thread:
───────────                     ──────────────
1. Composite frame N            1. Wait for params
   (front_buffer)

2. Send params for N+1  ───────►2. Receive params

3. Continue UI work             3. Encode commands
                                   (render to back_buffer)

4. Wait for frame_ready         4. Submit to queue

                                5. Signal frame_ready

5. Swap buffers         ◄───────6. Wait for next frame
   (front ↔ back)
```

**Key Code:**

```rust
impl AsyncRawSurface {
    pub fn new(/* ... */) -> Self {
        let (render_start_tx, render_start_rx) = channel();
        let (render_complete_tx, render_complete_rx) = channel();

        let device = Arc::clone(&device);
        let queue = Arc::clone(&queue);
        let back_buffer = Arc::clone(&back_buffer);
        let shutdown = Arc::clone(&shutdown);

        let render_thread = std::thread::spawn(move || {
            Self::render_loop(
                device,
                queue,
                back_buffer,
                render_start_rx,
                render_complete_tx,
                shutdown,
            )
        });

        Self {
            // ... fields
            render_thread: Some(render_thread),
        }
    }

    fn render_loop(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        back_buffer: Arc<RwLock<wgpu::Texture>>,
        params_rx: Receiver<RenderParams>,
        complete_tx: Sender<()>,
        shutdown: Arc<AtomicBool>,
    ) {
        while !shutdown.load(Ordering::Acquire) {
            // Wait for frame parameters
            let Ok(params) = params_rx.recv() else { break };

            // Render to back buffer
            let texture = back_buffer.read().unwrap();
            let mut encoder = device.create_command_encoder(&Default::default());

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Async RawSurface Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture.create_view(&Default::default()),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Widget-specific rendering
            Self::render_geometry(&mut render_pass, &params);

            drop(render_pass);
            queue.submit([encoder.finish()]);

            // Signal completion
            complete_tx.send(()).ok();
        }
    }

    fn render_geometry(render_pass: &mut wgpu::RenderPass, params: &RenderParams) {
        // Set pipeline
        // Draw geometry
        // ...
    }
}

// Main thread compositing
impl Widget for AsyncRawSurface {
    fn paint(&self, ctx: &mut PaintContext) {
        // Send parameters for next frame
        let params = RenderParams {
            rotation: self.calculate_rotation(),
            // ...
        };
        self.render_start_tx.send(params).ok();

        // If previous frame is ready, swap buffers
        if self.frame_ready.load(Ordering::Acquire) {
            std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
            self.frame_ready.store(false, Ordering::Release);
        }

        // Composite front buffer to screen
        let texture = self.front_buffer.read().unwrap();
        ctx.draw_texture(&texture, self.bounds);
    }
}
```

### Option 2: Thread Pool (For Many Widgets)

For multiple RawSurface widgets, use a shared thread pool.

```rust
pub struct RawSurfaceThreadPool {
    pool: ThreadPool,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl RawSurfaceThreadPool {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        let pool = ThreadPool::new(num_cpus::get());
        Self { pool, device, queue }
    }

    pub fn render_async<F>(&self, texture: Arc<RwLock<wgpu::Texture>>, render_fn: F)
    where
        F: FnOnce(&mut wgpu::RenderPass) + Send + 'static,
    {
        let device = Arc::clone(&self.device);
        let queue = Arc::clone(&self.queue);

        self.pool.execute(move || {
            let texture = texture.read().unwrap();
            let mut encoder = device.create_command_encoder(&Default::default());
            let mut render_pass = encoder.begin_render_pass(/* ... */);

            render_fn(&mut render_pass);

            drop(render_pass);
            queue.submit([encoder.finish()]);
        });
    }
}
```

### Option 3: WebGPU Native Async (Minimal Threading)

Use WebGPU's built-in async capabilities instead of manual threading.

```rust
impl RawSurface {
    fn render_async(&mut self) -> wgpu::SubmissionIndex {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut render_pass = encoder.begin_render_pass(/* ... */);

        self.paint_raw(&mut render_pass);

        drop(render_pass);

        // Returns submission index for tracking
        self.queue.submit([encoder.finish()])
    }

    fn wait_for_completion(&self, index: wgpu::SubmissionIndex) {
        // Poll until GPU completes work
        self.device.poll(wgpu::Maintain::WaitForSubmissionIndex(index));
    }

    fn is_complete(&self, index: wgpu::SubmissionIndex) -> bool {
        // Non-blocking check
        self.device.poll(wgpu::Maintain::Poll);
        // Check if submission has completed
        // (requires additional tracking mechanism)
        true
    }
}

// Usage
let submission = widget.render_async();
// Do other work...
widget.wait_for_completion(submission);
// Now safe to use results
```

## Synchronization Strategies

### 1. Double Buffering (Recommended)

Two textures: front (display) and back (rendering).

**Pros:**
- Simple to implement
- No blocking on main thread
- Predictable performance

**Cons:**
- Extra memory (2x texture size)
- One frame of latency

### 2. Triple Buffering (Smoother)

Three textures: display, rendering, ready.

**Pros:**
- Can render ahead
- Smoother frame pacing
- Better for variable render times

**Cons:**
- More memory (3x texture size)
- Additional complexity
- Up to 2 frames latency

### 3. Fence-Based Sync (Lowest Latency)

Single texture with GPU fences.

**Pros:**
- Minimal memory overhead
- Lowest latency

**Cons:**
- Can block main thread
- More complex
- Requires careful synchronization

```rust
struct FenceSync {
    fence: Arc<Mutex<Option<wgpu::Fence>>>,
    fence_value: Arc<AtomicU64>,
}

impl FenceSync {
    fn signal_render_complete(&self, queue: &wgpu::Queue) {
        let value = self.fence_value.fetch_add(1, Ordering::SeqCst);
        let fence = self.fence.lock().unwrap();
        if let Some(fence) = fence.as_ref() {
            queue.signal(fence, value);
        }
    }

    fn wait_for_render(&self) {
        let fence = self.fence.lock().unwrap();
        if let Some(fence) = fence.as_ref() {
            let target = self.fence_value.load(Ordering::SeqCst);
            while fence.get_completed_value() < target {
                std::thread::yield_now();
            }
        }
    }
}
```

## Memory Management

### Texture Size Considerations:

```
Single 4K texture (RGBA8):
- 3840 × 2160 × 4 bytes = 33 MB

Double buffering:
- 33 MB × 2 = 66 MB

Ten 1080p widgets with double buffering:
- 1920 × 1080 × 4 × 2 × 10 = 166 MB
```

**Mitigation:**
- Use smaller textures where possible
- Share thread pool across widgets
- Lazy allocation (create textures on-demand)
- Texture compression for static content

## Performance Considerations

### When Multi-Threading Helps:

✅ **Heavy CPU encoding** (>5ms per frame)
```rust
// Example: Complex mesh generation
for i in 0..1_000_000 {
    let vertex = generate_vertex(i);  // CPU-intensive
    vertices.push(vertex);
}
```

✅ **Multiple expensive widgets**
```rust
// 5 complex 3D viewports updating simultaneously
for viewport in viewports {
    pool.render_async(viewport);
}
```

### When Multi-Threading Doesn't Help:

❌ **GPU-bound rendering**
```rust
// GPU is bottleneck, not CPU
draw_call_with_complex_shader();
```

❌ **Simple geometry**
```rust
// Encoding 3 triangles is trivial
render_pass.draw(0..3, 0..1);
```

## Implementation Checklist

### Phase 1: Foundation (Current)
- [x] RawSurface trait
- [x] Single-threaded rendering
- [x] Framebuffer management
- [ ] Texture compositing in Window

### Phase 2: Basic Async
- [ ] Double-buffered textures
- [ ] Dedicated render thread per widget
- [ ] Channel-based communication
- [ ] Basic synchronization

### Phase 3: Optimized Async
- [ ] Shared thread pool
- [ ] Triple buffering
- [ ] Adaptive frame pacing
- [ ] Performance metrics

### Phase 4: Advanced
- [ ] GPU fence synchronization
- [ ] Compute shader integration
- [ ] Multi-GPU support
- [ ] Texture streaming

## Example Usage

```rust
// Create async 3D viewport widget
let viewport = AsyncRawSurface::new(
    widget_id,
    Arc::clone(&device),
    Arc::clone(&queue),
    Size::new(1920.0, 1080.0),
);

// Render thread automatically starts

// In main loop
window.add_widget(viewport);

// Widget handles async rendering internally
// Main thread just composites the result
```

## Migration Path

1. **Start simple** - Get single-threaded RawSurface working
2. **Measure** - Profile to identify bottlenecks
3. **Add async** - Only if measurements show benefit
4. **Optimize** - Fine-tune based on real-world usage

## References

- [WebGPU Spec - Multi-threading](https://www.w3.org/TR/webgpu/#thread-safety)
- [wgpu Threading Guide](https://github.com/gfx-rs/wgpu/wiki/Threading)
- [GPU-Driven Rendering](https://www.slideshare.net/slideshow/secrets-of-cryengine-3-graphics-technology/7655787)

## Conclusion

Multi-threaded RawSurface rendering is a powerful optimization for specific use cases but adds significant complexity. Start with the simple single-threaded implementation and add threading only when profiling shows it's needed.

The architecture is designed to support threading from the start (using `Arc<T>` for resources), making it easy to add later without major refactoring.
