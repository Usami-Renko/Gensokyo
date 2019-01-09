
use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::material::storage::{ GltfShareResource, GltfShareResourceTmp };
use crate::assets::gltf::error::GltfError;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor };
use gsvk::buffer::instance::GsUniformBlock;
use gsvk::memory::transfer::{ GsBufferDataUploader, GsBufferDataUpdater };
use gsvk::command::GsCommandRecorder;
use gsvk::memory::AllocatorError;

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

    fn update_uniform(&self, updater: &mut GsBufferDataUpdater, to: &GsUniformBlock, res: &GltfShareResource) -> Result<(), AllocatorError>;
}

pub(super) trait GltfHierachyIndex: Sized {
    type HierachyInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs;
}

pub(super) trait GltfHierachyInstance: Sized {
    type HierachyDataType;

    fn upload(&self, uploader: &mut GsBufferDataUploader, data: &Self::HierachyDataType) -> Result<(), AllocatorError>;
    fn record_command(&self, recorder: &GsCommandRecorder);
}
