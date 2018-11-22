
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::BufferInstance;
use buffer::allocator::BufferAllocateInfos;
use memory::structs::MemoryRange;
use memory::instance::HaMemoryEntity;
use memory::instance::UploadStagingResource;
use memory::error::AllocatorError;

use types::vkbytes;

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

    pub fn upload(&mut self, to: &impl BufferInstance, data: &[impl Copy]) -> Result<&mut BufferDataUploader<'a>, AllocatorError> {

        let block = to.as_block_ref();

        let (writer, range) = self.dst_memory.map_memory_ptr(&mut self.staging, block, block.memory_offset)?;
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
    offsets: &'a Vec<vkbytes>,
    ranges : Vec<MemoryRange>,
}

impl<'a> BufferDataUpdater<'a> {

    pub(crate) fn new(device: &HaDevice, memory: &'a mut HaMemoryEntity, offsets: &'a Vec<vkbytes>) -> BufferDataUpdater<'a> {

        BufferDataUpdater {
            device: device.clone(),
            memory, offsets, ranges: vec![],
        }
    }

    pub fn update(&mut self, to: &impl BufferInstance, data: &[impl Copy]) -> Result<&mut BufferDataUpdater<'a>, AllocatorError> {

        let block = to.as_block_ref();

        let (writer, range) = self.memory.map_memory_ptr(&mut None, block, block.memory_offset)?;
        writer.write_data(data);

        self.ranges.push(range);

        Ok(self)
    }

    pub fn done(&mut self) -> Result<(), AllocatorError> {

        self.memory.terminate_transfer(&self.device, &None, &self.ranges)?;

        Ok(())
    }
}

