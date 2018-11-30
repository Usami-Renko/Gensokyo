
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::BufferBlock;
use buffer::allocator::BufferAllocateInfos;
use memory::structs::HaMemoryType;
use memory::target::HaMemory;
use memory::traits::{ HaMemoryAbstract, MemoryMapable };
use memory::selector::MemorySelector;
use memory::instance::traits::{ HaImageMemoryAbs, HaBufferMemoryAbs };
use memory::instance::staging::UploadStagingResource;
use memory::transfer::MemoryDataDelegate;
use memory::error::{ MemoryError, AllocatorError };

use types::vkbytes;
use utils::memory::MemoryWritePtr;


pub struct HaCachedMemory  {

    target: HaMemory,
}

impl HaMemoryAbstract for HaCachedMemory {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::CachedMemory
    }

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaCachedMemory, MemoryError> {

        let memory = HaCachedMemory {
            target: HaMemory::allocate(device, size, selector)?,
        };
        Ok(memory)
    }

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMapable> {
        None
    }
}

impl HaBufferMemoryAbs for HaCachedMemory {

    fn to_agency(&self, device: &HaDevice, physical: &HaPhyDevice, allot_infos: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError> {

        let agency = CachedDataAgency::new(device, physical, allot_infos)?;
        Ok(Box::new(agency))
    }
}

impl HaImageMemoryAbs for HaCachedMemory {}


pub struct CachedDataAgency {

    res: UploadStagingResource,
}

impl CachedDataAgency {

    fn new(device: &HaDevice, physical: &HaPhyDevice, infos: &BufferAllocateInfos) -> Result<CachedDataAgency, MemoryError> {

        let agency = CachedDataAgency {
            res: UploadStagingResource::new(device, physical, infos)?,
        };
        Ok(agency)
    }
}

impl MemoryDataDelegate for CachedDataAgency {

    fn prepare(&mut self, _: &HaDevice) -> Result<(), MemoryError> {
        Ok(())
    }

    fn acquire_write_ptr(&mut self, block: &BufferBlock, repository_index: usize) -> Result<MemoryWritePtr, MemoryError> {

        let writer= self.res.append_dst_block(block, repository_index)?;
        Ok(writer)
    }

    fn finish(&mut self, device: &HaDevice) -> Result<(), AllocatorError> {

        self.res.finish_src_transfer(device)?;
        self.res.transfer(device)?;
        self.res.cleanup(device);

        Ok(())
    }
}
