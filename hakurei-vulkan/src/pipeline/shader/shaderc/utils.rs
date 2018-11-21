
use ash::vk;
use shaderc;

pub fn cast_shaderc_kind(stage: vk::ShaderStageFlags) -> shaderc::ShaderKind {
    match stage {
        | vk::ShaderStageFlags::VERTEX                  => shaderc::ShaderKind::Vertex,
        | vk::ShaderStageFlags::GEOMETRY                => shaderc::ShaderKind::Geometry,
        | vk::ShaderStageFlags::TESSELLATION_CONTROL    => shaderc::ShaderKind::TessControl,
        | vk::ShaderStageFlags::TESSELLATION_EVALUATION => shaderc::ShaderKind::TessEvaluation,
        | vk::ShaderStageFlags::FRAGMENT                => shaderc::ShaderKind::Fragment,
        | vk::ShaderStageFlags::COMPUTE                 => shaderc::ShaderKind::Compute,
        | vk::ShaderStageFlags::ALL_GRAPHICS
        | _ => shaderc::ShaderKind::InferFromSource,
    }
}
