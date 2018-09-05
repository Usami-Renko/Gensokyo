
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
    Image(ImageError),
    MemoryNotYetAllocated,
}

impl_from_err!(Buffer(BufferError)   -> AllocatorError);
impl_from_err!(Memory(MemoryError)   -> AllocatorError);
impl_from_err!(Command(CommandError) -> AllocatorError);
impl_from_err!(Image(ImageError)     -> AllocatorError);

impl Error for AllocatorError {}
impl fmt::Display for AllocatorError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | AllocatorError::Buffer(ref e)  => e.to_string(),
            | AllocatorError::Memory(ref e)  => e.to_string(),
            | AllocatorError::Command(ref e) => e.to_string(),
            | AllocatorError::Image(ref e)   => e.to_string(),
            | AllocatorError::MemoryNotYetAllocated   => {
                String::from("The memory is not allocated yet. Memory must be allocated first before using it.")
            },
        };
        write!(f, "{}", description)
    }
}


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


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageError {
    SourceLoadError,
    ImageCreationError,
    ViewCreationError,
    Memory(MemoryError),
    NoImageAttachError,
    SamplerCreationError,
}

impl_from_err!(Memory(MemoryError) -> ImageError);

impl Error for ImageError {}
impl fmt::Display for ImageError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            | ImageError::SourceLoadError      => write!(f, "Failed to load image from source."),
            | ImageError::ImageCreationError   => write!(f, "Failed to create Image Object."),
            | ImageError::ViewCreationError    => write!(f, "Failed to create ImageView Object."),
            | ImageError::Memory(ref e)        => write!(f, "{}", e.to_string()),
            | ImageError::NoImageAttachError   => write!(f, "There must be images attached to allocator before allocate memory."),
            | ImageError::SamplerCreationError => write!(f, "Failed to create Sampler Object."),
        }
    }
}

