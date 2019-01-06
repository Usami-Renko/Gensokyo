
use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::primitive::{ GsGltfPrimitive, GltfPrimitiveIndex, GltfPrimitiveInstance };
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

pub(super) struct GltfMeshIndex {

    indices: Vec<GltfPrimitiveIndex>,
}

pub(super) struct GltfMeshInstance {

    primitives: Vec<GltfPrimitiveInstance>,
}

impl<'a> GsGltfHierachy<'a> for GsGltfMesh {
    type HierachyRawType   = gltf::Mesh<'a>;
    type HierachyIndex     = GltfMeshIndex;
    type HierachyTransform = Matrix4F;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency) ->  Result<Self, GltfError> {

        let mut primitives = vec![];
        for raw_primitive in hierachy.primitives() {
            let primitive = GsGltfPrimitive::from_hierachy(raw_primitive, agency)?;
            primitives.push(primitive);
        }

        let mesh = GsGltfMesh { primitives };
        Ok(mesh)
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
