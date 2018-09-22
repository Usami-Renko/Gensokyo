
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::buffer::{ HaBuffer, BufferSubItem };
use resources::memory::{ HaMemoryAbstract, MemoryDataTransfer, MemoryPropertyFlag };
use resources::error::MemoryError;

use utility::memory::MemoryWritePtr;
use utility::marker::VulkanFlags;

use std::ptr;

pub struct HaHostMemory  {

    handle     : vk::DeviceMemory,
    _size      : vk::DeviceSize,
    mem_type   : Option<vk::MemoryType>,

    // TODO: Use a new object to manage the following two field.
    data_ptr   : *mut vk::c_void,
    ranges     : Vec<(vk::DeviceSize, vk::DeviceSize)>,
}

impl HaMemoryAbstract for HaHostMemory {

    fn handle(&self) -> vk::DeviceMemory {
        self.handle
    }

    fn flag(&self) -> vk::MemoryPropertyFlags {
        self.mem_type.as_ref().and_then(|m| Some(m.property_flags))
            .unwrap_or(vk::MemoryPropertyFlags::empty())
    }

    fn default_flag() -> vk::MemoryPropertyFlags {
        [
            MemoryPropertyFlag::HostVisibleBit,
            MemoryPropertyFlag::HostCoherentBit,
        ].flags()
    }

    fn allocate(device: &HaLogicalDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<HaHostMemory, MemoryError> {

        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: size,
            // an index identifying a memory type from the memoryTypes array of the vkPhysicalDeviceMemoryProperties structure.
            memory_type_index: mem_type_index as uint32_t,
        };

        let handle = unsafe {
            device.handle.allocate_memory(&allocate_info, None)
                .or(Err(MemoryError::AllocateMemoryError))?
        };

        let memory = HaHostMemory {
            handle,
            _size: size,
            mem_type,
            ranges: vec![],
            data_ptr: ptr::null_mut(),
        };
        Ok(memory)
    }
}

impl MemoryDataTransfer for HaHostMemory {

    fn prepare_data_transfer(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError> {

        self.data_ptr = self.map_whole(device)?;
        Ok(())
    }

    fn map_memory_ptr(&mut self, device: &HaLogicalDevice, item: &BufferSubItem, offset: vk::DeviceSize) -> Result<MemoryWritePtr, MemoryError> {

        let ptr = unsafe {
            self.data_ptr.offset(offset as isize)
        };
        // let ptr = self.map_range(device, offset, item.size)?;

        let writer = MemoryWritePtr::new(ptr, item.size);
        Ok(writer)
    }

    fn unmap_memory_ptr(&mut self, item: &BufferSubItem, offset: vk::DeviceSize) {

        self.ranges.push((item.size, offset));
    }

    fn transfer_data(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError> {

        if !self.is_coherent_memroy() {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.flush_ranges(device, &self.ranges)?;
        }
        self.ranges.clear();

        self.unmap(device);

        Ok(())
    }
}
