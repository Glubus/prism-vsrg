//! Texture cache for efficient texture loading and reuse.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use wgpu::{Device, Queue, Texture, TextureView};

/// Cached texture with its view.
pub struct CachedTexture {
    pub texture: Texture,
    pub view: TextureView,
    pub width: u32,
    pub height: u32,
}

/// Cache for loaded textures to avoid reloading.
pub struct TextureCache {
    cache: HashMap<PathBuf, Arc<CachedTexture>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl TextureCache {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self {
            cache: HashMap::new(),
            device,
            queue,
        }
    }

    /// Load a texture from path, using cache if available.
    pub fn load(&mut self, path: &Path) -> Option<Arc<CachedTexture>> {
        // Check cache first
        if let Some(cached) = self.cache.get(path) {
            return Some(Arc::clone(cached));
        }

        // Load from disk
        let image = match image::open(path) {
            Ok(img) => img.to_rgba8(),
            Err(e) => {
                log::warn!("Failed to load texture {:?}: {}", path, e);
                return None;
            }
        };

        let dimensions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: path.to_str(),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let cached = Arc::new(CachedTexture {
            texture,
            view,
            width: dimensions.0,
            height: dimensions.1,
        });

        self.cache.insert(path.to_path_buf(), Arc::clone(&cached));
        Some(cached)
    }

    /// Create a solid color texture (for fallbacks).
    pub fn create_solid_color(&self, color: [u8; 4], label: &str) -> CachedTexture {
        let size = wgpu::Extent3d {
            width: 4,
            height: 4,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let data: Vec<u8> = (0..16).flat_map(|_| color.iter().copied()).collect();
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(16),
                rows_per_image: Some(4),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        CachedTexture {
            texture,
            view,
            width: 4,
            height: 4,
        }
    }

    /// Clear the cache (call when changing skins).
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
