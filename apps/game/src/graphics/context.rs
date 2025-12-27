//! Graphics context - wgpu device, queue, and configuration.

use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

/// Core graphics context containing wgpu resources.
pub struct GraphicsContext {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
}

impl GraphicsContext {
    /// Create a new graphics context from an existing window.
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Main Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            })
            .await
            .expect("Failed to create device");

        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("Surface not supported");
        surface.configure(&device, &config);

        Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
            surface,
            config,
        }
    }

    /// Resize the surface.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Get screen dimensions.
    pub fn screen_size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }
}
