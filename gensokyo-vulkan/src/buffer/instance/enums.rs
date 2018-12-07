
use crate::memory::types::GsMemoryType;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferInstanceType {

    VertexBuffer,
    IndexBuffer,
    UniformBuffer,
    ImageSrcBuffer,
}

impl BufferInstanceType {

    pub fn check_storage_validity(&self, memory_type: GsMemoryType) -> bool {
        check_buffer_usage(memory_type, self.clone())
    }
}

fn check_buffer_usage(memory_type: GsMemoryType, instance: BufferInstanceType) -> bool {

    match memory_type {
        | GsMemoryType::HostMemory => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
                BufferInstanceType::UniformBuffer,
            ].contains(&instance)
        },
        | GsMemoryType::CachedMemory  => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
            ].contains(&instance)
        },
        | GsMemoryType::DeviceMemory  => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
            ].contains(&instance)
        },
        | GsMemoryType::StagingMemory => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
                BufferInstanceType::UniformBuffer,
                BufferInstanceType::ImageSrcBuffer,
            ].contains(&instance)
        },
    }
}
