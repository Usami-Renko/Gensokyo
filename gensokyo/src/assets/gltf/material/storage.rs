
use crate::assets::gltf::material::{ GsGltfMaterial, GsGltfTexture, GsGltfSampler };

use std::rc::Rc;
use std::collections::HashMap;

type GltfReferenceIndex = usize;
type GltfStorageIndex   = usize;

// ------------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct GltfShareResourceTmp {

    materials: Vec<Rc<GsGltfMaterial>>,
    #[allow(dead_code)]
    textures : Vec<Rc<GsGltfTexture>>,
    #[allow(dead_code)]
    samplers : Vec<Rc<GsGltfSampler>>,

    material_indices: HashMap<GltfReferenceIndex, GltfStorageIndex>,
    #[allow(dead_code)]
    texture_indices : HashMap<GltfReferenceIndex, GltfStorageIndex>,
    #[allow(dead_code)]
    sampler_indices : HashMap<GltfReferenceIndex, GltfStorageIndex>,
}

impl GltfShareResourceTmp {

    pub fn load_material(&mut self, primitive: &gltf::Primitive) -> Option<Rc<GsGltfMaterial>> {

        let raw_material = primitive.material();
        let gltf_index = raw_material.index()?;

        if let Some(res_index) = self.material_indices.get(&gltf_index) {

            let dst_material = self.materials[*res_index].clone();
            Some(dst_material.clone())
        } else {

            let new_material = Rc::new(GsGltfMaterial::new(&raw_material));
            let dst_material = new_material.clone();

            let res_index = self.materials.len();
            self.material_indices.insert(gltf_index, res_index);
            self.materials.push(new_material);
            Some(dst_material)
        }
    }

    pub fn into_resource(self) -> GltfShareResource {
        GltfShareResource {
            materials: self.materials,
            textures : self.textures,
            samplers : self.samplers,
        }
    }
}

#[derive(Default)]
pub(crate) struct GltfShareResource {

    materials: Vec<Rc<GsGltfMaterial>>,
    textures : Vec<Rc<GsGltfTexture>>,
    samplers : Vec<Rc<GsGltfSampler>>,
}
// ------------------------------------------------------------------------------------
