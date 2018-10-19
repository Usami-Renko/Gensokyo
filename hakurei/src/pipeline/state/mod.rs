
pub use self::vertex_input::HaVertexInputState;
pub use self::input_assembly::{ HaInputAssemblyState, PrimitiveTopology };
pub use self::viewport::{ ViewportStateType, HaViewportState, ViewportStateInfo, ViewportInfo, ScissorInfo };
pub use self::rasterizer::{
    HaRasterizerState, RasterizerPrefab,
    PolygonMode, CullModeType, FrontFaceType, DepthBiasInfo,
};
pub use self::multisample::{ HaMultisampleState, MultisamplePrefab, SampleCountType, SampleShading };
pub use self::depth_stencil::{
    HaDepthStencilState, HaDepthStencilPrefab,
    DepthTest, DepthBoundInfo, // depth
    StencilTest, StencilOpState, StencilFaceFlag, // stencil
};
pub use self::blend::{
    HaBlendState, HaBlendPrefab, // blending
    BlendAttachemnt, BlendAttachmentPrefab, ColorComponentFlag, BlendFactor, BlendOp, // attachment
    LogicalOp, CompareOp, // ops
};
pub use self::tessellation::HaTessellationState;

pub(crate) use self::dynamic::{ HaDynamicState, DynamicState, DynamicableValue };

mod vertex_input;
mod input_assembly;
mod viewport;
mod rasterizer;
mod multisample;
mod depth_stencil;
mod blend;
mod tessellation;
mod dynamic;

pub(crate) struct PipelineStates {

    pub(super) vertex_input  : HaVertexInputState,
    pub(super) input_assembly: HaInputAssemblyState,
    pub(super) viewport      : HaViewportState,
    pub(super) rasterizer    : HaRasterizerState,
    pub(super) multisample   : HaMultisampleState,
    pub(super) depth_stencil : HaDepthStencilState,
    pub(super) blend         : HaBlendState,
    pub(super) tessellation  : Option<HaTessellationState>,
    pub(super) dynamic       : HaDynamicState,
}

use pipeline::shader::VertexInputDescription;
impl PipelineStates {

    pub(crate) fn setup(input: VertexInputDescription) -> PipelineStates {
        PipelineStates {
            vertex_input  : input.desc(),
            input_assembly: HaInputAssemblyState::default(),
            viewport      : HaViewportState::default(),
            rasterizer    : HaRasterizerState::default(),
            multisample   : HaMultisampleState::default(),
            depth_stencil : HaDepthStencilState::default(),
            blend         : HaBlendState::default(),
            tessellation  : None,
            dynamic       : HaDynamicState::default(),
        }
    }
}
