
use ash::vk;

use crate::pipeline::{
    shader::{ GsShaderCI, VertexInputDescription },
    state::PipelineStates,
    state::vertex_input::GsVertexInputState,
    state::input_assembly::GsInputAssemblyState,
    state::viewport::ViewportStateType,
    state::rasterizer::GsRasterizerState,
    state::multisample::GsMultisampleState,
    state::depth_stencil::GsDepthStencilState,
    state::blend::GsBlendState,
    state::tessellation::GsTessellationState,
    pass::GsRenderPass,
    layout::{ GsPipelineLayout, PipelineLayoutBuilder, GsPushConstantRange },
};

use crate::types::vkDim2D;
use crate::descriptor::DescriptorSet;

// ------------------------------------------------------------------------------------------
pub struct GfxPipelineConfig {

    pub(super) shaders: Vec<GsShaderCI>,
    pub(super) states: PipelineStates,
    pub(super) render_pass: GsRenderPass,

    pub(super) layout_builder: PipelineLayoutBuilder,
}

impl GfxPipelineConfig {

    pub fn new(shaders: impl Into<Vec<GsShaderCI>>, input: VertexInputDescription, render_pass: GsRenderPass, dimension: vkDim2D) -> GfxPipelineConfig {

        GfxPipelineConfig {
            shaders : shaders.into(),
            states  : PipelineStates::setup(input, dimension),

            render_pass,
            layout_builder: GsPipelineLayout::new(),
        }
    }

    pub fn reset_shader(&mut self, shaders: Vec<GsShaderCI>) -> &mut GfxPipelineConfig {
        self.shaders = shaders;
        self
    }

    pub fn with_shader(mut self, shaders: Vec<GsShaderCI>) -> GfxPipelineConfig {
        self.reset_shader(shaders);
        self
    }

    pub fn finish(self) -> GfxPipelineConfig {
        self
    }

    pub fn reset_descriptor_sets(&mut self, sets: &[&DescriptorSet]) {

        for set in sets.into_iter() {
            self.layout_builder.add_descriptor_layout(&set.layout);
        }
    }

    pub fn with_descriptor_sets(mut self, sets: &[&DescriptorSet]) -> GfxPipelineConfig {
        self.reset_descriptor_sets(sets);
        self
    }

    pub fn reset_push_constants(&mut self, ranges: Vec<GsPushConstantRange>) -> &mut GfxPipelineConfig {

        for range in ranges.into_iter() {
            self.layout_builder.add_push_constant(range);
        }
        self
    }

    pub fn with_push_constants(mut self, ranges: Vec<GsPushConstantRange>) -> GfxPipelineConfig {
        self.reset_push_constants(ranges);
        self
    }

    pub fn reset_input_vertex(&mut self, state: GsVertexInputState) -> &mut GfxPipelineConfig {
        self.states.vertex_input = state;
        self
    }

    pub fn with_input_vertex(mut self, state: GsVertexInputState) -> GfxPipelineConfig {
        self.reset_input_vertex(state);
        self
    }

    pub fn reset_input_assembly(&mut self, state: GsInputAssemblyState) -> &mut GfxPipelineConfig {
        self.states.input_assembly = state;
        self
    }

    pub fn with_input_assembly(mut self, state: GsInputAssemblyState) -> GfxPipelineConfig {
        self.reset_input_assembly(state);
        self
    }

    pub fn reset_viewport(&mut self, state: ViewportStateType) -> &mut GfxPipelineConfig {

        match state {
            | ViewportStateType::Fixed { .. } => {},
            | ViewportStateType::Dynamic { .. } => {
                self.states.dynamic.add_state(vk::DynamicState::VIEWPORT);
                self.states.dynamic.add_state(vk::DynamicState::SCISSOR);
            },
            | ViewportStateType::DynamicViewportFixedScissor { .. } => {
                self.states.dynamic.add_state(vk::DynamicState::VIEWPORT);
            },
            | ViewportStateType::FixedViewportDynamicScissor { .. } => {
                self.states.dynamic.add_state(vk::DynamicState::SCISSOR);
            },
        }
        self.states.viewport = state.into();
        self
    }

    pub fn with_viewport(mut self, state: ViewportStateType) -> GfxPipelineConfig {
        self.reset_viewport(state);
        self
    }

    pub fn reset_rasterizer(&mut self, state: GsRasterizerState) -> &mut GfxPipelineConfig {

        if state.is_dynamic_lindwidth() {
            self.states.dynamic.add_state(vk::DynamicState::LINE_WIDTH);
        }
        if state.is_dynamic_depthbias() {
            self.states.dynamic.add_state(vk::DynamicState::DEPTH_BIAS);
        }

        self.states.rasterizer = state;
        self
    }

    pub fn with_rasterizer(mut self, state: GsRasterizerState) -> GfxPipelineConfig {
        self.reset_rasterizer(state);
        self
    }

    pub fn reset_multisample(&mut self, state: GsMultisampleState) -> &mut GfxPipelineConfig {
        self.states.multisample = state;
        self
    }

    pub fn with_multisample(mut self, state: GsMultisampleState) -> GfxPipelineConfig {
        self.reset_multisample(state);
        self
    }

    pub fn reset_depth_stencil(&mut self, state: GsDepthStencilState) -> &mut GfxPipelineConfig {

        if state.depth.is_dynamic_depthbound() {
            self.states.dynamic.add_state(vk::DynamicState::DEPTH_BOUNDS);
        }
        if state.stencil.is_dynamic_compare_mask() {
            self.states.dynamic.add_state(vk::DynamicState::STENCIL_COMPARE_MASK);
        }
        if state.stencil.is_dynamic_write_mask() {
            self.states.dynamic.add_state(vk::DynamicState::STENCIL_WRITE_MASK);
        }
        if state.stencil.is_dynamic_reference() {
            self.states.dynamic.add_state(vk::DynamicState::STENCIL_REFERENCE);
        }

        self.states.depth_stencil = state;
        self
    }

    pub fn with_depth_stencil(mut self, state: GsDepthStencilState) -> GfxPipelineConfig {
        self.reset_depth_stencil(state);
        self
    }

    pub fn reset_blend(&mut self, state: GsBlendState) -> &mut GfxPipelineConfig {

        if state.is_dynamic_blend_constants() {
            self.states.dynamic.add_state(vk::DynamicState::BLEND_CONSTANTS);
        }

        self.states.blend = state;
        self
    }

    pub fn with_blend(mut self, state: GsBlendState) -> GfxPipelineConfig {
        self.reset_blend(state);
        self
    }

    pub fn reset_tessellation(&mut self, tessellation: GsTessellationState) -> &mut GfxPipelineConfig {
        self.states.tessellation = Some(tessellation);
        self
    }

    pub fn with_tessellation(mut self, tessellation: GsTessellationState) -> GfxPipelineConfig {
        self.reset_tessellation(tessellation);
        self
    }
}
// ------------------------------------------------------------------------------------------
