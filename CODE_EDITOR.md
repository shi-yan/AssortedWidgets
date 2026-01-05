# Code Editor Architecture & Implementation Plan

> [!IMPORTANT]
> **Context & User Guidelines**
> This document acts as the single source of truth for the Code Editor implementation. It incorporates the user's original requirements, specific technical feedback, and architectural decisions.

## 1. Original Requirements & Context

### Core Features
1.  **Text Wrapping modes**: Support toggling between soft-wrap and no-wrap (horizontal scroll).
2.  **Gutter**: Left side displaying line numbers.
3.  **Active Line**: Highlight the line currently containing the cursor.
4.  **Minimap**: Right side overview functioning as a scrollbar.
5.  **Syntax Highlighting**: Support coloring via tree-sitter (eventually) or regex (initially).
6.  **Code Folding**: Ability to fold blocks (e.g., between brackets).

### Minimap Specifics ("The VSCode Approach")
*   **Technique**: Use a **Char Sheet** + **Pixmap** approach.
    1.  Rasterize characters onto an image (Char image).
    2.  Shrink them down to 1x2 or 2x2 pixels (for double width characters).
    3.  Blit the char image to a micro-glyph char sheet, as a char to char image hash table.
    4.  Cache these "micro-glyphs" onto a **Minimap Page** (a texture chunk).
*   **Modality**: Modifying the document updates only the corresponding pixels in the minimap.
*   **Constraint**: Minimap Page cutoffs must happen at **new lines (\n)** to prevent pixel migration during text reflow.

### Desired Visual Reference
![Minimap Visual Goal](/Users/shiy/.gemini/antigravity/brain/458fb22e-0ab1-402a-9433-1fdbe857f14a/uploaded_image_1767542499582.png)
*Note: The minimap displays a large chunk of the document, maintaining layout structure.*

---

## 2. Architecture & Design Decisions

### 2.1. Text Storage & Data Structures
*   **Primary Store**: **`ropey::Rope`**.
    *   *Why*: Industry standard, O(log N) random access to lines, efficient inserts/deletes.
*   **Line Access**: We break down by logical lines (`\n`) for storage, but the rendering layer handles visual (wrapped) lines.

### 2.2. Text Layout & Caching Strategy
*   **Challenge**: Text layout (`cosmic-text` shaping/wrapping) is expensive. We cannot layout the entire file.
*   **Strategy**: **LRU (Least Recently Used) Cache**.
    *   We maintain a restricted memory budget of cached `cosmic_text::Buffer` objects.
    *   **Visible Area**: Always high priority in the cache.
    *   **Minimap Generation**: When checking a Minimap Page, if the text layout is missing, we compute it (cache miss), render the pixels to the Minimap Page, and then allow the text layout to be dropped from the LRU cache if space is needed.
    *   This resolves the conflict: We *do* wrapping for the minimap, but we only store the *result* (pixels) permanently, not the heavy layout data.

### 2.3. Minimap Architecture (Deep Dive)
*   **Page Structure**: The minimap is divided into vertical chunks called **Pages**.
    *   **Default Size**: 512 logical lines (growing in 256 line batches).
    *   **Cutoff**: Strictly at `\n`. This ensures that editing line 50 doesn't shift the pixel alignment of line 600 (which is on a different page).
*   **Data Structure**: `BTreeMap<usize, MinimapPage>` where Key = Start Line Index.
    *   *Future Optimization*: **Gap Buffer** or **Interval Tree** to manage pages for faster shifting/updates.
*   **The Rasterization Pipeline**:
    1.  **Input**: A range of text lines (e.g., 0-100).
    2.  **Layout**: Run text wrapping to determine visual positions.
    3.  **Draw**: For each character:
        *   Look up its "Micro-Glyph" (2x2 pixel generic block or specific char representation) from the **Char Sheet**.
        *   Blit it onto the `MinimapPage`'s pixel buffer.
    4.  **Cache**: Store the `MinimapPage` (Resulting Bitmap).
    5.  **Discard**: The intermediate text layout can now be evicted from the LRU cache if needed.

### 2.4. Handling Resizes & Text Reflow
> [!IMPORTANT]
> **Scenario**: Window resize changes editor width.
*   **If Word Wrap is OFF**: No major changes. Minimap and text remain static, only the viewport clip changes.
*   **If Word Wrap is ON**:
    1.  **Invalidation**: The entire text layout is technically invalid because line breaks change.
    2.  **Debounce**: We debounce resize events (e.g., wait 50-100ms after last resize) to avoid thrashing.
    3.  **Visible Viewport**: Re-calculated immediately on the Main Thread. This is fast (only ~50 lines).
    4.  **Minimap**:
        *   Mark **ALL** Minimap Pages as `NeedsReflow`.
        *   **Lazy updates**: Do not regenerate all pages instantly.
        *   Regenerate the **visible** minimap page immediately (on background thread if possible, or main thread if urgent).
        *   Queue other pages for background regeneration.

### 2.5. Multi-Threading Strategy
> [!IMPORTANT]
> **Performance**: Text layout for thousands of lines (minimap) will freeze the UI if done on the main thread.
*   **ThreadPool**: We will use a thread pool (e.g., `rayon` or `std::thread` with a channel).
*   **Strategy**:
    *   **Main Thread**: Handles **Editor Viewport** layout/rendering (Must be responsive, low latency).
    *   **Background Threads**: Handle **Minimap** rasterization and **Syntax Highlighting** (Tree-sitter parsing).
*   **Workflow**:
    1.  Minimap Manager identifies a Page needing generation (dirty or missing).
    2.  It sends a "Job" to the thread pool with: `TextChunk` (string) + `Width` + `CharSheetReference`.
    3.  Background thread runs `cosmic-text` layout -> Rasterizes to `Bitmap`.
    4.  Background thread sends `Bitmap` back to Main Thread via channel.
    5.  Main Thread uploads `Bitmap` to GPU/Texture and updates the `MinimapPage`.

---

## 3. Implementation Plan

### Phase 1: Foundation (The Data Layer)
**Goal**: Basic cached rendering and editing of a "Notepad" like widget.

#### Structs
*   `struct CodeEditor`: The main Widget.
    *   `model: EditorModel`
    *   `layout_cache: LruCache<LineIndex, Buffer>`
*   `struct EditorModel`: Holds the document data.
    *   `rope: ropey::Rope`
    *   `cursors: Vec<Cursor>` (Designed for multi-cursor)
*   `struct EditorConfig`: Theme and settings.
    *   `wrap_mode: WrapMode`
    *   `tab_size: usize`

#### Member Functions
*   `CodeEditor::paint(ctx, bounds)`: Main render loop. Calculates visible lines, checks cache, draws.
*   `CodeEditor::handle_input(event)`: Maps keys to Model actions.
*   `EditorModel::insert_text(pos, text)`: Modifies Rope, updates cursors.
*   `EditorModel::delete_range(range)`: Modifies Rope.
*   `EditorModel::line(idx) -> Cow<str>`: Accessor for rendering.

#### Testing
*   **Load Test**: Load a 10,000 line file. Verify `paint` only processes ~50 lines.
*   **Interaction**: Typing inserts text correctly. Backspace deletes. Scaling the window up/down allows scrolling.

### Phase 2: The Minimap (Background Rasterization)
**Goal**: Pixel-perfect minimap with threading.

#### Structs
*   `struct MinimapManager`: Manages pages and dirty state.
    *   `pages: BTreeMap<usize, MinimapPage>`
    *   `char_sheet: CharSheet`
    *   `thread_pool: ThreadPool`
*   `struct MinimapPage`: A chunk of the minimap.
    *   `bitmap: Pixmap`
    *   `start_line: usize`
    *   `status: PageStatus` (Clean, Dirty, Processing)
*   `struct CharSheet`:
    *   `glyphs: HashMap<char, TinyBitmap>`

#### Member Functions
*   `MinimapManager::update(doc, viewport)`: Checks for dirty pages, queues jobs.
*   `MinimapManager::rasterize_background(job)`: Static function run by thread pool.
*   `CharSheet::generate()`: One-time init to create micro-glyphs.
*   `CodeEditor::handle_resize()`: Triggers `MinimapManager` reflow logic.

#### Testing
*   **Rasterization**: Verify 'A' renders as a specific 2px block.
*   **Update**: Type into line 10. Verify *only* Page 0 is regenerated.
*   **Threading**: Ensure UI does not freeze when generating the full minimap for a 10k line file.

### Phase 3: Text Features & Layout
**Goal**: Visual correctness (wrapping, reflow, colors).

#### Structs
*   `trait SyntaxHighlighter`: Interface for coloring.
    *   `fn highlight(text: &str) -> Vec<(Range, Color)>`
*   `struct RegexHighlighter`: Simple implementation.
*   `struct VirtualLineMap`: Helper to map Logical Lines (`\n` based) to Visual Lines (wrapped).

#### Member Functions
*   `CodeEditor::set_wrap_mode(mode)`: Triggers cache invalidation.
*   `EditorModel::reflow()`: updates `VirtualLineMap`.
*   `RegexHighlighter::highlight()`: Returns span attributes.

#### Testing
*   **Reflow**: Turn on Grid. Resize window. Verify text wraps.
*   **Minimap Sync**: Resize window (Wrap ON). Verify minimap updates to reflect new line counts.
*   **Syntax**: Python file. Verify keywords are colored.

### Phase 4: Interaction Polish
**Goal**: Usability features.

#### Structs
*   `struct Selection`: Represents start/end of a selection range.
    *   `anchor: usize`
    *   `head: usize`
*   `struct EditHistory`: Undo/Redo stack.
    *   `stack: Vec<EditAction>`

#### Member Functions
*   `CodeEditor::handle_drag(start, current)`: Updates selection.
*   `EditorModel::undo()`: Reverts last action from history.
*   `CodeEditor::scroll_to_cursor()`: Auto-scrolls viewport.

#### Testing
*   **Selection**: Drag mouse, Select All, Delete. Text should vanish. Undo. Text should reappear.
*   **Clipboard**: Copy/Paste text.

### Phase 5: Optimization
**Goal**: 100k+ line support.

#### Structs
*   `struct PageGapBuffer`: Optimization for `MinimapManager`.

#### Member Functions
*   `MinimapManager::shift_pages(offset)`: Efficiently rename keys in the map/buffer.

#### Testing
*   **Profile**: Time taken to insert text at line 1 in a 100k line file.
*   **Memory**: Verify LRU cache bounds (e.g., max 50MB).
