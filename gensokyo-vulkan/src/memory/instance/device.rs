
use crate::core::GsDevice;

use crate::buffer::BufferBlock;
use crate::buffer::allocator::BufferAllocateInfos;

use crate::memory::types::GsMemoryType;
use crate::memory::target::GsMemory;
use crate::memory::traits::{ GsMemoryAbstract, MemoryMappable };
use crate::memory::filter::MemoryFilter;
use crate::memory::utils::MemoryWritePtr;
use crate::memory::instance::traits::{ GsImageMemoryAbs, GsBufferMemoryAbs };
use crate::memory::instance::staging::UploadStagingResource;
use crate::memory::transfer::MemoryDataDelegate;

use crate::error::VkResult;
use crate::types::vkbytes;


pub struct GsDeviceMemory {

    target: GsMemory,
}

impl GsMemoryAbstract for GsDeviceMemory {

    fn memory_type(&self) -> GsMemoryType {
        GsMemoryType::DeviceMemory
    }

    fn target(&self) -> &GsMemory {
        &self.target
    }

    fn allocate(device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> VkResult<GsDeviceMemory> {

        let memory = GsDeviceMemory {
            target: GsMemory::allocate(device, size, filter)?,
        };
        Ok(memory)
    }

    fn as_mut_mappable(&mut self) -> Option<&mut MemoryMappable> {
        None
    }
}

impl GsBufferMemoryAbs for GsDeviceMemory {

    fn to_upload_agency(&self, device: &GsDevice, allot_infos: &BufferAllocateInfos) -> VkResult<Box<dyn MemoryDataDelegate>> {

        let agency = DeviceDataAgency::new(device, allot_infos)?;
        Ok(Box::new(agency))
    }

    fn to_update_agency(&self) -> VkResult<Box<dyn MemoryDataDelegate>> {
        /// Device memory is unable to update directly.
        unreachable!()
    }
}

impl GsImageMemoryAbs for GsDeviceMemory {}


pub struct DeviceDataAgency {

    res: UploadStagingResource,
}

impl DeviceDataAgency {

    fn new(device: &GsDevice, infos: &BufferAllocateInfos) -> VkResult<DeviceDataAgency> {

        let agency = DeviceDataAgency {
            res: UploadStagingResource::new(device, infos)?,
        };
        Ok(agency)
    }
}

impl MemoryDataDelegate for DeviceDataAgency {

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
        self.res.destroy(device);

        Ok(())
    }
}
