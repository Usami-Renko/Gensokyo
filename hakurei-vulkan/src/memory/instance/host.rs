
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::BufferBlock;
use buffer::allocator::BufferAllocateInfos;
use memory::{ HaMemory, HaMemoryType, HaMemoryAbstract, MemorySelector };
use memory::{ MemoryMapable, MemoryMapStatus, MemoryRange };
use memory::instance::{ HaMemoryEntityAbs, MemoryDataUploadable };
use memory::instance::staging::UploadStagingResource;
use memory::MemoryError;

use utils::memory::MemoryWritePtr;
use types::vkbytes;

pub struct HaHostMemory {

    target: HaMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMapable for HaHostMemory {}

impl HaMemoryEntityAbs for HaHostMemory {}

impl HaMemoryAbstract for HaHostMemory {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::HostMemory
    }

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaHostMemory, MemoryError> {

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

    fn map_memory_ptr(&mut self, _: &mut Option<UploadStagingResource>, block: &BufferBlock, offset: vkbytes)
        -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let ptr = unsafe {
            self.map_status.data_ptr.offset(offset as isize)
        };

        let writer = MemoryWritePtr::new(ptr, block.size);
        let range = MemoryRange { offset, size: block.size };

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
