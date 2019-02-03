
use ash::vk;

use crate::buffer::entity::BufferBlock;
use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::MemoryWritePtr;

use crate::error::VkResult;
use crate::types::vkbytes;

pub trait BufferInstance: BufferCopiable {
    type InfoType;

    fn build(block: BufferBlock, info: Self::InfoType, repository_index: usize) -> Self;

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> VkResult<MemoryWritePtr>;
}

pub trait BufferCopiable: Sized {

    fn full_copy(&self) -> BufferFullCopyInfo;

    fn copy_split_ranges(&self, range_count: usize) -> BufferCopyRanges {

        let full_range = self.full_copy();
        BufferCopyRanges::from_stride(full_range.handle, full_range.size / range_count as vkbytes, range_count)
    }
}

pub struct BufferFullCopyInfo {

    /// `handle` is the handle of buffer whose data is copied from or copy to.
    pub(crate) handle: vk::Buffer,
    /// If this is the buffer for data source, `size` is the number of bytes to copy.
    ///
    /// If this is the buffer for data destination, `size` will be ignored.
    pub(crate) size: vkbytes,
}

#[allow(dead_code)]
pub struct BufferCopyRange {

    /// `handle` is the handle of buffer whose data is copied from or copy to.
    pub(crate) handle: vk::Buffer,
    /// `offset` is the starting offset in bytes from the start of source or destination buffer.
    ///
    /// `offset` is not the starting offset of memory.
    pub(crate) offset: vkbytes,
    /// If this is the buffer for data source, `size` is the number of bytes to copy.
    ///
    /// If this is the buffer for data destination, `size` will be ignored.
    pub(crate) size: vkbytes,
}

pub struct BufferCopyRanges {

    /// `handle` is the handle of buffer whose data is copied from or copy to.
    pub(crate) handle: vk::Buffer,
    /// `offsets` is the starting offset of each subrange of the buffer.
    pub(crate) offsets: Vec<vkbytes>,
}

impl BufferCopyRanges {

    pub fn from_stride(handle: vk::Buffer, stride: vkbytes, stride_count: usize) -> BufferCopyRanges {

        let mut offsets = Vec::with_capacity(stride_count);
        for i in 0..(stride_count as vkbytes) {
            offsets.push(i * stride);
        }

        BufferCopyRanges { handle, offsets }
    }

    pub fn subrange_count(&self) -> usize {
        self.offsets.len()
    }
}
