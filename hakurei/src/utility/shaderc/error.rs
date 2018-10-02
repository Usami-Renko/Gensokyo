
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShaderCompileError {

    CompilerInitializeError,
    CompileOptionConflict,
    CompileFailedError(String),
}

impl Error for ShaderCompileError {}
impl fmt::Display for ShaderCompileError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            | ShaderCompileError::CompilerInitializeError => write!(f, "Failed to initialize shader compiler."),
            | ShaderCompileError::CompileOptionConflict   => write!(f, "There are conflict in Shader Compile Options."),
            | ShaderCompileError::CompileFailedError(tag_name) => {
                write!(f, "Failed to compile source codes({}) to spirv codes.", tag_name)
            },
        }
    }
}
