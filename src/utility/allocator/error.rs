
use std::fmt;
use std::error::Error;

use resources::error::BufferError;
use resources::error::MemoryError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AllocatorError {

    Buffer(BufferError),
    Memory(MemoryError),
    NoAvailableBufferAttach,
    MemoryAlreadyAllocated,
}

impl Error for AllocatorError {}
impl fmt::Display for AllocatorError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | AllocatorError::Buffer(ref e) => e.to_string(),
            | AllocatorError::Memory(ref e) => e.to_string(),
            | AllocatorError::NoAvailableBufferAttach => String::from("There must be buffer attached to allocator before allocate memory."),
            | AllocatorError::MemoryAlreadyAllocated  => String::from("The memory of this allocator has been allocated."),
        };
        write!(f, "{}", description)
    }
}
