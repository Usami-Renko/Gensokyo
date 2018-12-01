
use shaderc;

use pipeline::shader::shaderc::options::GsShadercOptions;
use pipeline::shader::shaderc::options::{ GsShaderOptimalLevel, GsShaderDebugPattern };

pub struct VulkanShadercOptions {

    profile: Option<GsGLSLProfile>,
    version: Option<GLSLVersion>,
    optimal: GsShaderOptimalLevel,
    debug  : GsShaderDebugPattern,
}

impl VulkanShadercOptions {

    pub fn new() -> VulkanShadercOptions {
        VulkanShadercOptions::default()
    }

    pub fn set_profile(&mut self, profile: GsGLSLProfile, version: GLSLVersion) {
        self.profile = Some(profile);
        self.version = Some(version);
    }

    pub fn set_optimal(&mut self, level: GsShaderOptimalLevel) {
        self.optimal = level;
    }

    pub fn set_debug(&mut self, pattern: GsShaderDebugPattern) {
        self.debug = pattern;
    }

    pub(super) fn to_shaderc_options(&self) -> GsShadercOptions {

        let mut options = GsShadercOptions {
            target_env    : shaderc::TargetEnv::Vulkan,
            target_version: 0, // 0 is only support value.
            lang          : shaderc::SourceLanguage::GLSL,

            optimal_level   : self.optimal.to_shaderc_option(),
            debug_info      : false,
            suppress_warning: false,
            error_warning   : false,

            glsl_profile: self.profile.and_then(|p| Some(p.to_shaderc_option())),
            glsl_version: self.version,
        };

        self.debug.set_shaderc_option(&mut options);

        options
    }
}

impl Default for VulkanShadercOptions {

    fn default() -> VulkanShadercOptions {

        VulkanShadercOptions {
            profile: None,
            version: None,
            optimal: GsShaderOptimalLevel::PerformanceOptimal,
            debug  : GsShaderDebugPattern::Warning,
        }
    }
}


pub type GLSLVersion = u32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GsGLSLProfile {
    /// Used if GLSL version did not specify the profile.
    Unknown,
    Core,
    Compatibility,
    Es,
}

impl GsGLSLProfile {
    
    fn to_shaderc_option(&self) -> shaderc::GlslProfile {
        match self {
            | GsGLSLProfile::Unknown       => shaderc::GlslProfile::None,
            | GsGLSLProfile::Core          => shaderc::GlslProfile::Core,
            | GsGLSLProfile::Compatibility => shaderc::GlslProfile::Compatibility,
            | GsGLSLProfile::Es            => shaderc::GlslProfile::Es,
        }
    }
}
