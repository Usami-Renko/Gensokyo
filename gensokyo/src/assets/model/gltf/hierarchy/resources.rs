
use gltf;

use assets::model::GltfMesh;

// The container of all the resources in Gltf.
#[derive(Default)]
pub(crate) struct GltfResources {

     pub meshes: Vec<GltfMesh>,
}

impl GltfResources {

    pub fn append_mesh(&mut self, mesh: GltfMesh) -> usize {

        let mesh_index = self.meshes.len();
        self.meshes.push(mesh);

        mesh_index
    }
}

pub(crate) struct GltfRawData {

    pub document: gltf::Document,
    pub buffers : Vec<gltf::buffer::Data>,
    pub images  : Vec<gltf::image::Data>,
}
