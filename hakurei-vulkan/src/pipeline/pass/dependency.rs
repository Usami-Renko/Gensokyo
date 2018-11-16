
use ash::vk;

use pipeline::stages::PipelineStageFlag;

use utils::types::vkint;
use utils::marker::{ VulkanFlags, VulkanEnum };

pub enum RenderDependencyPrefab {
    Common,
}

impl RenderDependencyPrefab {
    fn generate(&self) -> RenderDependency {
        match self {
            | RenderDependencyPrefab::Common => RenderDependency {
                ..Default::default()
            }
        }
    }
}

/// Using sub pass dependencies also adds implicit layout transitions for the attachment used.
/// So we don't need to add explicit image memory barriers to transform them
pub struct RenderDependency {

    flags: Vec<DependencyFlag>,

    /// src_subpass is the subpass index of the first subpass in the dependency, or vk::SUBPASS_EXTERNAL.
    src_subpass: vkint,
    /// dst_subpass is the subpass index of the second subpass in the dependency, or vk::SUBPASS_EXTERNAL.
    dst_subpass: vkint,

    /// src_stage specifies the source stage mask.
    src_stage: PipelineStageFlag,
    /// dst_stage specifies the destination stage mask.
    dst_stage: PipelineStageFlag,

    /// src_access specifies the source access mask.
    src_access: Vec<AccessFlag>,
    /// dst_access spacifies the destination access mask.
    dst_access: Vec<AccessFlag>,
}

impl RenderDependency {

    pub fn setup(prefab: RenderDependencyPrefab, src_subpass: vkint, dst_subpass: vkint) -> RenderDependency {

        RenderDependency {
            src_subpass,
            dst_subpass,
            ..prefab.generate()
        }
    }

    pub(super) fn desc(&self) -> vk::SubpassDependency {

        vk::SubpassDependency {
            dependency_flags: self.flags.flags(),
            src_subpass     : self.src_subpass,
            dst_subpass     : self.dst_subpass,
            src_stage_mask  : self.src_stage.value(),
            dst_stage_mask  : self.dst_stage.value(),
            src_access_mask : self.src_access.flags(),
            dst_access_mask : self.dst_access.flags(),
        }
    }

    pub fn set_flags(&mut self, flags: &[DependencyFlag]) {
        self.flags = flags.to_vec();
    }
    pub fn set_stage(&mut self, src: PipelineStageFlag, dst: PipelineStageFlag) {
        self.src_stage = src;
        self.dst_stage = dst;
    }
    pub fn set_access(&mut self, src: &[AccessFlag], dst: &[AccessFlag]) {
        self.src_access = src.to_vec();
        self.dst_access = dst.to_vec();
    }
}

impl Default for RenderDependency {

    fn default() -> RenderDependency {

        RenderDependency {
            flags: vec![],
            src_subpass: 0,
            dst_subpass: 0,
            src_stage: PipelineStageFlag::ColorAttachmentOutputBit,
            dst_stage: PipelineStageFlag::ColorAttachmentOutputBit,
            src_access: vec![],
            dst_access: vec![],
        }
    }
}


// TODO: Some enum is not available in ash crate yet.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DependencyFlag {
    /// ByRegionBit specifies that dependencies will be framebuffer-local.
    ByRegionBit,
//    /// DeviceGroupBit specifies a subpass has more than one view.
//    ViewLocalBit,
//    /// DeviceGroupBit specifies dependencies are non-device-local dependency.
//    DeviceGroupBit,
}


impl VulkanFlags for [DependencyFlag] {
    type FlagType = vk::DependencyFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::DependencyFlags::empty(), |acc, flag| {
            match flag {
                | DependencyFlag::ByRegionBit    => acc | vk::DEPENDENCY_BY_REGION_BIT,
//                | DependencyFlag::ViewLocalBit   => vk::DEPENDENCY_VIEW_LOCAL_BIT,
//                | DependencyFlag::DeviceGroupBit => vk::DEPENDENCY_DEVICE_GROUP_BIT,
            }
        })
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccessFlag {

    /// IndirectCommandReadBit specifies read access to an indirect command structure read as part of an indirect drawing or dispatch command.
    IndirectCommandReadBit,
    /// IndexReadBit specifies read access to an index buffer as part of an indexed drawing command, bound by vkCmdBindIndexBuffer.
    IndexReadBit,
    /// VertexAttributeReadBit specifies read access to a vertex buffer as part of a drawing command, bound by vkCmdBindVertexBuffers.
    VertexAttributeReadBit,
    /// UniformReadBit specifies read access to a uniform buffer.
    UniformReadBit,
    /// InputAttachmentReadBit specifies read access to an input attachment within a render pass during fragment shading.
    InputAttachmentReadBit,
    /// ShaderReadBit specifies read access to a storage buffer, uniform texel buffer, storage texel buffer, sampled image, or storage image.
    ShaderReadBit,
    /// ShaderWriteBit specifies write access to a storage buffer, storage texel buffer, or storage image.
    ShaderWriteBit,
    /// ColorAttachmentReadBit specifies read access to a color attachment, such as via blending, logic operations, or via certain subpass load operations.
    ColorAttachmentReadBit,
    /// ColorAttachmentWriteBit specifies write access to a color or resolve attachment during a render pass or via certain subpass load and store operations.
    ColorAttachmentWriteBit,
    /// DepthStencilAttachmentReadBit pecifies read access to a depth/stencil attachment, via depth or stencil operations or via certain subpass load operations.
    DepthStencilAttachmentReadBit,
    /// DepthStencilAttachmentWriteBit specifies write access to a depth/stencil attachment, via depth or stencil operations or via certain subpass load and store operations.
    DepthStencilAttachmentWriteBit,
    /// TransferReadBit specifies read access to an image or buffer in a copy operation.
    TransferReadBit,
    /// TransferWriteBit specifies write access to an image or buffer in a clear or copy operation.
    TransferWriteBit,
    /// HostReadBit specifies read access by a host operation.
    ///
    /// Accesses of this type are not performed through a resource, but directly on memory.
    HostReadBit,
    /// HostWriteBit specifies write access by a host operation.
    ///
    /// Accesses of this type are not performed through a resource, but directly on memory.
    HostWriteBit,
    /// MemoryReadBit specifies read access via non-specific entities.
    ///
    /// These entities include the Vulkan device and host, but may also include entities external to the Vulkan device or otherwise not part of the core Vulkan pipeline.
    ///
    /// When included in a destination access mask, makes all available writes visible to all future read accesses on entities known to the Vulkan device.
    MemoryReadBit,
    /// MemoryWriteBit specifies write access via non-specific entities.
    ///
    /// These entities include the Vulkan device and host, but may also include entities external to the Vulkan device or otherwise not part of the core Vulkan pipeline.
    ///
    /// When included in a source access mask, all writes that are performed by entities known to the Vulkan device are made available.
    ///
    /// When included in a destination access mask, makes all available writes visible to all future write accesses on entities known to the Vulkan device.
    MemoryWriteBit,
}

impl VulkanFlags for [AccessFlag] {
    type FlagType = vk::AccessFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::AccessFlags::empty(), |acc, flag| {
            match flag {
                | AccessFlag::IndirectCommandReadBit         => acc | vk::ACCESS_INDIRECT_COMMAND_READ_BIT,
                | AccessFlag::IndexReadBit                   => acc | vk::ACCESS_INDEX_READ_BIT,
                | AccessFlag::VertexAttributeReadBit         => acc | vk::ACCESS_VERTEX_ATTRIBUTE_READ_BIT,
                | AccessFlag::UniformReadBit                 => acc | vk::ACCESS_UNIFORM_READ_BIT,
                | AccessFlag::InputAttachmentReadBit         => acc | vk::ACCESS_INPUT_ATTACHMENT_READ_BIT,
                | AccessFlag::ShaderReadBit                  => acc | vk::ACCESS_SHADER_READ_BIT,
                | AccessFlag::ShaderWriteBit                 => acc | vk::ACCESS_SHADER_WRITE_BIT,
                | AccessFlag::ColorAttachmentReadBit         => acc | vk::ACCESS_COLOR_ATTACHMENT_READ_BIT,
                | AccessFlag::ColorAttachmentWriteBit        => acc | vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                | AccessFlag::DepthStencilAttachmentReadBit  => acc | vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
                | AccessFlag::DepthStencilAttachmentWriteBit => acc | vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
                | AccessFlag::TransferReadBit                => acc | vk::ACCESS_TRANSFER_READ_BIT,
                | AccessFlag::TransferWriteBit               => acc | vk::ACCESS_TRANSFER_WRITE_BIT,
                | AccessFlag::HostReadBit                    => acc | vk::ACCESS_HOST_READ_BIT,
                | AccessFlag::HostWriteBit                   => acc | vk::ACCESS_HOST_WRITE_BIT,
                | AccessFlag::MemoryReadBit                  => acc | vk::ACCESS_MEMORY_READ_BIT,
                | AccessFlag::MemoryWriteBit                 => acc | vk::ACCESS_MEMORY_WRITE_BIT,
            }
        })
    }
}
