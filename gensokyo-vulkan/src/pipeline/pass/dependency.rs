
use ash::vk;

use crate::types::vkuint;

/// Using sub pass dependencies also adds implicit layout transitions for the attachment used.
/// So we don't need to add explicit image memory barriers to transform them.
pub struct RenderDependencyCI(vk::SubpassDependency);

impl RenderDependencyCI {

    /// `src_subpass` is the subpass index of the first subpass in the dependency, or vk::SUBPASS_EXTERNAL.
    ///
    /// `dst_subpass` is the subpass index of the second subpass in the dependency, or vk::SUBPASS_EXTERNAL.
    pub fn new(src_subpass: SubpassStage, dst_subpass: SubpassStage) -> RenderDependencyCI {

        let dependency = vk::SubpassDependency {
            src_subpass: src_subpass.into_index(),
            dst_subpass: dst_subpass.into_index(),
            dependency_flags: vk::DependencyFlags::empty(),
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::empty(),
        };

        RenderDependencyCI(dependency)
    }

    // TODO: Add configuration for vk::DependencyFlags.
    /// `flags` specifying how execution and memory dependencies are formed.
    pub fn with_flags(mut self, flags: vk::DependencyFlags) -> RenderDependencyCI {
        self.0.dependency_flags = flags;
        self
    }

    /// `src` specifies the source stage mask.
    ///
    /// `dst` specifies the destination stage mask.
    pub fn stage(mut self, src: vk::PipelineStageFlags, dst: vk::PipelineStageFlags) -> RenderDependencyCI {
        self.0.src_stage_mask = src;
        self.0.dst_stage_mask = dst;
        self
    }

    /// `src` specifies the source access mask.
    ///
    /// `dst` specifies the destination access mask.
    pub fn access(mut self, src: vk::AccessFlags, dst: vk::AccessFlags) -> RenderDependencyCI {
        self.0.src_access_mask = src;
        self.0.dst_access_mask = dst;
        self
    }

    pub(crate) fn take(self) -> vk::SubpassDependency {
        self.0
    }
}

pub enum SubpassStage {
    BeginExternal,
    EndExternal,
    AtIndex(vkuint),
}

impl SubpassStage {

    fn into_index(self) -> vkuint {
        match self {
            | SubpassStage::BeginExternal  => vk::SUBPASS_EXTERNAL,
            | SubpassStage::AtIndex(index) => index,
            | SubpassStage::EndExternal    => vk::SUBPASS_EXTERNAL,
        }
    }
}
