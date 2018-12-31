
use crate::assets::gltf::scene::{ GsGltfScene, GltfSceneInstance, GltfSceneIndex };
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyInstance };

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::GsBufferAllocator;
use gsvk::buffer::GsBufferRepository;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;


pub struct GltfRawDataAgency {
    pub doc: gltf::Document,
    pub data_buffer: Vec<gltf::buffer::Data>,
    pub data_image : Vec<gltf::image::Data>,
}

pub struct GsGltfEntity {

    scene: GltfSceneInstance,
}

impl GsGltfEntity {

    pub(super) fn new(scene: GltfSceneInstance) -> GsGltfEntity {
        GsGltfEntity { scene }
    }

    pub fn record_command(&self, recorder: &GsCommandRecorder) {

        self.scene.record_command(recorder);
    }
}

pub struct GsModelStorage {

    scene: GsGltfScene,
}

impl GsModelStorage {

    pub(super) fn new(scene: GsGltfScene) -> GsModelStorage {
        GsModelStorage { scene }
    }

    pub(super) fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<GltfSceneIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {
        self.scene.allocate(allocator)
    }
}

pub struct GsGltfRepository<M> where M: BufferMemoryTypeAbs {

    repository: GsBufferRepository<M>,
}

impl<M> GsGltfRepository<M> where M: BufferMemoryTypeAbs {

    pub(super) fn new(repo: GsBufferRepository<M>) -> GsGltfRepository<M> {

        GsGltfRepository {
            repository: repo,
        }
    }

    pub fn data_uploader(&mut self) -> Result<GltfDataUploader<M>, AllocatorError> {

        let target = GltfDataUploader {
            uploader: self.repository.data_updater()?,
        };
        Ok(target)
    }
}

pub struct GltfDataUploader<M> where M: BufferMemoryTypeAbs {

    uploader: BufferDataUploader<M>,
}

impl<M> GltfDataUploader<M> where M: BufferMemoryTypeAbs {

    pub fn upload(&mut self, to: &GsGltfEntity, data_torage: &GsModelStorage) -> Result<&mut GltfDataUploader<M>, AllocatorError> {

        to.scene.upload(&mut self.uploader, &data_torage.scene)?;
        self.uploader.finish()?;

        Ok(self)
    }
}
