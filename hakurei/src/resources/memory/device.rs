
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;

use vk::resources::buffer::BufferItem;
use vk::resources::memory::{ HaMemory, HaMemoryType, HaMemoryAbstract, MemorySelector };
use vk::resources::memory::MemoryRange;
use vk::utils::memory::MemoryWritePtr;
use vk::utils::types::vkMemorySize;
use vk::resources::error::MemoryError;

use resources::memory::traits::{ HaMemoryEntityAbs, MemoryDataUploadable };
use resources::memory::staging::{ StagingUploader, UploadStagingResource };
use resources::allocator::buffer::BufferAllocateInfos;

pub struct HaDeviceMemory {

    target: HaMemory,
}

impl HaMemoryEntityAbs for HaDeviceMemory {}

impl HaMemoryAbstract for HaDeviceMemory {

    fn target(&self) -> &HaMemory {
        &self.target
    }

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::DeviceMemory
    }

    fn allocate(device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<HaDeviceMemory, MemoryError> {

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

    fn map_memory_ptr(&mut self, staging: &mut Option<UploadStagingResource>, item: &BufferItem, offset: vkMemorySize)
        -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        StagingUploader::map_memory_ptr(staging, item, offset)
    }

    fn terminate_transfer(&mut self, device: &HaDevice, staging: &Option<UploadStagingResource>, ranges_to_flush: &Vec<MemoryRange>)
        -> Result<(), MemoryError> {

        StagingUploader::terminate_transfer(device, staging, ranges_to_flush)
    }
}
