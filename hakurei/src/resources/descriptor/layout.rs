
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::descriptor::DescriptorBindingInfo;
use resources::error::DescriptorError;

use utility::marker::{ VulkanFlags, VulkanEnum, Handles };

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

    pub(crate) fn add_binding(&mut self, info: &Box<DescriptorBindingInfo>, stages: vk::ShaderStageFlags) -> usize {

        let binding = vk::DescriptorSetLayoutBinding {
            // binding is the binding number of this entry and corresponds to a resource of the same binding number in the shader stages.
            binding: info.binding_value(),
            // desc_type specifyies which type of resource descriptors are used for this binding.
            descriptor_type : info.descriptor_type(),
            // descriptor_count is the number of descriptors contained in the binding, accessed in a shader as an array.
            // If descriptor_count is zero, this binding entry is reserved and the resource must not be accessed from any stage via this binding within any pipeline using the set layout.
            descriptor_count: info.descritpor_count(),
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

    pub(crate) fn build(&self, device: &HaLogicalDevice) -> Result<HaDescriptorSetLayout, DescriptorError> {

        let info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DescriptorSetLayoutCreateInfo,
            p_next: ptr::null(),
            flags : self.flags,
            binding_count: self.bindings.len() as uint32_t,
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

pub struct HaDescriptorSetLayout {

    pub(crate) handle: vk::DescriptorSetLayout,
}

impl HaDescriptorSetLayout {

    pub(crate) fn cleanup(&self, device: &HaLogicalDevice) {
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


// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageDescriptorType {
    /// Sampler specifies a sampler descriptor.
    Sampler,
    /// CombinedImageSampler specifies a combined image sampler descriptor.
    CombinedImageSampler,
    /// SampledImage specifies a sampled image descriptor.
    SampledImage,
    /// StorageImage specifies a storage image descriptor.
    StorageImage,
}

impl VulkanEnum for ImageDescriptorType {
    type EnumType = vk::DescriptorType;

    fn value(&self) -> Self::EnumType {
        match self {
            | ImageDescriptorType::Sampler              => vk::DescriptorType::Sampler,
            | ImageDescriptorType::CombinedImageSampler => vk::DescriptorType::CombinedImageSampler,
            | ImageDescriptorType::SampledImage         => vk::DescriptorType::SampledImage,
            | ImageDescriptorType::StorageImage         => vk::DescriptorType::StorageImage,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferDescriptorType {
    /// UniformTexelBuffer specifies a uniform texel buffer descriptor.
    UniformTexelBuffer,
    /// StorageTexelBuffer specifies a storage texel buffer descriptor.
    StorageTexelBuffer,
    /// UniformBuffer specifies a uniform buffer descriptor.
    UniformBuffer,
    /// StorageBuffer specifies a storage buffer descriptor.
    StorageBuffer,
    /// UniformBufferDynamic specifies a dynamic uniform buffer descriptor.
    UniformBufferDynamic,
    /// StorageBufferDynamic specifies a dynamic storage buffer descriptor.
    StorageBufferDynamic,
    /// InputAttachment specifies a input attachment descriptor.
    InputAttachment,
}

impl VulkanEnum for BufferDescriptorType {
    type EnumType = vk::DescriptorType;

    fn value(&self) -> Self::EnumType {
        match self {
            | BufferDescriptorType::UniformTexelBuffer   => vk::DescriptorType::UniformTexelBuffer,
            | BufferDescriptorType::StorageTexelBuffer   => vk::DescriptorType::StorageTexelBuffer,
            | BufferDescriptorType::UniformBuffer        => vk::DescriptorType::UniformBuffer,
            | BufferDescriptorType::StorageBuffer        => vk::DescriptorType::StorageBuffer,
            | BufferDescriptorType::UniformBufferDynamic => vk::DescriptorType::UniformBufferDynamic,
            | BufferDescriptorType::StorageBufferDynamic => vk::DescriptorType::StorageBufferDynamic,
            | BufferDescriptorType::InputAttachment      => vk::DescriptorType::InputAttachment,
        }
    }
}
