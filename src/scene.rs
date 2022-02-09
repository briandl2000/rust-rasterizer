
use std::path::Path;

use crate::model::*;
use crate::transform::*;

use glam::Mat4;
use gltf::Node;
use crate::renderer::*;

struct Scene_Node{
    transform: Mat4,
    nodes: Vec<Scene_Node>
}

pub struct Scene
{
    meshes: Vec<Model>,
    nodes: Vec<Scene_Node>
}

impl Scene_Node{
    fn add_node(node: gltf::Node) -> Scene_Node
    {
        let transform: Mat4 = Mat4::from_cols_array_2d(&node.transform().matrix());
        let mut nodes:  Vec<Scene_Node> = vec![];
        
        for child in node.children()
        {
            nodes.push(Scene_Node::add_node(child));
        }

        Scene_Node {
            transform,
            nodes
        }
    }
}

impl Scene {
    pub fn load(path: &'static str) -> Self
    {
        let (document, buffers, _images) = gltf::import(path).unwrap();
        let mut meshes: Vec<Model> = Vec::new();

        for mesh in document.meshes()
        {
            meshes.push(Model::load_gltf(&mesh, &buffers));
        }

        let mut nodes: Vec<Scene_Node> = Vec::new();

        for scene in document.scenes()
        {
            for node in scene.nodes()
            {
                nodes.push(Scene_Node::add_node(node));
            }
        }

        Self 
        {
            meshes,
            nodes
        }

    }

    pub fn render(&self, renderer: &mut Renderer, mvp_matrix: Mat4)
    {
        for model in self.meshes.iter()
        {
            model.render(renderer, mvp_matrix);
        }
    }
}
