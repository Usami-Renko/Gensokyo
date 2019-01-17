
use shaderc;

use crate::pipeline::shader::shaderc::vulkan::GLSLVersion;
use crate::error::{ VkResult, VkError };

pub type ShadercTargetVersion = u32;

pub struct GsShadercOptions {

    pub target_env    : shaderc::TargetEnv,
    pub target_version: ShadercTargetVersion,
    pub lang          : shaderc::SourceLanguage,

    // common options
    pub optimal_level : shaderc::OptimizationLevel,
    pub debug_info      : bool,
    pub suppress_warning: bool,
    pub error_warning   : bool,

    // vulkan options
    pub glsl_profile  : Option<shaderc::GlslProfile>,
    pub glsl_version  : Option<GLSLVersion>,
}

impl GsShadercOptions {

    pub(crate) fn to_shaderc_options(&self) -> VkResult<shaderc::CompileOptions> {

        let mut options = shaderc::CompileOptions::new()
            .ok_or(VkError::shaderc("There are conflict in Shader Compile Options."))?;

        options.set_target_env(self.target_env, self.target_version);
        options.set_source_language(self.lang);
        options.set_optimization_level(self.optimal_level);

        if self.debug_info       {
            options.set_generate_debug_info();
        }
        if self.suppress_warning { options.set_suppress_warnings();  }
        if self.error_warning    { options.set_warnings_as_errors(); }

        if let Some(profile) = self.glsl_profile {
            if let Some(version) = self.glsl_version {
                options.set_forced_version_profile(version, profile);
            }
        }

        Ok(options)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GsShaderOptimalLevel {
    Unoptimal,
    CodeSizeOptimal,
    PerformanceOptimal,
}

impl GsShaderOptimalLevel {

    pub fn to_shaderc_option(&self) -> shaderc::OptimizationLevel {
        match self {
            | GsShaderOptimalLevel::Unoptimal          => shaderc::OptimizationLevel::Zero,
            | GsShaderOptimalLevel::CodeSizeOptimal    => shaderc::OptimizationLevel::Size,
            | GsShaderOptimalLevel::PerformanceOptimal => shaderc::OptimizationLevel::Performance,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GsShaderDebugPattern {
    Disable,
    Warning,
    SuppressWarning,
    Error,
}

impl GsShaderDebugPattern {

    pub fn set_shaderc_option(&self, options: &mut GsShadercOptions) {
        match self {
            | GsShaderDebugPattern::Disable => {
                // leave it empty
            },
            | GsShaderDebugPattern::Warning => {
                options.debug_info = true;
            },
            | GsShaderDebugPattern::SuppressWarning => {
                options.debug_info       = true;
                options.suppress_warning = true;
            },
            | GsShaderDebugPattern::Error => {
                options.debug_info    = true;
                options.error_warning = true;
            }
        }
    }
}
