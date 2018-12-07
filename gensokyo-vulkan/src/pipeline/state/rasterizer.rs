
use ash::vk;

use std::ptr;

use crate::pipeline::state::dynamic::DynamicableValue;

use crate::types::{ vkfloat, vkbool, VK_TRUE, VK_FALSE };

pub struct GsRasterizerState {

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
    line_width  : DynamicableValue<vkfloat>,

    depth_bias  : DynamicableValue<DepthBiasInfo>,

    /// Controls whether to clamp the fragment’s depth values instead of clipping primitives to the z planes of the frustum.
    depth_clamp_enable       : vkbool,
    /// Controls whether primitives are discarded immediately before the rasterization stage.
    rasterizer_discard_enable: vkbool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RasterizerPrefab {
    /// Common configuration.
    ///
    /// Cull back, clockwise, fill polygon, 1.0 line width, disable depth bias, disable depth clamp, disable rasterizer discard.
    Common,
}

impl RasterizerPrefab {

    fn generate(&self) -> GsRasterizerState {
        match self {
            | RasterizerPrefab::Common => GsRasterizerState { ..Default::default() },
        }
    }
}

impl GsRasterizerState {

    pub fn setup(prefab: RasterizerPrefab) -> GsRasterizerState {
        prefab.generate()
    }

    pub(crate) fn info(&self) -> vk::PipelineRasterizationStateCreateInfo {

        let depth_bias = self.depth_bias.to_depth_bias();

        vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
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

    pub fn set_polygon_mode(&mut self, mode: vk::PolygonMode) {
        self.polygon_mode = mode;
    }
    pub fn set_cull_mode(&mut self, mode: vk::CullModeFlags) {
        self.cull_mode = mode;
    }
    pub fn set_front_face(&mut self, face: vk::FrontFace) {
        self.front_face = face;
    }
    pub fn set_depth_bias(&mut self, bias: DynamicableValue<DepthBiasInfo>) {
        self.depth_bias = bias;
    }
    pub fn set_line_width(&mut self, width: DynamicableValue<vkfloat>) {
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

impl Default for GsRasterizerState {

    fn default() -> GsRasterizerState {
        GsRasterizerState {
            cull_mode   : vk::CullModeFlags::BACK,
            front_face  : vk::FrontFace::CLOCKWISE,
            polygon_mode: vk::PolygonMode::FILL,
            line_width  : DynamicableValue::Fixed { value: 1.0 },

            depth_bias: DynamicableValue::Fixed { value: DepthBiasInfo::disable() },

            depth_clamp_enable       : VK_FALSE,
            rasterizer_discard_enable: VK_FALSE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DepthBiasInfo {
    // TODO: Add explaination for each field
    enable              : vkbool,
    pub constant_factor : vkfloat,
    pub clamp           : vkfloat,
    pub slope_factor    : vkfloat,
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
            enable         : VK_FALSE,
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
    pub fn setup(constant_factor: vkfloat, clamp: vkfloat, slope_factor: vkfloat) -> DepthBiasInfo {
        DepthBiasInfo { enable: VK_TRUE, clamp, constant_factor, slope_factor, }
    }
}

