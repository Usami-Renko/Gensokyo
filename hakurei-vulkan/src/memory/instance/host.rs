
use ash::vk;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::BufferBlock;
use buffer::allocator::BufferAllocateInfos;

use memory::structs::{ HaMemoryType, MemoryMapStatus, MemoryRange, MemoryMapAlias };
use memory::target::HaMemory;
use memory::traits::{ HaMemoryAbstract, MemoryMapable };
use memory::selector::MemorySelector;
use memory::instance::HaBufferMemoryAbs;
use memory::transfer::MemoryDataDelegate;
use memory::error::{ MemoryError, AllocatorError };

use utils::memory::MemoryWritePtr;
use types::vkbytes;


pub struct HaHostMemory {

    target: HaMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMapable for HaHostMemory {

    fn map_handle(&self) -> vk::DeviceMemory {
        self.target.handle
    }

    fn mut_status(&mut self) -> &mut MemoryMapStatus {
        &mut self.map_status
    }
}

impl HaMemoryAbstract for HaHostMemory {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::HostMemory
    }

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaHostMemory, MemoryError> {

        let target = HaMemory::allocate(device, size, selector)?;
        let map_status = MemoryMapStatus::from_unmap();

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

impl HaBufferMemoryAbs for HaHostMemory {

    fn to_agency(&self, _: &HaDevice, _: &HaPhyDevice, _: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError> {

        let agency = HostDataAgency::new(self);
        Ok(Box::new(agency))
    }
}


pub struct HostDataAgency {

    map_alias: MemoryMapAlias,
    ranges_to_flush: Vec<MemoryRange>,
}

impl HostDataAgency {

    pub fn new(memory: &HaHostMemory) -> HostDataAgency {

        HostDataAgency {
            map_alias: MemoryMapAlias {
                handle: memory.target.handle,
                status: memory.map_status.clone(),
                is_coherent: memory.target.is_coherent_memroy(),
            },
            ranges_to_flush: vec![],
        }
    }
}

impl MemoryDataDelegate for HostDataAgency {

    fn prepare(&mut self, _: &HaDevice) -> Result<(), MemoryError> {
        Ok(())
    }

    fn acquire_write_ptr(&mut self, block: &BufferBlock, _: usize) -> Result<MemoryWritePtr, MemoryError> {

        self.ranges_to_flush.push(MemoryRange { offset: block.memory_offset, size: block.size });

        let data_ptr = unsafe {
            self.map_alias.status.data_ptr(block.memory_offset)
        }.ok_or(MemoryError::MemoryPtrInvalidError)?;

        let writer = MemoryWritePtr::new(data_ptr, block.size);
        Ok(writer)
    }

    fn finish(&mut self, device: &HaDevice) -> Result<(), AllocatorError> {

        if !self.map_alias.is_coherent {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.map_alias.flush_ranges(device, &self.ranges_to_flush)?;
        }

        Ok(())
    }
}
