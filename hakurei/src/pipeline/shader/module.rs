
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use pipeline::error::ShaderError;

use utility::marker::{ VulkanFlags, VulkanEnum };

use std::path::{ Path, PathBuf };
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

pub struct HaShaderInfo {

    stage: ShaderStageFlag,
    path  : PathBuf,
    main  : CString,
}

impl HaShaderInfo {

    pub fn setup(stage: ShaderStageFlag, source_path: &Path, main_func: Option<&str>) -> HaShaderInfo {
        let main = main_func.and_then(|s| Some(CString::new(s).unwrap()))
            .unwrap_or(CString::new("main").unwrap());
        let path = PathBuf::from(source_path);

        HaShaderInfo { stage, path, main, }
    }

    pub fn build(&self, device: &HaLogicalDevice) -> Result<HaShaderModule, ShaderError> {

        let codes = self.load_source_from_bytes()?;
        let handle = self.create_module(device, &codes)?;

        let shader_module = HaShaderModule {
            handle,
            stage: self.stage,
            main: self.main.clone()
        };
        Ok(shader_module)
    }

    fn load_source_from_bytes(&self) -> Result<Vec<u8>, ShaderError> {

        let spv = File::open(self.path.to_owned())
            .or(Err(ShaderError::SourceNotFoundError))?;
        let bytes: Vec<u8> = spv.bytes()
            .filter_map(|byte| byte.ok()).collect();

        Ok(bytes)
    }

    #[allow(dead_code)]
    fn load_source_from_string(&self) -> Result<Vec<u8>, ShaderError> {
        unimplemented!()
    }

    fn create_module(&self, device: &HaLogicalDevice, codes: &Vec<u8>) -> Result<vk::ShaderModule, ShaderError> {

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ShaderStageFlag {

    /// VertexStage specifies the vertex stage.
    VertexStage,
    /// TessellationControlStage the tessellation control stage.
    TessellationControlStage,
    /// TessellationEvaluationStage specifies the tessellation evaluation stage.
    TessellationEvaluationStage,
    /// GeometryStage specifies the geometry stage.
    GeometryStage,
    /// FragmentStage specifies the fragment stage.
    FragmentStage,
    /// ComputeStage specifies the compute stage.
    ComputeStage,
    /// AllGraphicsStage is a combination of bits used as shorthand to specify all graphics stages (excluding the compute stage).
    AllGraphicsStage,
    /// AllStage is a combination of bits used as shorthand to specify all shader stages supported by the device,
    /// including all additional stages which are introduced by extensions.
    AllStage,
}

impl VulkanFlags for [ShaderStageFlag] {
    type FlagType = vk::ShaderStageFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::ShaderStageFlags::empty(), |acc, flag| {
            match *flag {
                | ShaderStageFlag::VertexStage                 => acc | vk::SHADER_STAGE_VERTEX_BIT,
                | ShaderStageFlag::GeometryStage               => acc | vk::SHADER_STAGE_GEOMETRY_BIT,
                | ShaderStageFlag::TessellationControlStage    => acc | vk::SHADER_STAGE_TESSELLATION_CONTROL_BIT,
                | ShaderStageFlag::TessellationEvaluationStage => acc | vk::SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
                | ShaderStageFlag::FragmentStage               => acc | vk::SHADER_STAGE_FRAGMENT_BIT,
                | ShaderStageFlag::ComputeStage                => acc | vk::SHADER_STAGE_COMPUTE_BIT,
                | ShaderStageFlag::AllGraphicsStage            => acc | vk::SHADER_STAGE_ALL_GRAPHICS,
                | ShaderStageFlag::AllStage                    => acc | vk::SHADER_STAGE_ALL,
            }
        })
    }
}

impl VulkanEnum for ShaderStageFlag {
    type EnumType = vk::ShaderStageFlags;

    fn value(&self) -> Self::EnumType {
        match *self {
            | ShaderStageFlag::VertexStage                 => vk::SHADER_STAGE_VERTEX_BIT,
            | ShaderStageFlag::GeometryStage               => vk::SHADER_STAGE_GEOMETRY_BIT,
            | ShaderStageFlag::TessellationControlStage    => vk::SHADER_STAGE_TESSELLATION_CONTROL_BIT,
            | ShaderStageFlag::TessellationEvaluationStage => vk::SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
            | ShaderStageFlag::FragmentStage               => vk::SHADER_STAGE_FRAGMENT_BIT,
            | ShaderStageFlag::ComputeStage                => vk::SHADER_STAGE_COMPUTE_BIT,
            | ShaderStageFlag::AllGraphicsStage            => vk::SHADER_STAGE_ALL_GRAPHICS,
            | ShaderStageFlag::AllStage                    => vk::SHADER_STAGE_ALL,
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

    pub(crate) fn cleanup(&self, device: &HaLogicalDevice) {

        unsafe {
            device.handle.destroy_shader_module(self.handle, None);
        }
    }
}
