
use std::error::Error;
use std::fmt;

use gsma::impl_from_err;

use crate::buffer::BufferError;
use crate::image::ImageError;
use crate::command::CommandError;
use crate::descriptor::DescriptorError;
use crate::sync::SyncError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AllocatorError {

    Buffer(BufferError),
    Memory(MemoryError),
    Command(CommandError),
    Image(ImageError),
    Sync(SyncError),
    Descriptor(DescriptorError),
    UnsupportBufferUsage,
    DuplicateAppendImage,
}

impl_from_err!(Buffer(BufferError)         -> AllocatorError);
impl_from_err!(Memory(MemoryError)         -> AllocatorError);
impl_from_err!(Command(CommandError)       -> AllocatorError);
impl_from_err!(Image(ImageError)           -> AllocatorError);
impl_from_err!(Sync(SyncError)             -> AllocatorError);
impl_from_err!(Descriptor(DescriptorError) -> AllocatorError);

impl Error for AllocatorError {}
impl fmt::Display for AllocatorError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | AllocatorError::Buffer(e)     => e.to_string(),
            | AllocatorError::Memory(e)     => e.to_string(),
            | AllocatorError::Command(e)    => e.to_string(),
            | AllocatorError::Image(e)      => e.to_string(),
            | AllocatorError::Sync(e)       => e.to_string(),
            | AllocatorError::Descriptor(e) => e.to_string(),
            | AllocatorError::UnsupportBufferUsage => {
                String::from("The type of buffer is not support on this allocator.")
            },
            | AllocatorError::DuplicateAppendImage => {
                String::from("Duplicate append image to allocator.")
            },
        };
        write!(f, "{}", description)
    }
}

/// possible error may occur during the use of vk::DeviceMemory.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MemoryError {

    NoSuitableMemoryError,
    AllocateMemoryError,
    BindMemoryError,
    MapMemoryError,
    MemoryPtrInvalidError,
    FlushMemoryError,
    MemoryUnableToUpdate,
}

impl Error for MemoryError {}
impl fmt::Display for MemoryError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | MemoryError::NoSuitableMemoryError     => "Failed to find suitable memory type for Buffer memory allocation.",
            | MemoryError::AllocateMemoryError       => "Failed to allocate memory for buffer or image object.",
            | MemoryError::BindMemoryError           => "Failed to bind memory to buffer object.",
            | MemoryError::MapMemoryError            => "Failed to map memory for buffer object.",
            | MemoryError::MemoryPtrInvalidError     => "Failed to get mapped memory pointer.",
            | MemoryError::FlushMemoryError          => "Failed to flush certain range of memory.",
            | MemoryError::MemoryUnableToUpdate      => "This type of memory is not support to use updater.",
        };
        write!(f, "{}", description)
    }
}
