//! Animated particle background component.
//!
//! A reusable wgpu component for rendering animated particles
//! with connection lines between nearby particles.

use rand::Rng;
use std::borrow::Cow;
use wgpu::util::DeviceExt;

use crate::shaders::constants::PARTICLE_SHADER_SRC;

/// Configuration for the particle system
#[derive(Clone, Debug)]
pub struct ParticleConfig {
    /// Number of particles (default: 80)
    pub count: u32,
    /// Maximum connection distance in pixels (default: 120.0)
    pub connection_distance: f32,
    /// Particle speed multiplier (default: 1.0)
    pub speed: f32,
    /// Minimum particle size (default: 1.0)
    pub min_size: f32,
    /// Maximum particle size (default: 3.0)
    pub max_size: f32,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            count: 80,
            connection_distance: 120.0,
            speed: 0.3,
            min_size: 1.0,
            max_size: 3.0,
        }
    }
}

impl ParticleConfig {
    /// Dense particle configuration (more particles, closer connections)
    pub fn dense() -> Self {
        Self {
            count: 120,
            connection_distance: 100.0,
            speed: 0.8,
            min_size: 1.0,
            max_size: 2.5,
        }
    }

    /// Sparse particle configuration (fewer particles, further connections)
    pub fn sparse() -> Self {
        Self {
            count: 40,
            connection_distance: 180.0,
            speed: 0.5,
            min_size: 1.5,
            max_size: 4.0,
        }
    }

    /// Custom particle count
    pub fn with_count(count: u32) -> Self {
        Self {
            count,
            ..Default::default()
        }
    }
}

/// Uniform data for the particle shader
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ParticleUniforms {
    time: f32,
    width: f32,
    height: f32,
    particle_count: u32,
}

/// Individual particle data
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Particle {
    position: [f32; 2],
    velocity: [f32; 2],
    size: f32,
    _padding: f32,
}

/// Line instance for connecting particles
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct LineInstance {
    start_idx: u32,
    end_idx: u32,
    alpha: f32,
    _padding: f32,
}

/// Animated particle system renderer.
///
/// # Example
/// ```ignore
/// let particles = ParticleSystem::new(&device, format, 1280.0, 720.0, ParticleConfig::default());
/// // In render loop:
/// particles.render(&mut render_pass, &queue);
/// ```
pub struct ParticleSystem {
    // Particle data (updated on CPU)
    particles: Vec<Particle>,
    config: ParticleConfig,

    // GPU resources for particles
    particle_buffer: wgpu::Buffer,
    particle_pipeline: wgpu::RenderPipeline,
    particle_bind_group: wgpu::BindGroup,

    // GPU resources for lines
    line_buffer: wgpu::Buffer,
    line_pipeline: wgpu::RenderPipeline,
    line_bind_group: wgpu::BindGroup,
    line_count: u32,

    // Shared uniform buffer
    uniform_buffer: wgpu::Buffer,

    // Screen size for bounds
    width: f32,
    height: f32,
}

impl ParticleSystem {
    /// Create a new particle system with the given configuration.
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: f32,
        height: f32,
        config: ParticleConfig,
    ) -> Self {
        // Initialize particles with random positions and velocities
        let mut rng = rand::rng();
        let base_speed = 0.4 * config.speed;
        let particles: Vec<Particle> = (0..config.count)
            .map(|_| Particle {
                position: [rng.random::<f32>() * width, rng.random::<f32>() * height],
                velocity: [
                    (rng.random::<f32>() - 0.5) * base_speed,
                    (rng.random::<f32>() - 0.5) * base_speed,
                ],
                size: rng.random::<f32>() * (config.max_size - config.min_size) + config.min_size,
                _padding: 0.0,
            })
            .collect();

        // Create uniform buffer
        let uniforms = ParticleUniforms {
            time: 0.0,
            width,
            height,
            particle_count: config.count,
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Particle Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create particle storage buffer
        let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Particle Storage Buffer"),
            contents: bytemuck::cast_slice(&particles),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        // Create line instance buffer (max possible connections)
        let max_lines = (config.count * config.count / 2) as usize;
        let line_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Line Instance Buffer"),
            size: (max_lines * std::mem::size_of::<LineInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout (shared for both pipelines)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Particle Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Particle Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: particle_buffer.as_entire_binding(),
                },
            ],
        });

        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Particle Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(PARTICLE_SHADER_SRC)),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Particle Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create particle render pipeline
        let particle_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Particle Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_particle"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Create line render pipeline
        let line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_line"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<LineInstance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Uint32,
                        1 => Uint32,
                        2 => Float32
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            particles,
            config,
            particle_buffer,
            particle_pipeline,
            particle_bind_group: bind_group.clone(),
            line_buffer,
            line_pipeline,
            line_bind_group: bind_group,
            line_count: 0,
            uniform_buffer,
            width,
            height,
        }
    }

    /// Update particle positions (CPU simulation).
    pub fn update(&mut self) {
        for p in &mut self.particles {
            // Update position
            p.position[0] += p.velocity[0];
            p.position[1] += p.velocity[1];

            // Bounce at edges
            if p.position[0] < 0.0 || p.position[0] > self.width {
                p.velocity[0] *= -1.0;
                p.position[0] = p.position[0].clamp(0.0, self.width);
            }
            if p.position[1] < 0.0 || p.position[1] > self.height {
                p.velocity[1] *= -1.0;
                p.position[1] = p.position[1].clamp(0.0, self.height);
            }
        }
    }

    /// Calculate line connections between nearby particles.
    fn calculate_lines(&self) -> Vec<LineInstance> {
        let mut lines = Vec::new();
        let connection_dist = self.config.connection_distance;

        for i in 0..self.particles.len() {
            for j in (i + 1)..self.particles.len() {
                let dx = self.particles[i].position[0] - self.particles[j].position[0];
                let dy = self.particles[i].position[1] - self.particles[j].position[1];
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < connection_dist {
                    let alpha = 1.0 - dist / connection_dist;
                    lines.push(LineInstance {
                        start_idx: i as u32,
                        end_idx: j as u32,
                        alpha,
                        _padding: 0.0,
                    });
                }
            }
        }

        lines
    }

    /// Resize the particle system to new dimensions.
    pub fn resize(&mut self, width: f32, height: f32) {
        // Scale existing particle positions to new size
        let scale_x = width / self.width;
        let scale_y = height / self.height;

        for p in &mut self.particles {
            p.position[0] *= scale_x;
            p.position[1] *= scale_y;
        }

        self.width = width;
        self.height = height;
    }

    /// Render the particle system.
    pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, queue: &wgpu::Queue) {
        // Update particles on CPU
        self.update();

        // Update uniforms
        let uniforms = ParticleUniforms {
            time: 0.0, // Not used in CPU version
            width: self.width,
            height: self.height,
            particle_count: self.config.count,
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        // Update particle buffer
        queue.write_buffer(
            &self.particle_buffer,
            0,
            bytemuck::cast_slice(&self.particles),
        );

        // Calculate and update line buffer
        let lines = self.calculate_lines();
        self.line_count = lines.len() as u32;
        if !lines.is_empty() {
            queue.write_buffer(&self.line_buffer, 0, bytemuck::cast_slice(&lines));
        }

        // Draw lines first (behind particles)
        if self.line_count > 0 {
            render_pass.set_pipeline(&self.line_pipeline);
            render_pass.set_bind_group(0, &self.line_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.line_buffer.slice(..));
            render_pass.draw(0..2, 0..self.line_count);
        }

        // Draw particles
        render_pass.set_pipeline(&self.particle_pipeline);
        render_pass.set_bind_group(0, &self.particle_bind_group, &[]);
        render_pass.draw(0..6, 0..self.config.count);
    }

    /// Get the current configuration.
    pub fn config(&self) -> &ParticleConfig {
        &self.config
    }

    /// Get the current screen dimensions.
    pub fn dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}
