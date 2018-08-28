
use ash::vk;

use utility::marker::VulkanFlags;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferCreateFlag {
    /// SparseBindingBit specifies that the buffer will be backed using sparse memory binding.
    SparseBindingBit,
    /// SparseResidency specifies that the buffer can be partially backed using sparse memory binding.
    ///
    /// Buffers created with this flag must also be created with the VK_BUFFER_CREATE_SPARSE_BINDING_BIT flag.
    SparseResidency,
    /// SparseAliased specifies that the buffer will be backed using sparse memory binding with memory ranges that might also simultaneously be backing another buffer (or another portion of the same buffer).
    ///
    /// Buffers created with this flag must also be created with the VK_BUFFER_CREATE_SPARSE_BINDING_BIT flag.
    SparseAliased,
}
impl VulkanFlags for [BufferCreateFlag] {
    type FlagType = vk::BufferCreateFlags;

    /// Convenient method to combine flags.
    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::BufferCreateFlags::empty(), |acc, flag| {
            match *flag {
                | BufferCreateFlag::SparseBindingBit => acc | vk::BUFFER_CREATE_SPARSE_BINDING_BIT,
                | BufferCreateFlag::SparseResidency  => acc | vk::BUFFER_CREATE_SPARSE_RESIDENCY_BIT,
                | BufferCreateFlag::SparseAliased    => acc | vk::BUFFER_CREATE_SPARSE_ALIASED_BIT,
            }
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferUsage {
    /// TransferSrcBit specifies that the buffer can be used as the source of a transfer command.
    ///
    ///  (see the definition of VK_PIPELINE_STAGE_TRANSFER_BIT).
    TransferSrcBit,
    /// TransferDstBit specifies that the buffer can be used as the destination of a transfer command,
    TransferDstBit,
    /// UniformTexelBufferBit specifies that the buffer can be used to create a VkBufferView suitable
    /// for occupying a VkDescriptorSet slot of type VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER.
    UniformTexelBufferBit,
    /// StorageTexelBufferBit specifies that the buffer can be used to create a VkBufferView suitable
    /// for occupying a VkDescriptorSet slot of type VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER.
    StorageTexelBufferBit,
    /// UniformBufferBit specifies that the buffer can be used in a VkDescriptorBufferInfo suitable
    /// for occupying a VkDescriptorSet slot either of type VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER or
    /// VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC.
    UniformBufferBit,
    /// StorageBufferBit specifies that the buffer can be used in a VkDescriptorBufferInfo suitable
    /// for occupying a VkDescriptorSet slot either of type VK_DESCRIPTOR_TYPE_STORAGE_BUFFER or
    /// VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC.
    StorageBufferBit,
    /// IndexBufferBit specifies that the buffer is suitable for passing as the buffer parameter to vkCmdBindIndexBuffer.
    IndexBufferBit,
    /// VertexBufferBit specifies that the buffer is suitable for passing as an element of the pBuffers array to vkCmdBindVertexBuffers.
    VertexBufferBit,
    /// IndirectBufferBit specifies that the buffer is suitable for passing as the buffer parameter to vkCmdDrawIndirect,
    /// vkCmdDrawIndexedIndirect, or vkCmdDispatchIndirect.
    IndirectBufferBit,
}
impl BufferUsage {

    pub(super) fn value(&self) -> vk::BufferUsageFlags {
        match *self {
            | BufferUsage::TransferSrcBit        => vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
            | BufferUsage::TransferDstBit        => vk::BUFFER_USAGE_TRANSFER_DST_BIT,
            | BufferUsage::UniformTexelBufferBit => vk::BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT,
            | BufferUsage::StorageTexelBufferBit => vk::BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT,
            | BufferUsage::UniformBufferBit      => vk::BUFFER_USAGE_UNIFORM_BUFFER_BIT,
            | BufferUsage::StorageBufferBit      => vk::BUFFER_USAGE_STORAGE_BUFFER_BIT,
            | BufferUsage::IndexBufferBit        => vk::BUFFER_USAGE_INDEX_BUFFER_BIT,
            | BufferUsage::VertexBufferBit       => vk::BUFFER_USAGE_VERTEX_BUFFER_BIT,
            | BufferUsage::IndirectBufferBit     => vk::BUFFER_USAGE_INDIRECT_BUFFER_BIT,
        }
    }
}
