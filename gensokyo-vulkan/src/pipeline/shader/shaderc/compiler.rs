
use shaderc;

use crate::pipeline::shader::shaderc::options::GsShadercOptions;
use crate::pipeline::shader::shaderc::vulkan::VulkanShadercOptions;
use crate::error::{ VkResult, VkError };

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

    pub fn setup(prefab: ShaderCompilePrefab) -> VkResult<GsShaderCompiler> {

        let compiler = shaderc::Compiler::new()
            .ok_or(VkError::shaderc("Failed to initialize shader compiler."))?;
        let options = prefab.options();

        let shader_compiler = GsShaderCompiler {
            compiler, options,
        };

        Ok(shader_compiler)
    }

    pub fn from_configuration(configuration: ShadercConfiguration) -> VkResult<GsShaderCompiler> {

        let compiler = match configuration {
            | ShadercConfiguration::Vulkan(options) => {
                let compiler = shaderc::Compiler::new()
                    .ok_or(VkError::shaderc("Failed to initialize shader compiler."))?;
                let options = options.to_shaderc_options();

                GsShaderCompiler { compiler, options }
            }
        };

        Ok(compiler)
    }

    pub fn compile_source_into_spirv(&mut self, source: &str, kind: shaderc::ShaderKind, input_name: &str, entry_name: &str) -> VkResult<Vec<u8>> {

        let compile_options = self.options.to_shaderc_options()?;

        // FIXME: The compiler seems failed to output the debug error.
        let result = self.compiler.compile_into_spirv(source, kind, input_name, entry_name, Some(&compile_options))
            .map_err(|e| VkError::shaderc(format!("Failed to compile {}({})", input_name, e)))?;

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
