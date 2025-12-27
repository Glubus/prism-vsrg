// Prism 3D Cube Shader
// Renders a wireframe cube with rotation animation

struct Uniforms {
    time: f32,
    aspect: f32,
    _padding: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) edge_factor: f32,
};

// Cube size (half-extent)
const CUBE_SIZE: f32 = 0.3;

// Rotation speed
const ROTATION_SPEED: f32 = 0.05;

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) edge_factor: f32) -> VertexOutput {
    var out: VertexOutput;
    
    // Calculate rotation angles based on time
    let angle_x = uniforms.time * ROTATION_SPEED;
    let angle_y = uniforms.time * ROTATION_SPEED * 1.3;
    
    // Rotation matrix around X axis
    let cos_x = cos(angle_x);
    let sin_x = sin(angle_x);
    let rot_x = mat3x3<f32>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, cos_x, -sin_x),
        vec3<f32>(0.0, sin_x, cos_x)
    );
    
    // Rotation matrix around Y axis
    let cos_y = cos(angle_y);
    let sin_y = sin(angle_y);
    let rot_y = mat3x3<f32>(
        vec3<f32>(cos_y, 0.0, sin_y),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(-sin_y, 0.0, cos_y)
    );
    
    // Apply rotations
    var rotated = rot_y * rot_x * position;
    
    // Simple perspective projection
    let z_offset = 2.0; // Camera distance
    let perspective_scale = 1.0 / (rotated.z + z_offset);
    
    // Apply aspect ratio correction and perspective
    out.position = vec4<f32>(
        rotated.x * perspective_scale / uniforms.aspect,
        rotated.y * perspective_scale,
        (rotated.z + z_offset) / 4.0, // Normalize depth
        1.0
    );
    
    out.edge_factor = edge_factor;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Prism primary color with glow based on edge factor
    let base_color = vec3<f32>(1.0, 0.0, 0.235); // #ff003c
    let glow_intensity = 0.3 + in.edge_factor * 0.7;
    
    return vec4<f32>(base_color * glow_intensity, 0.9);
}
