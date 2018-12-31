
use gltf::Semantic;

use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::error::GltfError;

use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor, BufferBlockIndex };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::instance::{ GsVertexBlock, GsIndexBlock };
use gsvk::buffer::instance::{ VertexBlockInfo, IndexBlockInfo };
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;
use gsvk::types::{ vkuint, vkbytes };

use gsma::data_size;


pub(super) struct GsGltfPrimitive {

    positions: Vec<[f32; 3]>,
    indices  : Option<Vec<vkuint>>,
}

pub(super) struct GltfPrimitiveIndex {

    element_count: vkuint,
    ind_index: Option<BufferBlockIndex>,
    pos_index: BufferBlockIndex,
}

pub(super) struct GltfPrimitiveInstance {

    element_count: vkuint,
    index_block: Option<GsIndexBlock>,
    pos_block  : GsVertexBlock,
}

impl<'a> GsGltfHierachy<'a> for GsGltfPrimitive {
    type HierachyRawType = gltf::Primitive<'a>;
    type HierachyIndex   = GltfPrimitiveIndex;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency) ->  Result<Self, GltfError> {

        let mut primitive = GsGltfPrimitive::default();
        let reader = hierachy.reader(|b| Some(&agency.data_buffer[b.index()]));

        if hierachy.indices().is_some() {

            let index_iter = reader.read_indices()
                .ok_or(GltfError::ModelContentMissing)?; // missing index attribute.
            primitive.indices = Some(index_iter.into_u32().collect());
        }

        for (semantic, _accessor) in hierachy.attributes() {

            match semantic {
                | Semantic::Positions => {
                    let pos_iter = reader.read_positions()
                        .ok_or(GltfError::ModelContentMissing)?; // missing position attribute.
                    primitive.positions = pos_iter.collect();
                },
                _ => {},
            }
        }

        Ok(primitive)
    }

    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let index_index = if let Some(ref indices) = self.indices {
            let index_info = IndexBlockInfo::new(data_size!(indices, vkuint));
            Some(allocator.append_buffer(index_info)?)
        } else {
            None
        };

        let vertex_info = VertexBlockInfo::new(data_size!(self.positions, [f32; 3]));
        let vertex_pos_index = allocator.append_buffer(vertex_info)?;

        let element_count = self.indices.as_ref()
            .and_then(|indices| Some(indices.len()))
            .unwrap_or(self.positions.len());

        let index = GltfPrimitiveIndex {
            element_count: element_count as _,
            pos_index: vertex_pos_index,
            ind_index: index_index,
        };
        Ok(index)
    }
}

impl GltfHierachyIndex for GltfPrimitiveIndex {
    type HierachyInstance = GltfPrimitiveInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs {

        if let Some(index) = self.ind_index {
            let index_block = distributor.acquire_index(index);

            GltfPrimitiveInstance {
                element_count: self.element_count,
                index_block: Some(index_block),
                pos_block  : distributor.acquire_vertex(self.pos_index),
            }
        } else {
            GltfPrimitiveInstance {
                element_count: self.element_count,
                index_block: None,
                pos_block  : distributor.acquire_vertex(self.pos_index),
            }
        }
    }
}

impl GltfHierachyInstance for GltfPrimitiveInstance {
    type HierachyDataType = GsGltfPrimitive;

    fn upload<M>(&self, uploader: &mut BufferDataUploader<M>, data: &Self::HierachyDataType) -> Result<(), AllocatorError>
        where M: BufferMemoryTypeAbs {

        uploader.upload(&self.pos_block, &data.positions)?;

        if let Some(ref index_block) = self.index_block {
            if let Some(ref index_data) = data.indices {
                uploader.upload(index_block, index_data)?;
            }
        }

        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        if let Some(ref index_block) = self.index_block {
            recorder
                .bind_vertex_buffers(0, &[&self.pos_block])
                .bind_index_buffer(index_block, 0)
                .draw_indexed(self.element_count, 1, 0, 0, 0);
        } else {
            recorder
                .bind_vertex_buffers(0, &[&self.pos_block])
                .draw(self.element_count, 1, 0, 0);
        }
    }
}

impl Default for GsGltfPrimitive {

    fn default() -> GsGltfPrimitive {

        GsGltfPrimitive {
            positions: vec![],
            indices  : None,
        }
    }
}
