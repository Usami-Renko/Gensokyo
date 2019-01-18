
use ash::vk;

use crate::types::{ VK_TRUE, VK_FALSE };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlendAttachmentPrefab {

    Disable,
}

impl BlendAttachmentPrefab {

    fn generate(&self) -> BlendAttachment {
        match self {
            | BlendAttachmentPrefab::Disable => BlendAttachment::init(false),
        }
    }
}

pub struct BlendAttachment(vk::PipelineColorBlendAttachmentState);

impl BlendAttachment {

    pub fn init(enable: bool) -> BlendAttachment {

        let mut attachment = BlendAttachment::default();
        attachment.0.blend_enable = if enable { VK_TRUE } else { VK_FALSE };

        attachment
    }

    pub fn setup(prefab: BlendAttachmentPrefab) -> BlendAttachment {
        prefab.generate()
    }

    pub(crate) fn take(self) -> vk::PipelineColorBlendAttachmentState {
        self.0
    }

    pub fn set_enable(&mut self, enable: bool) {
        self.0.blend_enable = if enable { VK_TRUE } else { VK_FALSE };
    }
    pub fn with_color_blend(&mut self, op: vk::BlendOp, src_factor: vk::BlendFactor, dst_factor: vk::BlendFactor) {
        self.0.color_blend_op = op;
        self.0.src_color_blend_factor = src_factor;
        self.0.dst_color_blend_factor = dst_factor;
    }
    pub fn with_alpha_blend(&mut self, op: vk::BlendOp, src_factor: vk::BlendFactor, dst_factor: vk::BlendFactor) {
        self.0.alpha_blend_op = op;
        self.0.src_alpha_blend_factor = src_factor;
        self.0.dst_alpha_blend_factor = dst_factor;
    }
    /// Color write mask determine whether the final color values R, G, B and A are written to the framebuffer attachment.
    pub fn with_color_masks(&mut self, masks: vk::ColorComponentFlags) {
        self.0.color_write_mask = masks;
    }
}

impl Default for BlendAttachment {

    fn default() -> BlendAttachment {

        let attachment = vk::PipelineColorBlendAttachmentState {
            blend_enable: VK_FALSE,
            src_color_blend_factor : vk::BlendFactor::ONE,
            dst_color_blend_factor : vk::BlendFactor::ZERO,
            color_blend_op         : vk::BlendOp::ADD,
            src_alpha_blend_factor : vk::BlendFactor::ONE,
            dst_alpha_blend_factor : vk::BlendFactor::ZERO,
            alpha_blend_op         : vk::BlendOp::ADD,
            color_write_mask       : vk::ColorComponentFlags::R
                                   | vk::ColorComponentFlags::G
                                   | vk::ColorComponentFlags::B
                                   | vk::ColorComponentFlags::A
        };
        BlendAttachment(attachment)
    }
}
