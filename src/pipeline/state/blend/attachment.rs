
use ash::vk;
use ash::vk::Bool32;

use utility::marker::VulkanFlags;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlendAttachmentPrefab {
    Disable,
}
impl BlendAttachmentPrefab {
    fn generate(&self) -> BlendAttachemnt {
        match *self {
            | BlendAttachmentPrefab::Disable => BlendAttachemnt {
                enable: vk::VK_FALSE,
                ..Default::default()
            }
        }
    }
}

pub struct BlendAttachemnt {

    // TODO: Add explaination for each field
    enable: Bool32,
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
            enable: if enable { vk::VK_TRUE } else { vk::VK_FALSE },
            ..Default::default()
        }
    }

    pub fn setup(prefab: BlendAttachmentPrefab) -> BlendAttachemnt {
        prefab.generate()
    }

    pub fn state(&self) -> vk::PipelineColorBlendAttachmentState {
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
        self.enable = if enable { 1 } else { 0 };
    }
    pub fn set_color_blend(&mut self, src_factor: vk::BlendFactor, op: vk::BlendOp, dst_factor: vk::BlendFactor) {
        self.src_color_factor = src_factor;
        self.color_op = op;
        self.dst_color_factor = dst_factor;
    }
    pub fn set_alpha_blend(&mut self, src_factor: vk::BlendFactor, op: vk::BlendOp, dst_factor: vk::BlendFactor) {
        self.src_alpha_factor = src_factor;
        self.alpha_op = op;
        self.dst_alpha_factor = dst_factor;
    }
    pub fn set_color_masks(&mut self, masks: &[ColorComponentFlag]) {
        self.color_write_mask = masks.flags();
    }
}

impl Default for BlendAttachemnt {

    fn default() -> BlendAttachemnt {
        BlendAttachemnt {
            enable: vk::VK_FALSE,
            src_color_factor : vk::BlendFactor::One,
            dst_color_factor : vk::BlendFactor::Zero,
            color_op         : vk::BlendOp::Add,
            src_alpha_factor : vk::BlendFactor::One,
            dst_alpha_factor : vk::BlendFactor::Zero,
            alpha_op         : vk::BlendOp::Add,
            color_write_mask : [
                ColorComponentFlag::R,
                ColorComponentFlag::G,
                ColorComponentFlag::B,
                ColorComponentFlag::A,
            ].flags(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColorComponentFlag { R, G, B, A }
impl VulkanFlags for [ColorComponentFlag] {
    type FlagType = vk::ColorComponentFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::ColorComponentFlags::empty(), |acc, flag| {
            match *flag {
                | ColorComponentFlag::R => acc | vk::COLOR_COMPONENT_R_BIT,
                | ColorComponentFlag::G => acc | vk::COLOR_COMPONENT_G_BIT,
                | ColorComponentFlag::B => acc | vk::COLOR_COMPONENT_B_BIT,
                | ColorComponentFlag::A => acc | vk::COLOR_COMPONENT_A_BIT,
            }
        })
    }
}
