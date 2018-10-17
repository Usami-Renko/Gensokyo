
use ash::vk;

use std::os::raw::c_float;
use std::ptr;

use utility::marker::VulkanEnum;
use utility::marker::Prefab;

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

impl Prefab for RasterizerPrefab {
    type PrefabType = HaRasterizer;

    fn generate(&self) -> Self::PrefabType {
        match *self {
            | RasterizerPrefab::Common => HaRasterizer { ..Default::default() },
        }
    }
}

impl HaRasterizer {

    pub fn setup(prefab: RasterizerPrefab) -> HaRasterizer {
        prefab.generate()
    }

    pub(crate) fn info(&self) -> vk::PipelineRasterizationStateCreateInfo {
        vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PipelineRasterizationStateCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineRasterizationStateCreateFlags::empty(),

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


    pub fn set_polygon_mode(&mut self, mode: PolygonMode) {
        self.polygon_mode = mode.value();
    }
    pub fn set_cull_mode(&mut self, mode: CullModeType) {
        self.cull_mode = mode.value();
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

impl VulkanEnum for CullModeType {
    type EnumType = vk::CullModeFlags;

    fn value(&self) -> Self::EnumType {
        match *self {
            | CullModeType::None         => vk::CULL_MODE_NONE,
            | CullModeType::Front        => vk::CULL_MODE_FRONT_BIT,
            | CullModeType::Back         => vk::CULL_MODE_BACK_BIT,
            | CullModeType::FrontAndBack => vk::CULL_MODE_FRONT_AND_BACK,
        }
    }
}



/// PolygonMode specifies the method of rasterization for polygons.
///
/// These modes affect only the final rasterization of polygons:
/// in particular, a polygon’s vertices are shaded and the polygon is clipped and possibly culled before these modes are applied.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PolygonMode {
    /// Fill specifies that polygon vertices are drawn as points.
    Fill,
    /// Line specifies that polygon edges are drawn as line segments.
    Line,
    /// Point specifies that polygons are rendered using the polygon rasterization rules in this section.
    Point
}

impl VulkanEnum for PolygonMode {
    type EnumType = vk::PolygonMode;

    fn value(&self) -> Self::EnumType {
        match *self {
            | PolygonMode::Fill  => vk::PolygonMode::Fill,
            | PolygonMode::Line  => vk::PolygonMode::Line,
            | PolygonMode::Point => vk::PolygonMode::Point,
        }
    }
}


/// FrontFaceType determine whether the triangle is back-facing or front-facing.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FrontFaceType {
    /// CounterClockwise specifies that the face of triangle drawed with CounterClockwise order is considered front-facing.
    CounterClockwise,
    /// Clockwise specifies that the face of triangle drawed with Clockwise order is considered front-facing.
    Clockwise,
}

impl VulkanEnum for FrontFaceType {
    type EnumType = vk::FrontFace;

    fn value(&self) -> Self::EnumType {
        match *self {
            | FrontFaceType::CounterClockwise => vk::FrontFace::CounterClockwise,
            | FrontFaceType::Clockwise        => vk::FrontFace::Clockwise,
        }
    }
}
