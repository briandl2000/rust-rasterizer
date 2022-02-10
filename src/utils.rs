use glam::{Vec3, Vec4};

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

pub fn from_vec3_to_argb8(color: Vec3) -> u32 {
    to_argb8(
        255,
        (color.x * 255.) as u8,
        (color.y * 255.) as u8,
        (color.z * 255.) as u8,
    )
}

pub fn to_vec3_color(col: u32) -> Vec3 {
    let r = (((col & 0xFF0000) >> 16) as u8) as f32 / 255.;
    let g = (((col & 0x00FF00) >> 8) as u8) as f32 / 255.;
    let b = ((col & 0x0000FF) as u8) as f32 / 255.;
    glam::vec3(r, g, b)
}

pub fn to_vec4_color(col: u32) -> Vec4 {
    let w = (((col & 0xFF000000) >> 24) as u8) as f32 / 255.;
    let r = (((col & 0x00FF0000) >> 16) as u8) as f32 / 255.;
    let g = (((col & 0x0000FF00) >> 8) as u8) as f32 / 255.;
    let b = ((col & 0x000000FF) as u8) as f32 / 255.;
    glam::vec4(r, g, b, w)
}

pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    x + y * (width)
}
