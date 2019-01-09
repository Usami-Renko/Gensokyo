
use crate::assets::gltf::traits::{ GsGltfHierachy, GltfHierachyIndex };
use crate::assets::gltf::storage::{ GltfRawDataAgency, GsGltfRepository, GsGltfStorage, GsGltfEntity };
use crate::assets::gltf::scene::{ GsGltfScene, GltfSceneIndex };
use crate::assets::gltf::material::storage::GltfShareResourceTmp;
use crate::assets::gltf::material::GltfPbrUniform;
use crate::assets::gltf::error::GltfError;
use crate::assets::error::AssetsError;

use gsvk::core::physical::GsPhyDevice;
use gsvk::core::device::GsDevice;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor, BufferBlockIndex };
use gsvk::buffer::instance::UniformBlockInfo;
use gsvk::memory::AllocatorError;
use gsvk::types::vkuint;

use gsma::data_size;

use std::path::Path;

pub struct GsGltfImporter;

impl GsGltfImporter {

    /// Try to load a glTF file(read to memory) with its path, and return its model data if succeed.
    pub fn load(path: impl AsRef<Path>) -> Result<GsGltfStorage, AssetsError> {

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

        let target = GsGltfStorage::new(scene_data, share_resource.into_resource());
        Ok(target)
    }
}

pub struct GsGltfAllocator<M> where M: BufferMemoryTypeAbs {

    allocator: GsBufferAllocator<M>,
}

pub struct GsModelIndex {

    scene_index: GltfSceneIndex,
    pbr_uniform_index: BufferBlockIndex,
}

pub struct GltfRenderInfo {
    pbr_uniform_binding: vkuint,
}

impl<M> GsGltfAllocator<M> where M: BufferMemoryTypeAbs {

    pub(crate) fn new(physical: &GsPhyDevice, device: &GsDevice, typ: M) -> GsGltfAllocator<M> {
        GsGltfAllocator {
            allocator: GsBufferAllocator::new(physical, device, typ),
        }
    }

    pub fn append_model(&mut self, model: &GsGltfStorage, render_info: GltfRenderInfo) -> Result<GsModelIndex, AllocatorError> {

        // allocate uniform buffer for pbr shading.
        // uniform count should always be 1.
        let pbr_uniform_info = UniformBlockInfo::new(render_info.pbr_uniform_binding, 1, data_size!(GltfPbrUniform));
        let pbr_uniform_index = self.allocator.append_buffer(pbr_uniform_info)?;

        // allocate vertex buffer.
        let scene_index = model.allocate(&mut self.allocator)?;

        let model_index = GsModelIndex { scene_index, pbr_uniform_index };
        Ok(model_index)
    }

    pub fn allocate(self) -> Result<GsGltfDistributor<M>, AllocatorError> {

        let result = GsGltfDistributor {
            distributor: self.allocator.allocate()?,
        };
        Ok(result)
    }
}

impl GltfRenderInfo {

    pub fn new(pbr_uniform_binding: vkuint) -> GltfRenderInfo {
        GltfRenderInfo { pbr_uniform_binding }
    }
}

pub struct GsGltfDistributor<M> where M: BufferMemoryTypeAbs {

    distributor: GsBufferDistributor<M>,
}

impl<M> GsGltfDistributor<M> where M: BufferMemoryTypeAbs {

    pub fn acquire_model(&self, index: GsModelIndex) -> Result<GsGltfEntity, AllocatorError> {

        let pbr_uniform = self.distributor.acquire_uniform(index.pbr_uniform_index)?;
        let scene = index.scene_index.distribute(&self.distributor);

        let entity = GsGltfEntity::new(scene, pbr_uniform);
        Ok(entity)
    }

    pub fn into_repository(self) -> GsGltfRepository<M> {

        GsGltfRepository::new(self.distributor.into_repository())
    }
}
