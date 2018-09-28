
pub use self::buffer::{
    HaHostBufferAllocator,     // host
    HaCachedBufferAllocator,   // cached
    HaDeviceBufferAllocator,   // device
    HaStagingBufferAllocator,  // staging
    HaBufferAllocatorAbstract, // traits
};
pub use self::descriptor::HaDescriptorAllocator;
pub use self::image::HaImageAllocator;

pub(crate) use self::buffer::BufferAllocateInfos;
pub(crate) use self::buffer::BufferConfigsAllocatable;

mod buffer;
mod descriptor;
mod image;
