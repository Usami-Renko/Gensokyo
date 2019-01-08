
use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

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
use crate::memory::error::{ MemoryError, AllocatorError };

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

    fn allocate(device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> Result<GsDeviceMemory, MemoryError> {

        let memory = GsDeviceMemory {
            target: GsMemory::allocate(device, size, filter)?,
        };
        Ok(memory)
    }

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMappable> {
        None
    }
}

impl GsBufferMemoryAbs for GsDeviceMemory {

    fn to_upload_agency(&self, device: &GsDevice, physical: &GsPhyDevice, allot_infos: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError> {

        let agency = DeviceDataAgency::new(device, physical, allot_infos)?;
        Ok(Box::new(agency))
    }

    fn to_update_agency(&self) -> Result<Box<dyn MemoryDataDelegate>, MemoryError> {
        /// Device memory is unable to update directly.
        unreachable!()
    }
}

impl GsImageMemoryAbs for GsDeviceMemory {}


pub struct DeviceDataAgency {

    res: UploadStagingResource,
}

impl DeviceDataAgency {

    fn new(device: &GsDevice, physical: &GsPhyDevice, infos: &BufferAllocateInfos) -> Result<DeviceDataAgency, MemoryError> {

        let agency = DeviceDataAgency {
            res: UploadStagingResource::new(device, physical, infos)?,
        };
        Ok(agency)
    }
}

impl MemoryDataDelegate for DeviceDataAgency {

    fn prepare(&mut self, _: &GsDevice) -> Result<(), MemoryError> {
        Ok(())
    }

    fn acquire_write_ptr(&mut self, block: &BufferBlock, repository_index: usize) -> Result<MemoryWritePtr, MemoryError> {

        let writer= self.res.append_dst_block(block, repository_index)?;
        Ok(writer)
    }

    fn finish(&mut self, device: &GsDevice) -> Result<(), AllocatorError> {

        self.res.finish_src_transfer(device)?;
        self.res.transfer(device)?;
        self.res.destroy(device);

        Ok(())
    }
}
