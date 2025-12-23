# Mouse Capture & Drag-Drop Demo

This demo showcases the mouse capture system implemented in Phase 2.2, demonstrating how widgets can capture mouse input during drag operations.

## Features Demonstrated

### 1. **Mouse Capture**
- When you click and hold on a rectangle, the mouse is captured
- All mouse events are routed to the captured widget
- Mouse capture continues even when the cursor moves outside the window
- Capture is released when you release the mouse button

### 2. **Draggable Rectangles**
- Each window contains two draggable colored rectangles
- Click and drag to move them around
- Visual feedback: rectangles become semi-transparent while dragging
- Terminal logging shows capture/release events

### 3. **Multi-Window Support**
- Two independent windows, each with their own draggable elements
- Mouse capture works independently in each window
- Demonstrates per-window event handling

## Running the Demo

```bash
cargo run --example drag_drop_demo
```

## What to Try

1. **Basic Dragging**
   - Click and hold on any colored rectangle
   - Drag it around the window
   - Release to drop

2. **Outside Window Dragging**
   - Start dragging a rectangle
   - Move your mouse OUTSIDE the window bounds
   - The rectangle still follows! (mouse is captured)
   - Release anywhere to drop

3. **Terminal Logging**
   Watch the terminal for:
   - `Mouse captured by WidgetId(X)` - when drag starts
   - `Dragging to (x, y)` - position updates
   - `Mouse capture released` - when drag ends

## Architecture

### DraggableRect Widget
- Located in `src/elements/draggable_rect.rs`
- Implements `MouseHandler` trait
- Tracks drag state (is_dragging, drag_offset)
- Returns `EventResponse::Handled` on mouse down to trigger capture

### Mouse Capture Flow

```
1. User clicks rectangle
   └─> MouseDown event dispatched to DraggableRect
       └─> Returns EventResponse::Handled
           └─> Window calls mouse_capture.capture(widget_id)

2. User drags mouse (even outside window!)
   └─> MouseMove events routed to captured widget
       └─> DraggableRect updates its position

3. User releases mouse
   └─> MouseUp event dispatched to DraggableRect
       └─> Returns EventResponse::Handled
           └─> Window calls mouse_capture.release()
```

### Code Locations
- **Widget**: `src/elements/draggable_rect.rs`
- **Mouse Capture**: `src/event/capture.rs`
- **Window Integration**: `src/window.rs` (dispatch_input_event)
- **Demo**: `examples/drag_drop_demo.rs`

## Current Limitations

### Cross-Window Drag-Drop
The current implementation demonstrates mouse capture **within** a window. True cross-window drag-drop with a floating proxy window would require:

1. **Dynamic Window Creation**
   - API to create windows at runtime
   - Borderless/transparent window support
   - Window positioning and sizing

2. **Floating Proxy Window**
   - Create a borderless, semi-transparent window on drag start
   - Update proxy position to follow mouse globally
   - Show dragged element's preview in proxy

3. **Global Drag State**
   - Application-level tracking of active drag operation
   - Source window, dragged widget, drag data
   - Mouse position tracking across all windows

4. **Drop Detection**
   - Detect mouse release over target window
   - Transfer dragged element to target window
   - Update scene graph and layout

5. **Platform Integration**
   - macOS: NSPasteboard for drag data
   - Window-level drag/drop APIs
   - Screen coordinate conversions

## Cross-Window Drag-Drop API (Phase 1 & 2 - IMPLEMENTED)

The framework now supports cross-window drag-drop with floating proxy windows!

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│ Application (Global Drag State)                            │
├─────────────────────────────────────────────────────────────┤
│ • start_drag(source_window, drag_data, screen_pos)         │
│ • update_drag(screen_pos)                                   │
│ • end_drag(screen_pos) -> Option<target_window>            │
│ • get_window_at_screen_position(screen_pos)                │
└─────────────────────────────────────────────────────────────┘
                          ↕
┌─────────────────────────────────────────────────────────────┐
│ GuiEvent Queue                                              │
├─────────────────────────────────────────────────────────────┤
│ • StartCrossWindowDrag { widget_id, drag_data, screen_pos }│
│ • UpdateCrossWindowDrag { screen_pos }                      │
│ • EndCrossWindowDrag { screen_pos }                         │
└─────────────────────────────────────────────────────────────┘
                          ↕
┌─────────────────────────────────────────────────────────────┐
│ Window (Per-Window Event Handling)                         │
├─────────────────────────────────────────────────────────────┤
│ • Detects drag operations via MouseCapture                  │
│ • Converts window coords to screen coords                   │
│ • Emits cross-window drag events when appropriate           │
└─────────────────────────────────────────────────────────────┘
```

### API Reference

#### Application Methods

```rust
// Start cross-window drag - creates floating proxy window
pub fn start_drag(
    &mut self,
    source_window: WindowId,
    drag_data: DragData,
    screen_position: Point,
) -> Result<(), String>

// Update proxy window position as mouse moves
pub fn update_drag(&mut self, screen_position: Point)

// End drag - detects target window and transfers widget
// Returns Some(target_window) if dropped on valid target
pub fn end_drag(&mut self, screen_position: Point) -> Option<WindowId>

// Helper: Find window at screen coordinates
pub fn get_window_at_screen_position(&self, screen_pos: Point) -> Option<WindowId>
```

#### DragData Structure

```rust
pub struct DragData {
    pub widget_id: WidgetId,
    pub color: Color,
    pub label: String,
    pub size: Size,
    pub drag_offset: Point,  // Mouse offset from widget origin
}
```

#### GuiEvent Variants

```rust
pub enum GuiEvent {
    // ... existing events ...

    StartCrossWindowDrag {
        widget_id: WidgetId,
        color: Color,
        label: String,
        size: Size,
        drag_offset: Point,
        screen_position: Point,
    },
    UpdateCrossWindowDrag {
        screen_position: Point,
    },
    EndCrossWindowDrag {
        screen_position: Point,
    },
}
```

### Platform Support

#### Window Options (Implemented)

```rust
pub struct WindowOptions {
    pub borderless: bool,      // No title bar or resize controls
    pub transparent: bool,     // For floating drag proxies
    pub always_on_top: bool,   // Keep proxy above other windows
    pub utility: bool,         // Don't appear in Dock/taskbar
}
```

#### Screen Coordinate Tracking (Implemented)

```rust
// PlatformWindow trait methods
fn window_screen_origin(&self) -> Point;
fn window_to_screen(&self, window_pos: Point) -> Point;
fn screen_to_window(&self, screen_pos: Point) -> Point;
```

### Usage Example

```rust
// In widget's mouse event handler:
fn on_mouse_down(&mut self, event: &mut MouseEvent) -> EventResponse {
    self.is_dragging = true;
    self.drag_offset = Point::new(
        event.position.x - self.bounds.origin.x,
        event.position.y - self.bounds.origin.y,
    );

    // Trigger cross-window drag via event queue
    // (Window converts local coords to screen coords)
    EventResponse::Handled
}

// In Application event loop:
GuiEvent::StartCrossWindowDrag { widget_id, color, label, size, drag_offset, screen_position } => {
    let drag_data = DragData {
        widget_id,
        color,
        label,
        size,
        drag_offset,
    };
    app.start_drag(source_window, drag_data, screen_position)?;
}

GuiEvent::UpdateCrossWindowDrag { screen_position } => {
    app.update_drag(screen_position);
}

GuiEvent::EndCrossWindowDrag { screen_position } => {
    if let Some(target_window) = app.end_drag(screen_position) {
        // Widget was dropped on target_window
        // TODO: Transfer widget from source to target
    }
}
```

### Current Status

✅ **Implemented:**
- Borderless, transparent, always-on-top windows (proxy window support)
- Screen coordinate tracking across windows
- Cross-window drag event system
- Proxy window creation and positioning
- Target window detection via hit testing

⏳ **TODO:**
- Widget transfer logic (remove from source, add to target)
- Scene graph and layout manager updates during transfer
- Update DraggableRect to emit cross-window drag events
- Proxy window visual rendering (currently empty)
- Smooth transition animations

## Future Enhancements

### Phase 3: Drag Data Payload
- Generic drag data with MIME types
- Serialization for complex data structures
- Support for multiple data formats simultaneously

### Phase 4: External Drag-Drop
- System clipboard integration
- Drag-drop from/to external applications (Finder, Chrome, etc.)
- Platform-specific pasteboard/clipboard APIs (NSPasteboard on macOS)

## Technical Notes

### Why Mouse Capture?
Mouse capture is essential for drag operations because:
- Without capture, MouseMove events only fire when cursor is over the widget
- With capture, the widget receives **all** mouse events until release
- Enables smooth dragging even outside window bounds
- Critical for scrollbars, sliders, and drag-drop

### Platform Behavior
On macOS:
- Mouse events are delivered even when cursor is outside the window
- This matches native macOS behavior (try dragging a scrollbar)
- The OS handles mouse tracking during capture

### Event Flow
```
Platform (NSEvent)
  ↓
Event Queue (GuiEvent::InputEvent)
  ↓
Window.dispatch_input_event()
  ↓
Check mouse_capture.captured_id()
  ↓ (if captured)
Route to captured widget
  ↓ (if not captured)
Hit test to find widget under cursor
```

## Testing Checklist

- [ ] Drag a rectangle within window bounds
- [ ] Drag a rectangle outside window bounds
- [ ] Verify rectangle follows mouse when outside
- [ ] Release mouse outside window
- [ ] Verify capture is released
- [ ] Test with both windows
- [ ] Check terminal logs for capture events
- [ ] Verify no capture leaks (release always called)

## Related Documentation

- **Phase 2.2 Implementation**: See `EVENT_HANDLING.md` (Phase 2.2 section)
- **Mouse Capture API**: See `src/event/capture.rs`
- **Focus Management**: See `src/event/focus.rs`
- **Event Handling**: See `EVENT_HANDLING.md` (complete architecture)
