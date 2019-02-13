
use ash::vk;
use std::ptr;

use crate::pipeline::state::depth_stencil::DepthTest;
use crate::pipeline::state::depth_stencil::StencilTest;

pub struct GsDepthStencilState {

    pub depth  : DepthTest,
    pub stencil: StencilTest,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GsDepthStencilPrefab {
    Disable,
    EnableDepth,
    EnableStencil,
    EnableDepthStencil,
}

impl GsDepthStencilPrefab {

    fn generate(&self) -> GsDepthStencilState {

        match self {
            | GsDepthStencilPrefab::Disable => GsDepthStencilState {
                depth  : DepthTest::disable(),
                stencil: StencilTest::disable(),
            },
            | GsDepthStencilPrefab::EnableDepth => GsDepthStencilState {
                depth  : DepthTest::enable(true, vk::CompareOp::LESS, false),
                stencil: StencilTest::disable(),
            },
            | GsDepthStencilPrefab::EnableStencil => GsDepthStencilState {
                depth  : DepthTest::disable(),
                stencil: StencilTest::enalbe(),
            },
            | GsDepthStencilPrefab::EnableDepthStencil => GsDepthStencilState {
                depth  : DepthTest::enable(true, vk::CompareOp::LESS, false),
                stencil: StencilTest::enalbe(),
            },
        }
    }
}

impl GsDepthStencilState {

    pub fn setup(prefab: GsDepthStencilPrefab) -> GsDepthStencilState {
        prefab.generate()
    }

    pub fn set_depth(&mut self, depth: DepthTest) {
        self.depth = depth;
    }
    pub fn set_stencil(&mut self, stencil: StencilTest) {
        self.stencil = stencil;
    }

    #[inline]
    pub(crate) fn ci(&self) -> vk::PipelineDepthStencilStateCreateInfo {

        let depth_bound = self.depth.depth_bound.to_depth_bound();

        vk::PipelineDepthStencilStateCreateInfo {
            s_type : vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags  : vk::PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable  : self.depth.test_enable,
            depth_write_enable : self.depth.write_enable,
            depth_compare_op   : self.depth.compare_op,
            depth_bounds_test_enable: depth_bound.enable,
            min_depth_bounds: depth_bound.min_bound,
            max_depth_bounds: depth_bound.max_bound,

            stencil_test_enable: self.stencil.enable,
            front: self.stencil.front.origin(),
            back : self.stencil.back.origin(),
        }
    }
}

impl Default for GsDepthStencilState {

    /// Initialize GsDepthStencil with default setting (enable depth test, disable stencil test).
    fn default() -> GsDepthStencilState {
        GsDepthStencilPrefab::EnableDepth.generate()
    }
}
