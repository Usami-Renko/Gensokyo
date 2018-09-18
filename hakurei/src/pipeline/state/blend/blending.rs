
use ash::vk;
use ash::vk::uint32_t;

use pipeline::state::blend::attachment::BlendAttachemnt;
use pipeline::state::blend::ops::LogicalOp;

use utility::marker::{ VulkanEnum, Prefab };

use std::ptr;
use std::os::raw::c_float;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HaBlendPrefab {
    Default,
    Unset,
}

impl Prefab for HaBlendPrefab {
    type PrefabType = HaBlend;

    fn generate(&self) -> Self::PrefabType {
        match *self {
            | HaBlendPrefab::Default => HaBlend {
                logic_op_enable: false,
                logic_op: LogicalOp::Copy.value(),
                attachments: vec![
                    BlendAttachemnt::default(),
                ],
                blend_constants: [0.0, 0.0, 0.0, 0.0],
            },
            | HaBlendPrefab::Unset => HaBlend {
                logic_op_enable: false,
                logic_op: LogicalOp::NoOp.value(),
                attachments: vec![],
                blend_constants: [0.0, 0.0, 0.0, 0.0],
            },
        }
    }
}

pub struct HaBlend {

    /// logic_op_enable indicate if use logical operation in blending.
    logic_op_enable: bool,
    /// LogicOp selects which logical operation to apply.
    logic_op: vk::LogicOp,
    /// attachments is array of per target attachment states.
    attachments: Vec<BlendAttachemnt>,
    /// Blend constants is an array of four values used as the R, G, B, and A components of the blend constant that are used in blending, depending on the blend factor.
    blend_constants: [c_float; 4],
}

impl HaBlend {

    pub fn setup(prefab: HaBlendPrefab) -> HaBlend {
        prefab.generate()
    }

    pub(crate) fn info(&self) -> vk::PipelineColorBlendStateCreateInfo {
        let attchement_infos: Vec<vk::PipelineColorBlendAttachmentState> = self.attachments.iter()
            .map(|a| a.state()).collect();

        vk::PipelineColorBlendStateCreateInfo {
            s_type : vk::StructureType::PipelineColorBlendStateCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags  : vk::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable  : if self.logic_op_enable { vk::VK_TRUE } else { vk::VK_FALSE },
            logic_op         : self.logic_op,
            attachment_count : attchement_infos.len() as uint32_t,
            p_attachments    : attchement_infos.as_ptr(),
            blend_constants  : self.blend_constants,
        }
    }

    pub fn set_logical_operation(&mut self, logic_op: LogicalOp) {
        self.logic_op = logic_op.value();
    }
    pub fn add_attachment(&mut self, attachment: BlendAttachemnt) {
        self.attachments.push(attachment);
    }
    pub fn set_blend_constants(&mut self, constants: [c_float; 4]) {
        self.blend_constants = constants;
    }
}

impl Default for HaBlend {

    fn default() -> HaBlend {
        HaBlendPrefab::Default.generate()
    }
}
