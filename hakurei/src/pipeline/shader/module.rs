
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use pipeline::shader::ShaderStageFlag;
use pipeline::error::{ ShaderError, PipelineError };

use utility::shaderc::HaShaderCompiler;
use utility::marker::VulkanEnum;

use std::path::{ Path, PathBuf };
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

pub struct HaShaderInfo {

    stage: ShaderStageFlag,
    path : PathBuf,
    main : String,

    pattern : ShaderSourcePattern,
    tag_name: Option<String>,
}

enum ShaderSourcePattern {
    SourceCode,
    SprivCode,
}

impl HaShaderInfo {

    pub fn from_source(stage: ShaderStageFlag, source_path: &Path, main_func: Option<&str>, tag_name: &str) -> HaShaderInfo {

        let path = PathBuf::from(source_path);
        let main = main_func
            .and_then(|m| Some(m.to_owned()))
            .unwrap_or(String::from("main"));

        HaShaderInfo {
            stage, path, main,
            pattern : ShaderSourcePattern::SourceCode,
            tag_name: Some(tag_name.to_owned()),
        }
    }

    pub fn from_spirv(stage: ShaderStageFlag, spirv_path: &Path, main_func: Option<&str>) -> HaShaderInfo {

        let path = PathBuf::from(spirv_path);
        let main = main_func
            .and_then(|m| Some(m.to_owned()))
            .unwrap_or(String::from("main"));

        HaShaderInfo {
            stage, path, main,
            pattern : ShaderSourcePattern::SprivCode,
            tag_name: None,
        }
    }

    pub fn build(&self, device: &HaDevice, compiler: &mut HaShaderCompiler) -> Result<HaShaderModule, PipelineError> {

        let codes = match self.pattern {
            | ShaderSourcePattern::SourceCode => {
                let source = load_to_str(&self.path)?;
                let kind = self.stage.to_shaderc_kind();

                compiler.compile_source_into_spirv(&source, kind, &self.tag_name.as_ref().unwrap(), &self.main)?
            },
            | ShaderSourcePattern::SprivCode => {
                load_spriv_bytes(&self.path)?
            },
        };

        let handle = self.create_module(device, &codes)?;

        let shader_module = HaShaderModule {
            handle,
            stage: self.stage,
            main : CString::new(self.main.as_str()).unwrap(),
        };
        Ok(shader_module)
    }

    fn create_module(&self, device: &HaDevice, codes: &Vec<u8>) -> Result<vk::ShaderModule, ShaderError> {

        let module_create_info = vk::ShaderModuleCreateInfo {
            s_type    : vk::StructureType::ShaderModuleCreateInfo,
            p_next    : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags     : vk::ShaderModuleCreateFlags::empty(),
            code_size : codes.len(),
            p_code    : codes.as_ptr() as *const u32,
        };

        unsafe {
            device.handle.create_shader_module(&module_create_info, None)
                .or(Err(ShaderError::ModuleCreationError))
        }
    }
}


pub struct HaShaderModule {

    pub(super) main   : CString,
    pub(super) stage  : ShaderStageFlag,
    pub(super) handle : vk::ShaderModule,
}

impl HaShaderModule {

    pub(crate) fn info(&self) -> vk::PipelineShaderStageCreateInfo {
        vk::PipelineShaderStageCreateInfo {
            s_type : vk::StructureType::PipelineShaderStageCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags  : vk::PipelineShaderStageCreateFlags::empty(),
            stage  : self.stage.value(),
            module : self.handle,
            p_name : self.main.as_ptr(),
            // TODO: This field has not been covered.
            p_specialization_info: ptr::null(),
        }
    }

    pub(crate) fn cleanup(&self, device: &HaDevice) {

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
        .collect::<Vec<_>>();

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
