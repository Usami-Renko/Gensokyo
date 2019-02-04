
use ash::vk;

use crate::core::GsDevice;

use crate::buffer::BufferBlock;
use crate::buffer::allocator::BufferAllocateInfos;

use crate::memory::types::GsMemoryType;
use crate::memory::utils::{ MemoryMapStatus, MemoryRange, MemoryMapAlias, MemoryWritePtr };
use crate::memory::target::GsMemory;
use crate::memory::traits::{ GsMemoryAbstract, MemoryMappable };
use crate::memory::filter::MemoryFilter;
use crate::memory::instance::BufferMemoryAbs;
use crate::memory::transfer::MemoryDataDelegate;

use crate::error::{ VkResult, VkError };
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

    fn allocate(device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> VkResult<GsHostMemory> {

        let target = GsMemory::allocate(device, size, filter)?;
        let map_status = MemoryMapStatus::from_unmap();

        let memory = GsHostMemory {
            target, map_status,
        };
        Ok(memory)
    }

    fn as_mut_mappable(&mut self) -> Option<&mut MemoryMappable> {
        Some(self)
    }

    fn discard(&mut self, device: &GsDevice) {

        self.unmap(device);
        self.target.discard(device);
    }
}

impl BufferMemoryAbs for GsHostMemory {

    fn to_upload_agency(&self, _: &GsDevice, _: &BufferAllocateInfos) -> VkResult<Box<dyn MemoryDataDelegate>> {

        let agency = HostDataAgency::new(self);
        Ok(Box::new(agency))
    }

    fn to_update_agency(&self) -> VkResult<Box<dyn MemoryDataDelegate>> {

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
                is_coherent: memory.target.is_coherent_memory(),
            },
            ranges_to_flush: vec![],
        }
    }
}

impl MemoryDataDelegate for HostDataAgency {

    fn prepare(&mut self, _: &GsDevice) -> VkResult<()> {
        Ok(())
    }

    fn acquire_write_ptr(&mut self, block: &BufferBlock, _: usize) -> VkResult<MemoryWritePtr> {

        self.ranges_to_flush.push(MemoryRange { offset: block.memory_offset, size: block.size });

        let data_ptr = unsafe {
            self.map_alias.status.data_ptr(block.memory_offset)
        }.ok_or(VkError::device("Failed to get mapped memory pointer."))?;

        let writer = MemoryWritePtr::new(data_ptr, block.size);
        Ok(writer)
    }

    fn finish(&mut self, device: &GsDevice) -> VkResult<()> {

        if !self.map_alias.is_coherent {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.map_alias.flush_ranges(device, &self.ranges_to_flush)?;
        }

        Ok(())
    }
}
