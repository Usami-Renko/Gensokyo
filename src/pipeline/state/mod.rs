
pub use self::vertex_input::HaVertexInput;
pub use self::input_assembly::HaInputAssembly;
pub use self::viewport::HaViewport;
pub use self::rasterizer::{ HaRasterizer, RasterizerPrefab, CullModeType, DepthBias };
pub use self::multisample::{ HaMultisample, MultisamplePrefab, SampleCountType, SampleShading };
pub use self::depth_stencil::*;
pub use self::blend::*;
pub use self::tessellation::HaTessellation;
pub use self::dynamic::HaDynamicState;

use ash;
pub type PrimitiveTopology = ash::vk::PrimitiveTopology;

mod vertex_input;
mod input_assembly;
mod viewport;
mod rasterizer;
mod multisample;
mod depth_stencil;
mod blend;
mod tessellation;
mod dynamic;

pub struct PipelineStates {

    pub(super) vertex_input  : HaVertexInput,
    pub(super) input_assembly: HaInputAssembly,
    pub(super) viewport      : HaViewport,
    pub(super) rasterizer    : HaRasterizer,
    pub(super) multisample   : HaMultisample,
    pub(super) depth_stencil : HaDepthStencil,
    pub(super) blend         : HaBlend,
    pub(super) tessellation  : Option<HaTessellation>,
    pub(super) dynamic       : Option<HaDynamicState>,
}

impl Default for PipelineStates {

    fn default() -> PipelineStates {
        PipelineStates {
            vertex_input  : HaVertexInput::default(),
            input_assembly: HaInputAssembly::default(),
            viewport      : HaViewport::default(),
            rasterizer    : HaRasterizer::default(),
            multisample   : HaMultisample::default(),
            depth_stencil : HaDepthStencil::default(),
            blend         : HaBlend::default(),
            tessellation  : None,
            dynamic       : None,
        }
    }
}
