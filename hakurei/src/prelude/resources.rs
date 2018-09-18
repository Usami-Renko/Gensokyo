
pub use resources::allocator::{
    ResourceGenerator,     // generator
    HaBufferAllocator,     // buffer
    HaDescriptorAllocator, // descriptor
    HaImageAllocator,      // image
};

pub use resources::buffer::{
    BufferCreateFlag, BufferUsageFlag,       // flag
    BufferConfig, BufferItem, BufferSubItem, // item
};

pub use resources::command::{
    HaCommandBuffer, CommandBufferUsage,       // buffer
    HaCommandPool, CommandPoolFlag,            // pool
    HaCommandRecorder, CommandBufferUsageFlag, // record
};

pub use resources::descriptor::{
    DescriptorSetConfig, DescriptorItem, DescriptorSetItem,  // item
    DescriptorBufferBindingInfo, DescriptorImageBindingInfo, // item
    DescriptorPoolFlag, // pool
    HaDescriptorSetLayout, DescriptorSetLayoutFlag, BufferDescriptorType, ImageDescriptorType, // layout
};

// currently no framebuffer API is public
pub use resources::framebuffer::{};

pub use resources::image::{
    ImageDescInfo,     // image
    ImageViewDescInfo, // view
    ImageLayout, ImageAspectFlag, ImageCreateFlag, ImageUsageFlag, // flag
    ImageViewItem, // item
    HaSampler, SamplerDescInfo, // sampler
    ImageType, ImageViewType, ImageTiling, Filter, MipmapMode, CompareOp, BorderColor, // enums
};

pub use resources::memory::{
    MemoryPropertyFlag
};

pub use resources::repository::{
    HaBufferRepository, CmdVertexBindingInfos, CmdIndexBindingInfo, // buffer
    HaDescriptorRepository, CmdDescriptorBindingInfos, // descriptor
    HaImageRepository, // image
};
