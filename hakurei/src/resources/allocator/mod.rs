
pub use self::generator::ResourceGenerator;
pub use self::buffer::{
    HaDeviceBufferAllocator,   // device
    HaHostBufferAllocator,     // host
    HaBufferAllocatorAbstract, // traits
};
pub use self::descriptor::HaDescriptorAllocator;
pub use self::image::HaImageAllocator;

pub(crate) use self::buffer::DeviceBufferAllocateInfos;

mod generator;
mod buffer;
mod descriptor;
mod image;
