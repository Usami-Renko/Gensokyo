
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::descriptor::binding::DescriptorBindingCI;
use crate::error::{ VkResult, VkError };

use std::ptr;

pub struct DescriptorSetLayoutCI {

    flags   : vk::DescriptorSetLayoutCreateFlags,
    bindings: Vec<vk::DescriptorSetLayoutBinding>,
}

impl GsDescriptorSetLayout {

    pub fn new(binding_count: usize) -> DescriptorSetLayoutCI {
        DescriptorSetLayoutCI {
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            bindings: Vec::with_capacity(binding_count),
        }
    }
}

impl DescriptorSetLayoutCI {

    pub fn add_binding(&mut self, info: &impl DescriptorBindingCI, stages: vk::ShaderStageFlags) {

        let meta = info.meta_mirror();

        let binding = vk::DescriptorSetLayoutBinding {
            // binding is the binding number of this entry and corresponds to a resource of the same binding number in the shader stages.
            binding: meta.binding,
            // desc_type specifies which type of resource descriptors are used for this binding.
            descriptor_type: meta.descriptor_type.into(),
            // descriptor_count is the number of descriptors contained in the binding, accessed in a shader as an array.
            // If descriptor_count is zero, this binding entry is reserved and the resource must not be accessed from any stage via this binding within any pipeline using the set layout.
            descriptor_count: meta.count,
            // stage_flags specifying which pipeline shader stages can access a resource for this binding.
            // ShaderStageType::AllStage is a shorthand specifying that all defined shader stages,
            // including any additional stages defined by extensions, can access the resource.
            stage_flags: stages,
            // TODO: Add configuration for this field.
            p_immutable_samplers: ptr::null(),
        };

        self.bindings.push(binding);
    }

    pub fn set_flags(&mut self, flags: vk::DescriptorSetLayoutCreateFlags) {
        self.flags = flags;
    }

    pub fn build(&self, device: &GsDevice) -> VkResult<GsDescriptorSetLayout> {

        let layout_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags : self.flags,
            binding_count: self.bindings.len() as _,
            p_bindings   : self.bindings.as_ptr(),
        };

        let handle = unsafe {
            device.logic.handle.create_descriptor_set_layout(&layout_info, None)
                .or(Err(VkError::create("Descriptor Set Layout")))?
        };

        let set_layout = GsDescriptorSetLayout { handle };
        Ok(set_layout)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GsDescriptorSetLayout {

    pub(crate) handle: vk::DescriptorSetLayout,
}

impl GsDescriptorSetLayout {

    pub fn destroy(&self, device: &GsDevice) {
        unsafe {
            device.logic.handle.destroy_descriptor_set_layout(self.handle, None);
        }
    }
}
