
use vk::resources::buffer::BufferStorageType;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferBranch {

    Vertex,
    Index,
    Uniform,
    ImageSrc,
}

impl BufferBranch {

    pub fn check_storage_validity(&self, storage: BufferStorageType) -> bool {
        check_buffer_usage(storage, self.clone())
    }
}

fn check_buffer_usage(storage: BufferStorageType, branch: BufferBranch) -> bool {

    match storage {
        | BufferStorageType::Host => {
            [
                BufferBranch::Vertex,
                BufferBranch::Index,
                BufferBranch::Uniform,
            ].contains(&branch)
        },
        | BufferStorageType::Cached  => {
            [
                BufferBranch::Vertex,
                BufferBranch::Index,
            ].contains(&branch)
        },
        | BufferStorageType::Device  => {
            [
                BufferBranch::Vertex,
                BufferBranch::Index,
            ].contains(&branch)
        },
        | BufferStorageType::Staging => {
            [
                BufferBranch::Vertex,
                BufferBranch::Index,
                BufferBranch::Uniform,
                BufferBranch::ImageSrc,
            ].contains(&branch)
        },
    }
}
