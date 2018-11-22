
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::BufferBlock;
use buffer::allocator::BufferAllocateInfos;
use memory::{ HaMemory, HaMemoryType, HaMemoryAbstract, MemorySelector };
use memory::structs::MemoryRange;
use memory::instance::{ HaMemoryEntityAbs, MemoryDataUploadable };
use memory::instance::staging::{ StagingUploader, UploadStagingResource };
use memory::error::MemoryError;

use utils::memory::MemoryWritePtr;
use types::vkbytes;

pub struct HaDeviceMemory {

    target: HaMemory,
}

impl HaMemoryEntityAbs for HaDeviceMemory {}

impl HaMemoryAbstract for HaDeviceMemory {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::DeviceMemory
    }

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaDeviceMemory, MemoryError> {

        let target = HaMemory::allocate(device, size, selector)?;

        let memory = HaDeviceMemory {
            target,
        };
        Ok(memory)
    }
}

impl MemoryDataUploadable for HaDeviceMemory {

    fn prepare_data_transfer(&mut self, physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>)
        -> Result<Option<UploadStagingResource>, MemoryError> {

        StagingUploader::prepare_data_transfer(physical, device, allocate_infos)
    }

    fn map_memory_ptr(&mut self, staging: &mut Option<UploadStagingResource>, block: &BufferBlock, offset: vkbytes)
        -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        StagingUploader::map_memory_ptr(staging, block, offset)
    }

    fn terminate_transfer(&mut self, device: &HaDevice, staging: &Option<UploadStagingResource>, ranges_to_flush: &Vec<MemoryRange>)
        -> Result<(), MemoryError> {

        StagingUploader::terminate_transfer(device, staging, ranges_to_flush)
    }
}
