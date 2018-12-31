
use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::node::{ GsGltfNode, GltfNodeIndex, GltfNodeInstance };
use crate::assets::gltf::error::GltfError;

use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;


pub(super) struct GsGltfScene {

    nodes: Vec<GsGltfNode>,
}

pub(super) struct GltfSceneIndex {

    indices: Vec<GltfNodeIndex>,
}

pub(super) struct GltfSceneInstance {

    nodes: Vec<GltfNodeInstance>,
}

impl<'a> GsGltfHierachy<'a> for GsGltfScene {
    type HierachyRawType = gltf::Scene<'a>;
    type HierachyIndex   = GltfSceneIndex;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency) ->  Result<Self, GltfError> {

        let mut nodes = vec![];
        for raw_node in hierachy.nodes().into_iter() {
            let node = GsGltfNode::from_hierachy(raw_node, agency)?;
            nodes.push(node);
        }

        let result = GsGltfScene { nodes };

        Ok(result)
    }

    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let mut indices = vec![];

        for node in self.nodes.iter() {
            let index = node.allocate(allocator)?;
            indices.push(index);
        }

        Ok(GltfSceneIndex { indices })
    }
}

impl GltfHierachyIndex for GltfSceneIndex {
    type HierachyInstance = GltfSceneInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs {

        let nodes = self.indices.into_iter().map(|index| {
            index.distribute(distributor)
        }).collect();

        GltfSceneInstance { nodes }
    }
}

impl GltfHierachyInstance for GltfSceneInstance {
    type HierachyDataType = GsGltfScene;

    fn upload<M>(&self, uploader: &mut BufferDataUploader<M>, data: &Self::HierachyDataType) -> Result<(), AllocatorError>
        where M: BufferMemoryTypeAbs {

        for (node_instance, node_data) in self.nodes.iter().zip(data.nodes.iter()) {
            node_instance.upload(uploader, node_data)?;
        }
        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        self.nodes.iter().for_each(|node| {
            node.record_command(recorder);
        });
    }
}
