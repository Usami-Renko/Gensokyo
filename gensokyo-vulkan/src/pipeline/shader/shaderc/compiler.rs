
use shaderc;

use pipeline::shader::shaderc::options::GsShadercOptions;
use pipeline::shader::shaderc::vulkan::VulkanShadercOptions;
use pipeline::shader::shaderc::error::ShaderCompileError;

pub struct GsShaderCompiler {

    compiler: shaderc::Compiler,
    options : GsShadercOptions,
}

pub enum ShaderCompilePrefab {
    Vulkan,
}

pub enum ShadercConfiguration {
    Vulkan(VulkanShadercOptions),
}

impl GsShaderCompiler {

    pub fn setup(prefab: ShaderCompilePrefab) -> Result<GsShaderCompiler, ShaderCompileError> {

        let compiler = shaderc::Compiler::new()
            .ok_or(ShaderCompileError::CompilerInitializeError)?;
        let options = prefab.options();

        let shader_compiler = GsShaderCompiler {
            compiler, options,
        };

        Ok(shader_compiler)
    }

    pub fn setup_from_configuration(configuration: ShadercConfiguration) -> Result<GsShaderCompiler, ShaderCompileError> {

        let compiler = match configuration {
            | ShadercConfiguration::Vulkan(options) => {
                let mut compiler = shaderc::Compiler::new()
                    .ok_or(ShaderCompileError::CompilerInitializeError)?;
                let options = options.to_shaderc_options();

                GsShaderCompiler {
                    compiler, options
                }
            }
        };

        Ok(compiler)
    }

    pub fn compile_source_into_spirv(&mut self, source: &str, kind: shaderc::ShaderKind, input_name: &str, entry_name: &str) -> Result<Vec<u8>, ShaderCompileError> {

        let compile_options = self.options.to_shaderc_options()?;

        // FIXME: The compiler seems failed to output the debug error.
        let result = self.compiler.compile_into_spirv(source, kind, input_name, entry_name, Some(&compile_options))
            .or_else(|_| {

                Err(ShaderCompileError::CompileFailedError(input_name.to_owned()))
            })?;

        if result.get_num_warnings() > 0 {
            println!("{}: {}", input_name, result.get_warning_messages());
        }

        let spirv = result.as_binary_u8().to_owned();
        Ok(spirv)
    }
}

impl ShaderCompilePrefab {

    fn options(&self) -> GsShadercOptions {
        match self {
            | ShaderCompilePrefab::Vulkan => {
                VulkanShadercOptions::default().to_shaderc_options()
            },
        }
    }
}
