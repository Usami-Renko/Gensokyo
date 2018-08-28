
pub(crate) mod vertex_input;
pub(crate) mod input_assembly;
pub(crate) mod viewport;
pub(crate) mod rasterizer;
pub(crate) mod multisample;
pub(crate) mod depth_stencil;
pub(crate) mod blend;
pub(crate) mod tessellation;
pub(crate) mod dynamic;

pub mod prelude;

pub struct PipelineStates {

    pub(super) vertex_input  : vertex_input::HaVertexInput,
    pub(super) input_assembly: input_assembly::HaInputAssembly,
    pub(super) viewport      : viewport::HaViewport,
    pub(super) rasterizer    : rasterizer::HaRasterizer,
    pub(super) multisample   : multisample::HaMultisample,
    pub(super) depth_stencil : depth_stencil::HaDepthStencil,
    pub(super) blend         : blend::blending::HaBlend,
    pub(super) tessellation  : Option<tessellation::HaTessellation>,
    pub(super) dynamic       : Option<dynamic::HaDynamicState>,
}

use pipeline::shader::input::VertexInputDescription;
impl PipelineStates {

    pub fn setup(input: VertexInputDescription) -> PipelineStates {
        PipelineStates {
            vertex_input  : input.desc(),
            input_assembly: input_assembly::HaInputAssembly::default(),
            viewport      : viewport::HaViewport::default(),
            rasterizer    : rasterizer::HaRasterizer::default(),
            multisample   : multisample::HaMultisample::default(),
            depth_stencil : depth_stencil::HaDepthStencil::default(),
            blend         : blend::blending::HaBlend::default(),
            tessellation  : None,
            dynamic       : None,
        }
    }
}
