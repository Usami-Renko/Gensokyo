
use ash::vk;

use resources::memory::MemPtr;

#[inline]
pub fn bind_to_alignment(bind_value: vk::DeviceSize, alignment: vk::DeviceSize) -> vk::DeviceSize {
    if bind_value < alignment {
        alignment
    } else {
        bind_value - (bind_value % alignment) + alignment
    }
}

pub fn spaces_to_offsets(spaces: &Vec<vk::DeviceSize>) -> Vec<vk::DeviceSize> {

    let mut current: vk::DeviceSize = 0;
    let mut offsets = vec![];
    for &space in spaces.iter() {
        offsets.push(current);
        current += space;
    }

    offsets
}

pub(crate) struct MemoryWritePtr {

    ptr : MemPtr,
    size: vk::DeviceSize,
}

impl MemoryWritePtr {

    pub fn new(ptr: MemPtr, size: vk::DeviceSize) -> MemoryWritePtr {
        MemoryWritePtr { ptr, size }
    }

    pub fn write_data<D: Copy>(&self, data: &[D]) {

        use ash;
        use std::mem;

        let mut vert_algn = unsafe {
            ash::util::Align::new(
                self.ptr,
                mem::align_of::<D>() as vk::DeviceSize,
                self.size,
            )
        };

        vert_algn.copy_from_slice(data);
    }
}
