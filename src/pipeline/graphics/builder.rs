
use ash::vk;

use pipeline::{
    shader::{ HaShaderInfo, HaShaderModule },
    input_assembly::HaInputAssembly,
    tessellation::HaTessellationState,
    viewport::HaViewport,
    rasterizer::HaRasterizer,
    multisample::HaMultisample,
    depth_stencil::HaDepthStencil,
    blend::HaBlend,
    dynamic::HaDynamicState,
    layout::HaPipelineLayout,
};

pub struct GraphicsPipelineBuilder {

    shader        : HaShaderModule,
    input         : HaInputAssembly,
    tessellation  : Option<HaInputAssembly>,
    viewport      : Option<HaViewport>,
    rasterizer    : Option<HaRasterizer>,
    multisample   : Option<HaMultisample>,
    depth_stencil : Option<HaDepthStencil>,
    blend         : Option<HaBlend>,
    dynamic       : Option<HaDynamicState>,
    layout        : HaPipelineLayout,
}

impl GraphicsPipelineBuilder {

    fn setup(shader: HaShaderModule, input: HaInputAssembly) -> GraphicsPipelineBuilder {

    }
}
