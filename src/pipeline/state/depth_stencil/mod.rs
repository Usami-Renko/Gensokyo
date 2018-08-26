
mod depth;
mod stencil;

use ash::vk;

pub use self::stencil::{ StencilTest, StencilOpState };
pub use self::depth::DepthTest;

use std::ptr;

pub struct HaDepthStencil {

    depth: DepthTest,
    stencil: StencilTest,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HaDepthStencilPrefab {
    Disable,
    EnableDepth,
    EnableStencil,
    EnableDepthStencil,
}
impl HaDepthStencilPrefab {
    fn generate(&self) -> HaDepthStencil {
        match *self {
            | HaDepthStencilPrefab::Disable => HaDepthStencil {
                depth:   DepthTest::disable(),
                stencil: StencilTest::disable(),
            },
            | HaDepthStencilPrefab::EnableDepth => HaDepthStencil {
                depth:   DepthTest::enable(true, vk::CompareOp::Less, false),
                stencil: StencilTest::disable(),
            },
            | HaDepthStencilPrefab::EnableStencil => HaDepthStencil {
                depth:   DepthTest::disable(),
                stencil: StencilTest::enalbe(),
            },
            | HaDepthStencilPrefab::EnableDepthStencil => HaDepthStencil {
                depth:   DepthTest::enable(true, vk::CompareOp::Less, false),
                stencil: StencilTest::enalbe(),
            },
        }
    }
}

impl HaDepthStencil {


    pub fn setup(prefab: HaDepthStencilPrefab) -> HaDepthStencil {
        prefab.generate()
    }

    pub fn set_depth(&mut self, depth: DepthTest) {
        self.depth = depth;
    }
    pub fn set_stencil(&mut self, stencil: StencilTest) {
        self.stencil = stencil;
    }

    pub fn info(&self) -> vk::PipelineDepthStencilStateCreateInfo {
        vk::PipelineDepthStencilStateCreateInfo {
            s_type : vk::StructureType::PipelineDepthStencilStateCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags  : vk::PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable  : self.depth.test_enable,
            depth_write_enable : self.depth.write_enable,
            depth_compare_op   : self.depth.compare_op,
            depth_bounds_test_enable: self.depth.bounds_enable,
            min_depth_bounds: self.depth.min_bounds,
            max_depth_bounds: self.depth.max_bounds,

            stencil_test_enable: self.stencil.enable,
            front: self.stencil.front.origin(),
            back:  self.stencil.back.origin(),

        }
    }
}

impl Default for HaDepthStencil {

    /// Initialize HaDepthStencil with default setting (enable depth test, disable stencil test).
    fn default() -> HaDepthStencil {
        HaDepthStencilPrefab::EnableDepth.generate()
    }
}
