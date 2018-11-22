
use std::error::Error;
use std::fmt;

use buffer::BufferError;
use image::ImageError;
use command::CommandError;
use descriptor::DescriptorError;
use sync::SyncError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AllocatorError {

    Buffer(BufferError),
    Memory(MemoryError),
    Command(CommandError),
    Image(ImageError),
    Sync(SyncError),
    Descriptor(DescriptorError),
    UnsupportBufferUsage,
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
            | AllocatorError::Buffer(ref e)     => e.to_string(),
            | AllocatorError::Memory(ref e)     => e.to_string(),
            | AllocatorError::Command(ref e)    => e.to_string(),
            | AllocatorError::Image(ref e)      => e.to_string(),
            | AllocatorError::Sync(ref e)       => e.to_string(),
            | AllocatorError::Descriptor(ref e) => e.to_string(),
            | AllocatorError::UnsupportBufferUsage => {
                String::from("The type of buffer is not support on this allocator.")
            },
        };
        write!(f, "{}", description)
    }
}

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
