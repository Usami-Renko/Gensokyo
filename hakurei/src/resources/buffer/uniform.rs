
use vk::resources::buffer::BufferItem;
use vk::resources::buffer::{ BufferCreateFlag, BufferUsageFlag };
use vk::resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use vk::resources::buffer::{ BufferCopiable, BufferCopyInfo };

use vk::resources::descriptor::DescriptorBufferBindableTarget;
use vk::resources::descriptor::{ DescriptorBindingContent, DescriptorBufferBindingInfo, DescriptorImageBindingInfo };
use vk::resources::descriptor::{ HaDescriptorType, BufferDescriptorType };

use resources::buffer::BufferBranch;
use resources::allocator::buffer::BufferInfosAllocatable;

use vk::utils::types::{ vkint, vkMemorySize };

#[derive(Debug, Clone)]
pub struct UniformBlockInfo {

    flags: Vec<BufferCreateFlag>,

    binding: DescriptorBindingContent,
    element_size: vkMemorySize,
}

impl UniformBlockInfo {

    pub fn new(binding: vkint, count: vkint, element_size: vkMemorySize) -> UniformBlockInfo {
        UniformBlockInfo {
            binding: DescriptorBindingContent {
                binding, count,
                descriptor_type: HaDescriptorType::Buffer(BufferDescriptorType::UniformBuffer),
            },
            element_size,
            flags: vec![],
        }
    }

    pub fn add_flag(&mut self, flag: BufferCreateFlag) {
        self.flags.push(flag);
    }
}

impl BufferBlockInfo for UniformBlockInfo {

    fn create_flags(&self) -> &[BufferCreateFlag] {
        &self.flags
    }

    fn usage_flags(&self) -> &[BufferUsageFlag] {
        &[BufferUsageFlag::UniformBufferBit]
    }

    fn estimate_size(&self) -> vkMemorySize {
        self.element_size * (self.binding.count as vkMemorySize)
    }
}

impl BufferInfosAllocatable for UniformBlockInfo {

    fn branch_type(&self) -> BufferBranch {
        BufferBranch::Uniform
    }
}


pub struct HaUniformBlock {

    binding: DescriptorBindingContent,

    item: BufferItem,
    repository_index: usize,
    element_size: vkMemorySize,
}

impl HaUniformBlock {

    pub fn uninitialize() -> HaUniformBlock {

        unimplemented!()
    }

    pub(super) fn new(info: &UniformBlockInfo, item: BufferItem, repository_index: usize) -> HaUniformBlock {

        HaUniformBlock {
            binding: info.binding.clone(),
            element_size: info.element_size,
            item,
            repository_index,
        }
    }
}

impl DescriptorBufferBindableTarget for HaUniformBlock {

    fn binding_info(&self, sub_block_indices: Option<Vec<vkint>>) -> DescriptorBufferBindingInfo {

        DescriptorBufferBindingInfo {
            content: self.binding.clone(),
            element_indices: sub_block_indices.unwrap_or(vec![0]),
            element_size: self.element_size,
            buffer: &self.item,
        }
    }
}

impl BufferBlockEntity for HaUniformBlock {

    fn item(&self) -> &BufferItem {
        &self.item
    }

    fn offset(&self, sub_index: usize) -> vkMemorySize {
        self.element_size * (sub_index as vkMemorySize)
    }
}

impl BufferCopiable for HaUniformBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.item, 0, self.item.size)
    }
}
