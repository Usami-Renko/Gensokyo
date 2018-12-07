
use ash::vk;

use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::buffer::BufferBlock;
use crate::buffer::allocator::BufferAllocateInfos;

use crate::memory::types::GsMemoryType;
use crate::memory::utils::{ MemoryMapStatus, MemoryRange, MemoryMapAlias, MemoryWritePtr };
use crate::memory::target::GsMemory;
use crate::memory::traits::{ GsMemoryAbstract, MemoryMappable };
use crate::memory::selector::MemorySelector;
use crate::memory::instance::GsBufferMemoryAbs;
use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::error::{ MemoryError, AllocatorError };

use crate::types::vkbytes;


pub struct GsHostMemory {

    target: GsMemory,
    map_status: MemoryMapStatus,
}

impl MemoryMappable for GsHostMemory {

    fn map_handle(&self) -> vk::DeviceMemory {
        self.target.handle
    }

    fn mut_status(&mut self) -> &mut MemoryMapStatus {
        &mut self.map_status
    }
}

impl GsMemoryAbstract for GsHostMemory {

    fn memory_type(&self) -> GsMemoryType {
        GsMemoryType::HostMemory
    }

    fn target(&self) -> &GsMemory {
        &self.target
    }

    fn allocate(device: &GsDevice, size: vkbytes, selector: &MemorySelector) -> Result<GsHostMemory, MemoryError> {

        let target = GsMemory::allocate(device, size, selector)?;
        let map_status = MemoryMapStatus::from_unmap();

        let memory = GsHostMemory {
            target, map_status,
        };
        Ok(memory)
    }

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMappable> {
        Some(self)
    }

    fn cleanup(&mut self, device: &GsDevice) {

        self.unmap(device);
        self.target.cleanup(device);
    }
}

impl GsBufferMemoryAbs for GsHostMemory {

    fn to_agency(&self, _: &GsDevice, _: &GsPhyDevice, _: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError> {

        let agency = HostDataAgency::new(self);
        Ok(Box::new(agency))
    }
}


pub struct HostDataAgency {

    map_alias: MemoryMapAlias,
    ranges_to_flush: Vec<MemoryRange>,
}

impl HostDataAgency {

    pub fn new(memory: &GsHostMemory) -> HostDataAgency {

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

    fn prepare(&mut self, _: &GsDevice) -> Result<(), MemoryError> {
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

    fn finish(&mut self, device: &GsDevice) -> Result<(), AllocatorError> {

        if !self.map_alias.is_coherent {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.map_alias.flush_ranges(device, &self.ranges_to_flush)?;
        }

        Ok(())
    }
}
