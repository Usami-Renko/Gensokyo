
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;

use vk::resources::buffer::BufferItem;
use vk::resources::memory::{ HaMemory, HaMemoryType, HaMemoryAbstract, MemorySelector };
use vk::resources::memory::{ MemoryMapable, MemoryMapStatus, MemoryRange };
use vk::utils::memory::MemoryWritePtr;
use vk::utils::types::vkMemorySize;
use vk::resources::error::MemoryError;

use resources::memory::traits::{ HaMemoryEntityAbs, MemoryDataUploadable };
use resources::memory::staging::UploadStagingResource;
use resources::allocator::buffer::BufferAllocateInfos;

use std::ptr;

pub struct HaHostMemory {

    target: HaMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMapable for HaHostMemory {}

impl HaMemoryEntityAbs for HaHostMemory {}

impl HaMemoryAbstract for HaHostMemory {

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::HostMemory
    }

    fn allocate(device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<HaHostMemory, MemoryError> {

        let target = HaMemory::allocate(device, size, selector)?;

        let memory = HaHostMemory {
            target,
            map_status: MemoryMapStatus::from_unmap(),
        };
        Ok(memory)
    }

    fn cleanup(&self, device: &HaDevice) {

        self.unmap(device);
        self.target.cleanup(device);
    }
}

impl MemoryDataUploadable for HaHostMemory {

    fn prepare_data_transfer(&mut self, _: &HaPhyDevice, _: &HaDevice, _: &Option<BufferAllocateInfos>) -> Result<Option<UploadStagingResource>, MemoryError> {

        Ok(None)
    }

    fn map_memory_ptr(&mut self, _: &mut Option<UploadStagingResource>, item: &BufferItem, offset: vkMemorySize)
        -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let ptr = unsafe {
            self.map_status.data_ptr.offset(offset as isize)
        };

        let writer = MemoryWritePtr::new(ptr, item.size);
        let range = MemoryRange { offset, size: item.size };

        Ok((writer, range))
    }

    fn terminate_transfer(&mut self, device: &HaDevice, _: &Option<UploadStagingResource>, ranges_to_flush: &Vec<MemoryRange>)
        -> Result<(), MemoryError> {

        if !self.target.is_coherent_memroy() {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.flush_ranges(device, ranges_to_flush)?;
        }

        Ok(())
    }
}

impl HaHostMemory {

    pub fn map_whole(&mut self, device: &HaDevice) -> Result<(), MemoryError> {

        if !self.map_status.is_map {
            let ptr = self.map_range(device, None)?;
            self.map_status.set_map(ptr);
        }

        Ok(())
    }
}
