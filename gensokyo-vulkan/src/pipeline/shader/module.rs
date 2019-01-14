
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::pipeline::target::GsPipelineStage;
use crate::pipeline::shader::shaderc::GsShaderCompiler;
use crate::pipeline::error::{ ShaderError, PipelineError };

use std::path::{ Path, PathBuf };
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

pub struct GsShaderInfo {

    stage: GsPipelineStage,
    path : PathBuf,
    main : String,

    pattern : ShaderSourcePattern,
    tag_name: Option<String>,
}

enum ShaderSourcePattern {
    SourceCode,
    SprivCode,
}

impl GsShaderInfo {

    pub fn from_source(stage: GsPipelineStage, source_path: impl AsRef<Path>, main_func: Option<&str>, tag_name: &str) -> GsShaderInfo {

        let path = PathBuf::from(source_path.as_ref());
        let main = main_func
            .and_then(|m| Some(m.to_owned()))
            .unwrap_or(String::from("main"));

        GsShaderInfo {
            stage, path, main,
            pattern : ShaderSourcePattern::SourceCode,
            tag_name: Some(tag_name.to_owned()),
        }
    }

    pub fn from_spirv(stage: GsPipelineStage, spirv_path: impl AsRef<Path>, main_func: Option<&str>) -> GsShaderInfo {

        let path = PathBuf::from(spirv_path.as_ref());
        let main = main_func
            .and_then(|m| Some(m.to_owned()))
            .unwrap_or(String::from("main"));

        GsShaderInfo {
            stage, path, main,
            pattern : ShaderSourcePattern::SprivCode,
            tag_name: None,
        }
    }

    pub fn build(&self, device: &GsDevice, compiler: &mut GsShaderCompiler) -> Result<GsShaderModule, PipelineError> {

        use crate::pipeline::shader::shaderc::cast_shaderc_kind;

        let codes = match self.pattern {
            | ShaderSourcePattern::SourceCode => {
                let source = load_to_str(&self.path)?;
                let kind = cast_shaderc_kind(self.stage.0);
                // TODO: handle unwrap().
                let tag_name = self.tag_name.as_ref().unwrap();

                compiler.compile_source_into_spirv(&source, kind, tag_name, &self.main)?
            },
            | ShaderSourcePattern::SprivCode => {
                load_spriv_bytes(&self.path)?
            },
        };

        let handle = self.create_module(device, &codes)?;

        let shader_module = GsShaderModule {
            handle,
            stage: self.stage.0,
            // TODO: handle unwrap().
            main : CString::new(self.main.as_str()).unwrap(),
        };
        Ok(shader_module)
    }

    fn create_module(&self, device: &GsDevice, codes: &Vec<u8>) -> Result<vk::ShaderModule, ShaderError> {

        let module_create_info = vk::ShaderModuleCreateInfo {
            s_type    : vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next    : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags     : vk::ShaderModuleCreateFlags::empty(),
            code_size : codes.len(),
            p_code    : codes.as_ptr() as _,
        };

        unsafe {
            device.handle.create_shader_module(&module_create_info, None)
                .or(Err(ShaderError::ModuleCreationError))
        }
    }
}


pub struct GsShaderModule {

    main   : CString,
    stage  : vk::ShaderStageFlags,
    handle : vk::ShaderModule,
}

impl GsShaderModule {

    pub(crate) fn info(&self) -> vk::PipelineShaderStageCreateInfo {

        vk::PipelineShaderStageCreateInfo {
            s_type : vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags  : vk::PipelineShaderStageCreateFlags::empty(),
            stage  : self.stage,
            module : self.handle,
            p_name : self.main.as_ptr(),
            // TODO: This field has not been covered.
            p_specialization_info: ptr::null(),
        }
    }

    pub fn destroy(&self, device: &GsDevice) {

        unsafe {
            device.handle.destroy_shader_module(self.handle, None);
        }
    }
}


fn load_spriv_bytes(path: &PathBuf) -> Result<Vec<u8>, ShaderError> {

    let file = File::open(path.to_owned())
        .or(Err(ShaderError::SpirvReadError))?;
    let bytes = file.bytes()
        .filter_map(|byte| byte.ok())
        .collect();

    Ok(bytes)
}

fn load_to_str(path: &PathBuf) -> Result<String, ShaderError> {

    let mut file = File::open(path.to_owned())
        .or(Err(ShaderError::SourceReadError))?;
    let mut contents = String::new();
    let _size = file.read_to_string(&mut contents)
        .or(Err(ShaderError::SourceReadError))?;

    Ok(contents)
}
