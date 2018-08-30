
use std::fmt;
use std::error::Error;

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
    RenderPassCreationError,
    PipelineCreationError,
    LayoutCreationError,
}

impl Error for PipelineError {}
impl fmt::Display for PipelineError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            | PipelineError::Shader(ref e)           => write!(f, "{}", e.to_string()),
            | PipelineError::RenderPassCreationError => write!(f, "Failed to create RenderPass object"),
            | PipelineError::PipelineCreationError   => write!(f, "Failed to create Pipeline."),
            | PipelineError::LayoutCreationError     => write!(f, "Failed to create Pipeline Layout."),
        }
    }
}

