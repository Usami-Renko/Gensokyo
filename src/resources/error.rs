
use std::fmt;
use std::error::Error;

/// possible error may occur during the creation of vk::Framebuffer.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FramebufferError {

    FramebufferCreationError,
}

impl Error for FramebufferError {}
impl fmt::Display for FramebufferError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | FramebufferError::FramebufferCreationError => "Failed to create Framebuffer Object.",
        };
        write!(f, "{}", description)
    }
}

/// possible error may occur during the use of vk::CommandPool and vk::CommandBuffer.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommandError {

    QueueFamilyUnavailable,
    QueueSubmitError,
    PoolCreationError,
    BufferAllocateError,
    RecordBeginError,
    RecordEndError,
}

impl Error for CommandError {}
impl fmt::Display for CommandError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | CommandError::QueueFamilyUnavailable => "Graphics Queue Family is not available.",
            | CommandError::QueueSubmitError       => "Failed to submit command to device.",
            | CommandError::PoolCreationError      => "Failed to create Command Pool.",
            | CommandError::BufferAllocateError    => "Failed to allocate Command Buffer.",
            | CommandError::RecordBeginError       => "Failed to begin Command Buffer recording.",
            | CommandError::RecordEndError         => "Failed to end Command Buffer recording.",
        };
        write!(f, "{}", description)
    }
}


/// possible error may occur during the use of vk::Buffer.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferError {

    BufferCreationError,
}

impl Error for BufferError {}
impl fmt::Display for BufferError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | BufferError::BufferCreationError       => "Failed to create Buffer object",

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
}

impl Error for MemoryError {}
impl fmt::Display for MemoryError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | MemoryError::NoSuitableMemoryError     => "Failed to find suitable memory type for Buffer memory allocation.",
            | MemoryError::MemoryNotYetAllocateError => "The memory must be allocated before transfering data.",
            | MemoryError::AllocateMemoryError       => "Failed to allocate memory for buffer object.",
            | MemoryError::BindMemoryError           => "Failed to bind memory to buffer object.",
            | MemoryError::MapMemoryError            => "Failed to map memory for buffer object.",
            | MemoryError::FlushMemoryError          => "Failed to flush certain range of memory.",
        };
        write!(f, "{}", description)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AllocatorError {

    Buffer(BufferError),
    Memory(MemoryError),
    Command(CommandError),
    NoAvailableBufferAttach,
    MemoryNotYetAllocated,
}

impl From<CommandError> for AllocatorError {

    fn from(error: CommandError) -> Self {
        AllocatorError::Command(error)
    }
}

impl Error for AllocatorError {}
impl fmt::Display for AllocatorError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | AllocatorError::Buffer(ref e)  => e.to_string(),
            | AllocatorError::Memory(ref e)  => e.to_string(),
            | AllocatorError::Command(ref e) => e.to_string(),
            | AllocatorError::NoAvailableBufferAttach => {
                String::from("There must be buffer attached to allocator before allocate memory.")
            },
            | AllocatorError::MemoryNotYetAllocated   => {
                String::from("The memory is not allocated yet. Memory must be allocated first before using it.")
            },
        };
        write!(f, "{}", description)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
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
