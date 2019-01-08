
use crate::assets::gltf::material::GsGltfMaterial;
use crate::assets::gltf::material::storage::GltfShareResourceTmp;

use std::rc::Rc;


pub(super) struct GltfPrimitiveMaterial {

    data: Option<Rc<GsGltfMaterial>>,
}

impl GltfPrimitiveMaterial {

    pub fn read(primitive: &gltf::Primitive, res: &mut GltfShareResourceTmp) -> GltfPrimitiveMaterial {

        let data = res.load_material(primitive);
        GltfPrimitiveMaterial { data }
    }

    pub fn is_contain_material(&self) -> bool {
        self.data.is_some()
    }
}
