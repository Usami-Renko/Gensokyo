
use ash::vk;

use core::device::HaLogicalDevice;

use pipeline::error::ShaderError;

use constant::VERBOSE;

use std::path::Path;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

pub struct HaShaderInfo<'p> {

    usage : ShaderStageType,
    path  : &'p Path,
    main  : Option<CString>,
}

impl HaShaderInfo {

    pub fn setup(usage: ShaderStageType, path: &Path, main_func: Option<&str>) -> HaShaderInfo {
        let main = main_func.and_then(|s| Some(CString::new(s).unwrap()));
        HaShaderInfo { usage, path, main, }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ShaderStageType {

    VertexStage,
    GeometryStage,
    TessellationControlStage,
    TessellationEvaluationStage,
    FragmentStage,
    ComputeStage,
    AllGraphicsStage,
    AllStage,
}

impl ShaderStageType {
    fn module(&self) -> vk::ShaderStageFlags {
        match *self {
            | ShaderStageType::VertexStage                 => vk::SHADER_STAGE_VERTEX_BIT,
            | ShaderStageType::GeometryStage               => vk::SHADER_STAGE_GEOMETRY_BIT,
            | ShaderStageType::TessellationControlStage    => vk::SHADER_STAGE_TESSELLATION_CONTROL_BIT,
            | ShaderStageType::TessellationEvaluationStage => vk::SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
            | ShaderStageType::FragmentStage               => vk::SHADER_STAGE_FRAGMENT_BIT,
            | ShaderStageType::ComputeStage                => vk::SHADER_STAGE_COMPUTE_BIT,
            | ShaderStageType::AllGraphicsStage            => vk::SHADER_STAGE_ALL_GRAPHICS,
            | ShaderStageType::AllStage                    => vk::SHADER_STAGE_ALL,
        }
    }
}

struct ShaderBuilder<'d, 'p> {

    device : &'d HaLogicalDevice,
    info   : HaShaderInfo<'p>,
}

impl ShaderBuilder {

    pub fn setup(device: &HaLogicalDevice, info: HaShaderInfo) -> ShaderBuilder {
        ShaderBuilder { device, info, }
    }

    pub fn build(&self) -> Result<HaShaderModule, ShaderError> {

        let codes = self.load_source()?;
        let handle = self.create_module(&codes)?;

        let info = vk::PipelineShaderStageCreateInfo {
            s_type : vk::StructureType::PipelineShaderStageCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags  : vk::PipelineShaderStageCreateFlags::empty(),
            stage  : self.info.usage.module(),
            module : handle,
            p_name : self.info.main.as_ptr(),
            // TODO: This field has not been covered.
            p_specialization_info: ptr::null(),
        };

        let shader_module = HaShaderModule {
            handle,
            info,
        };

        Ok(shader_module)
    }

    fn load_source(&self) -> Result<Vec<u8>, ShaderError> {

        let spv = File::open(self.info.path)
            .or(Err(ShaderError::SourceNotFoundError))?;
        let bytes: Vec<u8> = spv.bytes()
            .filter_map(|byte| byte.ok()).collect();

        Ok(bytes)
    }

    fn create_module(&self, codes: &Vec<u8>) -> Result<vk::ShaderModule, ShaderError> {

        let module_create_info = vk::ShaderModuleCreateInfo {
            s_type    : vk::StructureType::ShaderModuleCreateInfo,
            p_next    : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags     : vk::ShaderModuleCreateFlags::empty(),
            code_size : codes.len(),
            p_code    : codes.as_ptr() as *const u32,
        };

        unsafe {
            self.device.handle.create_shader_module(&module_create_info, None)
                .or(Err(ShaderError::ModuleCreationError))
        }
    }

}

pub struct HaShaderModule {

    handle : vk::ShaderModule,
    info   : vk::PipelineShaderStageCreateInfo,
}

impl HaShaderModule {

    pub fn cleanup(&self, device: &HaLogicalDevice) {

        unsafe {
            device.handle.destroy_shader_module(self.handle, None);
        }

        if VERBOSE {
            println!("[Info] Shader Module had been destroy.");
        }
    }
}
