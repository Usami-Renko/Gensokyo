
use ash::vk;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GsDescriptorType {

    Image(ImageDescriptorType),
    Buffer(BufferDescriptorType),
}

impl From<GsDescriptorType> for vk::DescriptorType {

    fn from(descriptor: GsDescriptorType) -> vk::DescriptorType {

        match descriptor {
            | GsDescriptorType::Image(i)  => i.into(),
            | GsDescriptorType::Buffer(b) => b.into(),
        }
    }
}

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

impl From<ImageDescriptorType> for vk::DescriptorType {

    fn from(descriptor: ImageDescriptorType) -> vk::DescriptorType {

        match descriptor {
            | ImageDescriptorType::Sampler              => vk::DescriptorType::SAMPLER,
            | ImageDescriptorType::CombinedImageSampler => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            | ImageDescriptorType::SampledImage         => vk::DescriptorType::SAMPLED_IMAGE,
            | ImageDescriptorType::StorageImage         => vk::DescriptorType::STORAGE_IMAGE,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BufferDescriptorType {

    /// `TexelUniformBuffer` specifies a uniform texel buffer descriptor.
    TexelUniformBuffer,
    /// `TexelStorageBuffer` specifies a storage texel buffer descriptor.
    TexelStorageBuffer,
    /// `UniformBuffer` specifies a uniform buffer descriptor.
    UniformBuffer,
    /// `StorageBuffer` specifies a storage buffer descriptor.
    StorageBuffer,
    /// `DynamicUniformBuffer` specifies a dynamic uniform buffer descriptor.
    DynamicUniformBuffer,
    /// `DynamicStorageBuffer` specifies a dynamic storage buffer descriptor.
    DynamicStorageBuffer,
    /// `InputAttachment` specifies a input attachment descriptor.
    InputAttachment,
}

impl From<BufferDescriptorType> for vk::DescriptorType {

    fn from(descriptor: BufferDescriptorType) -> vk::DescriptorType {

        match descriptor {
            | BufferDescriptorType::TexelUniformBuffer => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
            | BufferDescriptorType::TexelStorageBuffer => vk::DescriptorType::STORAGE_TEXEL_BUFFER,
            | BufferDescriptorType::UniformBuffer        => vk::DescriptorType::UNIFORM_BUFFER,
            | BufferDescriptorType::StorageBuffer        => vk::DescriptorType::STORAGE_BUFFER,
            | BufferDescriptorType::DynamicUniformBuffer => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            | BufferDescriptorType::DynamicStorageBuffer => vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            | BufferDescriptorType::InputAttachment      => vk::DescriptorType::INPUT_ATTACHMENT,
        }
    }
}
