
use core::device::GsDevice;
use core::physical::GsPhyDevice;

use buffer::BufferBlock;
use buffer::allocator::BufferAllocateInfos;

use memory::types::GsMemoryType;
use memory::target::GsMemory;
use memory::traits::{ GsMemoryAbstract, MemoryMappable };
use memory::selector::MemorySelector;
use memory::utils::MemoryWritePtr;
use memory::instance::traits::{ GsImageMemoryAbs, GsBufferMemoryAbs };
use memory::instance::staging::UploadStagingResource;
use memory::transfer::MemoryDataDelegate;
use memory::error::{ MemoryError, AllocatorError };

use types::vkbytes;

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

    fn allocate(device: &GsDevice, size: vkbytes, selector: &MemorySelector) -> Result<GsDeviceMemory, MemoryError> {

        let memory = GsDeviceMemory {
            target: GsMemory::allocate(device, size, selector)?,
        };
        Ok(memory)
    }

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMappable> {
        None
    }
}

impl GsBufferMemoryAbs for GsDeviceMemory {

    fn to_agency(&self, device: &GsDevice, physical: &GsPhyDevice, allot_infos: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError> {

        let agency = DeviceDataAgency::new(device, physical, allot_infos)?;
        Ok(Box::new(agency))
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
        self.res.cleanup(device);

        Ok(())
    }
}
