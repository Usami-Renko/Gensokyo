
use buffer::target::BufferStorageType;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferInstanceType {

    VertexBuffer,
    IndexBuffer,
    UniformBuffer,
    ImageSrcBuffer,
}

impl BufferInstanceType {

    pub fn check_storage_validity(&self, storage: BufferStorageType) -> bool {
        check_buffer_usage(storage, self.clone())
    }
}

fn check_buffer_usage(storage: BufferStorageType, instance: BufferInstanceType) -> bool {

    match storage {
        | BufferStorageType::Host => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
                BufferInstanceType::UniformBuffer,
            ].contains(&instance)
        },
        | BufferStorageType::Cached  => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
            ].contains(&instance)
        },
        | BufferStorageType::Device  => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
            ].contains(&instance)
        },
        | BufferStorageType::Staging => {
            [
                BufferInstanceType::VertexBuffer,
                BufferInstanceType::IndexBuffer,
                BufferInstanceType::UniformBuffer,
                BufferInstanceType::ImageSrcBuffer,
            ].contains(&instance)
        },
    }
}
