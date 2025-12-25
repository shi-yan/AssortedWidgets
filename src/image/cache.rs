use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Arc;

/// Identifier for an image resource
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ImageId {
    /// Load from file path
    File(PathBuf),
    /// Load from URL (future)
    Url(String),
    /// Embedded resource by name
    Embedded(&'static str),
}

impl ImageId {
    /// Create an ImageId from a file path
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Self {
        ImageId::File(path.into())
    }
}

/// Cached GPU texture for an image
pub struct CachedImage {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub size: (u32, u32),
    pub format: wgpu::TextureFormat,
    /// Size in bytes (for LRU eviction tracking)
    pub byte_size: usize,
}

impl CachedImage {
    /// Calculate byte size of the texture
    fn calculate_size(width: u32, height: u32, format: wgpu::TextureFormat) -> usize {
        let bytes_per_pixel = match format {
            wgpu::TextureFormat::Rgba8UnormSrgb => 4,
            wgpu::TextureFormat::Bgra8UnormSrgb => 4,
            _ => 4, // Default to 4 bytes
        };
        (width * height * bytes_per_pixel) as usize
    }

    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image_data: &image::DynamicImage,
    ) -> Self {
        let rgba = image_data.to_rgba8();
        let size = (rgba.width(), rgba.height());
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;

        println!("📐 Image dimensions: {}×{} pixels", size.0, size.1);

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("image_texture"),
            view_formats: &[],
        });

        // Upload pixel data to GPU (using same API as GlyphAtlas)
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.0),
                rows_per_image: Some(size.1),
            },
            wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let byte_size = Self::calculate_size(size.0, size.1, format);

        Self {
            texture,
            view,
            size,
            format,
            byte_size,
        }
    }
}

/// Image cache with LRU eviction
pub struct ImageCache {
    /// LRU cache of loaded images
    cache: LruCache<ImageId, CachedImage>,

    /// GPU device for texture creation
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,

    /// Current cache size in bytes
    current_size_bytes: usize,

    /// Maximum cache size in bytes (default: 256 MB)
    max_size_bytes: usize,
}

#[derive(Debug)]
pub enum ImageError {
    LoadFailed(image::ImageError),
    NotFound(String),
    CacheFull,
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::LoadFailed(e) => write!(f, "Failed to load image: {}", e),
            ImageError::NotFound(path) => write!(f, "Image not found: {}", path),
            ImageError::CacheFull => write!(f, "Image cache is full"),
        }
    }
}

impl std::error::Error for ImageError {}

impl ImageCache {
    /// Create a new image cache
    ///
    /// # Arguments
    /// * `device` - WebGPU device for texture creation
    /// * `queue` - WebGPU queue for texture uploads
    /// * `max_size_mb` - Maximum cache size in megabytes (default: 256 MB)
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>, max_size_mb: usize) -> Self {
        let max_size_bytes = max_size_mb * 1024 * 1024;
        // Start with capacity for ~100 images
        let capacity = NonZeroUsize::new(100).unwrap();

        Self {
            cache: LruCache::new(capacity),
            device,
            queue,
            current_size_bytes: 0,
            max_size_bytes,
        }
    }

    /// Get a cached image or load it from disk
    pub fn get_or_load(&mut self, id: &ImageId) -> Result<&CachedImage, ImageError> {
        // Check if already cached
        if self.cache.contains(id) {
            return Ok(self.cache.get(id).unwrap());
        }

        // Load image data
        let image_data = self.load_image_data(id)?;

        // Create GPU texture
        let cached = CachedImage::new(&self.device, &self.queue, &image_data);
        let image_size = cached.byte_size;

        // Evict old images if cache is full
        while self.current_size_bytes + image_size > self.max_size_bytes && !self.cache.is_empty()
        {
            if let Some((_, evicted)) = self.cache.pop_lru() {
                self.current_size_bytes -= evicted.byte_size;
                println!(
                    "ImageCache: Evicted image ({} bytes), cache now {} MB / {} MB",
                    evicted.byte_size,
                    self.current_size_bytes / (1024 * 1024),
                    self.max_size_bytes / (1024 * 1024)
                );
            } else {
                break;
            }
        }

        // Check if we have space after eviction
        if self.current_size_bytes + image_size > self.max_size_bytes {
            return Err(ImageError::CacheFull);
        }

        // Insert into cache
        self.current_size_bytes += image_size;
        self.cache.put(id.clone(), cached);

        println!(
            "ImageCache: Loaded {:?} ({} bytes), cache now {} MB / {} MB",
            id,
            image_size,
            self.current_size_bytes / (1024 * 1024),
            self.max_size_bytes / (1024 * 1024)
        );

        Ok(self.cache.get(id).unwrap())
    }

    /// Load image data from source
    fn load_image_data(&self, id: &ImageId) -> Result<image::DynamicImage, ImageError> {
        match id {
            ImageId::File(path) => {
                image::open(path).map_err(ImageError::LoadFailed)
            }
            ImageId::Url(_url) => {
                // TODO: Implement network image loading
                Err(ImageError::NotFound("URL loading not yet implemented".to_string()))
            }
            ImageId::Embedded(_name) => {
                // TODO: Implement embedded image loading via rust-embed
                Err(ImageError::NotFound("Embedded images not yet implemented".to_string()))
            }
        }
    }

    /// Get current cache size in bytes
    pub fn size_bytes(&self) -> usize {
        self.current_size_bytes
    }

    /// Get number of cached images
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Clear all cached images
    pub fn clear(&mut self) {
        self.cache.clear();
        self.current_size_bytes = 0;
    }
}
