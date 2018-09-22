
use ash::vk;

use core::device::HaLogicalDevice;

use resources::buffer::{ HaBuffer, BufferSubItem };
use resources::command::CommandBufferUsageFlag;
use resources::memory::{ HaMemoryAbstract, MemoryDataTransfer };
use resources::error::{ AllocatorError, MemoryError };

use utility::memory::spaces_to_offsets;

pub struct HaBufferRepository {

    buffers: Vec<HaBuffer>,
    memory : Option<Box<HaMemoryAbstract>>,

    /// The offset of each buffer in memory.
    offsets: Vec<vk::DeviceSize>,

    is_data_transfering: bool,
}

pub struct CmdVertexBindingInfos {

    pub(crate) handles: Vec<vk::Buffer>,
    pub(crate) offsets: Vec<vk::DeviceSize>,
}
pub struct CmdIndexBindingInfo {

    pub(crate) handle: vk::Buffer,
    pub(crate) offset: vk::DeviceSize,
}

impl HaBufferRepository {

    pub fn empty() -> HaBufferRepository {
        HaBufferRepository {
            buffers: vec![],
            memory : None,

            offsets: vec![],

            is_data_transfering: false,
        }
    }

    pub(crate) fn store(buffers: Vec<HaBuffer>, memory: Box<HaMemoryAbstract>, spaces: Vec<vk::DeviceSize>) -> HaBufferRepository {

        let offsets = spaces_to_offsets(&spaces);

        HaBufferRepository { buffers, memory: Some(memory), offsets, is_data_transfering: false, }
    }

    pub fn prepare_data_transfer(&mut self, device: &HaLogicalDevice) -> Result<(), AllocatorError> {
        self.is_data_transfering = true;

        if let Some(ref mut memory) = self.memory {

            memory.prepare_data_transfer(device)?;
        } else {
            return Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }

        Ok(())
    }

    pub fn upload_data<D: Copy>(&mut self, device: &HaLogicalDevice, item: &BufferSubItem, data: &Vec<D>) -> Result<(), AllocatorError> {

        if self.is_data_transfering == false {
            return Err(AllocatorError::DataTransferNotActivate)
        }

        if let Some(ref mut memory) = self.memory {

            let offset = self.offsets[item.buffer_index] + item.offset;
            memory.add_transfer_data(device, item, data, offset)?;
        } else {
            return Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }

        Ok(())
    }

    pub fn execute_data_transfer(&mut self, device: &HaLogicalDevice) -> Result<(), AllocatorError> {

        if self.is_data_transfering == false {
            return Err(AllocatorError::DataTransferNotActivate)
        }

        if let Some(ref mut memory) = self.memory {

            memory.transfer_data(device)?;
            self.is_data_transfering = false;
        } else {
            return Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }

        Ok(())
    }

    pub(crate) fn buffer_at(&self, index: usize) -> &HaBuffer {
        &self.buffers[index]
    }
    pub(crate) fn borrow_mut_memory(&mut self) -> &mut Box<HaMemoryAbstract> {
        self.memory.as_mut().unwrap()
    }

    pub fn vertex_binding_infos(&self, items: &[&BufferSubItem]) -> CmdVertexBindingInfos {

        let mut handles = vec![];
        let mut offsets  = vec![];
        for item in items.iter() {
            handles.push(item.handle);
            offsets.push(item.offset);
        }

        CmdVertexBindingInfos {
            handles,
            offsets,
        }
    }
    pub fn index_binding_info(&self, item: &BufferSubItem) -> CmdIndexBindingInfo {

        CmdIndexBindingInfo {
            handle: item.handle,
            offset: item.offset,
        }
    }

    pub fn cleanup(&mut self, device: &HaLogicalDevice) {
        for buffer in self.buffers.iter() {
            buffer.cleanup(device);
        }

        if let Some(ref memory) = self.memory {
            memory.cleanup(device);
        }

        self.buffers.clear();
        self.offsets.clear();
    }
}

impl HaBufferRepository {

    // TODO: Make this function to support multiple buffer copy operation.
    pub fn copy_buffers_to_buffers(device: &HaLogicalDevice, from_items: &[&BufferSubItem], to_items: &[&BufferSubItem])
        -> Result<(), AllocatorError> {

        let mut transfer = device.transfer();
        {
            let command_buffer = transfer.command()?;

            let recorder = command_buffer.setup_record(device);
            recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?;

            for (&from, &to) in from_items.iter().zip(to_items.iter()) {
                // TODO: Only support one region.
                let copy_region = [
                    vk::BufferCopy {
                        src_offset: from.offset,
                        dst_offset: to.offset,
                        size      : to.size,
                    },
                ];

                recorder.copy_buffer(from.handle, to.handle, &copy_region);
            }

            recorder.finish()?;
        }

        transfer.excute()?;

        Ok(())
    }
}
