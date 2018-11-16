
use ash::vk;
use ash::vk::Bool32;

use utils::marker::{ VulkanFlags, VulkanEnum, Prefab };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlendAttachmentPrefab {

    Disable,
}

impl Prefab for BlendAttachmentPrefab {
    type PrefabType = BlendAttachemnt;

    fn generate(&self) -> Self::PrefabType {
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


// TODO: Add description for BlendFactor.
/// The source and destination color and alpha blending factors are selected from this enum.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlendFactor {

    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
    Src1Color,
    OneMinusSrc1Color,
    Src1Alpha,
    OneMinusSrc1Alpha,
}

impl VulkanEnum for BlendFactor {
    type EnumType = vk::BlendFactor;

    fn value(&self) -> Self::EnumType {
        match self {
            | BlendFactor::Zero                  => vk::BlendFactor::Zero,
            | BlendFactor::One                   => vk::BlendFactor::One,
            | BlendFactor::SrcColor              => vk::BlendFactor::SrcColor,
            | BlendFactor::OneMinusSrcColor      => vk::BlendFactor::OneMinusSrcColor,
            | BlendFactor::DstColor              => vk::BlendFactor::DstColor,
            | BlendFactor::OneMinusDstColor      => vk::BlendFactor::OneMinusDstColor,
            | BlendFactor::SrcAlpha              => vk::BlendFactor::SrcAlpha,
            | BlendFactor::OneMinusSrcAlpha      => vk::BlendFactor::OneMinusSrcAlpha,
            | BlendFactor::DstAlpha              => vk::BlendFactor::DstAlpha,
            | BlendFactor::OneMinusDstAlpha      => vk::BlendFactor::OneMinusDstAlpha,
            | BlendFactor::ConstantColor         => vk::BlendFactor::ConstantColor,
            | BlendFactor::OneMinusConstantColor => vk::BlendFactor::OneMinusConstantColor,
            | BlendFactor::ConstantAlpha         => vk::BlendFactor::ConstantAlpha,
            | BlendFactor::OneMinusConstantAlpha => vk::BlendFactor::OneMinusConstantAlpha,
            | BlendFactor::SrcAlphaSaturate      => vk::BlendFactor::SrcAlphaSaturate,
            | BlendFactor::Src1Color             => vk::BlendFactor::Src1Color,
            | BlendFactor::OneMinusSrc1Color     => vk::BlendFactor::OneMinusSrc1Color,
            | BlendFactor::Src1Alpha             => vk::BlendFactor::Src1Alpha,
            | BlendFactor::OneMinusSrc1Alpha     => vk::BlendFactor::OneMinusSrc1Alpha,
        }
    }
}


// TODO: Add description for BlendFactor.
/// BlendOp define the operation in blending operation.
///
/// RGB and alpha components can use different operations in blending.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlendOp {

    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

impl VulkanEnum for BlendOp {
    type EnumType = vk::BlendOp;

    fn value(&self) -> Self::EnumType {
        match self {
            | BlendOp::Add             => vk::BlendOp::Add,
            | BlendOp::Subtract        => vk::BlendOp::Subtract,
            | BlendOp::ReverseSubtract => vk::BlendOp::ReverseSubtract,
            | BlendOp::Min             => vk::BlendOp::Min,
            | BlendOp::Max             => vk::BlendOp::Max,
        }
    }
}

