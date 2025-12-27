//! Rotating 3D wireframe cube component.
//!
//! A reusable wgpu component for rendering an animated 3D cube.
//! Can be configured with custom size and colors.

use std::borrow::Cow;
use wgpu::util::DeviceExt;

use crate::shaders::constants::CUBE_SHADER_SRC;

/// Configuration for the cube renderer
#[derive(Clone, Debug)]
pub struct CubeConfig {
    /// Half-size of the cube (default: 0.25)
    pub size: f32,
    /// Rotation speed multiplier (default: 1.0)
    pub rotation_speed: f32,
}

impl Default for CubeConfig {
    fn default() -> Self {
        Self {
            size: 0.25,
            rotation_speed: 1.0,
        }
    }
}

impl CubeConfig {
    /// Create a large cube (for main menu)
    pub fn large() -> Self {
        Self {
            size: 0.65,
            rotation_speed: 1.8,
        }
    }

    /// Create a small cube (for icons/decorations)
    pub fn small() -> Self {
        Self {
            size: 0.15,
            rotation_speed: 1.5,
        }
    }

    /// Create a cube with custom size
    pub fn with_size(size: f32) -> Self {
        Self {
            size,
            rotation_speed: 1.0,
        }
    }
}

/// Uniform data for the cube shader
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CubeUniforms {
    time: f32,
    aspect: f32,
    _padding: [f32; 2],
}

/// Cube vertex with position and edge factor
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CubeVertex {
    position: [f32; 3],
    edge_factor: f32,
}

impl CubeVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CubeVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Rotating 3D wireframe cube renderer.
///
/// # Example
/// ```ignore
/// let cube = CubeRenderer::new(&device, format, CubeConfig::large());
/// // In render loop:
/// cube.render(&mut render_pass, &queue, aspect_ratio);
/// ```
pub struct CubeRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    vertex_count: u32,
    start_time: std::time::Instant,
    config: CubeConfig,
}

impl CubeRenderer {
    /// Create a new cube renderer with the given configuration.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, config: CubeConfig) -> Self {
        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cube Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(CUBE_SHADER_SRC)),
        });

        // Create uniform buffer
        let uniforms = CubeUniforms {
            time: 0.0,
            aspect: 16.0 / 9.0,
            _padding: [0.0; 2],
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Cube Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cube Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cube Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[CubeVertex::desc()],
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

        // Create cube vertices (wireframe edges)
        let vertices = Self::create_cube_vertices(config.size);
        let vertex_count = vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            uniform_buffer,
            bind_group,
            vertex_count,
            start_time: std::time::Instant::now(),
            config,
        }
    }

    /// Create cube wireframe vertices (12 edges = 24 vertices for LineList)
    fn create_cube_vertices(size: f32) -> Vec<CubeVertex> {
        let s = size;
        // 8 corners of the cube
        let corners = [
            [-s, -s, -s], // 0: back-bottom-left
            [s, -s, -s],  // 1: back-bottom-right
            [s, s, -s],   // 2: back-top-right
            [-s, s, -s],  // 3: back-top-left
            [-s, -s, s],  // 4: front-bottom-left
            [s, -s, s],   // 5: front-bottom-right
            [s, s, s],    // 6: front-top-right
            [-s, s, s],   // 7: front-top-left
        ];

        // 12 edges as pairs of corner indices
        let edges: [(usize, usize); 12] = [
            // Back face
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            // Front face
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            // Connecting edges
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];

        let mut vertices = Vec::with_capacity(24);
        for (i, (a, b)) in edges.iter().enumerate() {
            let edge_factor = i as f32 / 11.0;
            vertices.push(CubeVertex {
                position: corners[*a],
                edge_factor,
            });
            vertices.push(CubeVertex {
                position: corners[*b],
                edge_factor,
            });
        }

        vertices
    }

    /// Update uniforms and render the cube.
    ///
    /// # Arguments
    /// * `render_pass` - Active render pass to draw into
    /// * `queue` - GPU queue for buffer updates
    /// * `aspect_ratio` - Screen aspect ratio (width / height)
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        aspect_ratio: f32,
    ) {
        // Update time uniform
        let elapsed = self.start_time.elapsed().as_secs_f32() * self.config.rotation_speed;
        let uniforms = CubeUniforms {
            time: elapsed,
            aspect: aspect_ratio,
            _padding: [0.0; 2],
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        // Render
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count, 0..1);
    }

    /// Get the current configuration
    pub fn config(&self) -> &CubeConfig {
        &self.config
    }
}
