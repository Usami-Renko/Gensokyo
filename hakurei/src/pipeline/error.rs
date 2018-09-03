
use std::fmt;
use std::error::Error;

use resources::error::FramebufferError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ShaderError {

    SourceNotFoundError,
    ModuleCreationError,
}

impl Error for ShaderError {}
impl fmt::Display for ShaderError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ShaderError::SourceNotFoundError => "Unable to locate Shader Source.",
            | ShaderError::ModuleCreationError => "Failed to create Shader Module.",
        };

        write!(f, "{}", description)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PipelineError {

    Shader(ShaderError),
    RenderPass(RenderPassError),
    PipelineCreationError,
    LayoutCreationError,
}

impl Error for PipelineError {}
impl fmt::Display for PipelineError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            | PipelineError::Shader(ref e)         => write!(f, "{}", e),
            | PipelineError::RenderPass(ref e)     => write!(f, "{}", e),
            | PipelineError::PipelineCreationError => write!(f, "Failed to create Pipeline."),
            | PipelineError::LayoutCreationError   => write!(f, "Failed to create Pipeline Layout."),
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

