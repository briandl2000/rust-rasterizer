
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

    pub fn lerp(v1: Self, v2: Self, t: f32) -> Self
    {
        Self {
            pos: v1.pos.lerp(v2.pos, t),
            uv: v1.uv.lerp(v2.uv, t),
            color: v1.color.lerp(v2.color, t)
        }
    }
    pub fn is_inside_view_frustum(self) -> bool
    {
        f32::abs(self.pos.x) <= f32::abs(self.pos.w) &&
        f32::abs(self.pos.y) <= f32::abs(self.pos.w) &&
        f32::abs(self.pos.z) <= f32::abs(self.pos.w)
    }
}