
mod attributes;
mod attrpatterns;
mod indices;
mod material;
mod traits;

use crate::assets::gltf::storage::GltfRawDataAgency;
use crate::assets::gltf::importer::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::error::GltfError;
use crate::utils::types::Matrix4F;

use self::traits::GltfPrimitiveProperty;
use self::attributes::GltfPrimitiveAttributes;
use self::indices::GltfPrimitiveIndices;
use self::material::GltfPrimitiveMaterial;

use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor, BufferBlockIndex };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::instance::{ GsVertexBlock, GsIndexBlock };
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;
use gsvk::types::vkuint;

use gltf::Semantic;

/// A wrapper class for primitive level in glTF, containing the data read from glTF file.
pub(super) struct GsGltfPrimitive {

    element_count: usize,
    attributes: GltfPrimitiveAttributes,
    indices   : GltfPrimitiveIndices,
    material  : GltfPrimitiveMaterial,
}

pub(super) struct GltfPrimitiveIndex {

    element_count: vkuint,
    attributes_index: BufferBlockIndex,
    indices_index: Option<BufferBlockIndex>,
}

pub(super) struct GltfPrimitiveInstance {

    element_count: vkuint,
    attributes_block: GsVertexBlock,
    index_block: Option<GsIndexBlock>,
}

impl<'a> GsGltfHierachy<'a> for GsGltfPrimitive {
    type HierachyRawType   = gltf::Primitive<'a>;
    type HierachyIndex     = GltfPrimitiveIndex;
    type HierachyTransform = Matrix4F;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency) ->  Result<Self, GltfError> {

        let reader = hierachy.reader(|b| Some(&agency.data_buffer[b.index()]));

        let attributes = GltfPrimitiveAttributes::read(&hierachy, &reader)?;
        let indices = GltfPrimitiveIndices::read(&hierachy, &reader)?;
        let material = GltfPrimitiveMaterial::read(&hierachy, &reader)?;

        let element_count = indices.indices_count() // get the vertex count by its indices property in glTF.
            .or_else(||{
                // or get the vertex count by its element count of position attribute in glTF.
                hierachy.get(&Semantic::Positions)
                    .and_then(|accessor| {
                        match accessor.dimensions() {
                            | gltf::accessor::Dimensions::Vec2 => Some(accessor.count() / 2),
                            | gltf::accessor::Dimensions::Vec3 => Some(accessor.count() / 3),
                            | gltf::accessor::Dimensions::Vec4 => Some(accessor.count() / 4),
                            | _ => None
                        }
                    })
            }).ok_or(GltfError::ModelContentMissing)?;

        let primitive = GsGltfPrimitive {
            element_count, attributes, indices, material,
        };
        Ok(primitive)
    }

    fn apply_transform(&mut self, transform: &Self::HierachyTransform) {
        self.attributes.apply_transform(transform);
    }

    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let index = GltfPrimitiveIndex {
            element_count: self.element_count as _,
            attributes_index: self.attributes.append_allocation(allocator)?,
            indices_index: self.indices.append_allocation(allocator)?,
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
            index_block: self.indices_index.and_then(|block| Some(distributor.acquire_index(block))),
            attributes_block: distributor.acquire_vertex(self.attributes_index),
        }
    }
}

impl GltfHierachyInstance for GltfPrimitiveInstance {
    type HierachyDataType = GsGltfPrimitive;

    fn upload(&self, uploader: &mut BufferDataUploader, data: &Self::HierachyDataType) -> Result<(), AllocatorError> {

        // upload attribute data to vulkan.
        data.attributes.upload(&self.attributes_block, uploader)?;
        // upload indices data to vulkan.
        data.indices.upload(&self.index_block, uploader)?;

        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        if let Some(ref index_block) = self.index_block {
            recorder
                .bind_vertex_buffers(0, &[&self.attributes_block])
                .bind_index_buffer(index_block, 0)
                .draw_indexed(self.element_count, 1, 0, 0, 0);
        } else {
            recorder
                .bind_vertex_buffers(0, &[&self.attributes_block])
                .draw(self.element_count, 1, 0, 0);
        }
    }
}
