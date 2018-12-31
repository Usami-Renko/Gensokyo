
use crate::assets::gltf::storage::{ GltfRawDataAgency, GsGltfRepository, GsModelStorage, GsGltfEntity };
use crate::assets::gltf::scene::{ GsGltfScene, GltfSceneIndex };
use crate::assets::gltf::error::GltfError;
use crate::assets::error::AssetsError;

use gsvk::core::physical::GsPhyDevice;
use gsvk::core::device::GsDevice;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor };
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;

use std::path::Path;

pub struct GsGltfImporter;

impl GsGltfImporter {

    pub fn load(path: impl AsRef<Path>) -> Result<GsModelStorage, AssetsError> {

        let (doc, data_buffer, data_image) = gltf::import(path)
            .map_err(|e| AssetsError::Gltf(GltfError::Loading(e)))?;

        let data_agency = GltfRawDataAgency {
            doc, data_buffer, data_image
        };

        // Only support loading the default scene or first scene in gltf file.
        let dst_scene = data_agency.doc.default_scene()
            .or(data_agency.doc.scenes().next())
            .ok_or(GltfError::ModelContentMissing)?;

        let scene_data = GsGltfScene::from_hierachy(dst_scene, &data_agency)
            .map_err(|e| AssetsError::Gltf(e))?;
        let target = GsModelStorage::new(scene_data);
        Ok(target)
    }
}

pub struct GsGltfAllocator<M> where M: BufferMemoryTypeAbs {

    allocator: GsBufferAllocator<M>,
}

pub struct GsModelIndex {
    index: GltfSceneIndex,
}

impl<M> GsGltfAllocator<M> where M: BufferMemoryTypeAbs {

    pub(crate) fn new(physical: &GsPhyDevice, device: &GsDevice, typ: M) -> GsGltfAllocator<M> {
        GsGltfAllocator {
            allocator: GsBufferAllocator::new(physical, device, typ),
        }
    }

    pub fn append_model(&mut self, model: &GsModelStorage) -> Result<GsModelIndex, AllocatorError> {

        let index = model.allocate(&mut self.allocator)?;
        Ok(GsModelIndex { index })
    }

    pub fn allocate(self) -> Result<GsGltfDistributor<M>, AllocatorError> {

        let result = GsGltfDistributor {
            distributor: self.allocator.allocate()?,
        };
        Ok(result)
    }
}

pub struct GsGltfDistributor<M> where M: BufferMemoryTypeAbs {

    distributor: GsBufferDistributor<M>,
}

impl<M> GsGltfDistributor<M> where M: BufferMemoryTypeAbs {

    pub fn acquire_model(&self, index: GsModelIndex) -> GsGltfEntity {

        let scene = index.index.distribute(&self.distributor);
        GsGltfEntity::new(scene)
    }

    pub fn into_repository(self) -> GsGltfRepository<M> {

        GsGltfRepository::new(self.distributor.into_repository())
    }
}

pub(super) trait GsGltfHierachy<'a> where Self: Sized {
    type HierachyRawType;
    type HierachyIndex;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency) -> Result<Self, GltfError>;
    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs;
}

pub(super) trait GltfHierachyIndex where Self: Sized {
    type HierachyInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs;
}

pub(super) trait GltfHierachyInstance where Self: Sized {
    type HierachyDataType;

    fn upload<M>(&self, uploader: &mut BufferDataUploader<M>, data: &Self::HierachyDataType) -> Result<(), AllocatorError>
        where M: BufferMemoryTypeAbs;
    fn record_command(&self, recorder: &GsCommandRecorder);
}
