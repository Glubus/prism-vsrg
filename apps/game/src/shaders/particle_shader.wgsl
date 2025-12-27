// Prism Particle Background Shader
// Animated particles with connection lines

struct Uniforms {
    time: f32,
    width: f32,
    height: f32,
    particle_count: u32,
};

struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    size: f32,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<storage, read> particles: array<Particle>;

// ============================================================================
// Vertex/Fragment Shaders - Render particles and lines
// ============================================================================

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) alpha: f32,
    @location(1) is_particle: f32,
};

// Particle rendering (point sprites)
@vertex
fn vs_particle(@builtin(vertex_index) vertex_idx: u32, @builtin(instance_index) instance_idx: u32) -> VertexOutput {
    var out: VertexOutput;
    
    let p = particles[instance_idx];
    
    // Quad vertices for particle sprite
    let offsets = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );
    
    let offset = offsets[vertex_idx] * p.size;
    let screen_pos = (p.position + offset) / vec2<f32>(uniforms.width, uniforms.height) * 2.0 - 1.0;
    
    out.position = vec4<f32>(screen_pos.x, -screen_pos.y, 0.0, 1.0);
    out.alpha = 1.0;
    out.is_particle = 1.0;
    
    return out;
}

// Line rendering between particles
@vertex
fn vs_line(@builtin(vertex_index) vertex_idx: u32, @location(0) start_idx: u32, @location(1) end_idx: u32, @location(2) alpha: f32) -> VertexOutput {
    var out: VertexOutput;
    
    // Get both particle positions
    let start_pos = particles[start_idx].position;
    let end_pos = particles[end_idx].position;
    
    // Select based on vertex index (0 = start, 1 = end)
    let is_start = vertex_idx % 2u == 0u;
    let pos = select(end_pos, start_pos, is_start);
    
    let screen_pos = pos / vec2<f32>(uniforms.width, uniforms.height) * 2.0 - 1.0;
    
    out.position = vec4<f32>(screen_pos.x, -screen_pos.y, 0.0, 1.0);
    out.alpha = alpha;
    out.is_particle = 0.0;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Prism primary color
    let color = vec3<f32>(1.0, 0.0, 0.235); // #ff003c
    
    if (in.is_particle > 0.5) {
        // Particle - solid with slight glow
        return vec4<f32>(color, in.alpha);
    } else {
        // Line - faded based on distance
        return vec4<f32>(color, in.alpha * 0.5);
    }
}
