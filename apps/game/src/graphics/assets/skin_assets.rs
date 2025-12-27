//! Skin assets - centralized texture loading from skin configuration.

use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{BindGroup, Device, Queue, Sampler};

use super::texture_cache::TextureCache;
use skin::Skin;

/// Maximum columns supported.
pub const MAX_COLUMNS: usize = 18;
/// Minimum columns supported.
pub const MIN_COLUMNS: usize = 4;

/// Assets for a single column.
#[derive(Clone)]
pub struct ColumnAssets {
    /// Note texture bind group
    pub note: Arc<BindGroup>,
    /// Receptor texture bind group (unpressed)
    pub receptor: Arc<BindGroup>,
    /// Receptor texture bind group (pressed)
    pub receptor_pressed: Arc<BindGroup>,
}

/// Assets for a specific key mode (4K, 7K, etc.)
pub struct KeyModeAssets {
    pub columns: Vec<ColumnAssets>,
    pub mine: Option<Arc<BindGroup>>,
    pub hold_body: Option<Arc<BindGroup>>,
    pub hold_end: Option<Arc<BindGroup>>,
    pub burst_body: Option<Arc<BindGroup>>,
    pub burst_end: Option<Arc<BindGroup>>,
}

/// All gameplay-related assets loaded from a skin.
/// Caches all key modes (4K to 18K) at startup.
pub struct SkinAssets {
    /// Assets per key mode (key = key_count: 4, 5, 6, ..., 18)
    key_modes: HashMap<usize, KeyModeAssets>,
    /// Currently selected key mode
    current_key_count: usize,
    /// Sampler for all textures
    sampler: Sampler,
    /// Background (if loaded)
    pub background: Option<Arc<BindGroup>>,
}

impl SkinAssets {
    /// Load assets for ALL key modes (4K to 18K) at startup.
    pub fn load_all(
        device: &Device,
        queue: &Queue,
        skin: &mut Skin,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let mut texture_cache =
            TextureCache::new(Arc::new(device.clone()), Arc::new(queue.clone()));
        let sampler = Self::create_sampler(device);

        let mut key_modes = HashMap::new();

        // Load all key modes from 4K to 18K
        for key_count in MIN_COLUMNS..=MAX_COLUMNS {
            skin.load_key_mode(key_count);

            let mode_assets = Self::load_key_mode(
                device,
                &mut texture_cache,
                skin,
                key_count,
                bind_group_layout,
                &sampler,
            );
            key_modes.insert(key_count, mode_assets);
        }

        log::info!(
            "SKIN_ASSETS: Loaded {} key modes ({}K to {}K)",
            key_modes.len(),
            MIN_COLUMNS,
            MAX_COLUMNS
        );

        Self {
            key_modes,
            current_key_count: 4, // Default
            sampler,
            background: None,
        }
    }

    /// Load assets for a single key mode.
    fn load_key_mode(
        device: &Device,
        cache: &mut TextureCache,
        skin: &Skin,
        key_count: usize,
        layout: &wgpu::BindGroupLayout,
        sampler: &Sampler,
    ) -> KeyModeAssets {
        // Default colors from skin
        let receptor_color = skin.gameplay.receptors.color;
        let def_receptor = [
            (receptor_color[0] * 255.) as u8,
            (receptor_color[1] * 255.) as u8,
            (receptor_color[2] * 255.) as u8,
            (receptor_color[3] * 255.) as u8,
        ];

        let note_color = skin.gameplay.notes.note.color;
        let def_note = [
            (note_color[0] * 255.) as u8,
            (note_color[1] * 255.) as u8,
            (note_color[2] * 255.) as u8,
            (note_color[3] * 255.) as u8,
        ];

        // Load column assets
        let mut columns = Vec::with_capacity(key_count);
        for col in 0..key_count {
            let note = Self::load_column_texture(
                device,
                cache,
                skin.get_note_image(key_count, col),
                def_note,
                &format!("{}K Note Col {}", key_count, col),
                layout,
                sampler,
            );

            let receptor = Self::load_column_texture(
                device,
                cache,
                skin.get_receptor_image(key_count, col),
                def_receptor,
                &format!("{}K Receptor Col {}", key_count, col),
                layout,
                sampler,
            );

            let receptor_pressed = Self::load_column_texture(
                device,
                cache,
                skin.get_receptor_pressed_image(key_count, col)
                    .or_else(|| skin.get_receptor_image(key_count, col)),
                def_receptor,
                &format!("{}K Receptor Pressed Col {}", key_count, col),
                layout,
                sampler,
            );

            columns.push(ColumnAssets {
                note,
                receptor,
                receptor_pressed,
            });
        }

        // Load special note types
        let mine = Self::load_optional_texture(
            device,
            cache,
            skin.get_mine_image(key_count, 0),
            layout,
            sampler,
        );
        let hold_body = Self::load_optional_texture(
            device,
            cache,
            skin.get_hold_body_image(key_count, 0),
            layout,
            sampler,
        );
        let hold_end = Self::load_optional_texture(
            device,
            cache,
            skin.get_hold_end_image(key_count, 0),
            layout,
            sampler,
        );
        let burst_body = Self::load_optional_texture(
            device,
            cache,
            skin.get_burst_body_image(key_count, 0),
            layout,
            sampler,
        );
        let burst_end = Self::load_optional_texture(
            device,
            cache,
            skin.get_burst_end_image(key_count, 0),
            layout,
            sampler,
        );

        KeyModeAssets {
            columns,
            mine,
            hold_body,
            hold_end,
            burst_body,
            burst_end,
        }
    }

    /// Set the current key mode.
    pub fn set_key_count(&mut self, key_count: usize) {
        let clamped = key_count.clamp(MIN_COLUMNS, MAX_COLUMNS);
        self.current_key_count = clamped;
    }

    /// Get the current key count.
    pub fn key_count(&self) -> usize {
        self.current_key_count
    }

    /// Get assets for the current key mode.
    pub fn current_mode(&self) -> Option<&KeyModeAssets> {
        self.key_modes.get(&self.current_key_count)
    }

    /// Get assets for a specific key mode.
    pub fn mode(&self, key_count: usize) -> Option<&KeyModeAssets> {
        self.key_modes.get(&key_count)
    }

    /// Get column assets for current mode.
    pub fn columns(&self) -> &[ColumnAssets] {
        self.current_mode()
            .map(|m| m.columns.as_slice())
            .unwrap_or(&[])
    }

    /// Get a specific column for current mode.
    pub fn column(&self, index: usize) -> Option<&ColumnAssets> {
        self.current_mode()?.columns.get(index)
    }

    fn create_sampler(device: &Device) -> Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Skin Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        })
    }

    fn load_column_texture(
        device: &Device,
        cache: &mut TextureCache,
        path: Option<std::path::PathBuf>,
        default_color: [u8; 4],
        label: &str,
        layout: &wgpu::BindGroupLayout,
        sampler: &Sampler,
    ) -> Arc<BindGroup> {
        let texture = path
            .as_ref()
            .and_then(|p| cache.load(p))
            .unwrap_or_else(|| Arc::new(cache.create_solid_color(default_color, label)));

        Arc::new(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        }))
    }

    fn load_optional_texture(
        device: &Device,
        cache: &mut TextureCache,
        path: Option<std::path::PathBuf>,
        layout: &wgpu::BindGroupLayout,
        sampler: &Sampler,
    ) -> Option<Arc<BindGroup>> {
        let path = path?;
        let texture = cache.load(&path)?;

        Some(Arc::new(device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: path.to_str(),
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
            },
        )))
    }
}
