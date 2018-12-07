
use ash::vk;

use crate::descriptor::GsDescriptorSet;
use crate::types::vkuint;

#[derive(Debug, Clone, Default)]
pub struct DescriptorSetEntity {

    pub(crate) handle: vk::DescriptorSet,
    binding_indices: Vec<vkuint>,
}

impl DescriptorSetEntity {

    pub fn new(set: &GsDescriptorSet, binding_indices: Vec<vkuint>) -> DescriptorSetEntity {

        DescriptorSetEntity {
            handle: set.handle,
            binding_indices,
        }
    }
}
