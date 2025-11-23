// Shader simple pour dessiner des quads colorés (panels, cards, etc.)
// Utilise des coordonnées normalisées directement

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    @location(0) @interpolate(flat) center: vec2<f32>,
    @location(1) @interpolate(flat) size: vec2<f32>,
    @location(2) @interpolate(flat) color: vec4<f32>,
) -> VertexOutput {
    // Créer un quad à partir de l'index du vertex (triangle strip)
    // 0: (0, 0), 1: (1, 0), 2: (0, 1), 3: (1, 1)
    let u = f32(in_vertex_index % 2u);
    let v = f32(in_vertex_index / 2u);
    
    var output: VertexOutput;
    // center et size sont déjà en coordonnées normalisées [-1, 1]
    // On calcule les coins du quad
    let corner_x = center.x + (u - 0.5) * size.x;
    let corner_y = center.y + (v - 0.5) * size.y;
    
    output.clip_position = vec4<f32>(corner_x, corner_y, 0.0, 1.0);
    output.color = color;
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
