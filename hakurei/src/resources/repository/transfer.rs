
use ash::vk;

use core::device::HaLogicalDevice;

use resources::buffer::BufferSubItem;
use resources::memory::{ HaMemoryAbstract, MemoryRange };
use resources::error::AllocatorError;

pub struct BufferDataUploader<'mem> {

    memory  : &'mem mut Box<HaMemoryAbstract>,
    offsets : &'mem Vec<vk::DeviceSize>,
    ranges  : Vec<MemoryRange>,
}

impl<'mem> BufferDataUploader<'mem> {

    pub(crate) fn new(device: &HaLogicalDevice, memory: &'mem mut Box<HaMemoryAbstract>, offsets: &'mem Vec<vk::DeviceSize>) -> Result<BufferDataUploader<'mem>, AllocatorError> {

        memory.prepare_data_transfer(device)?;

        let uploader = BufferDataUploader { memory, offsets, ranges: vec![], };
        Ok(uploader)
    }

    pub fn upload<D: Copy>(&mut self, item: &BufferSubItem, data: &Vec<D>) -> Result<&mut BufferDataUploader<'mem>, AllocatorError> {

        let offset = self.offsets[item.buffer_index] + item.offset;

        let (writer, range) = self.memory.map_memory_ptr(item, offset)?;
        writer.write_data(data);

        self.ranges.push(range);

        Ok(self)
    }

    pub fn done(&mut self, device: &HaLogicalDevice) -> Result<(), AllocatorError> {

        self.memory.terminate_transfer(device, &self.ranges)?;
        self.memory.enable_map(device, false)?;

        Ok(())
    }
}

pub struct BufferDataUpdater<'mem> {

    memory  : &'mem mut Box<HaMemoryAbstract>,
    offsets : &'mem Vec<vk::DeviceSize>,
    ranges  : Vec<MemoryRange>,
}

impl<'mem> BufferDataUpdater<'mem> {

    pub(crate) fn new(memory: &'mem mut Box<HaMemoryAbstract>, offsets: &'mem Vec<vk::DeviceSize>) -> BufferDataUpdater<'mem> {

        BufferDataUpdater {
            memory, offsets, ranges: vec![],
        }
    }

    pub fn update<D: Copy>(&mut self, item: &BufferSubItem, data: &Vec<D>) -> Result<&mut BufferDataUpdater<'mem>, AllocatorError> {

        let offset = self.offsets[item.buffer_index] + item.offset;

        let (writer, range) = self.memory.map_memory_ptr(item, offset)?;
        writer.write_data(data);

        self.ranges.push(range);

        Ok(self)
    }

    pub fn done(&mut self, device: &HaLogicalDevice) -> Result<(), AllocatorError> {

        self.memory.terminate_transfer(device, &self.ranges)?;

        Ok(())
    }
}

// TODO: Fix the following code after Make the Rc<Device>.
//
//pub struct DataCopyer<'vk, 'buffer> where 'vk: 'buffer {
//
//    transfer: HaTransfer<'buffer>,
//    recorder: HaCommandRecorder<'buffer, 'vk>,
//}
//
//impl<'vk, 'buffer, 'device> DataCopyer<'vk, 'buffer> where 'vk: 'buffer, 'device: 'vk {
//
//    pub(crate) fn new(device: &'device HaLogicalDevice) -> Result<DataCopyer<'vk, 'buffer>, AllocatorError> {
//
//        let (recorder, transfer) = {
//            let mut transfer = device.transfer();
//            let command = transfer.command()?;
//            let recorder = command.setup_record(device);
//            let _ = recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?;
//            (recorder, transfer)
//        };
//
//        let copyer = DataCopyer { transfer, recorder };
//        Ok(copyer)
//    }

//    pub fn copy_buffer_to_buffer(&self, from: &BufferSubItem, to: &BufferSubItem) -> Result<&DataCopyer<'vk, 'buffer>, AllocatorError> {

//        let copy_region = [
//            vk::BufferCopy {
//                src_offset: from.offset,
//                dst_offset: to.offset,
//                size      : to.size,
//            },
//        ];
//        self.recorder.copy_buffer(from.handle, to.handle, &copy_region);
//
//        Ok(self)
//    }
//
//    pub fn done(&mut self) -> Result<(), AllocatorError> {
//
////        self.recorder.finish()?;
//        self.transfer.excute()?;
//
//        Ok(())
//    }
//}
