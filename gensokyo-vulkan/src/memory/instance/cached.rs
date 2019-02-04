
use crate::core::GsDevice;

use crate::buffer::BufferBlock;
use crate::buffer::allocator::BufferAllocateInfos;
use crate::memory::types::GsMemoryType;
use crate::memory::target::GsMemory;
use crate::memory::utils::MemoryWritePtr;
use crate::memory::traits::{ GsMemoryAbstract, MemoryMappable };
use crate::memory::filter::MemoryFilter;
use crate::memory::instance::traits::{ ImageMemoryAbs, BufferMemoryAbs };
use crate::memory::instance::staging::UploadStagingResource;
use crate::memory::transfer::MemoryDataDelegate;

use crate::error::VkResult;
use crate::types::vkbytes;

pub struct GsCachedMemory  {

    target: GsMemory,
}

impl GsMemoryAbstract for GsCachedMemory {

    fn memory_type(&self) -> GsMemoryType {
        GsMemoryType::CachedMemory
    }

    fn target(&self) -> &GsMemory {
        &self.target
    }

    fn allocate(device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> VkResult<GsCachedMemory> {

        let memory = GsCachedMemory {
            target: GsMemory::allocate(device, size, filter)?,
        };
        Ok(memory)
    }

    fn as_mut_mappable(&mut self) -> Option<&mut MemoryMappable> {
        None
    }
}

impl BufferMemoryAbs for GsCachedMemory {

    fn to_upload_agency(&self, device: &GsDevice, allot_infos: &BufferAllocateInfos) -> VkResult<Box<dyn MemoryDataDelegate>> {

        let agency = CachedDataAgency::new(device, allot_infos)?;
        Ok(Box::new(agency))
    }

    fn to_update_agency(&self) -> VkResult<Box<dyn MemoryDataDelegate>> {
        /// Cached memory is unable to update directly.
        unreachable!()
    }
}

impl ImageMemoryAbs for GsCachedMemory {}


pub struct CachedDataAgency {

    res: UploadStagingResource,
}

impl CachedDataAgency {

    fn new(device: &GsDevice, infos: &BufferAllocateInfos) -> VkResult<CachedDataAgency> {

        let agency = CachedDataAgency {
            res: UploadStagingResource::new(device, infos)?,
        };
        Ok(agency)
    }
}

impl MemoryDataDelegate for CachedDataAgency {

    fn prepare(&mut self, _: &GsDevice) -> VkResult<()> {
        Ok(())
    }

    fn acquire_write_ptr(&mut self, block: &BufferBlock, repository_index: usize) -> VkResult<MemoryWritePtr> {

        let writer= self.res.append_dst_block(block, repository_index)?;
        Ok(writer)
    }

    fn finish(&mut self, device: &GsDevice) -> VkResult<()> {

        self.res.finish_src_transfer(device)?;
        self.res.transfer(device)?;
        self.res.discard(device);

        Ok(())
    }
}
