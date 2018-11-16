
use ash::vk;

use utils::marker::VulkanEnum;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HaDescriptorType {

    Image(ImageDescriptorType),
    Buffer(BufferDescriptorType),
}

impl VulkanEnum for HaDescriptorType {
    type EnumType = vk::DescriptorType;

    fn value(&self) -> Self::EnumType {
        match self {
            | HaDescriptorType::Image(i)  => i.value(),
            | HaDescriptorType::Buffer(b) => b.value(),
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
