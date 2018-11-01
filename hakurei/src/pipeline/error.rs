
use std::fmt;
use std::error::Error;

use resources::error::FramebufferError;
use utility::shaderc::ShaderCompileError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ShaderError {
    SpirvReadError,
    SourceReadError,
    ModuleCreationError,
}

impl Error for ShaderError {}
impl fmt::Display for ShaderError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ShaderError::SpirvReadError      => "Unable to locate Shader Source.",
            | ShaderError::SourceReadError     => "Unable to read Shader Source code to spirv.",
            | ShaderError::ModuleCreationError => "Failed to create Shader Module.",
        };

        write!(f, "{}", description)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PipelineError {

    Shader(ShaderError),
    Shaderc(ShaderCompileError),
    RenderPass(RenderPassError),
    PipelineCreationError,
    LayoutCreationError,
    PipelineTakeError,
}

impl Error for PipelineError {}
impl fmt::Display for PipelineError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            | PipelineError::Shader(ref e)         => write!(f, "{}", e),
            | PipelineError::Shaderc(ref e)        => write!(f, "{}", e),
            | PipelineError::RenderPass(ref e)     => write!(f, "{}", e),
            | PipelineError::PipelineCreationError => write!(f, "Failed to create Pipeline."),
            | PipelineError::LayoutCreationError   => write!(f, "Failed to create Pipeline Layout."),
            | PipelineError::PipelineTakeError     => write!(f, "The pipeline has been taken."),
        }
    }
}

impl_from_err!(Shader(ShaderError)         -> PipelineError);
impl_from_err!(Shaderc(ShaderCompileError) -> PipelineError);
impl_from_err!(RenderPass(RenderPassError) -> PipelineError);


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RenderPassError {

    RenderPassCreationError,
    Framebuffer(FramebufferError),
}

impl Error for RenderPassError {}
impl fmt::Display for RenderPassError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            | RenderPassError::RenderPassCreationError  => write!(f, "Failed to create Render Pass object."),
            | RenderPassError::Framebuffer(ref e)       => write!(f, "{}", e),
        }
    }
}
