
use ash::vk;

use crate::buffer::target::BufferDescInfo;
use crate::buffer::entity::BufferBlock;
use crate::buffer::instance::enums::BufferInstanceType;
use crate::buffer::traits::{ BufferInstance, BufferBlockInfo };
use crate::buffer::traits::{ BufferCopiable, BufferCopyInfo };
use crate::buffer::allocator::{ BufferBlockIndex, BufferDistAttachment };
use crate::buffer::error::BufferError;

use crate::descriptor::DescriptorBufferBindableTarget;
use crate::descriptor::{ DescriptorBindingContent, DescriptorBufferBindingInfo };
use crate::descriptor::{ GsDescriptorType, BufferDescriptorType };

use crate::types::{ vkuint, vkbytes };

#[derive(Debug, Clone)]
pub struct UniformBlockInfo {

    info: BufferDescInfo,
    binding: DescriptorBindingContent,
    element_size: vkbytes,
}

impl UniformBlockInfo {

    pub fn new(binding: vkuint, count: vkuint, element_size: vkbytes) -> UniformBlockInfo {

        let estimate_size = (count as vkbytes) * element_size;

        UniformBlockInfo {
            info: BufferDescInfo::new(estimate_size, vk::BufferUsageFlags::UNIFORM_BUFFER),
            binding: DescriptorBindingContent {
                binding, count,
                descriptor_type: GsDescriptorType::Buffer(BufferDescriptorType::UniformBuffer),
            },
            element_size,
        }
    }
}

impl BufferBlockInfo for UniformBlockInfo {
    const INSTANCE_TYPE: BufferInstanceType = BufferInstanceType::UniformBuffer;

    fn as_desc_ref(&self) -> &BufferDescInfo {
        &self.info
    }

    fn into_desc(self) -> BufferDescInfo {
        self.info
    }

    fn to_block_index(&self, index: usize) -> BufferBlockIndex {

        let attachment = UniformAttachment {
            binding: self.binding.clone(),
            element_size: self.element_size,
        };

        BufferBlockIndex {
            value: index,
            attachment: Some(BufferDistAttachment::Uniform(attachment)),
        }
    }
}

pub struct GsUniformBlock {

    binding: DescriptorBindingContent,

    block: BufferBlock,
    repository_index: usize,
    element_size: vkbytes,
}

impl GsUniformBlock {

    pub(crate) fn new(block: BufferBlock, index: BufferBlockIndex) -> Result<GsUniformBlock, BufferError> {

        let repository_index = index.value;
        let attachment = index.attachment
            .and_then(|attachment| match attachment {
                | BufferDistAttachment::Uniform(uniform_attachment) => Some(uniform_attachment),
            }).ok_or(BufferError::NoBufferAttachError)?;

        let block = GsUniformBlock {
            binding      : attachment.binding,
            element_size : attachment.element_size,
            block,
            repository_index,
        };

        Ok(block)
    }
}

impl DescriptorBufferBindableTarget for GsUniformBlock {

    fn binding_info(&self, sub_block_indices: Option<Vec<vkuint>>) -> DescriptorBufferBindingInfo {

        DescriptorBufferBindingInfo {
            content: self.binding.clone(),
            element_indices: sub_block_indices.unwrap_or(vec![0]),
            element_size: self.element_size,
            buffer_handle: self.block.handle,
        }
    }
}

impl BufferInstance for GsUniformBlock {

    fn typ(&self) -> BufferInstanceType {
        BufferInstanceType::UniformBuffer
    }

    fn as_block_ref(&self) -> &BufferBlock {
        &self.block
    }

    fn repository_index(&self) -> usize {
        self.repository_index
    }
}

impl BufferCopiable for GsUniformBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}

pub struct UniformAttachment {

    binding: DescriptorBindingContent,
    element_size: vkbytes,
}
