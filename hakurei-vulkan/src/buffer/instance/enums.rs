
use memory::HaMemoryType;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferInstanceType {

    VertexBuffer,
    IndexBuffer,
    UniformBuffer,
    ImageSrcBuffer,
}

impl BufferInstanceType {

    pub fn check_storage_validity(&self, memory_type: HaMemoryType) -> bool {
        check_buffer_usage(memory_type, self.clone())
    }
}

fn check_buffer_usage(memory_type: HaMemoryType, instance: BufferInstanceType) -> bool {

    match memory_type {
        | HaMemoryType::HostMemory => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
                BufferInstanceType::UniformBuffer,
            ].contains(&instance)
        },
        | HaMemoryType::CachedMemory  => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
            ].contains(&instance)
        },
        | HaMemoryType::DeviceMemory  => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
            ].contains(&instance)
        },
        | HaMemoryType::StagingMemory => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
                BufferInstanceType::UniformBuffer,
                BufferInstanceType::ImageSrcBuffer,
            ].contains(&instance)
        },
    }
}
