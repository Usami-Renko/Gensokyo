
use ash::vk;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use resources::buffer::BufferBlockEntity;
use resources::memory::{ HaMemoryAbstract, MemoryRange, UploadStagingResource };
use resources::allocator::BufferAllocateInfos;
use resources::error::AllocatorError;

pub struct BufferDataUploader<'a> {

    device: HaDevice,
    dst_memory: &'a mut Box<HaMemoryAbstract>,

    /// the offset of each buffer in `dst_memory`.
    offsets: &'a Vec<vk::DeviceSize>,
    ranges : Vec<MemoryRange>,

    staging: Option<UploadStagingResource>,
}

impl<'a> BufferDataUploader<'a> {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, memory: &'a mut Box<HaMemoryAbstract>, offsets: &'a Vec<vk::DeviceSize>, allocate_infos: &Option<BufferAllocateInfos>) -> Result<BufferDataUploader<'a>, AllocatorError> {

        let staging = memory.prepare_data_transfer(physical, device, &allocate_infos)?;

        let uploader = BufferDataUploader {
            device: device.clone(),
            dst_memory: memory,
            ranges: vec![],
            offsets, staging,
        };
        Ok(uploader)
    }

    pub fn upload<D: Copy>(&mut self, to_block: &impl BufferBlockEntity, data: &Vec<D>) -> Result<&mut BufferDataUploader<'a>, AllocatorError> {

        let item = to_block.get_buffer_item();
        // offset is a zero-based byte offset of the buffer from the beginning of the memory object.
        let offset = self.offsets[item.buffer_index];

        let (writer, range) = self.dst_memory.map_memory_ptr(&mut self.staging, item, offset)?;
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
    memory : &'a mut Box<HaMemoryAbstract>,
    offsets: &'a Vec<vk::DeviceSize>,
    ranges : Vec<MemoryRange>,
}

impl<'a> BufferDataUpdater<'a> {

    pub(crate) fn new(device: &HaDevice, memory: &'a mut Box<HaMemoryAbstract>, offsets: &'a Vec<vk::DeviceSize>) -> BufferDataUpdater<'a> {

        BufferDataUpdater {
            device: device.clone(),
            memory, offsets, ranges: vec![],
        }
    }

    pub fn update(&mut self, block: &impl BufferBlockEntity, data: &[impl Copy]) -> Result<&mut BufferDataUpdater<'a>, AllocatorError> {

        let item = block.get_buffer_item();
        let offset = self.offsets[item.buffer_index];

        let (writer, range) = self.memory.map_memory_ptr(&mut None, item, offset)?;
        writer.write_data(data);

        self.ranges.push(range);

        Ok(self)
    }

    pub fn done(&mut self) -> Result<(), AllocatorError> {

        self.memory.terminate_transfer(&self.device, &None, &self.ranges)?;

        Ok(())
    }
}

