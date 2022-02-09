


use crate::vertex::*;
use crate::renderer::*;

use glam::{Mat4, Vec3, Vec2};

pub struct Mesh
{
    vertices: Vec<Vertex>,
    indices: Vec<(usize,usize,usize)>,
    texture: i32,
    normal_texture: i32
}

impl Mesh
{
    pub fn render(&self, renderer: &mut Renderer, mvp_matrix: Mat4, model_matrix: Mat4) {
        renderer.bound_texture = self.texture;
        renderer.bound_normal_texture = self.normal_texture;

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
            renderer.draw_triangle(triangle[0], triangle[1], triangle[2], model_matrix);
        }
    }
}

pub struct Model
{
    pub meshes: Vec<Mesh>
}


impl Model {
    pub fn load_gltf(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data], renderer: &mut Renderer) -> Self
    {
        let mut meshes: Vec<Mesh> = Vec::new();

        for primitive in mesh.primitives() {
            let mut vertices: Vec<Vertex>= vec![];
            let mut indices: Vec<u32> = vec![];
            
            let mut positions: Vec<Vec3> = Vec::new();
            let mut tex_coords: Vec<Vec2> = Vec::new();
            let mut normals: Vec<Vec3> = Vec::new();
            let mut colors: Vec<Vec3> = Vec::new();

            let mut texutre_index: i32 = -1;
            let mut normal_texture_index: i32 = -1;
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            if let Some(texture) = primitive.material().pbr_metallic_roughness().base_color_texture() {
                texutre_index = texture.texture().source().index() as i32 + renderer.texture_offset;
            }

            if let Some(normal_texture) = primitive.material().normal_texture() {
                normal_texture_index = normal_texture.texture().source().index() as i32 + renderer.texture_offset;
            }

            if let Some(indices_reader) = reader.read_indices() {
                indices_reader.into_u32().for_each(|i| {indices.push(i)});
            }
            
            if let Some(positions_reader) = reader.read_positions() {
                positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
            }
            
            if let Some(normals_reader) = reader.read_normals() {
                normals_reader.for_each(|n| normals.push(Vec3::new(n[0], n[1], n[2])));
            }
            
            if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                tex_coord_reader
                    .into_f32()
                    .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])));
            }

            if let Some(colors_reader) = reader.read_colors(0) {
                colors_reader.into_rgb_f32().for_each(|n| colors.push(Vec3::new(n[0], n[1], n[2])));
            }

            let has_uvs = !tex_coords.is_empty();
            
            let has_colors = !colors.is_empty();
            let has_normals = !normals.is_empty();
            for i in 0..positions.len() {
                let vertex = Vertex{
                    pos: positions[i].extend(1.0),
                    color: if has_colors { colors[i] } else { Vec3::ONE },
                    uv: if has_uvs { tex_coords[i] } else { Vec2::ZERO },
                    normal: if has_normals { normals[i] } else { Vec3::ONE },
                    tangent: {
                        let mut tangent = normals[i].cross(glam::vec3(0., 1., 0.));
                        if tangent.length() == 0.
                        {
                            tangent = normals[i].cross(glam::vec3(0., 0., 1.));
                        }
                        tangent.normalize()
                    }
                };
                vertices.push(vertex)
            }

            let triangles: Vec<(usize, usize, usize)> = indices
                    .chunks_exact(3)
                    .map(|tri| (tri[0] as usize, tri[1] as usize, tri[2] as usize))
                    .collect();

            meshes.push(Mesh {
                vertices,
                indices: triangles,
                texture: texutre_index,
                normal_texture: normal_texture_index
            });
        }
        Model
        {
            meshes
        }
        
    }


    pub fn render(&self, renderer: &mut Renderer, mvp_matrix: Mat4, model_matrix: Mat4)
    {
        for mesh in self.meshes.iter()
        {
            mesh.render(renderer, mvp_matrix, model_matrix);
        }
    }
}