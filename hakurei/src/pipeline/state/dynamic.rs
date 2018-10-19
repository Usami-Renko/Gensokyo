
use ash::vk;
use ash::vk::uint32_t;

use utility::marker::VulkanEnum;

use std::ptr;

/// Most states are baked into the pipeline, but there are still a few dynamic states that can be changed within a command buffer.
pub struct HaDynamicState {

    /// DynamicState specifies which pieces of pipeline state will use the values from dynamic state commands rather than from pipeline state creation info.
    states: Vec<vk::DynamicState>,
}

impl HaDynamicState {

    pub(crate) fn info(&self) -> vk::PipelineDynamicStateCreateInfo {
        vk::PipelineDynamicStateCreateInfo {
            s_type: vk::StructureType::PipelineDynamicStateCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineDynamicStateCreateFlags::empty(),

            dynamic_state_count: self.states.len() as uint32_t,
            p_dynamic_states   : self.states.as_ptr(),
        }
    }

    pub fn add_state(&mut self, state: DynamicState) {
        self.states.push(state.value());
    }

    pub fn is_contain_state(&self) -> bool {
        !self.states.is_empty()
    }
}

impl Default for HaDynamicState {

    fn default() -> HaDynamicState {
        HaDynamicState {
            states: vec![],
        }
    }
}

// TODO: Add configuration for Other Dynamic States.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DynamicState {
    /// `Viewport` specifies that the pViewports state in vk::PipelineViewportStateCreateInfo will be ignored and must be set dynamically with vk::CmdSetViewport before any draw commands.
    ///
    /// The number of viewports used by a pipeline is still specified by the viewportCount member of vk::PipelineViewportStateCreateInfo.
    Viewport,
    /// `Scissor` specifies that the pScissors state in vk::PipelineViewportStateCreateInfo will be ignored and must be set dynamically with vk::CmdSetScissor before any draw commands.
    ///
    /// The number of scissor rectangles used by a pipeline is still specified by the scissorCount member of vk::PipelineViewportStateCreateInfo.
    Scissor,
    /// `LineWidth` specifies that the lineWidth state in vk::PipelineRasterizationStateCreateInfo will be ignored
    /// and must be set dynamically with vk::CmdSetLineWidth before any draw commands that generate line primitives for the rasterizer.
    LineWidth,
    /// `DepthBias` specifies that the depthBiasConstantFactor, depthBiasClamp and depthBiasSlopeFactor states in vk::PipelineRasterizationStateCreateInfo will be ignored and must be set dynamically with vk::CmdSetDepthBias before any draws are performed with depthBiasEnable in vk::PipelineRasterizationStateCreateInfo set to vk::TRUE.
    DepthBias,
    /// `BlendConstants` specifies that the blendConstants state in vk::PipelineColorBlendStateCreateInfo will be ignored and must be set dynamically with vk::CmdSetBlendConstants before any draws are performed with a pipeline state with vk::PipelineColorBlendAttachmentState member blendEnable set to vk::TRUE and any of the blend functions using a constant blend color.
    BlendConstants,
    /// `DepthBounds` specifies that the minDepthBounds and maxDepthBounds states of vk::PipelineDepthStencilStateCreateInfo will be ignored and must be set dynamically with vk::CmdSetDepthBounds before any draws are performed with a pipeline state with vk::PipelineDepthStencilStateCreateInfo member depthBoundsTestEnable set to vk::TRUE.
    DepthBounds,
    /// `StencilCompareMask` specifies that the compareMask state in vk::PipelineDepthStencilStateCreateInfo for both front and back will be ignored and must be set dynamically with vk::CmdSetStencilCompareMask before any draws are performed with a pipeline state with vk::PipelineDepthStencilStateCreateInfo member stencilTestEnable set to vk::TRUE.
    StencilCompareMask,
    /// `StencilWriteMask` specifies that the writeMask state in vk::PipelineDepthStencilStateCreateInfo for both front and back will be ignored and must be set dynamically with vk::CmdSetStencilWriteMask before any draws are performed with a pipeline state with vk::PipelineDepthStencilStateCreateInfo member stencilTestEnable set to vk::TRUE.
    StencilWriteMask,
    /// `StencilReference` specifies that the reference state in vk::PipelineDepthStencilStateCreateInfo for both front and back will be ignored and must be set dynamically with vk::CmdSetStencilReference before any draws are performed with a pipeline state with vk::PipelineDepthStencilStateCreateInfo member stencilTestEnable set to vk::TRUE.
    StencilReference,
}

impl VulkanEnum for DynamicState {
    type EnumType = vk::DynamicState;

    fn value(&self) -> Self::EnumType {
        match self {
            | DynamicState::Viewport           => vk::DynamicState::Viewport,
            | DynamicState::Scissor            => vk::DynamicState::Scissor,
            | DynamicState::LineWidth          => vk::DynamicState::LineWidth,
            | DynamicState::DepthBias          => vk::DynamicState::DepthBias,
            | DynamicState::BlendConstants     => vk::DynamicState::BlendConstants,
            | DynamicState::DepthBounds        => vk::DynamicState::DepthBounds,
            | DynamicState::StencilCompareMask => vk::DynamicState::StencilCompareMask,
            | DynamicState::StencilWriteMask   => vk::DynamicState::StencilWriteMask,
            | DynamicState::StencilReference   => vk::DynamicState::StencilReference,
        }
    }
}

#[derive(Debug)]
pub enum DynamicableValue<T> {
    Fixed { value: T },
    Dynamic,
}

impl<T> DynamicableValue<T>{

    pub fn is_dynamic(&self) -> bool {
        match self {
            | DynamicableValue::Fixed { .. } => false,
            | DynamicableValue::Dynamic => true,
        }
    }
}

impl Clone for DynamicableValue<uint32_t> {
    fn clone(&self) -> Self {
        self.to_owned()
    }
}
