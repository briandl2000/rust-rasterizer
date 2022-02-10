use crate::utils::*;
use stb_image;
use std::path::Path;
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub depth: usize,
}

impl Texture {
    pub fn load(path: &Path) -> Self {
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image {
            // we are not taking into accoung pngs yet :)
            let data = (0..image.data.len() / 3)
                .map(|id| {
                    to_argb8(
                        255,
                        image.data[id * 3],
                        image.data[id * 3 + 1],
                        image.data[id * 3 + 2],
                    )
                })
                .collect();
            Self {
                width: image.width,
                height: image.height,
                data,
                depth: image.depth,
            }
        } else {
            panic!("Unsupported texture type");
        }
    }

    pub fn argb_at_uv(&self, u: f32, v: f32) -> u32 {
        let (u, v) = (
            u.abs().fract() * (self.width - 1) as f32,
            (v.abs().fract()) * (self.height - 1) as f32,
        );
        let id = coords_to_index(u as usize, v as usize, self.width);
        if id < self.data.len() {
            self.data[id]
        } else {
            to_argb8(255, 255, 0, 255)
        }
    }

    pub fn create(data: Vec<u8>, width: u32, height: u32, num_chanels: i32) -> Self {
        let data = match num_chanels {
            1 => (0..data.len())
                .map(|id| to_argb8(255, data[id], 0, 0))
                .collect(),
            2 => (0..data.len() / 2)
                .map(|id| to_argb8(255, data[id * 2], data[id * 2 + 1], 0))
                .collect(),
            3 => (0..data.len() / 3)
                .map(|id| to_argb8(255, data[id * 3], data[id * 3 + 1], data[id * 3 + 2]))
                .collect(),
            4 => (0..data.len() / 4)
                .map(|id| {
                    to_argb8(
                        data[id * 4 + 3],
                        data[id * 4],
                        data[id * 4 + 1],
                        data[id * 4 + 2],
                    )
                })
                .collect(),
            _ => panic!("Non suported number of chanels"),
        };

        Self {
            width: width as usize,
            height: height as usize,
            data,
            depth: num_chanels as usize,
        }
    }
}
