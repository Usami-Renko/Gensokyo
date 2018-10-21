
pub use resources::allocator::{
    HaBufferAllocator, BufferStorageType, // buffer
    HaDescriptorAllocator, // descriptor
    HaImageAllocator, ImageStorageType,   // image
};

pub use resources::buffer::{
    BufferCreateFlag, // flag
    BufferItem, BufferSubItem, // item
    HaVertexBlock, VertexBlockInfo, // block
    HaIndexBlock, IndexBlockInfo, // block
    HaUniformBlock, UniformBlockInfo, // block
};

pub use resources::command::{
    HaCommandBuffer, CommandBufferUsage,       // buffer
    HaCommandPool, CommandPoolFlag,            // pool
    HaCommandRecorder, CommandBufferUsageFlag, // record
    CmdViewportInfo, CmdScissorInfo, // infos
};

pub use resources::descriptor::{
    DescriptorSetConfig, DescriptorItem, DescriptorSetItem,  // item
    DescriptorPoolFlag, // pool
    HaDescriptorSetLayout, DescriptorSetLayoutFlag, BufferDescriptorType, ImageDescriptorType, // layout
};

// currently no framebuffer API is public.
pub use resources::framebuffer::{};

pub use resources::image::{
    ImagePipelineStage, DepthStencilImageFormat, // enums
    ImageTiling, Filter, MipmapMode, CompareOp, BorderColor, // enums
    SampleImageInfo, HaSampleImage, DepthStencilImageInfo, HaDepthStencilImage, // branch
};

// currently no memory API is public,
pub use resources::memory::{};

pub use resources::repository::{
    HaBufferRepository, // buffer
    HaDescriptorRepository, // descriptor
    HaImageRepository, // image
    BufferDataUploader, BufferDataUpdater, // transfer
};

pub use resources::toolkit::{
    AllocatorKit, // allocator
    PipelineKit,  // pipeline
    CommandKit,   // command
};
