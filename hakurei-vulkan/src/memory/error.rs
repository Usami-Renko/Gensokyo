
use std::error::Error;
use std::fmt;

/// possible error may occur during the use of vk::DeviceMemory.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MemoryError {

    NoSuitableMemoryError,
    MemoryNotYetAllocateError,
    AllocateMemoryError,
    BindMemoryError,
    MapMemoryError,
    FlushMemoryError,
    BufferToBufferCopyError,
    AllocateInfoMissing,
    MemoryUnableToUpdate,
}

impl Error for MemoryError {}
impl fmt::Display for MemoryError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | MemoryError::NoSuitableMemoryError     => "Failed to find suitable memory type for Buffer memory allocation.",
            | MemoryError::MemoryNotYetAllocateError => "The memory must be allocated before transfering data.",
            | MemoryError::AllocateMemoryError       => "Failed to allocate memory for buffer or image object.",
            | MemoryError::BindMemoryError           => "Failed to bind memory to buffer object.",
            | MemoryError::MapMemoryError            => "Failed to map memory for buffer object.",
            | MemoryError::FlushMemoryError          => "Failed to flush certain range of memory.",
            | MemoryError::BufferToBufferCopyError   => "Failed to copy buffer from another buffer",
            | MemoryError::AllocateInfoMissing       => "The allocate info is missing, check the logic of code.",
            | MemoryError::MemoryUnableToUpdate      => "This type of memory is not support to use updater.",
        };
        write!(f, "{}", description)
    }
}
