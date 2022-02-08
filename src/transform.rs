use glam::{Mat4, Vec3};

use glam::Vec4Swizzles;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Vec3::ZERO,
        scale: Vec3::ONE,
    };

    pub fn new(translation: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn orientaion(&self) -> Mat4
    {
        Mat4::from_rotation_y(self.rotation.y) * Mat4::from_rotation_x(self.rotation.x) * Mat4::from_rotation_z(self.rotation.z)
    }

    pub fn local(&self) -> Mat4 {
        Mat4::from_translation(self.translation)
            * self.orientaion()
            * Mat4::from_scale(self.scale)
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    pub fn from_rotation(rotation: Vec3) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation,
            scale: Vec3::ONE,
        }
    }

    pub fn from_translation_rotation(translation: Vec3, rotation: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale: Vec3::ONE,
        }
    }

    pub fn right(&self) -> Vec3 {
        (self.orientaion() * glam::vec4(0., 1., 0., 0.)).xyz()
    }

    pub fn up(&self) -> Vec3 {
        (self.orientaion() * glam::vec4(0., 1., 0., 0.)).xyz()
    }

    pub fn forward(&self) -> Vec3 {
        (self.orientaion() * glam::vec4(0., 1., 0., 0.)).xyz()
    }
}

impl From<Transform> for Mat4 {
    fn from(transform: Transform) -> Mat4 {
        transform.local()
    }
}

pub enum TransformInitialParams {
    Identity,
    Translation(Vec3),
    Rotation(Vec3),
    TranslationRotation(Vec3, Vec3)
}

impl From<TransformInitialParams> for Transform {
    fn from(params: TransformInitialParams) -> Self {
        match params {
            TransformInitialParams::Identity => Self::IDENTITY,
            TransformInitialParams::Translation(translation) => Self::from_translation(translation),
            TransformInitialParams::Rotation(rotation) => Self::from_rotation(rotation),
            TransformInitialParams::TranslationRotation(translation, rotation) => {
                Self::from_translation_rotation(translation, rotation)
            }
        }
    }
}