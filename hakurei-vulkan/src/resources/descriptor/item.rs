
use ash::vk;

use resources::descriptor::HaDescriptorSet;
use utils::types::vkint;

#[derive(Debug, Clone)]
pub struct DescriptorSetItem {

    pub(crate) handle: vk::DescriptorSet,
    binding_indices: Vec<vkint>,
}

impl DescriptorSetItem {

    pub fn unset() -> DescriptorSetItem {

        DescriptorSetItem {
            handle: vk::DescriptorSet::null(),
            binding_indices: vec![],
        }
    }

    pub fn from(set: &HaDescriptorSet, binding_indices: Vec<vkint>) -> DescriptorSetItem {

        DescriptorSetItem {
            handle: set.handle,
            binding_indices,
        }
    }
}
