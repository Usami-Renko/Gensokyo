
use ash::vk;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HaDescriptorType {

    Image(ImageDescriptorType),
    Buffer(BufferDescriptorType),
}

impl HaDescriptorType {

    pub(crate) fn to_raw(&self) -> vk::DescriptorType {
        match self {
            | HaDescriptorType::Image(i)  => i.to_raw(),
            | HaDescriptorType::Buffer(b) => b.to_raw(),
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

impl ImageDescriptorType {

    fn to_raw(&self) -> vk::DescriptorType {
        match self {
            | ImageDescriptorType::Sampler              => vk::DescriptorType::SAMPLER,
            | ImageDescriptorType::CombinedImageSampler => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            | ImageDescriptorType::SampledImage         => vk::DescriptorType::SAMPLED_IMAGE,
            | ImageDescriptorType::StorageImage         => vk::DescriptorType::STORAGE_IMAGE,
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

impl BufferDescriptorType {

    fn to_raw(&self) -> vk::DescriptorType {
        match self {
            | BufferDescriptorType::UniformTexelBuffer   => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
            | BufferDescriptorType::StorageTexelBuffer   => vk::DescriptorType::STORAGE_TEXEL_BUFFER,
            | BufferDescriptorType::UniformBuffer        => vk::DescriptorType::UNIFORM_BUFFER,
            | BufferDescriptorType::StorageBuffer        => vk::DescriptorType::STORAGE_BUFFER,
            | BufferDescriptorType::UniformBufferDynamic => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            | BufferDescriptorType::StorageBufferDynamic => vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            | BufferDescriptorType::InputAttachment      => vk::DescriptorType::INPUT_ATTACHMENT,
        }
    }
}
