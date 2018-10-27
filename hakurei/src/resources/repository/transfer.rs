
use ash::vk;

use core::device::{ HaDevice, HaLogicalDevice, HaTransfer };
use core::physical::HaPhyDevice;

use resources::buffer::BufferBlockEntity;
use resources::memory::{ HaMemoryAbstract, MemoryRange, UploadStagingResource };
use resources::allocator::BufferAllocateInfos;
use resources::command::{ HaCommandRecorder, CommandBufferUsageFlag };
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
            offsets, ranges: vec![], staging,
        };
        Ok(uploader)
    }

    pub fn upload<D: Copy>(&mut self, block: &impl BufferBlockEntity, data: &Vec<D>) -> Result<&mut BufferDataUploader<'a>, AllocatorError> {

        let item = block.get_buffer_item();
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

    pub fn update<D: Copy>(&mut self, block: &impl BufferBlockEntity, data: &Vec<D>) -> Result<&mut BufferDataUpdater<'a>, AllocatorError> {

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


pub struct DataCopyer {

    transfer: HaTransfer,
    recorder: HaCommandRecorder,
}

impl DataCopyer {

    pub(crate) fn new(device: &HaDevice) -> Result<DataCopyer, AllocatorError> {

        let transfer = HaLogicalDevice::transfer(device);
        let command = transfer.command()?;
        let recorder = command.setup_record(device);

        let _ = recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?;

        let copyer = DataCopyer {
            transfer, recorder,
        };

        Ok(copyer)
    }

    pub fn copy_buffer_to_buffer(&self, from: &impl BufferBlockEntity, to: &impl BufferBlockEntity) -> Result<&DataCopyer, AllocatorError> {

        let from = from.get_buffer_item();
        let to   = to.get_buffer_item();

        // TODO: Only support one region.
        let copy_region = [
            vk::BufferCopy {
                // TODO: Only support copy buffer from beginning.
                src_offset: 0,
                dst_offset: 0,
                size: to.size,
            },
        ];

        self.recorder.copy_buffer(from.handle, to.handle, &copy_region);

        Ok(self)
    }

    pub fn done(&mut self) -> Result<(), AllocatorError> {

        let command = self.recorder.end_record()?;
        self.transfer.commit(command);
        self.transfer.excute()?;

        Ok(())
    }
}
