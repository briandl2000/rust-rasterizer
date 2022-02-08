

use crate::vertex::*;
use crate::renderer::*;

use glam::Mat4;

pub struct Model
{
    vertices: Vec<Vertex>,
    indices: Vec<(usize,usize,usize)>
}

impl Model {
    pub fn create() -> Self
    {
        Model {
            vertices: vec![
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
            ],
            indices: vec![(0, 1, 2), (3, 2, 1), (5, 4, 6), (6, 7, 5),
                   (8, 9, 10), (11, 10, 9), (13, 12, 14), (14, 15, 13),
                   (16, 17, 18), (19, 18, 17), (21, 20, 22), (22, 23, 21)]
        }
    }

    pub fn render(&self, renderer: &mut Renderer, mvp_matrix: Mat4) {
        for i in self.indices.iter()
        {
            let triangle_indices = i;
            let mut triangle = [
                self.vertices[triangle_indices.0],
                self.vertices[triangle_indices.1],
                self.vertices[triangle_indices.2]
            ];
            triangle[0].pos = mvp_matrix * triangle[0].pos;
            triangle[1].pos = mvp_matrix * triangle[1].pos;
            triangle[2].pos = mvp_matrix * triangle[2].pos;
            
            renderer.draw_triangle(triangle[0], triangle[1], triangle[2]);
        }
    }
}