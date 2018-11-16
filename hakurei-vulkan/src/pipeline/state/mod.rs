
pub mod vertex_input;
pub mod input_assembly;
pub mod viewport;
pub mod rasterizer;
pub mod multisample;
pub mod depth_stencil;
pub mod blend;
pub mod tessellation;
pub mod dynamic;

pub(super) struct PipelineStates {

    pub vertex_input  : self::vertex_input::HaVertexInputState,
    pub input_assembly: self::input_assembly::HaInputAssemblyState,
    pub viewport      : self::viewport::HaViewportState,
    pub rasterizer    : self::rasterizer::HaRasterizerState,
    pub multisample   : self::multisample::HaMultisampleState,
    pub depth_stencil : self::depth_stencil::HaDepthStencilState,
    pub blend         : self::blend::HaBlendState,
    pub tessellation  : Option<self::tessellation::HaTessellationState>,
    pub dynamic       : self::dynamic::HaDynamicState,
}

use pipeline::shader::VertexInputDescription;

impl PipelineStates {

    pub(super) fn setup(input: VertexInputDescription) -> PipelineStates {
        PipelineStates {
            vertex_input  : input.desc(),
            input_assembly: self::input_assembly::HaInputAssemblyState::default(),
            viewport      : self::viewport::HaViewportState::default(),
            rasterizer    : self::rasterizer::HaRasterizerState::default(),
            multisample   : self::multisample::HaMultisampleState::default(),
            depth_stencil : self::depth_stencil::HaDepthStencilState::default(),
            blend         : self::blend::HaBlendState::default(),
            tessellation  : None,
            dynamic       : self::dynamic::HaDynamicState::default(),
        }
    }
}
