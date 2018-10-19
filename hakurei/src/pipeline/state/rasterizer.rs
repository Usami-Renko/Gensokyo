
use ash::vk;

use std::os::raw::c_float;
use std::ptr;

use pipeline::state::DynamicableValue;

use utility::marker::VulkanEnum;
use utility::marker::Prefab;

pub struct HaRasterizerState {

    /// The method of rasterization for polygons.
    ///
    /// Possible values: Fill, Line, Point.
    polygon_mode: vk::PolygonMode,
    /// The triangle facing direction used for primitive culling.
    ///
    /// Possible values: None, Front, Back, FrontAndBack.
    cull_mode   : vk::CullModeFlags,
    /// The front-facing triangle orientation to be used for culling.
    ///
    /// Possible values: Clockwise, Counter-Clockwise.
    front_face  : vk::FrontFace,
    /// The width of rasterized line segments
    line_width  : DynamicableValue<c_float>,

    depth_bias  : DynamicableValue<DepthBiasInfo>,

    /// Controls whether to clamp the fragment’s depth values instead of clipping primitives to the z planes of the frustum.
    depth_clamp_enable       : vk::Bool32,
    /// Controls whether primitives are discarded immediately before the rasterization stage.
    rasterizer_discard_enable: vk::Bool32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RasterizerPrefab {
    /// Common configuration.
    ///
    /// Cull back, clockwise, fill polygon, 1.0 line width, disable depth bias, disable depth clamp, disable rasterizer discard.
    Common,
}

impl Prefab for RasterizerPrefab {
    type PrefabType = HaRasterizerState;

    fn generate(&self) -> Self::PrefabType {
        match self {
            | RasterizerPrefab::Common => HaRasterizerState { ..Default::default() },
        }
    }
}

impl HaRasterizerState {

    pub fn setup(prefab: RasterizerPrefab) -> HaRasterizerState {
        prefab.generate()
    }

    pub(crate) fn info(&self) -> vk::PipelineRasterizationStateCreateInfo {

        let depth_bias = self.depth_bias.to_depth_bias();

        vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PipelineRasterizationStateCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineRasterizationStateCreateFlags::empty(),

            polygon_mode: self.polygon_mode,
            cull_mode   : self.cull_mode,
            front_face  : self.front_face,
            line_width  : match self.line_width {
                | DynamicableValue::Fixed { value } => value,
                | DynamicableValue::Dynamic => 1.0,
            },

            depth_bias_enable         : depth_bias.enable,
            depth_bias_constant_factor: depth_bias.constant_factor,
            depth_bias_clamp          : depth_bias.clamp,
            depth_bias_slope_factor   : depth_bias.slope_factor,

            depth_clamp_enable       : self.depth_clamp_enable,
            rasterizer_discard_enable: self.rasterizer_discard_enable,
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
    pub fn set_depth_bias(&mut self, bias: DynamicableValue<DepthBiasInfo>) {
        self.depth_bias = bias;
    }
    pub fn set_line_width(&mut self, width: DynamicableValue<c_float>) {
        self.line_width = width;
    }
    pub fn set_depth_clamp_enable(&mut self, enable: bool) {
        self.depth_clamp_enable = if enable { 1 } else { 0 };
    }
    pub fn set_rasterizer_discard_enable(&mut self, enable: bool) {
        self.rasterizer_discard_enable = if enable { 1 } else { 0 };
    }

    pub(crate) fn is_dynamic_lindwidth(&self) -> bool {
        self.line_width.is_dynamic()
    }
    pub(crate) fn is_dynamic_depthbias(&self) -> bool {
        self.depth_bias.is_dynamic()
    }
}

impl Default for HaRasterizerState {

    fn default() -> HaRasterizerState {
        HaRasterizerState {
            cull_mode   : vk::CULL_MODE_BACK_BIT,
            front_face  : vk::FrontFace::Clockwise,
            polygon_mode: vk::PolygonMode::Fill,
            line_width  : DynamicableValue::Fixed { value: 1.0 },

            depth_bias: DynamicableValue::Fixed { value: DepthBiasInfo::disable() },

            depth_clamp_enable       : vk::VK_FALSE,
            rasterizer_discard_enable: vk::VK_FALSE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DepthBiasInfo {
    // TODO: Add explaination for each field
    pub(crate) enable          : vk::Bool32,
    pub(crate) constant_factor : c_float,
    pub(crate) clamp           : c_float,
    pub(crate) slope_factor    : c_float,
}

impl DynamicableValue<DepthBiasInfo> {

    fn to_depth_bias(&self) -> DepthBiasInfo {
        match self {
            | DynamicableValue::Fixed { value } => value.clone(),
            | DynamicableValue::Dynamic => DepthBiasInfo::disable(),
        }
    }
}

impl DepthBiasInfo {

    pub fn disable() -> DepthBiasInfo {
        DepthBiasInfo {
            enable         : vk::VK_FALSE,
            constant_factor: 0.0,
            clamp          : 0.0,
            slope_factor   : 0.0,
        }
    }

    /// Initialize DepthBias value.
    ///
    /// `constant_factor` is a scalar factor controlling the constant depth value added to each fragment.
    ///
    /// `clamp` is the maximum (or minimum) depth bias of a fragment.
    ///
    /// `slope_factor` is a scalar factor applied to a fragment’s slope in depth bias calculations.
    pub fn setup(constant_factor: c_float, clamp: c_float, slope_factor: c_float) -> DepthBiasInfo {
        DepthBiasInfo { enable: vk::VK_TRUE, clamp, constant_factor, slope_factor, }
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
