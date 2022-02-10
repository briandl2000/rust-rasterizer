use glam::{Mat4, Vec3};

pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub translation: Vec3,
    pub rotation: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near: 0.1,
            far: 1000.,
            fov: f32::to_radians(45.),
            aspect_ratio: 1.0,
            translation: glam::vec3(0., 0., 0.),
            rotation: glam::vec3(0., 0., 0.),
        }
    }
}

impl Camera {
    pub fn projection(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    pub fn orientaion(&self) -> Mat4 {
        Mat4::from_rotation_y(self.rotation.y)
            * Mat4::from_rotation_x(self.rotation.x)
            * Mat4::from_rotation_z(self.rotation.z)
    }

    pub fn view(&self) -> Mat4 {
        Mat4::from_translation(self.translation) * self.orientaion()
    }
}
