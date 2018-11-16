
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;

use vk::resources::buffer::BufferBlockEntity;
use vk::resources::memory::MemoryRange;
use vk::resources::error::AllocatorError;
use vk::utils::types::vkMemorySize;

use resources::memory::HaMemoryEntity;
use resources::memory::UploadStagingResource;
use resources::allocator::buffer::BufferAllocateInfos;

pub struct BufferDataUploader<'a> {

    device: HaDevice,
    // TODO: change this to MemoryDataUploadable.
    dst_memory: &'a mut HaMemoryEntity,
    ranges : Vec<MemoryRange>,

    staging: Option<UploadStagingResource>,
}

impl<'a> BufferDataUploader<'a> {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, memory: &'a mut HaMemoryEntity, allocate_infos: &Option<BufferAllocateInfos>) -> Result<BufferDataUploader<'a>, AllocatorError> {

        let staging = memory.prepare_data_transfer(physical, device, &allocate_infos)?;

        let uploader = BufferDataUploader {
            device: device.clone(),
            dst_memory: memory,
            ranges: vec![],
            staging,
        };
        Ok(uploader)
    }

    pub fn upload(&mut self, to_block: &impl BufferBlockEntity, data: &[impl Copy]) -> Result<&mut BufferDataUploader<'a>, AllocatorError> {

        let item = to_block.item();

        let (writer, range) = self.dst_memory.map_memory_ptr(&mut self.staging, item, item.memory_offset)?;
        writer.write_data(data);

        self.ranges.push(range);

        Ok(self)
    }

    pub fn done(&mut self) -> Result<(), AllocatorError> {

        if let Some(ref mut staging) = self.staging {
            staging.finish_src_transfer(&self.device, &self.ranges)?;
        }

        self.dst_memory.terminate_transfer(&self.device, &self.staging, &self.ranges)?;

        if let Some(ref mut staging) = self.staging {
            staging.cleanup(&self.device);
        }

        Ok(())
    }
}

// TODO: Use MemoryDataUpdatable instead of HaMemoryAbstract as bound trait.
pub struct BufferDataUpdater<'a> {

    device : HaDevice,
    memory : &'a mut HaMemoryEntity,
    offsets: &'a Vec<vkMemorySize>,
    ranges : Vec<MemoryRange>,
}

impl<'a> BufferDataUpdater<'a> {

    pub(super) fn new(device: &HaDevice, memory: &'a mut HaMemoryEntity, offsets: &'a Vec<vkMemorySize>) -> BufferDataUpdater<'a> {

        BufferDataUpdater {
            device: device.clone(),
            memory, offsets, ranges: vec![],
        }
    }

    pub fn update(&mut self, block: &impl BufferBlockEntity, data: &[impl Copy]) -> Result<&mut BufferDataUpdater<'a>, AllocatorError> {

        let item = block.item();

        let (writer, range) = self.memory.map_memory_ptr(&mut None, item, item.memory_offset)?;
        writer.write_data(data);

        self.ranges.push(range);

        Ok(self)
    }

    pub fn done(&mut self) -> Result<(), AllocatorError> {

        self.memory.terminate_transfer(&self.device, &None, &self.ranges)?;

        Ok(())
    }
}

