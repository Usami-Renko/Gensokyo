
pub use resources::allocator::{
    HaBufferAllocator, BufferStorageType, // buffer
    HaDescriptorAllocator, // descriptor
    HaImageAllocator, ImageStorageType,   // image
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
    ImagePipelineStage, DepthStencilImageFormat, // enums
    ImageTiling, Filter, MipmapMode, CompareOp, BorderColor, // enums
    SampleImageInfo, HaSampleImage, DepthStencilImageInfo, HaDepthStencilImage, // variety
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
