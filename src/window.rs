use crate::element_manager::ElementManager;
use crate::layout::LayoutManager;
use crate::paint::PaintContext;
use crate::render::{RenderContext, SharedRenderState};
use crate::scene_graph::SceneGraph;
use crate::types::{Size, WindowId};
use crate::window_render_state::WindowRenderState;

#[cfg(target_os = "macos")]
use crate::platform::{PlatformWindow, PlatformWindowImpl};

use std::sync::Arc;

// ============================================================================
// Window - Per-window state and UI tree
// ============================================================================

/// Represents a single window with its UI tree and rendering state
///
/// Each window has its own:
/// - UI element tree (ElementManager, SceneGraph)
/// - Layout system (LayoutManager)
/// - Rendering surface (WindowRenderState)
///
/// Windows share:
/// - GPU context (RenderContext) - passed by reference
/// - Glyph atlas, fonts, text engine (SharedRenderState) - passed by reference
pub struct Window {
    // ========================================
    // Identity
    // ========================================
    id: WindowId,

    // ========================================
    // Platform Window
    // ========================================
    #[cfg(target_os = "macos")]
    platform_window: PlatformWindowImpl,

    // ========================================
    // UI State (Per-Window)
    // ========================================
    element_manager: ElementManager,
    scene_graph: SceneGraph,
    layout_manager: LayoutManager,
    window_size: Size,
    needs_layout: bool,

    // ========================================
    // Rendering (Per-Window)
    // ========================================
    /// Per-window rendering resources (surface, renderers, scale factor)
    /// References shared state (atlas, fonts) via Arc
    render_state: WindowRenderState,
}

impl Window {
    /// Create a new window (internal - called by Application)
    #[cfg(target_os = "macos")]
    pub(crate) fn new(
        id: WindowId,
        platform_window: PlatformWindowImpl,
        render_state: WindowRenderState,
        window_size: Size,
    ) -> Self {
        Window {
            id,
            platform_window,
            element_manager: ElementManager::new(),
            scene_graph: SceneGraph::new(),
            layout_manager: LayoutManager::new(),
            window_size,
            needs_layout: true,
            render_state,
        }
    }

    /// Get window ID
    pub fn id(&self) -> WindowId {
        self.id
    }

    /// Get window size
    pub fn size(&self) -> Size {
        self.window_size
    }

    /// Set window size and mark layout as dirty
    pub fn set_size(&mut self, size: Size) {
        if self.window_size != size {
            self.window_size = size;
            self.needs_layout = true;
        }
    }

    /// Mark layout as dirty (forces recomputation on next frame)
    pub fn mark_layout_dirty(&mut self) {
        self.needs_layout = true;
    }

    /// Get reference to element manager
    pub fn element_manager(&self) -> &ElementManager {
        &self.element_manager
    }

    /// Get mutable reference to element manager
    pub fn element_manager_mut(&mut self) -> &mut ElementManager {
        &mut self.element_manager
    }

    /// Get reference to scene graph
    pub fn scene_graph(&self) -> &SceneGraph {
        &self.scene_graph
    }

    /// Get mutable reference to scene graph
    pub fn scene_graph_mut(&mut self) -> &mut SceneGraph {
        &mut self.scene_graph
    }

    /// Get reference to layout manager
    pub fn layout_manager(&self) -> &LayoutManager {
        &self.layout_manager
    }

    /// Get mutable reference to layout manager
    pub fn layout_manager_mut(&mut self) -> &mut LayoutManager {
        &mut self.layout_manager
    }

    /// Get reference to render state
    pub fn render_state(&self) -> &WindowRenderState {
        &self.render_state
    }

    /// Get mutable reference to render state
    pub fn render_state_mut(&mut self) -> &mut WindowRenderState {
        &mut self.render_state
    }

    /// Get reference to platform window
    #[cfg(target_os = "macos")]
    pub fn platform_window(&self) -> &PlatformWindowImpl {
        &self.platform_window
    }

    /// Get mutable reference to platform window
    #[cfg(target_os = "macos")]
    pub fn platform_window_mut(&mut self) -> &mut PlatformWindowImpl {
        &mut self.platform_window
    }

    /// Resize window and update all rendering resources
    #[cfg(target_os = "macos")]
    pub fn resize(&mut self, bounds: crate::types::Rect, render_context: &RenderContext) {
        println!("Window {:?} resized to {:.0}x{:.0}", self.id, bounds.size.width, bounds.size.height);

        // Update window size and mark layout as dirty
        self.window_size = bounds.size;
        self.needs_layout = true;

        // Get scale factor from platform window
        let scale_factor = self.platform_window.scale_factor();

        // Calculate physical pixel size for Retina displays
        let physical_size = Size::new(
            bounds.size.width * scale_factor,
            bounds.size.height * scale_factor
        );

        // Resize window renderer (surface)
        self.render_state.renderer.resize(render_context, bounds, scale_factor);

        // Update rect renderer screen size
        self.render_state.rect_renderer.update_screen_size(render_context, physical_size);

        // Update text renderer screen size
        self.render_state.text_renderer.update_screen_size(render_context, physical_size);
    }

    /// Render a single frame
    ///
    /// This performs the full rendering pipeline:
    /// 1. Compute layout if needed
    /// 2. Paint elements (collect draw commands)
    /// 3. Render batched primitives to screen
    #[cfg(target_os = "macos")]
    pub fn render_frame(
        &mut self,
        render_context: &RenderContext,
        shared_render_state: &Arc<SharedRenderState>,
    ) {
        // Get surface texture
        let surface_texture = match self.render_state.renderer.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Failed to get surface texture: {:?}", e);
                return;
            }
        };

        // Create texture view with sRGB format
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(self.render_state.renderer.format.add_srgb_suffix()),
            ..Default::default()
        });

        // 1. Compute layout if needed (skip if no scene graph)
        if self.needs_layout && self.scene_graph.root().is_some() {
            println!("[Window {:?}] Computing layout...", self.id);

            // Mark all elements that need measurement as dirty in Taffy
            let widget_ids: Vec<_> = self.element_manager.widget_ids().collect();
            for widget_id in widget_ids {
                if let Some(element) = self.element_manager.get(widget_id) {
                    if element.needs_measure() {
                        println!("[Window {:?}] Marking widget {:?} as dirty for re-measurement", self.id, widget_id);
                        if let Err(e) = self.layout_manager.mark_dirty(widget_id) {
                            eprintln!("Failed to mark widget {:?} dirty: {}", widget_id, e);
                        }
                    }
                }
            }

            // Use compute_layout_with_measure to support elements with dynamic sizing
            if let Err(e) = self.layout_manager.compute_layout_with_measure(
                self.window_size,
                |known, available, _node_id, context, _style| {
                    // Dispatch to element's measure method if needed
                    if let Some(ctx) = context {
                        if ctx.needs_measure {
                            // Look up element and call its measure() method
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
                    // Default: no intrinsic size
                    taffy::Size::ZERO
                },
            ) {
                eprintln!("Layout computation failed: {}", e);
                return;
            }

            // Apply layout results to elements
            let widget_ids: Vec<_> = self.element_manager.widget_ids().collect();
            for widget_id in widget_ids {
                if let Some(bounds) = self.layout_manager.get_layout(widget_id) {
                    if let Some(element) = self.element_manager.get_mut(widget_id) {
                        element.set_bounds(bounds);
                        element.set_dirty(false);
                    }
                }
            }

            self.needs_layout = false;
        }

        // 2. Paint elements in tree order (collect draw commands)
        let scale_factor = self.platform_window.scale_factor() as f32;

        // Begin new frame for render state (atlas + text engine)
        self.render_state.begin_frame();

        // Paint phase - collect draw commands
        let (rect_instances, text_instances) = {
            // Lock shared resources for the duration of the frame
            let mut atlas_lock = shared_render_state.glyph_atlas.lock().unwrap();
            let mut font_system_lock = shared_render_state.font_system.lock().unwrap();
            let mut text_engine_lock = shared_render_state.text_engine.lock().unwrap();

            // Create render bundle with references to locked resources
            let bundle = crate::paint::RenderBundle {
                atlas: &mut *atlas_lock,
                font_system: &mut *font_system_lock,
                text_engine: &mut *text_engine_lock,
                queue: &render_context.queue,
                device: &render_context.device,
                scale_factor,
            };

            let mut paint_ctx = PaintContext::new(self.window_size, bundle);

            if let Some(root) = self.scene_graph.root() {
                root.traverse(&mut |widget_id| {
                    if let Some(element) = self.element_manager.get(widget_id) {
                        element.paint(&mut paint_ctx);
                    }
                });
            }

            // Extract instances before paint_ctx is dropped
            let rect_instances = paint_ctx.rect_instances().to_vec();
            let text_instances = paint_ctx.text_instances().to_vec();

            (rect_instances, text_instances)
        };

        // 3. Render batched primitives
        let mut encoder = render_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Scene Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Scene Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            // Render all rectangles
            self.render_state.rect_renderer.render(render_context, &mut render_pass, &rect_instances);

            // Get atlas texture view from shared glyph atlas
            let atlas_lock = shared_render_state.glyph_atlas.lock().unwrap();
            let atlas_texture_view = atlas_lock.texture_view();
            self.render_state.text_renderer.render(
                render_context,
                &mut render_pass,
                &text_instances,
                atlas_texture_view,
            );
            drop(atlas_lock);
        }

        // Submit commands
        render_context.queue.submit([encoder.finish()]);

        // Present the frame
        surface_texture.present();
    }
}
