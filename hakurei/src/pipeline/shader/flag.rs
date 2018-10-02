
use ash::vk;
use shaderc;
use shaderc::ShaderKind;

use utility::marker::{ VulkanEnum, VulkanFlags };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ShaderStageFlag {

    /// VertexStage specifies the vertex stage.
    VertexStage,
    /// TessControlStage the tessellation control stage.
    TessControlStage,
    /// TessEvaluationStage specifies the tessellation evaluation stage.
    TessEvaluationStage,
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
                | ShaderStageFlag::VertexStage         => acc | vk::SHADER_STAGE_VERTEX_BIT,
                | ShaderStageFlag::GeometryStage       => acc | vk::SHADER_STAGE_GEOMETRY_BIT,
                | ShaderStageFlag::TessControlStage    => acc | vk::SHADER_STAGE_TESSELLATION_CONTROL_BIT,
                | ShaderStageFlag::TessEvaluationStage => acc | vk::SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
                | ShaderStageFlag::FragmentStage       => acc | vk::SHADER_STAGE_FRAGMENT_BIT,
                | ShaderStageFlag::ComputeStage        => acc | vk::SHADER_STAGE_COMPUTE_BIT,
                | ShaderStageFlag::AllGraphicsStage    => acc | vk::SHADER_STAGE_ALL_GRAPHICS,
                | ShaderStageFlag::AllStage            => acc | vk::SHADER_STAGE_ALL,
            }
        })
    }
}

impl VulkanEnum for ShaderStageFlag {
    type EnumType = vk::ShaderStageFlags;

    fn value(&self) -> Self::EnumType {
        match self {
            | ShaderStageFlag::VertexStage         => vk::SHADER_STAGE_VERTEX_BIT,
            | ShaderStageFlag::GeometryStage       => vk::SHADER_STAGE_GEOMETRY_BIT,
            | ShaderStageFlag::TessControlStage    => vk::SHADER_STAGE_TESSELLATION_CONTROL_BIT,
            | ShaderStageFlag::TessEvaluationStage => vk::SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
            | ShaderStageFlag::FragmentStage       => vk::SHADER_STAGE_FRAGMENT_BIT,
            | ShaderStageFlag::ComputeStage        => vk::SHADER_STAGE_COMPUTE_BIT,
            | ShaderStageFlag::AllGraphicsStage    => vk::SHADER_STAGE_ALL_GRAPHICS,
            | ShaderStageFlag::AllStage            => vk::SHADER_STAGE_ALL,
        }
    }
}

impl ShaderStageFlag {

    pub(crate) fn to_shaderc_kind(&self) -> ShaderKind {
        match self {
            | ShaderStageFlag::VertexStage         => shaderc::ShaderKind::Vertex,
            | ShaderStageFlag::GeometryStage       => shaderc::ShaderKind::Geometry,
            | ShaderStageFlag::TessControlStage    => shaderc::ShaderKind::TessControl,
            | ShaderStageFlag::TessEvaluationStage => shaderc::ShaderKind::TessEvaluation,
            | ShaderStageFlag::FragmentStage       => shaderc::ShaderKind::Fragment,
            | ShaderStageFlag::ComputeStage        => shaderc::ShaderKind::Compute,
            | ShaderStageFlag::AllGraphicsStage
            | ShaderStageFlag::AllStage => {
                shaderc::ShaderKind::InferFromSource
            },
        }
    }
}
