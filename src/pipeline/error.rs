
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

        write!(f, "Error: {}", description)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PipelineError {

    PipelineCreationError,
    LayoutCreationError,
}

impl Error for PipelineError {}
impl fmt::Display for PipelineError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | PipelineError::PipelineCreationError => "Failed to create Pipeline.",
            | PipelineError::LayoutCreationError   => "Failed to create Pipeline Layout.",
        };

        write!(f, "Error: {}", description)
    }
}

