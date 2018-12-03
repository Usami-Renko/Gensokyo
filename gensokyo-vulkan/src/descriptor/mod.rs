
pub use self::binding::{
    DescriptorWriteInfo,
    DescriptorBindingInfo, DescriptorBindingContent,
    DescriptorBufferBindingInfo, DescriptorImageBindingInfo,
    DescriptorBufferBindableTarget, DescriptorImageBindableTarget,
};
pub use self::layout::{ GsDescriptorSetLayout, DescriptorSetLayoutInfo };
pub use self::set::{ GsDescriptorSet, DescriptorSetConfig, DescriptorSet };
pub use self::pool::{ GsDescriptorPool, DescriptorPoolInfo };
pub use self::entity::DescriptorSetEntity;
pub use self::enums::{ GsDescriptorType, BufferDescriptorType, ImageDescriptorType };
pub use self::repository::GsDescriptorRepository;
pub use self::error::DescriptorError;

pub mod allocator;

mod pool;
mod layout;
mod set;
mod entity;
mod binding;
mod enums;
mod repository;
mod error;
