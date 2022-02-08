extern crate minifb;

use minifb::{Key, Window, WindowOptions};

use glam::{ Vec4};

pub mod utils;
pub mod renderer;
pub mod vertex;
pub mod texture;
pub mod transform;
pub mod camera;
pub mod model;

pub use utils::*;
pub use renderer::*;
pub use vertex::*;
pub use texture::*;
pub use transform::*;
pub use camera::*;
pub use model::*;

const WIDTH: usize = 900;
const HEIGHT: usize = 600;

fn main() {
    let mut renderer = Renderer::create(WIDTH, HEIGHT);
    
    let mut window = Window::new(
        "Rusterizer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut camera: Camera = Camera {
        aspect_ratio: WIDTH as f32 / HEIGHT as f32,
        transform: Transform::from_translation(glam::vec3(0., 0., 5.)),
        ..Default::default()
    };
    
    let model: Model = Model::create();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Up)
        {
            camera.transform.rotation.x += 0.05;
        }
        if window.is_key_down(Key::Down)
        {
            camera.transform.rotation.x -= 0.05;
        }

        if window.is_key_down(Key::Left)
        {
            camera.transform.rotation.y += 0.05;
        }
        if window.is_key_down(Key::Right)
        {
            camera.transform.rotation.y -= 0.05;
        }

        let mut dir: Vec4 = glam::vec4(0., 0., 0., 0.);

        dir.z -= window.is_key_down(Key::W)as i32 as f32;
        dir.z += window.is_key_down(Key::S)as i32 as f32;
    
        dir.x -= window.is_key_down(Key::A)as i32 as f32;
        dir.x += window.is_key_down(Key::D) as i32 as f32;

        dir.y += window.is_key_down(Key::Space)as i32 as f32;
        dir.y -= window.is_key_down(Key::LeftShift)as i32 as f32;

        dir = camera.transform.orientaion() * dir;
        if dir.length() > 0.
        {
            camera.transform.translation += glam::vec3(dir.x, dir.y, dir.z).normalize() * 0.07;
        }

        let mut _mvp_matrix = camera.projection()* camera.view().inverse();

        renderer.clear();
    
        
        model.render(&mut renderer, _mvp_matrix);
        
        window
            .update_with_buffer(&renderer.pixel_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}