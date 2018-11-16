
use ash::vk;
use std::ptr;

use utils::marker::Prefab;

use pipeline::state::depth_stencil::DepthTest;
use pipeline::state::depth_stencil::StencilTest;

pub struct HaDepthStencilState {

    pub depth  : DepthTest,
    pub stencil: StencilTest,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HaDepthStencilPrefab {
    Disable,
    EnableDepth,
    EnableStencil,
    EnableDepthStencil,
}

impl Prefab for HaDepthStencilPrefab {
    type PrefabType = HaDepthStencilState;

    fn generate(&self) -> Self::PrefabType {

        match self {
            | HaDepthStencilPrefab::Disable => HaDepthStencilState {
                depth  : DepthTest::disable(),
                stencil: StencilTest::disable(),
            },
            | HaDepthStencilPrefab::EnableDepth => HaDepthStencilState {
                depth  : DepthTest::enable(true, vk::CompareOp::Less, false),
                stencil: StencilTest::disable(),
            },
            | HaDepthStencilPrefab::EnableStencil => HaDepthStencilState {
                depth  : DepthTest::disable(),
                stencil: StencilTest::enalbe(),
            },
            | HaDepthStencilPrefab::EnableDepthStencil => HaDepthStencilState {
                depth  : DepthTest::enable(true, vk::CompareOp::Less, false),
                stencil: StencilTest::enalbe(),
            },
        }
    }
}

impl HaDepthStencilState {

    pub fn setup(prefab: HaDepthStencilPrefab) -> HaDepthStencilState {
        prefab.generate()
    }

    pub fn set_depth(&mut self, depth: DepthTest) {
        self.depth = depth;
    }
    pub fn set_stencil(&mut self, stencil: StencilTest) {
        self.stencil = stencil;
    }

    pub(crate) fn info(&self) -> vk::PipelineDepthStencilStateCreateInfo {

        let depth_bound = self.depth.depth_bound.to_depth_bound();

        vk::PipelineDepthStencilStateCreateInfo {
            s_type : vk::StructureType::PipelineDepthStencilStateCreateInfo,
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

impl Default for HaDepthStencilState {

    /// Initialize HaDepthStencil with default setting (enable depth test, disable stencil test).
    fn default() -> HaDepthStencilState {
        HaDepthStencilPrefab::EnableDepth.generate()
    }
}
