
use shaderc;

use utility::shaderc::HaShadercOptions;
use utility::shaderc::{ HaShaderOptimalLevel, HaShaderDebugPattern };

pub struct VulkanShadercOptions {

    profile: Option<HaGLSLProfile>,
    version: Option<GLSLVersion>,
    optimal: HaShaderOptimalLevel,
    debug  : HaShaderDebugPattern,
}

impl VulkanShadercOptions {

    pub fn new() -> VulkanShadercOptions {
        VulkanShadercOptions::default()
    }

    pub fn set_profile(&mut self, profile: HaGLSLProfile, version: GLSLVersion) {
        self.profile = Some(profile);
        self.version = Some(version);
    }

    pub fn set_optimal(&mut self, level: HaShaderOptimalLevel) {
        self.optimal = level;
    }

    pub fn set_debug(&mut self, pattern: HaShaderDebugPattern) {
        self.debug = pattern;
    }

    pub(crate) fn to_shaderc_options(&self) -> HaShadercOptions {

        let mut options = HaShadercOptions {
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
            optimal: HaShaderOptimalLevel::PerformanceOptimal,
            debug  : HaShaderDebugPattern::Warning,
        }
    }
}


pub type GLSLVersion = u32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HaGLSLProfile {
    /// Used if GLSL version did not specify the profile.
    Unknown,
    Core,
    Compatibility,
    Es,
}

impl HaGLSLProfile {
    
    fn to_shaderc_option(&self) -> shaderc::GlslProfile {
        match self {
            | HaGLSLProfile::Unknown       => shaderc::GlslProfile::None,
            | HaGLSLProfile::Core          => shaderc::GlslProfile::Core,
            | HaGLSLProfile::Compatibility => shaderc::GlslProfile::Compatibility,
            | HaGLSLProfile::Es            => shaderc::GlslProfile::Es,
        }
    }
}
