
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::descriptor::binding::DescriptorBindingInfo;
use resources::error::DescriptorError;

use utils::types::vkint;
use utils::marker::{ VulkanFlags, VulkanEnum, Handles };

use std::ptr;

pub struct DescriptorSetLayoutInfo {

    flags   : vk::DescriptorSetLayoutCreateFlags,
    bindings: Vec<vk::DescriptorSetLayoutBinding>,
}

impl DescriptorSetLayoutInfo {

    pub fn setup(flags: vk::DescriptorSetLayoutCreateFlags) -> DescriptorSetLayoutInfo {
        DescriptorSetLayoutInfo {
            flags,
            bindings: vec![],
        }
    }

    pub fn add_binding(&mut self, info: &Box<DescriptorBindingInfo>, stages: vk::ShaderStageFlags) -> usize {

        let binding_contnet = info.binding_content();

        let binding = vk::DescriptorSetLayoutBinding {
            // binding is the binding number of this entry and corresponds to a resource of the same binding number in the shader stages.
            binding: binding_contnet.binding,
            // desc_type specifyies which type of resource descriptors are used for this binding.
            descriptor_type : binding_contnet.descriptor_type.value(),
            // descriptor_count is the number of descriptors contained in the binding, accessed in a shader as an array.
            // If descriptor_count is zero, this binding entry is reserved and the resource must not be accessed from any stage via this binding within any pipeline using the set layout.
            descriptor_count: binding_contnet.count,
            // stage_flags specifying which pipeline shader stages can access a resource for this binding.
            // ShaderStageType::AllStage is a shorthand specifying that all defined shader stages,
            // including any additional stages defined by extensions, can access the resource.
            stage_flags: stages,
            // TODO: Add configuration for this field.
            p_immutable_samplers: ptr::null(),
        };

        let binding_index = self.bindings.len();
        self.bindings.push(binding);

        binding_index
    }

    pub fn build(&self, device: &HaDevice) -> Result<HaDescriptorSetLayout, DescriptorError> {

        let info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DescriptorSetLayoutCreateInfo,
            p_next: ptr::null(),
            flags : self.flags,
            binding_count: self.bindings.len() as vkint,
            p_bindings   : self.bindings.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_descriptor_set_layout(&info, None)
                .or(Err(DescriptorError::SetLayoutCreationError))?
        };

        let set_layout = HaDescriptorSetLayout {
            handle,
        };
        Ok(set_layout)
    }
}

#[derive(Debug, Clone)]
pub struct HaDescriptorSetLayout {

    pub(crate) handle: vk::DescriptorSetLayout,
}

impl HaDescriptorSetLayout {

    pub fn unset() -> HaDescriptorSetLayout {

        HaDescriptorSetLayout {
            handle: vk::DescriptorSetLayout::null(),
        }
    }

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_descriptor_set_layout(self.handle, None);
        }
    }
}

impl Handles for [HaDescriptorSetLayout] {
    type HandleType = vk::DescriptorSetLayout;

    #[inline]
    fn handles(&self) -> Vec<Self::HandleType> {
        self.iter().map(|c| c.handle).collect()
    }
}

// TODO: Some enum is not available in ash crate yet.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DescriptorSetLayoutFlag {
//    /// PushDescriptorBitKHR specifies that descriptor sets must not be allocated using this layout, and descriptors are instead pushed by vkCmdPushDescriptorSetKHR.
//    PushDescriptorBitKHR,
}

impl VulkanFlags for [DescriptorSetLayoutFlag] {
    type FlagType = vk::DescriptorSetLayoutCreateFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::DescriptorSetLayoutCreateFlags::empty(), |acc, _flag| {
//            match *flag {
//                | DescriptorSetLayoutFlag::PushDescriptorBitKHR => acc | vk::DESCRIPTOR_SET_LAYOUT_PUSH_DESCRIPTOR_BIT_KHR,
//            }
            acc
        })
    }
}

pub trait ToDescriptorSetLayout {

    fn to_set_layout(&self) -> &HaDescriptorSetLayout;
}