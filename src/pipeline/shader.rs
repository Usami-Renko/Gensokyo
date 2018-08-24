
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use pipeline::error::ShaderError;

use std::path::{ Path, PathBuf };
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

pub struct HaShaderInfo {

    stage: ShaderStageType,
    path  : PathBuf,
    main  : CString,
}

impl HaShaderInfo {

    pub fn setup(stage: ShaderStageType, source_path: &Path, main_func: Option<&str>) -> HaShaderInfo {
        let main = main_func.and_then(|s| Some(CString::new(s).unwrap()))
            .unwrap_or(CString::new("main").unwrap());
        let path = PathBuf::from(source_path);

        HaShaderInfo { stage, path, main, }
    }

    pub fn build(&self, device: &HaLogicalDevice) -> Result<HaShaderModule, ShaderError> {

        let codes = self.load_source()?;
        let handle = self.create_module(device, &codes)?;

        let shader_module = HaShaderModule {
            handle,
            stage: self.stage,
            main: self.main.clone()
        };
        Ok(shader_module)
    }

    fn load_source(&self) -> Result<Vec<u8>, ShaderError> {

        let spv = File::open(self.path.to_owned())
            .or(Err(ShaderError::SourceNotFoundError))?;
        let bytes: Vec<u8> = spv.bytes()
            .filter_map(|byte| byte.ok()).collect();

        Ok(bytes)
    }

    fn create_module(&self, device: &HaLogicalDevice, codes: &Vec<u8>) -> Result<vk::ShaderModule, ShaderError> {

        let module_create_info = vk::ShaderModuleCreateInfo {
            s_type    : vk::StructureType::ShaderModuleCreateInfo,
            p_next    : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
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
    pub(super) fn stage(&self) -> vk::ShaderStageFlags {
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


pub struct HaShaderModule {

    pub(in super) main   : CString,
    pub(in super) stage  : ShaderStageType,
    pub(in super) handle : vk::ShaderModule,
}

impl HaShaderModule {

    pub fn info(&self) -> vk::PipelineShaderStageCreateInfo {
        vk::PipelineShaderStageCreateInfo {
            s_type : vk::StructureType::PipelineShaderStageCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags  : vk::PipelineShaderStageCreateFlags::empty(),
            stage  : self.stage.stage(),
            module : self.handle,
            p_name : self.main.as_ptr(),
            // TODO: This field has not been covered.
            p_specialization_info: ptr::null(),
        }
    }

    pub fn cleanup(&self, device: &HaLogicalDevice) {

        unsafe {
            device.handle.destroy_shader_module(self.handle, None);
        }
    }
}
