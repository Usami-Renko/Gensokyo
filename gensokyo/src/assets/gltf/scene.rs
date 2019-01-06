
use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::node::{ GsGltfNode, GltfNodeIndex, GltfNodeInstance };
use crate::assets::gltf::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;

/// A wrapper class for scene level in glTF, containing the data read from glTF file.
pub(super) struct GsGltfScene {

    nodes: Vec<Box<GsGltfNode>>,
}

pub(super) struct GltfSceneIndex {

    indices: Vec<GltfNodeIndex>,
}

pub(super) struct GltfSceneInstance {

    nodes: Vec<Box<GltfNodeInstance>>,
}

impl<'a> GsGltfHierachy<'a> for GsGltfScene {
    type HierachyRawType   = gltf::Scene<'a>;
    type HierachyIndex     = GltfSceneIndex;
    type HierachyTransform = ();

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency) ->  Result<Self, GltfError> {

        let mut nodes = vec![];
        for raw_node in hierachy.nodes().into_iter() {
            let node = GsGltfNode::from_hierachy(raw_node, agency)?;
            nodes.push(Box::new(node));
        }

        let result = GsGltfScene { nodes };

        Ok(result)
    }

    fn apply_transform(&mut self, _: &Self::HierachyTransform) {

        self.nodes.iter_mut().for_each(|node| {
            node.apply_transform(&Matrix4F::identity())
        });
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
            let node_instance = index.distribute(distributor);
            Box::new(node_instance)
        }).collect();

        GltfSceneInstance { nodes }
    }
}

impl GltfHierachyInstance for GltfSceneInstance {
    type HierachyDataType = GsGltfScene;

    fn upload(&self, uploader: &mut BufferDataUploader, data: &Self::HierachyDataType) -> Result<(), AllocatorError> {

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
