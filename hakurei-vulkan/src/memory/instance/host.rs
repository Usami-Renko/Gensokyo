
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::BufferBlock;
use buffer::allocator::BufferAllocateInfos;
use memory::structs::{ HaMemoryType, MemoryMapStatus, MemoryRange };
use memory::target::HaMemory;
use memory::traits::{ HaMemoryAbstract, MemoryMapable };
use memory::selector::MemorySelector;
use memory::instance::{ HaBufferMemoryAbs, MemoryDataUploadable };
use memory::instance::staging::UploadStagingResource;
use memory::error::MemoryError;

use utils::memory::MemoryWritePtr;
use types::vkbytes;

pub struct HaHostMemory {

    target: HaMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMapable for HaHostMemory {

    fn mut_status(&mut self) -> &mut MemoryMapStatus {
        &mut self.map_status
    }
}

impl HaBufferMemoryAbs for HaHostMemory {}

impl HaMemoryAbstract for HaHostMemory {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::HostMemory
    }

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaHostMemory, MemoryError> {

        let target = HaMemory::allocate(device, size, selector)?;
        let map_status = MemoryMapStatus::from_unmap(target.size);

        let memory = HaHostMemory {
            target, map_status,
        };
        Ok(memory)
    }

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMapable> {
        Some(self)
    }

    fn cleanup(&mut self, device: &HaDevice) {

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
            self.map_status.data_ptr
                .ok_or(MemoryError::MemoryPtrInvalidError)?
                .offset(offset as isize)
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
