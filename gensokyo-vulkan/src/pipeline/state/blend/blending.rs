
use ash::vk;

use crate::pipeline::state::blend::attachment::BlendAttachment;
use crate::pipeline::state::dynamic::DynamicableValue;

use crate::types::{ vkfloat, VK_TRUE, VK_FALSE };

use std::ptr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GsBlendPrefab {

    Default,
    Unset,
}

impl GsBlendPrefab {

    fn generate(&self) -> GsBlendState {

        match self {
            | GsBlendPrefab::Default => GsBlendState {
                logic_op_enable: false,
                logic_op: vk::LogicOp::COPY,
                attachments: vec![
                    BlendAttachment::default().take(),
                ],
                blend_constants: DynamicableValue::Fixed { value: [0.0; 4] },
            },
            | GsBlendPrefab::Unset => GsBlendState {
                logic_op_enable: false,
                logic_op: vk::LogicOp::NO_OP,
                attachments: vec![],
                blend_constants: DynamicableValue::Fixed { value: [0.0; 4] },
            },
        }
    }
}

pub struct GsBlendState {

    /// logic_op_enable indicate if use logical operation in blending.
    logic_op_enable: bool,
    /// LogicOp selects which logical operation to apply.
    logic_op: vk::LogicOp,
    /// attachments is array of per target attachment states.
    attachments: Vec<vk::PipelineColorBlendAttachmentState>,
    /// Blend constants is an array of four values used as the R, G, B, and A components of the blend constant that are used in blending, depending on the blend factor.
    blend_constants: DynamicableValue<[vkfloat; 4]>,
}

impl GsBlendState {

    pub fn setup(prefab: GsBlendPrefab) -> GsBlendState {
        prefab.generate()
    }

    #[inline]
    pub(crate) fn ci(&self) -> vk::PipelineColorBlendStateCreateInfo {

        vk::PipelineColorBlendStateCreateInfo {
            s_type : vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags  : vk::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable  : if self.logic_op_enable { VK_TRUE } else { VK_FALSE },
            logic_op         : self.logic_op,
            attachment_count : self.attachments.len() as _,
            p_attachments    : self.attachments.as_ptr(),
            blend_constants  : self.blend_constants.to_blend_contents(),
        }
    }

    pub fn set_logical_operation(&mut self, logic_op: vk::LogicOp) {
        self.logic_op = logic_op;
    }

    pub fn add_attachment(&mut self, attachment: BlendAttachment) {

        let state = attachment.take();
        self.attachments.push(state);
    }

    pub fn set_blend_constants(&mut self, constants: DynamicableValue<[vkfloat; 4]>) {
        self.blend_constants = constants;
    }

    pub(crate) fn is_dynamic_blend_constants(&self) -> bool {
        self.blend_constants.is_dynamic()
    }
}

impl Default for GsBlendState {

    fn default() -> GsBlendState {
        GsBlendPrefab::Default.generate()
    }
}

impl DynamicableValue<[vkfloat; 4]> {

    fn to_blend_contents(&self) -> [vkfloat; 4] {
        match self {
            | DynamicableValue::Fixed { value } => value.clone(),
            | DynamicableValue::Dynamic => [0.0; 4],
        }
    }
}
