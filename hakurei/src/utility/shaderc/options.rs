
use shaderc;

use utility::shaderc::GLSLVersion;
use utility::shaderc::ShaderCompileError;

pub type ShadercTargetVertion = u32;

pub struct HaShadercOptions {

    pub target_env    : shaderc::TargetEnv,
    pub target_version: ShadercTargetVertion,
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

impl HaShadercOptions {

    pub(crate) fn to_shaderc_options(&self) -> Result<shaderc::CompileOptions, ShaderCompileError> {

        let mut options = shaderc::CompileOptions::new()
            .ok_or(ShaderCompileError::CompileOptionConflict)?;

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
pub enum HaShaderOptimalLevel {
    Unoptimal,
    CodeSizeOptimal,
    PerformanceOptimal,
}

impl HaShaderOptimalLevel {

    pub fn to_shaderc_option(&self) -> shaderc::OptimizationLevel {
        match self {
            | HaShaderOptimalLevel::Unoptimal          => shaderc::OptimizationLevel::Zero,
            | HaShaderOptimalLevel::CodeSizeOptimal    => shaderc::OptimizationLevel::Size,
            | HaShaderOptimalLevel::PerformanceOptimal => shaderc::OptimizationLevel::Performance,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HaShaderDebugPattern {
    Disable,
    Warning,
    SuppressWarning,
    Error,
}

impl HaShaderDebugPattern {

    pub fn set_shaderc_option(&self, options: &mut HaShadercOptions) {
        match self {
            | HaShaderDebugPattern::Disable => {
                // leave it empty
            },
            | HaShaderDebugPattern::Warning => {
                options.debug_info = true;
            },
            | HaShaderDebugPattern::SuppressWarning => {
                options.debug_info       = true;
                options.suppress_warning = true;
            },
            | HaShaderDebugPattern::Error => {
                options.debug_info    = true;
                options.error_warning = true;
            }
        }
    }
}
