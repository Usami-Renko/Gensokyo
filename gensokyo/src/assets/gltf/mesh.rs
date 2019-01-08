
use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::primitive::{ GsGltfPrimitive, GltfPrimitiveIndex, GltfPrimitiveInstance, GltfPrimitiveVerification };
use crate::assets::gltf::material::storage::GltfShareResourceTmp;
use crate::assets::gltf::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;

/// A wrapper class for mesh level in glTF, containing the data read from glTF file.
pub(super) struct GsGltfMesh {

    primitives: Vec<GsGltfPrimitive>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct GltfMeshVerification {

    verification: GltfPrimitiveVerification,
}

pub(super) struct GltfMeshIndex {

    indices: Vec<GltfPrimitiveIndex>,
}

pub(super) struct GltfMeshInstance {

    primitives: Vec<GltfPrimitiveInstance>,
}

impl<'a> GsGltfHierachy<'a> for GsGltfMesh {
    type HierachyRawType    = gltf::Mesh<'a>;
    type HierachyVerifyType = GltfMeshVerification;
    type HierachyIndex      = GltfMeshIndex;
    type HierachyTransform  = Matrix4F;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency, res: &mut GltfShareResourceTmp) -> Result<Self, GltfError> {

        let mut primitives = vec![];
        for raw_primitive in hierachy.primitives() {
            let primitive = GsGltfPrimitive::from_hierachy(raw_primitive, agency, res)?;
            primitives.push(primitive);
        }

        let mesh = GsGltfMesh { primitives };
        Ok(mesh)
    }

    fn generate_verification(&self) -> Option<Self::HierachyVerifyType> {

        self.primitives.first()
            .and_then(|p| p.generate_verification())
            .and_then(|verification| Some(GltfMeshVerification { verification }))
    }

    fn verify(&self, verification: &Self::HierachyVerifyType) -> bool {

        self.primitives.iter().all(|primitive| {
            primitive.verify(&verification.verification)
        })
    }

    fn apply_transform(&mut self, transform: &Self::HierachyTransform) {

        self.primitives.iter_mut().for_each(|primitive| {
            primitive.apply_transform(transform);
        });
    }

    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let mut indices = vec![];

        for primitive in self.primitives.iter() {
            let index = primitive.allocate(allocator)?;
            indices.push(index);
        }

        Ok(GltfMeshIndex { indices })
    }
}

impl GltfHierachyIndex for GltfMeshIndex {
    type HierachyInstance = GltfMeshInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs {

        let primitives = self.indices.into_iter().map(|primitive| {
            primitive.distribute(distributor)
        }).collect();

        GltfMeshInstance { primitives }
    }
}

impl GltfHierachyInstance for GltfMeshInstance {
    type HierachyDataType = GsGltfMesh;

    fn upload(&self, uploader: &mut BufferDataUploader, data: &Self::HierachyDataType) -> Result<(), AllocatorError> {

        for (primitive_instance, primitive_data) in self.primitives.iter()
            .zip(data.primitives.iter()) {

            primitive_instance.upload(uploader, primitive_data)?;
        }
        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        self.primitives.iter().for_each(|primitive| {
            primitive.record_command(recorder);
        });
    }
}
