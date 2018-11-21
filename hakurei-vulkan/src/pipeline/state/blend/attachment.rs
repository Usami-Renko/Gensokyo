
use ash::vk;

use types::{ vkbool, VK_TRUE, VK_FALSE };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlendAttachmentPrefab {

    Disable,
}

impl BlendAttachmentPrefab {

    fn generate(&self) -> BlendAttachemnt {
        match self {
            | BlendAttachmentPrefab::Disable => BlendAttachemnt {
                enable: VK_FALSE,
                ..Default::default()
            }
        }
    }
}

pub struct BlendAttachemnt {

    // TODO: Add explaination for each field
    enable: vkbool,
    src_color_factor : vk::BlendFactor,
    dst_color_factor : vk::BlendFactor,
    color_op         : vk::BlendOp,
    src_alpha_factor : vk::BlendFactor,
    dst_alpha_factor : vk::BlendFactor,
    alpha_op         : vk::BlendOp,
    /// Color write mask determine whether the final color values R, G, B and A are written to the framebuffer attachment.
    color_write_mask : vk::ColorComponentFlags,
}

impl BlendAttachemnt {

    pub fn init(enable: bool) -> BlendAttachemnt {

        BlendAttachemnt {
            enable: if enable { VK_TRUE } else { VK_FALSE },
            ..Default::default()
        }
    }

    pub fn setup(prefab: BlendAttachmentPrefab) -> BlendAttachemnt {
        prefab.generate()
    }

    pub(crate) fn state(&self) -> vk::PipelineColorBlendAttachmentState {

        vk::PipelineColorBlendAttachmentState {
            blend_enable: self.enable,
            src_color_blend_factor : self.src_color_factor,
            dst_color_blend_factor : self.dst_color_factor,
            color_blend_op         : self.color_op,
            src_alpha_blend_factor : self.src_alpha_factor,
            dst_alpha_blend_factor : self.dst_alpha_factor,
            alpha_blend_op         : self.alpha_op,
            color_write_mask       : self.color_write_mask,
        }
    }

    pub fn set_enable(&mut self, enable: bool) {
        self.enable = if enable { VK_TRUE } else { VK_FALSE };
    }
    pub fn set_color_blend(&mut self, op: vk::BlendOp, src_factor: vk::BlendFactor, dst_factor: vk::BlendFactor) {
        self.color_op = op;
        self.src_color_factor = src_factor;
        self.dst_color_factor = dst_factor;
    }
    pub fn set_alpha_blend(&mut self, op: vk::BlendOp, src_factor: vk::BlendFactor, dst_factor: vk::BlendFactor) {
        self.alpha_op = op;
        self.src_alpha_factor = src_factor;
        self.dst_alpha_factor = dst_factor;
    }
    // TODO: Add configuration for vk::ColorComponentFlag.
    pub fn set_color_masks(&mut self, masks: vk::ColorComponentFlags) {
        self.color_write_mask = masks;
    }
}

impl Default for BlendAttachemnt {

    fn default() -> BlendAttachemnt {

        BlendAttachemnt {
            enable: VK_FALSE,
            src_color_factor : vk::BlendFactor::ONE,
            dst_color_factor : vk::BlendFactor::ZERO,
            color_op         : vk::BlendOp::ADD,
            src_alpha_factor : vk::BlendFactor::ONE,
            dst_alpha_factor : vk::BlendFactor::ZERO,
            alpha_op         : vk::BlendOp::ADD,
            color_write_mask :
                vk::ColorComponentFlags::R |
                vk::ColorComponentFlags::G |
                vk::ColorComponentFlags::B |
                vk::ColorComponentFlags::A,
        }
    }
}
