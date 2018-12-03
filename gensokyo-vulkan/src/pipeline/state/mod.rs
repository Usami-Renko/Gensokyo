
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

    pub vertex_input  : self::vertex_input::GsVertexInputState,
    pub input_assembly: self::input_assembly::GsInputAssemblyState,
    pub viewport      : self::viewport::GsViewportState,
    pub rasterizer    : self::rasterizer::GsRasterizerState,
    pub multisample   : self::multisample::GsMultisampleState,
    pub depth_stencil : self::depth_stencil::GsDepthStencilState,
    pub blend         : self::blend::GsBlendState,
    pub tessellation  : Option<self::tessellation::GsTessellationState>,
    pub dynamic       : self::dynamic::GsDynamicState,
}

use pipeline::shader::VertexInputDescription;
use types::vkDim2D;

impl PipelineStates {

    pub(super) fn setup(input: VertexInputDescription, dimension: vkDim2D) -> PipelineStates {
        PipelineStates {
            vertex_input  : input.desc(),
            input_assembly: self::input_assembly::GsInputAssemblyState::default(),
            viewport      : self::viewport::GsViewportState::single(self::viewport::ViewportStateInfo::new(dimension)),
            rasterizer    : self::rasterizer::GsRasterizerState::default(),
            multisample   : self::multisample::GsMultisampleState::default(),
            depth_stencil : self::depth_stencil::GsDepthStencilState::default(),
            blend         : self::blend::GsBlendState::default(),
            tessellation  : None,
            dynamic       : self::dynamic::GsDynamicState::default(),
        }
    }
}
