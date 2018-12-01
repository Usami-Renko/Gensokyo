
use std::error::Error;
use std::fmt;

/// possible error may occur during the use of vk::Buffer.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferError {

    BufferCreationError,
    NoBufferAttachError,
}

impl Error for BufferError {}
impl fmt::Display for BufferError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | BufferError::BufferCreationError => "Failed to create Buffer object",
            | BufferError::NoBufferAttachError => "There must be buffer attached to allocator before allocate memory.",
        };
        write!(f, "{}", description)
    }
}
