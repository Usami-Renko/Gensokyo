
use ash::vk;

use types::vkuint;

/// Using sub pass dependencies also adds implicit layout transitions for the attachment used.
/// So we don't need to add explicit image memory barriers to transform them.
pub struct RenderDependency(vk::SubpassDependency);

impl RenderDependency {

    /// `src_subpass` is the subpass index of the first subpass in the dependency, or vk::SUBPASS_EXTERNAL.
    ///
    /// `dst_subpass` is the subpass index of the second subpass in the dependency, or vk::SUBPASS_EXTERNAL.
    pub fn setup(src_subpass: vkuint, dst_subpass: vkuint) -> RenderDependency {

        let dependency = vk::SubpassDependency {
            src_subpass,
            dst_subpass,
            dependency_flags: vk::DependencyFlags::empty(),
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::empty(),
        };

        RenderDependency(dependency)
    }

    // TODO: Add configuration for vk::DependencyFlags.
    /// `flags` specifying how execution and memory dependencies are formed.
    pub fn with_flags(mut self, flags: vk::DependencyFlags) -> RenderDependency {
        self.0.dependency_flags = flags;
        self
    }

    /// `src` specifies the source stage mask.
    ///
    /// `dst` specifies the destination stage mask.
    pub fn stage(mut self, src: vk::PipelineStageFlags, dst: vk::PipelineStageFlags) -> RenderDependency {
        self.0.src_stage_mask = src;
        self.0.dst_stage_mask = dst;
        self
    }

    /// `src` specifies the source access mask.
    ///
    /// `dst` spacifies the destination access mask.
    pub fn set_access(mut self, src: vk::AccessFlags, dst: vk::AccessFlags) -> RenderDependency {
        self.0.src_access_mask = src;
        self.0.dst_access_mask = dst;
        self
    }

    pub(crate) fn build(self) -> vk::SubpassDependency {
        self.0
    }
}
