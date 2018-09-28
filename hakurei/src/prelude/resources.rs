
pub use resources::allocator::{
    HaHostBufferAllocator, HaDeviceBufferAllocator, HaCachedBufferAllocator, HaStagingBufferAllocator, HaBufferAllocatorAbstract, // buffer
    HaDescriptorAllocator, // descriptor
    HaImageAllocator,      // image
};

pub use resources::buffer::{
    BufferCreateFlag, HostBufferUsage, CachedBufferUsage, DeviceBufferUsage, StagingBufferUsage, // flag
    BufferItem, BufferSubItem, // item
    HostBufferConfig, CachedBufferConfig, DeviceBufferConfig, StagingBufferConfig, // config
    BufferConfigModifiable, // traits
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

// currently no framebuffer API is public.
pub use resources::framebuffer::{};

pub use resources::image::{
    ImageDescInfo,     // image
    ImageViewDescInfo, // view
    ImageLayout, ImageAspectFlag, ImageCreateFlag, ImageUsageFlag, // flag
    ImageViewItem, // item
    HaSampler, SamplerDescInfo, // sampler
    ImageType, ImageViewType, ImageTiling, Filter, MipmapMode, CompareOp, BorderColor, // enums
};

// currently no memory API is public,
pub use resources::memory::{};

pub use resources::repository::{
    HaBufferRepository, CmdVertexBindingInfos, CmdIndexBindingInfo, // buffer
    HaDescriptorRepository, CmdDescriptorBindingInfos, // descriptor
    HaImageRepository, // image
    BufferDataUploader, BufferDataUpdater, // transfer
};

pub use resources::toolkit::{
    AllocatorKit, // allocator
    PipelineKit,  // pipeline
    CommandKit,   // command
};
