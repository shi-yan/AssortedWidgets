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

## Future Enhancements

### Phase 1: Proxy Window (Next Step)
- Add borderless window creation to `WindowOptions`
- Create floating proxy on drag start
- Track global mouse position
- Update proxy window position in real-time

### Phase 2: Cross-Window Drop
- Detect mouse release over target window
- Transfer widget between windows
- Update element manager and scene graph
- Smooth transition animation

### Phase 3: Drag Data
- Generic drag data payload
- MIME type support
- External application drag-drop (system clipboard)

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
