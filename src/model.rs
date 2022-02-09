

use crate::vertex::*;
use crate::renderer::*;

use glam::{Mat4, Vec3, Vec2};

pub struct Model
{
    vertices: Vec<Vertex>,
    indices: Vec<(usize,usize,usize)>
}

impl Model {
    pub fn load_gltf(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> Self
    {
        let mut vertices: Vec<Vertex>= vec![];
        let mut indices: Vec<u32> = vec![];
        
        let mut positions: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec2> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut colors: Vec<Vec3> = positions.iter().map(|_| Vec3::ONE).collect();

        let mut indices_offset: u32 = 0;
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            if let Some(indices_reader) = reader.read_indices() {
                indices_reader.into_u32().for_each(|i| {indices.push(i+indices_offset)});
            }
            if let Some(indices_reader) = reader.read_indices() {
                indices_offset += indices_reader.into_u32().len() as u32;
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

            colors = positions.iter().map(|_| Vec3::ONE).collect();
            println!("Num indices: {:?}", indices.len());
            println!("tex_coords: {:?}", tex_coords.len());
            println!("positions: {:?}", positions.len());
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
            };
            vertices.push(vertex)
        }

        let triangles: Vec<(usize, usize, usize)> = indices
                .chunks_exact(3)
                .map(|tri| (tri[0] as usize, tri[1] as usize, tri[2] as usize))
                .collect();


        Model {
            vertices,
            indices: triangles
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