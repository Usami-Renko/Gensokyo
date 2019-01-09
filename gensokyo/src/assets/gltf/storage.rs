
use crate::assets::gltf::traits::{ GsGltfHierachy, GltfHierachyInstance };
use crate::assets::gltf::scene::{ GsGltfScene, GltfSceneInstance, GltfSceneIndex };
use crate::assets::gltf::material::storage::GltfShareResource;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::GsBufferAllocator;
use gsvk::buffer::instance::GsUniformBlock;
use gsvk::buffer::GsBufferRepository;
use gsvk::memory::transfer::{ GsBufferDataUploader, GsBufferDataUpdater };
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;


// ------------------------------------------------------------------------------------
pub(super) struct GltfRawDataAgency {
    pub doc: gltf::Document,
    pub data_buffer: Vec<gltf::buffer::Data>,
    pub data_image : Vec<gltf::image::Data>,
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GsGltfEntity {

    scene: GltfSceneInstance,
    uniform: GsUniformBlock,
}

impl GsGltfEntity {

    pub(super) fn new(scene: GltfSceneInstance, uniform: GsUniformBlock) -> GsGltfEntity {
        GsGltfEntity { scene, uniform }
    }

    pub fn record_command(&self, recorder: &GsCommandRecorder) {

        self.scene.record_command(recorder);
    }

    pub fn uniform_ref(&self) -> &GsUniformBlock {
        &self.uniform
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GsGltfStorage {

    scene: GsGltfScene,
    #[allow(dead_code)]
    resource: GltfShareResource, // just keep it exist until it drops.
}

impl GsGltfStorage {

    pub(super) fn new(scene: GsGltfScene, res: GltfShareResource) -> GsGltfStorage {
        GsGltfStorage { scene, resource: res }
    }

    pub(super) fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<GltfSceneIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {
        self.scene.allocate(allocator)
    }

    pub fn apply_transform(&mut self) {
        self.scene.apply_transform(&());
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GsGltfRepository<M> where M: BufferMemoryTypeAbs {

    repository: GsBufferRepository<M>,
}

impl<M> GsGltfRepository<M> where M: BufferMemoryTypeAbs {

    pub(super) fn new(repo: GsBufferRepository<M>) -> GsGltfRepository<M> {

        GsGltfRepository {
            repository: repo,
        }
    }

    pub fn data_uploader(&mut self) -> Result<GltfDataUploader, AllocatorError> {

        let target = GltfDataUploader {
            uploader: self.repository.data_uploader()?,
        };
        Ok(target)
    }

    pub fn uniform_updater(&mut self) -> Result<GltfDataUniformUpdater, AllocatorError> {

        let target = GltfDataUniformUpdater {
            updater: self.repository.data_updater()?,
        };
        Ok(target)
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GltfDataUploader {

    uploader: GsBufferDataUploader,
}

impl GltfDataUploader {

    pub fn upload(&mut self, to: &GsGltfEntity, data_storage: &GsGltfStorage) -> Result<&mut GltfDataUploader, AllocatorError> {

        to.scene.upload(&mut self.uploader, &data_storage.scene)?;

        Ok(self)
    }

    pub fn finish(&mut self) -> Result<(), AllocatorError> {

        self.uploader.finish()
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GltfDataUniformUpdater {

    updater: GsBufferDataUpdater,
}

impl GltfDataUniformUpdater {

    pub fn update_uniform(&mut self, to: &GsGltfEntity, data_storage: &GsGltfStorage) -> Result<&mut GltfDataUniformUpdater, AllocatorError> {

        data_storage.scene.update_uniform(&mut self.updater, &to.uniform, &data_storage.resource)?;

        Ok(self)
    }

    pub fn finish(&mut self) -> Result<(), AllocatorError> {

        self.updater.finish()
    }
}
// ------------------------------------------------------------------------------------
