//! Glyph Atlas - GPU texture cache for character glyphs
//!
//! Implements a multi-page RGBA8 texture array with etagere bin packing.
//! Supports both monochrome text and color emoji in a unified system.

use std::collections::HashMap;
use etagere::BucketedAtlasAllocator;
use wgpu;

/// UV rectangle in normalized coordinates (0.0 to 1.0)
#[derive(Debug, Clone, Copy)]
pub struct UvRect {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

/// Key for caching glyphs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    /// Font ID from cosmic-text
    pub font_id: usize,
    /// Font size in fixed-point (size * 1024)
    pub size_bits: u32,
    /// The character
    pub character: char,
    /// Subpixel offset (0-3) for crisp positioning
    pub subpixel_offset: u8,
}

impl GlyphKey {
    pub fn new(font_id: usize, font_size: f32, character: char, subpixel_offset: u8) -> Self {
        Self {
            font_id,
            size_bits: (font_size * 1024.0) as u32,
            character,
            subpixel_offset: subpixel_offset & 0x03, // Clamp to 0-3
        }
    }
}

/// Location of a glyph in the atlas
#[derive(Debug, Clone, Copy)]
pub struct GlyphLocation {
    /// Which page (texture array layer) this glyph is on
    pub page_index: u32,
    /// UV coordinates in the atlas
    pub uv_rect: UvRect,
    /// Glyph metrics for positioning
    pub width: u32,
    pub height: u32,
    pub offset_x: i32,  // Bearing X
    pub offset_y: i32,  // Bearing Y
    /// Is this a color glyph (emoji)?
    pub is_color: bool,
    /// Last frame this glyph was used (for LRU eviction)
    pub last_used_frame: u64,
}

/// One page in the multi-page atlas
struct GlyphPage {
    /// Bin packer for this page
    allocator: BucketedAtlasAllocator,
    /// Page index in texture array
    layer_index: u32,
    /// Last frame any glyph on this page was used
    last_used_frame: u64,
    /// Number of active glyphs on this page
    active_glyph_count: usize,
}

impl GlyphPage {
    fn new(layer_index: u32, size: i32) -> Self {
        let size = etagere::Size::new(size, size);
        Self {
            allocator: BucketedAtlasAllocator::new(size),
            layer_index,
            last_used_frame: 0,
            active_glyph_count: 0,
        }
    }
}

/// Multi-page glyph atlas using RGBA8 texture array
///
/// Phase 3.1 Implementation:
/// - Single page, simple growth strategy
/// - No eviction (rely on page growth)
/// - Defers compaction to Phase 3.2+
pub struct GlyphAtlas {
    /// GPU texture array (each layer is a page)
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,

    /// Pages in the atlas
    pages: Vec<GlyphPage>,

    /// Cache: GlyphKey -> GlyphLocation
    cache: HashMap<GlyphKey, GlyphLocation>,

    /// Current frame counter (for LRU tracking)
    current_frame: u64,

    /// Size of each page (typically 2048x2048)
    page_size: i32,

    /// Maximum number of pages (texture array layers)
    max_pages: u32,
}

impl GlyphAtlas {
    /// Create a new glyph atlas
    ///
    /// # Arguments
    /// * `device` - WebGPU device
    /// * `page_size` - Size of each page (e.g., 2048)
    /// * `max_pages` - Maximum number of pages (layers in texture array)
    pub fn new(device: &wgpu::Device, page_size: i32, max_pages: u32) -> Self {
        // Create texture array with initial capacity for all pages
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glyph Atlas Texture Array"),
            size: wgpu::Extent3d {
                width: page_size as u32,
                height: page_size as u32,
                depth_or_array_layers: max_pages,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Glyph Atlas Texture View"),
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: Some(max_pages),
            usage: None,
        });

        // Start with one page
        let mut pages = Vec::new();
        pages.push(GlyphPage::new(0, page_size));

        Self {
            texture,
            texture_view,
            pages,
            cache: HashMap::new(),
            current_frame: 0,
            page_size,
            max_pages,
        }
    }

    /// Begin a new frame (increment frame counter)
    pub fn begin_frame(&mut self) {
        self.current_frame += 1;
    }

    /// Get a cached glyph location
    pub fn get(&self, key: &GlyphKey) -> Option<&GlyphLocation> {
        self.cache.get(key)
    }

    /// Mark a glyph as used this frame
    pub fn mark_glyph_used(&mut self, key: &GlyphKey) {
        if let Some(location) = self.cache.get_mut(key) {
            location.last_used_frame = self.current_frame;

            // Update page's last-used frame
            if let Some(page) = self.pages.get_mut(location.page_index as usize) {
                page.last_used_frame = self.current_frame;
            }
        }
    }

    /// Insert a new glyph into the atlas
    ///
    /// # Arguments
    /// * `queue` - WebGPU queue for texture upload
    /// * `key` - Glyph key
    /// * `pixels` - RGBA8 pixel data
    /// * `width` - Glyph width
    /// * `height` - Glyph height
    /// * `offset_x` - Bearing X
    /// * `offset_y` - Bearing Y
    /// * `is_color` - Whether this is a color glyph (emoji)
    ///
    /// # Returns
    /// The glyph location in the atlas
    pub fn insert(
        &mut self,
        queue: &wgpu::Queue,
        key: GlyphKey,
        pixels: &[u8],
        width: u32,
        height: u32,
        offset_x: i32,
        offset_y: i32,
        is_color: bool,
    ) -> Result<GlyphLocation, String> {
        // Check if already cached
        if let Some(location) = self.cache.get(&key) {
            return Ok(*location);
        }

        // Try to allocate space in existing pages
        let allocation_size = etagere::Size::new(width as i32, height as i32);
        let mut allocation_result = None;

        for (page_idx, page) in self.pages.iter_mut().enumerate() {
            if let Some(allocation) = page.allocator.allocate(allocation_size) {
                allocation_result = Some((page_idx, allocation));
                break;
            }
        }

        // If no space found, create a new page
        let (page_idx, allocation) = match allocation_result {
            Some(result) => result,
            None => {
                // Check if we can add a new page
                if self.pages.len() >= self.max_pages as usize {
                    return Err(format!(
                        "Atlas full: {} pages, {} max",
                        self.pages.len(),
                        self.max_pages
                    ));
                }

                let new_page_idx = self.pages.len();
                self.pages.push(GlyphPage::new(new_page_idx as u32, self.page_size));

                let page = self.pages.last_mut().unwrap();
                let allocation = page
                    .allocator
                    .allocate(allocation_size)
                    .ok_or_else(|| {
                        format!(
                            "Failed to allocate {}x{} in new page",
                            width, height
                        )
                    })?;

                (new_page_idx, allocation)
            }
        };

        // Upload texture data
        let page = &mut self.pages[page_idx];
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: allocation.rectangle.min.x as u32,
                    y: allocation.rectangle.min.y as u32,
                    z: page.layer_index,
                },
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        // Calculate UV coordinates
        let inv_width = 1.0 / self.page_size as f32;
        let inv_height = 1.0 / self.page_size as f32;

        let uv_rect = UvRect {
            min_x: allocation.rectangle.min.x as f32 * inv_width,
            min_y: allocation.rectangle.min.y as f32 * inv_height,
            max_x: allocation.rectangle.max.x as f32 * inv_width,
            max_y: allocation.rectangle.max.y as f32 * inv_height,
        };

        // Create glyph location
        let location = GlyphLocation {
            page_index: page.layer_index,
            uv_rect,
            width,
            height,
            offset_x,
            offset_y,
            is_color,
            last_used_frame: self.current_frame,
        };

        // Update page stats
        page.active_glyph_count += 1;
        page.last_used_frame = self.current_frame;

        // Cache it
        self.cache.insert(key, location);

        Ok(location)
    }

    /// Get the texture view for binding
    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }

    /// Get atlas statistics
    pub fn stats(&self) -> AtlasStats {
        AtlasStats {
            page_count: self.pages.len(),
            total_glyphs: self.cache.len(),
            page_size: self.page_size,
            current_frame: self.current_frame,
        }
    }
}

/// Atlas statistics for debugging
#[derive(Debug, Clone, Copy)]
pub struct AtlasStats {
    pub page_count: usize,
    pub total_glyphs: usize,
    pub page_size: i32,
    pub current_frame: u64,
}
