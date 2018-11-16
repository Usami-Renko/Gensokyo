
use utils::types::{ vkMemorySize, MemPtr };

#[inline]
pub fn bind_to_alignment(bind_value: vkMemorySize, alignment: vkMemorySize) -> vkMemorySize {
    if bind_value < alignment {
        alignment
    } else {
        bind_value - (bind_value % alignment) + alignment
    }
}

pub struct MemoryWritePtr {

    ptr : MemPtr,
    size: vkMemorySize,
}

impl MemoryWritePtr {

    pub fn new(ptr: MemPtr, size: vkMemorySize) -> MemoryWritePtr {
        MemoryWritePtr { ptr, size }
    }

    pub fn write_data<D: Copy>(&self, data: &[D]) {

        use ash;
        use std::mem;

        let mut vert_algn = unsafe {
            ash::util::Align::new(
                self.ptr,
                mem::align_of::<D>() as vkMemorySize,
                self.size,
            )
        };

        vert_algn.copy_from_slice(data);
    }
}
