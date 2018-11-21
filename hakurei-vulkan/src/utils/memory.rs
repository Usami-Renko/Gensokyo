
use types::{ vkbytes, vkptr };

#[inline]
pub fn bind_to_alignment(bind_value: vkbytes, alignment: vkbytes) -> vkbytes {
    if bind_value < alignment {
        alignment
    } else {
        bind_value - (bind_value % alignment) + alignment
    }
}

pub struct MemoryWritePtr {

    ptr : vkptr,
    size: vkbytes,
}

impl MemoryWritePtr {

    pub fn new(ptr: vkptr, size: vkbytes) -> MemoryWritePtr {
        MemoryWritePtr { ptr, size }
    }

    pub fn write_data<D: Copy>(&self, data: &[D]) {

        use ash;
        use std::mem;

        let mut vert_algn = unsafe {
            ash::util::Align::new(
                self.ptr,
                mem::align_of::<D>() as vkbytes,
                self.size,
            )
        };

        vert_algn.copy_from_slice(data);
    }
}
