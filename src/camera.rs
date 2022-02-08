
use crate::transform::*;

use glam::Mat4;

pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub transform: Transform,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near: 0.1,
            far: 100.,
            fov: f32::to_radians(90.),
            aspect_ratio: 1.0,
            transform: Transform::IDENTITY
        }
    }
}

impl Camera {
    pub fn projection(&self) -> Mat4 {
        
        Mat4::perspective_rh(
            self.fov,
            self.aspect_ratio,
            self.near,
            self.far
        )
    }

    pub fn view(&self) -> Mat4 {
        self.transform.local()
    }
}