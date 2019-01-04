
use gsvk::buffer::allocator::GsBufferAllocator;
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;

use gltf::mesh::Reader;

pub(crate) trait GltfPrimitiveProperty where Self: Sized {
    const PROPERTY_NAME: &'static str;

    type IndexType;
    type BlockType;

    fn read<'a, 's, F>(reader: &Reader<'a, 's, F>) -> Self
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>;

    fn append_allocation<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Option<Self::IndexType>, AllocatorError>
        where M: BufferMemoryTypeAbs;

    fn upload<M>(&self, to: &Option<Self::BlockType>, by: &mut BufferDataUploader<M>) -> Result<(), AllocatorError>
        where M: BufferMemoryTypeAbs;
}
