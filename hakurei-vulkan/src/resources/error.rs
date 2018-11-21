
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AllocatorError {

    Buffer(BufferError),
    Memory(MemoryError),
    Command(CommandError),
    Image(ImageError),
    Sync(SyncError),
    Descriptor(DescriptorError),
    MemoryNotYetAllocated,
    DataTransferNotActivate,
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
            | AllocatorError::MemoryNotYetAllocated   => {
                String::from("The memory is not allocated yet. Memory must be allocated first before using it.")
            },
            | AllocatorError::DataTransferNotActivate => {
                String::from("The repository must be activated before making data transfer operations.")
            },
            | AllocatorError::UnsupportBufferUsage => {
                String::from("The type of buffer is not support on this allocator.")
            },
        };
        write!(f, "{}", description)
    }
}

