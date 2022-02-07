
use glam::{Vec2, Vec3, Vec4};

#[derive(Debug, Copy, Clone)]
pub struct Vertex
{
    pub pos: Vec4,
    pub uv: Vec2,
    pub color: Vec3
}

impl Vertex {
    pub fn create(pos: Vec3, uv: Vec2, color: Vec3) -> Self {
        Self {
            pos: glam::vec4(pos.x, pos.y, pos.z, 1.),
            uv,
            color
        }
    }
}