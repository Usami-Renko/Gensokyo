
pub use self::layout::GsDescriptorSetLayout;
pub use self::set::{ GsDescriptorSet, DescriptorSetConfig, DescriptorSet };
pub use self::pool::{ GsDescriptorPool, DescriptorPoolCI };
pub use self::types::{ GsDescriptorType, BufferDescriptorType, ImageDescriptorType };
pub use self::repository::GsDescriptorRepository;

pub mod allocator;
pub mod binding;

mod pool;
mod layout;
mod set;
mod types;
mod repository;
