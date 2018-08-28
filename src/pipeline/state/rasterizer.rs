
use ash::vk;

use std::os::raw::c_float;
use std::ptr;

pub struct HaRasterizer {

    /// The method of rasterization for polygons.
    ///
    /// Possible values: Fill, Line, Point.
    polygon_mode : vk::PolygonMode,
    /// The triangle facing direction used for primitive culling.
    ///
    /// Possible values: None, Front, Back, FrontAndBack.
    cull_mode    : vk::CullModeFlags,
    /// The front-facing triangle orientation to be used for culling.
    ///
    /// Possible values: Clockwise, Counter-Clockwise.
    front_face   : vk::FrontFace,
    /// The width of rasterized line segments
    line_width   : c_float,

    depth_bias   : DepthBias,

    /// Controls whether to clamp the fragment’s depth values instead of clipping primitives to the z planes of the frustum.
    depth_clamp_enable        : vk::Bool32,
    /// Controls whether primitives are discarded immediately before the rasterization stage.
    rasterizer_discard_enable : vk::Bool32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RasterizerPrefab {
    /// Common configuration.
    ///
    /// Cull back, clockwise, fill polygon, 1.0 line width, disable depth bias, disable depth clamp, disable rasterizer discard.
    Common,
}

impl RasterizerPrefab {
    fn generate(&self) -> HaRasterizer {
        match *self {
            | RasterizerPrefab::Common => HaRasterizer { ..Default::default() },
        }
    }
}

impl HaRasterizer {

    pub fn setup(prefab: RasterizerPrefab) -> HaRasterizer {
        prefab.generate()
    }

    pub fn info(&self) -> vk::PipelineRasterizationStateCreateInfo {
        vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PipelineRasterizationStateCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags: vk::PipelineRasterizationStateCreateFlags::empty(),

            polygon_mode : self.polygon_mode,
            cull_mode    : self.cull_mode,
            front_face   : self.front_face,
            line_width   : self.line_width,

            depth_bias_enable          : self.depth_bias.enable,
            depth_bias_constant_factor : self.depth_bias.constant_factor,
            depth_bias_clamp           : self.depth_bias.clamp,
            depth_bias_slope_factor    : self.depth_bias.slope_factor,

            depth_clamp_enable        : self.depth_clamp_enable,
            rasterizer_discard_enable : self.rasterizer_discard_enable,
        }
    }


    pub fn set_polygon_mode(&mut self, mode: vk::PolygonMode) {
        self.polygon_mode = mode;
    }
    pub fn set_cull_mode(&mut self, mode: CullModeType) {
        self.cull_mode = mode.flag();
    }
    pub fn set_front_face(&mut self, face: vk::FrontFace) {
        self.front_face = face;
    }
    pub fn set_line_width(&mut self, width: c_float) {
        self.line_width = width
    }
    pub fn set_depth_bias(&mut self, bias: DepthBias) {
        self.depth_bias = bias;
    }
    pub fn set_depth_clamp_enable(&mut self, enable: bool) {
        self.depth_clamp_enable = if enable { 1 } else { 0 };
    }
    pub fn set_rasterizer_discard_enable(&mut self, enable: bool) {
        self.rasterizer_discard_enable = if enable { 1 } else { 0 };
    }
}

impl Default for HaRasterizer {

    fn default() -> HaRasterizer {
        HaRasterizer {
            cull_mode    : vk::CULL_MODE_BACK_BIT,
            front_face   : vk::FrontFace::Clockwise,
            polygon_mode : vk::PolygonMode::Fill,
            line_width   : 1.0,

            depth_bias: DepthBias::disable(),

            depth_clamp_enable        : vk::VK_FALSE,
            rasterizer_discard_enable : vk::VK_FALSE,
        }
    }
}


pub struct DepthBias {
    // TODO: Add explaination for each field
    enable          : vk::Bool32,
    clamp           : c_float,
    constant_factor : c_float,
    slope_factor    : c_float,
}

impl DepthBias {

    pub fn disable() -> DepthBias {
        DepthBias {
            enable          : vk::VK_FALSE,
            clamp           : 0.0,
            constant_factor : 0.0,
            slope_factor    : 0.0,
        }
    }

    pub fn setup(clamp: c_float, constant_factor: c_float, slope_factor: c_float) -> DepthBias {
        DepthBias { enable: vk::VK_TRUE, clamp, constant_factor, slope_factor, }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CullModeType {
    None, Front, Back, FrontAndBack,
}
impl CullModeType {

    fn flag(&self) -> vk::CullModeFlags {
        match *self {
            | CullModeType::None         => vk::CULL_MODE_NONE,
            | CullModeType::Front        => vk::CULL_MODE_FRONT_BIT,
            | CullModeType::Back         => vk::CULL_MODE_BACK_BIT,
            | CullModeType::FrontAndBack => vk::CULL_MODE_FRONT_AND_BACK,
        }
    }
}
