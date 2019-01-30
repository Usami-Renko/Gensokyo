
pub use self::binding::{
    DescriptorWriteInfo,
    DescriptorBindingInfo, DescriptorBindingContent,
    DescriptorBufferBindingInfo, DescriptorImageBindingInfo,
    DescriptorBufferBindableTarget, DescriptorImageBindableTarget,
};
pub use self::layout::GsDescriptorSetLayout;
pub use self::set::{ GsDescriptorSet, DescriptorSetConfig, DescriptorSet };
pub use self::pool::{ GsDescriptorPool, DescriptorPoolCI };
pub use self::entity::DescriptorSetEntity;
pub use self::types::{ GsDescriptorType, BufferDescriptorType, ImageDescriptorType };
pub use self::repository::GsDescriptorRepository;

pub mod allocator;

mod pool;
mod layout;
mod set;
mod entity;
mod binding;
mod types;
mod repository;
