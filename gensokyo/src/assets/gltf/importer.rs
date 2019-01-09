
use crate::assets::gltf::traits::{ GsGltfHierachy, GltfHierachyIndex };
use crate::assets::gltf::storage::{ GltfRawDataAgency, GsGltfStorage, GltfShareResource };
use crate::assets::gltf::scene::{ GsGltfScene, GltfSceneInstance, GltfSceneIndex };
use crate::assets::gltf::material::{ GltfShareResourceTmp, GltfPbrUniform };
use crate::assets::gltf::error::GltfError;
use crate::assets::error::AssetsError;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor, BufferBlockIndex };
use gsvk::buffer::instance::{ GsUniformBlock, UniformBlockInfo };
use gsvk::memory::types::Host;
use gsvk::memory::AllocatorError;
use gsvk::types::vkuint;

use gsma::data_size;

use std::path::Path;

pub struct GsGltfImporter;

impl GsGltfImporter {

    /// Try to load a glTF file(read to memory) with its path, and return its model data if succeed.
    pub fn load(path: impl AsRef<Path>) -> Result<(GsGltfStorage, GltfShareResource), AssetsError> {

        let (doc, data_buffer, data_image) = gltf::import(path)
            .map_err(|e| AssetsError::Gltf(GltfError::Loading(e)))?;

        let data_agency = GltfRawDataAgency {
            doc, data_buffer, data_image
        };

        // Only support loading the default scene or first scene in gltf file.
        let dst_scene = data_agency.doc.default_scene()
            .or(data_agency.doc.scenes().next())
            .ok_or(GltfError::ModelContentMissing)?;

        // fetch the data from glTF.
        let mut share_resource = GltfShareResourceTmp::default();
        let mut scene_data = GsGltfScene::from_hierachy(dst_scene, &data_agency, &mut share_resource)
            .map_err(|e| AssetsError::Gltf(e))?;

        // verify the content of glTF.
        let verify_content = scene_data.generate_verification()
            .ok_or(GltfError::VerificationError)?;
        if scene_data.verify(&verify_content) == false {
            return Err(AssetsError::from(GltfError::VerificationError))
        }

        // update the transformation of glTF data.
        scene_data.apply_transform(&());

        let target = GsGltfStorage::new(scene_data);
        Ok((target, share_resource.into_resource()))
    }

    pub fn set_render(pbr_uniform_binding: vkuint, uniform_allocator: &mut GsBufferAllocator<Host>) -> Result<GltfRenderInfo, AllocatorError> {

        // allocate uniform buffer for pbr shading.
        // uniform count should always be 1.
        let pbr_uniform_info = UniformBlockInfo::new(pbr_uniform_binding, 1, data_size!(GltfPbrUniform));
        let pbr_uniform_index = uniform_allocator.append_buffer(pbr_uniform_info)?;

        let target = GltfRenderInfo { pbr_uniform_index };
        Ok(target)
    }

    pub fn set_data<M>(model: &GsGltfStorage, share_res: GltfShareResource, to: &mut GsBufferAllocator<M>) -> Result<GltfDataInfo, AllocatorError> where M: BufferMemoryTypeAbs {

        let scene_index = model.allocate(to)?; // allocate vertex buffer.

        let target = GltfDataInfo { scene_index, share_res };
        Ok(target)
    }
}

// ------------------------------------------------------------------------------------
pub struct GltfRenderInfo {

    pbr_uniform_index: BufferBlockIndex,
}

impl GltfRenderInfo {

    pub fn into_entity(self, from: &GsBufferDistributor<Host>) -> Result<GltfRenderEntity, AllocatorError> {

        let target = GltfRenderEntity {
            pbr_uniform: from.acquire_uniform(self.pbr_uniform_index)?,
        };
        Ok(target)
    }
}

pub struct GltfRenderEntity {

    pub(super) pbr_uniform: GsUniformBlock,
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GltfDataInfo {

    scene_index: GltfSceneIndex,
    share_res: GltfShareResource,
}

impl GltfDataInfo {

    pub fn into_entity<M>(self, from: &GsBufferDistributor<M>) -> GltfDataEntity
        where M: BufferMemoryTypeAbs {

        GltfDataEntity {
            scene: self.scene_index.distribute(from),
            share_res: self.share_res,
        }
    }
}

pub struct GltfDataEntity {

    pub(super) scene: GltfSceneInstance,
    pub(super) share_res: GltfShareResource,
}
// ------------------------------------------------------------------------------------
