extern crate minifb;

use minifb::{Key, Window, WindowOptions};

use glam::{ Vec4, Quat};


pub mod utils;
pub mod renderer;
pub mod vertex;
pub mod texture;
pub mod transform;
pub mod camera;
pub mod model;
pub mod scene;

pub use utils::*;
pub use renderer::*;
pub use vertex::*;
pub use texture::*;
pub use transform::*;
pub use camera::*;
pub use model::*;
pub use scene::*;

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
        translation: glam::vec3(0., 3., 10.),
        ..Default::default()
    };
    
    let scene = Scene::load("res/mega-scan/scene.glb", &mut renderer);
    // let scene1 = Scene::load("res/Car/scene.gltf", &mut renderer);
    let scene2 = Scene::load("res/DamagedHelmet/scene.gltf", &mut renderer);
    let scene3 = Scene::load("res/sponza/Sponza.gltf", &mut renderer);

    let transform1 = Transform::from_translation(glam::vec3(-2., 0., 0.));
    // let transform2 = Transform::from_translation(glam::vec3(0., 0., 0.));
    let mut transform3 = Transform::from_translation(glam::vec3(2., 2., 3.));
    let transform4 = Transform::new(glam::vec3(0., 0., 0.), Quat::from_rotation_y((90. as f32).to_radians()), glam::vec3(4., 4., 4.));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::Up)
        {
            camera.rotation.x += 0.05;
        }
        if window.is_key_down(Key::Down)
        {
            camera.rotation.x -= 0.05;
        }

        if window.is_key_down(Key::Left)
        {
            camera.rotation.y += 0.05;
        }
        if window.is_key_down(Key::Right)
        {
            camera.rotation.y -= 0.05;
        }

        let mut dir: Vec4 = glam::vec4(0., 0., 0., 0.);

        dir.z -= window.is_key_down(Key::W)as i32 as f32;
        dir.z += window.is_key_down(Key::S)as i32 as f32;
    
        dir.x -= window.is_key_down(Key::A)as i32 as f32;
        dir.x += window.is_key_down(Key::D) as i32 as f32;

        dir.y += window.is_key_down(Key::Space)as i32 as f32;
        dir.y -= window.is_key_down(Key::LeftShift)as i32 as f32;

        dir = camera.orientaion() * dir;
        if dir.length() > 0.
        {
            camera.translation += glam::vec3(dir.x, dir.y, dir.z).normalize() * 0.07;
        }

        let vp = camera.projection()* camera.view().inverse();

        renderer.clear();

        transform3.rotation *= Quat::from_rotation_y(0.05);

        scene.render(&mut renderer, vp, transform1.local());
        // scene1.render(&mut renderer, vp, transform2.local());
        scene2.render(&mut renderer, vp, transform3.local());
        scene3.render(&mut renderer, vp, transform4.local());
        window
            .update_with_buffer(&renderer.pixel_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}