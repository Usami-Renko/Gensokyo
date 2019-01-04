
use gltf::Semantic;

use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::property::position::GltfPropertyPosition;
use crate::assets::gltf::property::indices::GltfPropertyIndices;
use crate::assets::gltf::error::GltfError;
use crate::assets::gltf::property::traits::GltfPrimitiveProperty;
use crate::utils::types::Matrix4F;

use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor, BufferBlockIndex };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::instance::{ GsVertexBlock, GsIndexBlock };
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;
use gsvk::types::vkuint;


/// A wrapper class for primitive level in glTF, containing the data read from glTF file.
pub(super) struct GsGltfPrimitive {

    indices  : GltfPropertyIndices,
    positions: GltfPropertyPosition,
}

pub(super) struct GltfPrimitiveIndex {

    element_count: vkuint,
    ind_index: Option<BufferBlockIndex>,
    pos_index: Option<BufferBlockIndex>,
}

pub(super) struct GltfPrimitiveInstance {

    element_count: vkuint,
    index_block: Option<GsIndexBlock>,
    pos_block  : Option<GsVertexBlock>,
}

impl<'a> GsGltfHierachy<'a> for GsGltfPrimitive {
    type HierachyRawType = gltf::Primitive<'a>;
    type HierachyIndex   = GltfPrimitiveIndex;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency) ->  Result<Self, GltfError> {

        let mut primitive = GsGltfPrimitive::default();
        let reader = hierachy.reader(|b| Some(&agency.data_buffer[b.index()]));

        primitive.indices = GltfPropertyIndices::read(&reader);

        for (semantic, _accessor) in hierachy.attributes() {

            match semantic {
                | Semantic::Positions => {
                    primitive.positions = GltfPropertyPosition::read(&reader);
                },
                _ => {},
            }
        }

        Ok(primitive)
    }

    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let element_count = self.indices.indices_count()
            .or(self.positions.vertex_count())
            .unwrap(); // TODO: handle unwrap().
            // .ok_or(GltfError::ModelContentMissing)?;

        let index = GltfPrimitiveIndex {
            element_count: element_count as _,
            pos_index: self.positions.append_allocation(allocator)?,
            ind_index: self.indices.append_allocation(allocator)?,
        };
        Ok(index)
    }
}

impl GltfHierachyIndex for GltfPrimitiveIndex {
    type HierachyInstance = GltfPrimitiveInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs {

        GltfPrimitiveInstance {
            element_count: self.element_count,
            index_block: self.ind_index.and_then(|block| Some(distributor.acquire_index(block))),
            pos_block  : self.pos_index.and_then(|block| Some(distributor.acquire_vertex(block))),
        }
    }
}

impl<'a> GltfHierachyInstance<'a> for GltfPrimitiveInstance {
    type HierachyDataType = GltfPrimitiveUploadData<'a>;

    fn upload<M>(&self, uploader: &mut BufferDataUploader<M>, data: Self::HierachyDataType) -> Result<(), AllocatorError>
        where M: BufferMemoryTypeAbs {

        // apply transform to position property.
        let pos_transformed = data.primitive.positions.apply_transform(data.transform);

        // upload indices data to vulkan.
        data.primitive.indices.upload(&self.index_block, uploader)?;
        // upload position data to vulkan.
        pos_transformed.upload(&self.pos_block, uploader)?;

        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        let pos_block = self.pos_block.as_ref()
            .expect("Unreachable code");

        if let Some(ref index_block) = self.index_block {
            recorder
                .bind_vertex_buffers(0, &[pos_block])
                .bind_index_buffer(index_block, 0)
                .draw_indexed(self.element_count, 1, 0, 0, 0);
        } else {
            recorder
                .bind_vertex_buffers(0, &[pos_block])
                .draw(self.element_count, 1, 0, 0);
        }
    }
}

impl Default for GsGltfPrimitive {

    fn default() -> GsGltfPrimitive {

        GsGltfPrimitive {
            indices  : GltfPropertyIndices::default(),
            positions: GltfPropertyPosition::default(),
        }
    }
}

pub(super) struct GltfPrimitiveUploadData<'a> {
    pub primitive: &'a GsGltfPrimitive,
    pub transform: &'a Matrix4F,
}
