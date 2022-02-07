extern crate minifb;

use minifb::{Key, Window, WindowOptions};

use glam::{ Mat4, Vec4};

pub mod utils;
pub mod renderer;
pub mod vertex;
pub mod texture;

pub use utils::*;
pub use renderer::*;
pub use vertex::*;
pub use texture::*;

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

    let vertices = 
    [
        Vertex::create(glam::vec3(-0.5, 0.5, -0.5), glam::vec2(1., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(-0.5, -0.5, -0.5), glam::vec2(1., 1.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, 0.5, -0.5), glam::vec2(0., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, -0.5, -0.5), glam::vec2(0., 1.), glam::vec3(1., 1., 1.)),

        Vertex::create(glam::vec3(-0.5, 0.5, 0.5), glam::vec2(1., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(-0.5, -0.5, 0.5), glam::vec2(1., 1.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, 0.5, 0.5), glam::vec2(0., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, -0.5, 0.5), glam::vec2(0., 1.), glam::vec3(1., 1., 1.)),

        Vertex::create(glam::vec3(-0.5, 0.5, 0.5), glam::vec2(1., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(-0.5, 0.5, -0.5), glam::vec2(1., 1.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, 0.5, 0.5), glam::vec2(0., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, 0.5, -0.5), glam::vec2(0., 1.), glam::vec3(1., 1., 1.)),

        Vertex::create(glam::vec3(-0.5, -0.5, 0.5), glam::vec2(1., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(-0.5, -0.5, -0.5), glam::vec2(1., 1.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, -0.5, 0.5), glam::vec2(0., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, -0.5, -0.5), glam::vec2(0., 1.), glam::vec3(1., 1., 1.)),
        
        Vertex::create(glam::vec3(0.5, 0.5, -0.5), glam::vec2(1., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, -0.5, -0.5), glam::vec2(1., 1.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, 0.5, 0.5), glam::vec2(0., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(0.5, -0.5, 0.5), glam::vec2(0., 1.), glam::vec3(1., 1., 1.)),

        Vertex::create(glam::vec3(-0.5, 0.5, -0.5), glam::vec2(1., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(-0.5, -0.5, -0.5), glam::vec2(1., 1.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(-0.5, 0.5, 0.5), glam::vec2(0., 0.), glam::vec3(1., 1., 1.)),
        Vertex::create(glam::vec3(-0.5, -0.5, 0.5), glam::vec2(0., 1.), glam::vec3(1., 1., 1.)),
    ];

    let indices = [(0, 1, 2), (3, 2, 1), (5, 4, 6), (6, 7, 5),
                   (8, 9, 10), (11, 10, 9), (13, 12, 14), (14, 15, 13),
                   (16, 17, 18), (19, 18, 17), (21, 20, 22), (22, 23, 21)];

    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut z: f32 = 0.;

    let mut pos = glam::vec3(0., 0., 5.);
    
    let mut rx: f32 = 0.;
    let mut ry: f32 = 0.;
    let  rz: f32 = 0.;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        x += 0.01;
        y += 0.012;
        z += 0.004;

        if window.is_key_down(Key::Up)
        {
            rx += 0.05;
        }
        if window.is_key_down(Key::Down)
        {
            rx -= 0.05;
        }

        if window.is_key_down(Key::Left)
        {
            ry += 0.05;
        }
        if window.is_key_down(Key::Right)
        {
            ry -= 0.05;
        }

        let mut dir: Vec4 = glam::vec4(0., 0., 0., 0.);

        dir.z -= window.is_key_down(Key::W)as i32 as f32;
        dir.z += window.is_key_down(Key::S)as i32 as f32;
    
        dir.x -= window.is_key_down(Key::A)as i32 as f32;
        dir.x += window.is_key_down(Key::D) as i32 as f32;

        dir.y += window.is_key_down(Key::Space)as i32 as f32;
        dir.y -= window.is_key_down(Key::LeftShift)as i32 as f32;

        let cam_rotation = Mat4::from_rotation_y(ry) * Mat4::from_rotation_x(rx) * Mat4::from_rotation_z(rz);
        dir = cam_rotation * dir;
        if dir.length() > 0.
        {
            pos += glam::vec3(dir.x, dir.y, dir.z).normalize() * 0.1;
        }
        let cam_translation = Mat4::from_translation(glam::vec3(pos.x, pos.y, pos.z));
        let camera_matrix = cam_translation * cam_rotation;

        let mut translation = Mat4::from_translation(glam::vec3(0., 0., 0.));
        let rotation = Mat4::from_rotation_x(x) * Mat4::from_rotation_y(y) * Mat4::from_rotation_z(z);
        let scale = Mat4::from_scale(glam::vec3(1., 1., 1.));
        let mut model_matrix = translation * rotation * scale;
        
        let projection_matrix =  Mat4::perspective_rh(std::f32::consts::PI/2., WIDTH as f32/HEIGHT as f32, 0.01, 100.);
        let mut mvp_matrix = projection_matrix* camera_matrix.inverse() * model_matrix;

        renderer.clear();
    
        for x in 0..5
        {
            for y in 0..5
            {
                for z in -5..0
                {
                    translation = Mat4::from_translation(glam::vec3(x as f32 * 2., y as f32 * 2., z as f32 * 2.));
                    model_matrix = translation * rotation * scale;
                    mvp_matrix = projection_matrix* camera_matrix.inverse() * model_matrix;
                    for i in indices
                    {
                        let triangle_indices = i;
                        let mut triangle = [
                            vertices[triangle_indices.0],
                            vertices[triangle_indices.1],
                            vertices[triangle_indices.2]
                        ];
                        triangle[0].pos = mvp_matrix * triangle[0].pos;
                        triangle[1].pos = mvp_matrix * triangle[1].pos;
                        triangle[2].pos = mvp_matrix * triangle[2].pos;
                        
                        renderer.draw_triangle(triangle[0], triangle[1], triangle[2]);
                    }
                }
            }
        }

        window
            .update_with_buffer(&renderer.pixel_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}