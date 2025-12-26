use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode, vertex_attr_array};

use crate::math::vec2::Vec2;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2D {
    pub pos: [f32; 2],
}

impl Vertex2D {
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex2D>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &vertex_attr_array![
                0 => Float32x2
            ],
        }
    }
}

impl From<Vec2> for Vertex2D {
    fn from(v: Vec2) -> Self {
        Self { pos: [v.x, v.y] }
    }
}
