
use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::mesh::{ GsGltfMesh, GltfMeshIndex, GltfMeshInstance };
use crate::assets::gltf::error::GltfError;

use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;


pub(super) struct GsGltfNode {

    mesh: GsGltfMesh,
}

pub(super) struct GltfNodeIndex {

    index: GltfMeshIndex,
}

pub(super) struct GltfNodeInstance {

    mesh: GltfMeshInstance,
}

impl<'a> GsGltfHierachy<'a> for GsGltfNode {
    type HierachyRawType = gltf::Node<'a>;
    type HierachyIndex   = GltfNodeIndex;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency) ->  Result<Self, GltfError> {

        if let Some(raw_mesh) = hierachy.mesh() {

            let mesh = GsGltfMesh::from_hierachy(raw_mesh, &agency)?;
            let result = GsGltfNode { mesh };
            Ok(result)
        } else {
            Err(GltfError::ModelContentMissing)
        }
    }

    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let index = self.mesh.allocate(allocator)?;
        Ok(GltfNodeIndex { index })
    }
}

impl GltfHierachyIndex for GltfNodeIndex {
    type HierachyInstance = GltfNodeInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs {

        let mesh = self.index.distribute(distributor);

        GltfNodeInstance { mesh }
    }
}

impl GltfHierachyInstance for GltfNodeInstance {
    type HierachyDataType = GsGltfNode;

    fn upload<M>(&self, uploader: &mut BufferDataUploader<M>, data: &Self::HierachyDataType) -> Result<(), AllocatorError>
        where M: BufferMemoryTypeAbs {

        self.mesh.upload(uploader, &data.mesh)?;

        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        self.mesh.record_command(recorder);
    }
}
