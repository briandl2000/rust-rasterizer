use crate::utils::*;

use crate::vertex::*;
use crate::texture::*;

use glam::{Vec3, Vec4, Mat4};
use std::path::Path;
pub struct Renderer {
    pub pixel_buffer: Vec<u32>,
    pub depth_buffer: Vec<f32>,
    scan_line_buffer: Vec<(i32, Vec3)>,
    width: usize,
    height: usize,
    screen_transform: Mat4,
    pub texture: Texture
}


impl Renderer {
    pub fn create(width: usize, height: usize) -> Self {
        let half_width = (width as f32)/2.;
        let half_height = (height as f32)/2.;

        let transformation_matrix = glam::mat4(glam::vec4(half_width, 0., 0., 0.),
                                                glam::vec4(0., -half_height, 0., 0.),
                                                glam::vec4(0., 0., 1., 0.),
                                                glam::vec4(half_width, half_height, 0., 1.));
        Self {
            pixel_buffer: vec![to_argb8(255, 0, 255, 0); width * height],
            depth_buffer: vec![std::f32::INFINITY; width * height],
            scan_line_buffer: vec![(0, glam::vec3(0., 0., 0.)); height * 2],
            width: width,
            height: height,
            screen_transform: transformation_matrix,
            texture: Texture::load(Path::new("bojan.jpg"))
        }
    }

    fn scan_convert_line(&mut self, min_y_vert: Vec4, max_y_vert: Vec4, which_side: usize, vertex_0_index: usize, vertex_1_index: usize) {
        let bary_centric_locations: [Vec3; 3] = [glam::vec3(1., 0., 0.), glam::vec3(0., 1., 0.), glam::vec3(0., 0., 1.)];
        let y_start = (min_y_vert.y).ceil() as i32;
        let y_end =  (max_y_vert.y).ceil() as i32;

        let y_dist = max_y_vert.y - min_y_vert.y;
        let x_dist = max_y_vert.x - min_y_vert.x;

        if y_dist <= 0. {
            return;
        }

        let x_step = (x_dist as f32)/(y_dist as f32);
        let y_prestep = y_start as f32 - min_y_vert.y;
        let mut current_x = min_y_vert.x + y_prestep * x_step;
        for j in y_start..y_end {
            self.scan_line_buffer[j as usize * 2 + which_side] = 
                (current_x.ceil() as i32,
                bary_centric_locations[vertex_0_index].lerp(bary_centric_locations[vertex_1_index], ((j as f32)-min_y_vert.y)/y_dist));
            current_x = current_x + x_step;
        }
        
    }

    fn fill_shape(&mut self, y_start: u32, y_end: u32, vertices: [Vertex; 3]) {
        for y in y_start..y_end {
            if y >= self.height as u32
            {
                break;
            }

            let start_x = self.scan_line_buffer[(y*2+0) as usize];
            
            let end_x = self.scan_line_buffer[(y*2+1) as usize];
            
            let step_x = 1./((end_x.0 - start_x.0) as f32);
            let mut lerp: f32 = 0.;
            for x in start_x.0..end_x.0 {
                
                let one_over_w = 1./glam::vec3(vertices[0].pos.w, vertices[1].pos.w, vertices[2].pos.w);
                
                let mut barycentric_coord = start_x.1.lerp(end_x.1, lerp);
                barycentric_coord =  (barycentric_coord*one_over_w) / (barycentric_coord.dot(one_over_w));
                
                lerp = lerp + step_x;

                let index: usize = (x as usize)+(y as usize * self.width);
                
                let w = vertices[0].pos.w * barycentric_coord.x + vertices[1].pos.w * barycentric_coord.y + vertices[2].pos.w * barycentric_coord.z;
            
                let depth = (vertices[0].pos.z * barycentric_coord.x + vertices[1].pos.z * barycentric_coord.y + vertices[2].pos.z * barycentric_coord.z)/w;

                if self.depth_buffer[index] < depth {
                    continue;
                }

                let uv = vertices[0].uv * barycentric_coord.x +
                              vertices[1].uv * barycentric_coord.y + 
                              vertices[2].uv * barycentric_coord.z;
                //let color = vertices[0].color * barycentric_coord.x + vertices[1].color * barycentric_coord.y + vertices[2].color * barycentric_coord.z;

                self.pixel_buffer[index] = to_argb8(255, (uv.x*255.) as u8, (uv.y*255.) as u8, (0.) as u8);
                //self.pixel_buffer[index] = self.texture.argb_at_uv(uv.x, uv.y);

                self.depth_buffer[index] = depth;
            }
        }
    }

    fn scan_convert_triangle(&mut self, min_y_vert: Vec4, mid_y_vert: Vec4, max_y_vert: Vec4, handedness: usize, indices: [usize; 3]) {

        self.scan_convert_line(min_y_vert, max_y_vert, 0 + handedness, indices[0], indices[2]);
        self.scan_convert_line(min_y_vert, mid_y_vert, 1 - handedness, indices[0], indices[1]);
        self.scan_convert_line(mid_y_vert, max_y_vert, 1 - handedness, indices[1], indices[2]);
    }

    pub fn rasterize_triangle(&mut self, v0: Vertex, v1: Vertex, v2: Vertex) {
        

        let mut transformed_min =  (self.screen_transform * (v0.pos/v0.pos.w), 0 as usize);
        let mut transformed_mid =  (self.screen_transform * (v1.pos/v1.pos.w), 1 as usize);
        let mut transformed_max =  (self.screen_transform * (v2.pos/v2.pos.w), 2 as usize);

        if (transformed_mid.0.x - transformed_min.0.x) * (transformed_max.0.y-transformed_min.0.y) - (transformed_mid.0.y - transformed_min.0.y) * (transformed_max.0.x-transformed_min.0.x) > 0.
        {
            return;
        }
        if transformed_max.0.y < transformed_mid.0.y {
            let temp_transformed = transformed_max;
            transformed_max = transformed_mid;
            transformed_mid = temp_transformed;
        }

        if transformed_mid.0.y < transformed_min.0.y {
            let temp_transformed = transformed_mid;
            transformed_mid = transformed_min;
            transformed_min = temp_transformed;
        }

        if transformed_max.0.y < transformed_mid.0.y {
            let temp_transformed = transformed_max;
            transformed_max = transformed_mid;
            transformed_mid = temp_transformed;
        }

        let vector0 = transformed_max.0 - transformed_min.0;
        let vector1 = transformed_mid.0 - transformed_min.0;

        let area = (vector0.x * vector1.y) - (vector0.y * vector1.x);
        
        let handedness: usize = (area > 0.) as usize;

        self.scan_convert_triangle(transformed_min.0, transformed_mid.0, transformed_max.0, handedness, [transformed_min.1, transformed_mid.1, transformed_max.1]);
        self.fill_shape(transformed_min.0.y.ceil() as u32, transformed_max.0.y.ceil() as u32, [v0, v1, v2]);
    }

    pub fn clear(&mut self) {
        for i in self.pixel_buffer.iter_mut() {
            *i = 0;
        }
        for i in self.depth_buffer.iter_mut() {
            *i = std::f32::INFINITY;
        }
    }
}