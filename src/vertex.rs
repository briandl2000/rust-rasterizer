use glam::{Vec2, Vec3, Vec4};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: Vec4,
    pub uv: Vec2,
    pub color: Vec3,
    pub normal: Vec3,
    pub tangent: Vec3,
}

impl Default for Vertex {
    fn default() -> Self {
        Self::new()
    }
}

impl Vertex {

    pub fn new() -> Self {
        Self {
            pos: Vec4::ZERO,
            uv: Vec2::ZERO,
            color: Vec3::ONE,
            normal: Vec3::ZERO,
            tangent: Vec3::ZERO,
        }
    }

    pub fn create(pos: Vec3, uv: Vec2, color: Vec3, normal: Vec3, tangent: Vec3) -> Self {
        Self {
            pos: glam::vec4(pos.x, pos.y, pos.z, 1.),
            uv,
            color,
            normal,
            tangent,
        }
    }

    pub fn lerp(v1: Self, v2: Self, t: f32) -> Self {
        Self {
            pos: v1.pos.lerp(v2.pos, t),
            uv: v1.uv.lerp(v2.uv, t),
            color: v1.color.lerp(v2.color, t),
            normal: v1.normal.lerp(v2.normal, t),
            tangent: v1.tangent.lerp(v2.tangent, t),
        }
    }
    pub fn is_inside_view_frustum(self) -> (bool, bool, bool, bool, bool, bool, bool) {
        let x1 = self.pos.x <= self.pos.w;
        let x2 = self.pos.x >= -self.pos.w;
        let y1 = self.pos.y <= self.pos.w;
        let y2 = self.pos.y >= -self.pos.w;
        let z1 = self.pos.z <= self.pos.w;
        let z2 = self.pos.z >= -self.pos.w;
        (
            (x1 && x2) && (y1 && y2) && (z1 && z2),
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        )
    }
}
