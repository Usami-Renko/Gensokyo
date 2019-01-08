
use crate::assets::gltf::storage::{ GltfRawDataAgency, GsGltfRepository, GsGltfStorage, GsGltfEntity };
use crate::assets::gltf::scene::{ GsGltfScene, GltfSceneIndex };
use crate::assets::gltf::material::storage::GltfShareResourceTmp;
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
    index: GltfSceneIndex,
}

impl<M> GsGltfAllocator<M> where M: BufferMemoryTypeAbs {

    pub(crate) fn new(physical: &GsPhyDevice, device: &GsDevice, typ: M) -> GsGltfAllocator<M> {
        GsGltfAllocator {
            allocator: GsBufferAllocator::new(physical, device, typ),
        }
    }

    pub fn append_model(&mut self, model: &GsGltfStorage) -> Result<GsModelIndex, AllocatorError> {

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

pub(super) trait GsGltfHierachy<'a>: Sized {
    type HierachyRawType;
    type HierachyVerifyType;
    type HierachyIndex;
    type HierachyTransform;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency, res: &mut GltfShareResourceTmp) -> Result<Self, GltfError>;

    fn generate_verification(&self) -> Option<Self::HierachyVerifyType>;

    fn verify(&self, verification: &Self::HierachyVerifyType) -> bool;

    fn apply_transform(&mut self, transform: &Self::HierachyTransform);

    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs;
}

pub(super) trait GltfHierachyIndex: Sized {
    type HierachyInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs;
}

pub(super) trait GltfHierachyInstance: Sized {
    type HierachyDataType;

    fn upload(&self, uploader: &mut BufferDataUploader, data: &Self::HierachyDataType) -> Result<(), AllocatorError>;
    fn record_command(&self, recorder: &GsCommandRecorder);
}
