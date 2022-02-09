
use crate::model::*;

use glam::Mat4;
use crate::renderer::*;
use crate::texture::*;

struct SceneNode{
    pub transform: Mat4,
    pub nodes: Vec<SceneNode>,
    pub mesh: (usize, bool)
}

pub struct Scene
{
    pub meshes: Vec<Model>,
    nodes: Vec<SceneNode>
}

impl SceneNode{
    fn add_node(node: gltf::Node) -> SceneNode
    {
        let transform: Mat4 = Mat4::from_cols_array_2d(&node.transform().matrix());
        let mut nodes:  Vec<SceneNode> = vec![];
        
        for child in node.children()
        {
            nodes.push(SceneNode::add_node(child));
        }

        let mesh_index = node.mesh();
        
        let mesh = match mesh_index {
            Some(index) => {
                (index.index(), true)
            },
            _ => {(0, false)},
        };

        SceneNode {
            transform,
            nodes,
            mesh
        }
    }
}

impl Scene {
    pub fn load(path: &'static str, renderer: &mut Renderer) -> Self
    {
        let (document, buffers, _images) = gltf::import(path).unwrap();
        let mut meshes: Vec<Model> = Vec::new();

        for mesh in document.meshes()
        {
            meshes.push(Model::load_gltf(&mesh, &buffers, renderer));
        }
        for image in _images
        {
            let chanels = match image.format {
                gltf::image::Format::R8 => {1}
                gltf::image::Format::R8G8 => {2}
                gltf::image::Format::R8G8B8 => {3}
                gltf::image::Format::R8G8B8A8 => {4}
                gltf::image::Format::B8G8R8 => {panic!("format not suported")}
                gltf::image::Format::B8G8R8A8 => {panic!("format not suported")}
                gltf::image::Format::R16 => {panic!("format not suported")}
                gltf::image::Format::R16G16 => {panic!("format not suported")}
                gltf::image::Format::R16G16B16 => {panic!("format not suported")}
                gltf::image::Format::R16G16B16A16 => {panic!("format not suported")}
            };

            renderer.add_texture(Texture::create(image.pixels, image.width, image.height, chanels));
        }
        renderer.texture_offset = renderer.textures.len() as i32;

        let mut nodes: Vec<SceneNode> = Vec::new();

        for scene in document.scenes()
        {
            for node in scene.nodes()
            {
                nodes.push(SceneNode::add_node(node));
            }
        }

        Self 
        {
            meshes,
            nodes
        }

    }

    fn render_node(&self, renderer: &mut Renderer, node: &SceneNode, parent_transform: Mat4, vp: Mat4)
    {
        let node_transform = parent_transform* node.transform;
        if node.mesh.1
        {
            self.meshes[node.mesh.0].render(renderer, vp * node_transform, node_transform);
        }
        for child_node in node.nodes.iter()
        {
            self.render_node(renderer, child_node, node_transform, vp);
        }
    }

    pub fn render(&self, renderer: &mut Renderer, vp: Mat4, transform: Mat4)
    {
        for node in self.nodes.iter()
        {
            self.render_node(renderer, node, transform, vp);
        }
    }
}
