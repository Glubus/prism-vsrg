use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct QuadInstance {
    pub center: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

pub fn quad_from_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: [f32; 4],
    screen_width: f32,
    screen_height: f32,
) -> QuadInstance {
    let center = [
        ((x + width / 2.0) / screen_width) * 2.0 - 1.0,
        -(((y + height / 2.0) / screen_height) * 2.0 - 1.0),
    ];
    let size = [(width / screen_width) * 2.0, (height / screen_height) * 2.0];

    QuadInstance {
        center,
        size,
        color,
    }
}
