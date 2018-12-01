
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DescriptorError {

    PoolCreationError,
    SetLayoutCreationError,
    SetAllocateError,
}

impl Error for DescriptorError {}
impl fmt::Display for DescriptorError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | DescriptorError::PoolCreationError      => "Failed to create Descriptor Pool.",
            | DescriptorError::SetLayoutCreationError => "Failed to create Descriptor Set Layout.",
            | DescriptorError::SetAllocateError       => "Failed to allocate Descriptor Set.",
        };
        write!(f, "{}", description)
    }
}
