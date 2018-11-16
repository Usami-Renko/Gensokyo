
use ash::vk;

use utils::marker::VulkanFlags;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageUsageFlag {

    /// TransferSrcBit specifies that the image can be used as the source of a transfer command.
    #[allow(dead_code)]
    TransferSrcBit,
    /// TransferDstBit specifies that the image can be used as the destination of a transfer command.
    TransferDstBit,
    /// SampledBit specifies that the image can be used to create a vk::ImageView suitable for occupying a vk::DescriptorSet slot either of type DescriptorType::SampledImage or DescriptorType::CombinedImageSampler, and be sampled by a shader.
    #[allow(dead_code)]
    SampledBit,
    /// StorageBit specifies that the image can be used to create a vk::ImageView suitable for occupying a vk::DescriptorSet slot of type DescriptorType::StorageImage.
    #[allow(dead_code)]
    StorageBit,
    /// ColorAttachmentBit specifies that the image can be used to create a vk::ImageView suitable for use as a color
    /// or resolve attachment in a vk::Framebuffer.
    #[allow(dead_code)]
    ColorAttachmentBit,
    /// DepthStencilAttachmentBit specifies that the image can be used to create a vk::ImageView suitable for use as a depth/stencil attachment in a vk::Framebuffer.
    #[allow(dead_code)]
    DepthStencilAttachmentBit,
    /// TransientAttachmentBit specifies that the memory bound to this image will have been allocated with the MemoryPropertyFlag::LazilyAllocatedBit.
    ///
    /// This bit can be set for any image that can be used to create a vk::ImageView suitable for use as a color, resolve, depth/stencil, or input attachment.
    #[allow(dead_code)]
    TransientAttachmentBit,
    /// InputAttachmentBit specifies that the image can be used to create a vk::ImageView suitable for occupying vk::DescriptorSet slot of type DescriptorType::InputAttachment; be read from a shader as an input attachment; and be used as an input attachment in a framebuffer.
    #[allow(dead_code)]
    InputAttachmentBit,
}

impl VulkanFlags for [ImageUsageFlag] {
    type FlagType = vk::ImageUsageFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::ImageUsageFlags::empty(), |acc, flag| {
            match flag {
                | ImageUsageFlag::TransferSrcBit            => acc | vk::IMAGE_USAGE_TRANSFER_SRC_BIT,
                | ImageUsageFlag::TransferDstBit            => acc | vk::IMAGE_USAGE_TRANSFER_DST_BIT,
                | ImageUsageFlag::SampledBit                => acc | vk::IMAGE_USAGE_SAMPLED_BIT,
                | ImageUsageFlag::StorageBit                => acc | vk::IMAGE_USAGE_STORAGE_BIT,
                | ImageUsageFlag::ColorAttachmentBit        => acc | vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
                | ImageUsageFlag::DepthStencilAttachmentBit => acc | vk::IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
                | ImageUsageFlag::TransientAttachmentBit    => acc | vk::IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
                | ImageUsageFlag::InputAttachmentBit        => acc | vk::IMAGE_USAGE_INPUT_ATTACHMENT_BIT,
            }
        })
    }
}

// TODO: Turn this flags into different type of HaImageObj.
// TODO: Some enum is not available in ash crate yet.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageCreateFlag {

    /// SparseBindingBit specifies that the image will be backed using sparse memory binding.
    SparseBindingBit,
    /// SparseResidencyBit specifies that the image can be partially backed using sparse memory binding.
    ///
    /// Images created with this flag must also be created with the ImageCreateFlag::SparseBindingBit flag.
    SparseResidencyBit,
    /// SparseAliasedBit specifies that the image will be backed using sparse memory binding with memory ranges that might also simultaneously be backing another image (or another portion of the same image).
    ///
    /// Images created with this flag must also be created with the ImageCreateFlag::SparseBindingBit flag.
    SparseAliasedBit,
    /// MutableFormatBit specifies that the image can be used to create a vk::ImageView with a different format from the image.
    ///
    /// For multi-planar formats, MutableFormatBit specifies that a vk::ImageView can be created of a plane of the image.
    MutableFormatBit,
    /// CubeCompatibleBit specifies that the image can be used to create a vk::ImageView of type ImageViewType::Cube or ImageViewType::CubeArray.
    CubeCompatibleBit,


//    /// Array2DCompatibleBit specifies that the image can be used to create a vk::ImageView of type ImageViewType::2D or ImageViewType::Array2D.
//    Array2DCompatibleBit,
//    /// SplitInstanceBindRegionsBit specifies that the image can be used with a non-zero value of the splitInstanceBindRegionCount member of a vk::BindImageMemoryDeviceGroupInfo structure passed into vk::BindImageMemory2.
//    ///
//    /// This flag also has the effect of making the image use the standard sparse image block dimensions.
//    SplitInstanceBindRegionsBit,
//    /// BlockTexelViewCompatibleBit specifies that the image having a compressed format can be used to create a vk::ImageView with an uncompressed format where each texel in the image view corresponds to a compressed texel block of the image.
//    BlockTexelViewCompatibleBit,
//    /// ExtendedUsageBit specifies that the image can be created with usage flags that are not supported for the format the image is created with but are supported for at least one format a vk::ImageView created from the image can have.
//    ExtendedUsageBit,
//    /// DisjointBit specifies that an image with a multi-planar format must have each plane separately bound to memory, rather than having a single memory binding for the whole image;
//    ///
//    /// the presence of this bit distinguishes a disjoint image from an image without this bit set.
//    DisjointBit,
//    /// AliasBit specifies that two images created with the same creation parameters and aliased to the same memory can interpret the contents of the memory consistently with each other, subject to the rules described in the Memory Aliasing section.
//    ///
//    /// This flag further specifies that each plane of a disjoint image can share an in-memory non-linear representation with
//    /// single-plane images, and that a single-plane image can share an in-memory non-linear representation with a plane of
//    /// a multi-planar disjoint image, according to the rules in Compatible formats of planes of multi-planar formats.
//    ///
//    /// If the pNext chain includes a vk::ExternalMemoryImageCreateInfo structure whose handleTypes member is not 0, it is as if AliasBit is set.
//    AliasBit,
}

impl VulkanFlags for [ImageCreateFlag] {
    type FlagType = vk::ImageCreateFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::ImageCreateFlags::empty(), |acc, flag| {
            match flag {
                | ImageCreateFlag::SparseBindingBit            => acc | vk::IMAGE_CREATE_SPARSE_BINDING_BIT,
                | ImageCreateFlag::SparseResidencyBit          => acc | vk::IMAGE_CREATE_SPARSE_RESIDENCY_BIT,
                | ImageCreateFlag::SparseAliasedBit            => acc | vk::IMAGE_CREATE_SPARSE_ALIASED_BIT,
                | ImageCreateFlag::MutableFormatBit            => acc | vk::IMAGE_CREATE_MUTABLE_FORMAT_BIT,
                | ImageCreateFlag::CubeCompatibleBit           => acc | vk::IMAGE_CREATE_CUBE_COMPATIBLE_BIT,
                // | ImageCreateFlag::Array2DCompatibleBit        => acc | vk::IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT,
                // | ImageCreateFlag::SplitInstanceBindRegionsBit => acc | vk::IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT,
                // | ImageCreateFlag::BlockTexelViewCompatibleBit => acc | vk::IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT,
                // | ImageCreateFlag::ExtendedUsageBit            => acc | vk::IMAGE_CREATE_EXTENDED_USAGE_BIT,
                // | ImageCreateFlag::DisjointBit                 => acc | vk::IMAGE_CREATE_DISJOIN_BIT,
                // | ImageCreateFlag::AliasBit                    => acc | vk::IMAGE_CREATE_ALIAS_BIT,
            }
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageAspectFlag {

    /// ColorBit specifies the color aspect.
    ColorBit,
    /// DepthBit specifies the depth aspect.
    DepthBit,
    /// StencilBit specifies the stencil aspect.
    StencilBit,
    /// MetadataBit specifies the metadata aspect, used for sparse resource operations.
    MetadataBit,
    // Plane0Bit,
    // Plane1Bit,
    // Plane2Bit,
}

impl VulkanFlags for [ImageAspectFlag] {
    type FlagType = vk::ImageAspectFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::ImageAspectFlags::empty(), |acc, flag| {
            match flag {
                | ImageAspectFlag::ColorBit    => acc | vk::IMAGE_ASPECT_COLOR_BIT,
                | ImageAspectFlag::DepthBit    => acc | vk::IMAGE_ASPECT_DEPTH_BIT,
                | ImageAspectFlag::StencilBit  => acc | vk::IMAGE_ASPECT_STENCIL_BIT,
                | ImageAspectFlag::MetadataBit => acc | vk::IMAGE_ASPECT_METADATA_BIT,
            }
        })
    }
}
