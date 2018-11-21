
use ash::vk;

use descriptor::HaDescriptorSet;
use types::vkuint;

#[derive(Debug, Clone, Default)]
pub struct DescriptorSetEntity {

    pub(crate) handle: vk::DescriptorSet,
    binding_indices: Vec<vkuint>,
}

impl DescriptorSetEntity {

    pub fn new(set: &HaDescriptorSet, binding_indices: Vec<vkuint>) -> DescriptorSetEntity {

        DescriptorSetEntity {
            handle: set.handle,
            binding_indices,
        }
    }
}
