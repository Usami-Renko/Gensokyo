
use ash::vk;

use core::device::HaLogicalDevice;

use resources::buffer::{ HaBuffer, BufferSubItem };
use resources::command::CommandBufferUsageFlag;
use resources::memory::{ HaMemoryAbstract, HaMemoryType };
use resources::repository::{ BufferDataUploader, BufferDataUpdater };
use resources::error::{ AllocatorError, MemoryError };

use utility::memory::spaces_to_offsets;

pub struct HaBufferRepository {

    buffers: Vec<HaBuffer>,
    memory : Option<Box<HaMemoryAbstract>>,

    /// The offset of each buffer in memory.
    offsets: Vec<vk::DeviceSize>,
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

        }
    }

    pub(crate) fn store(buffers: Vec<HaBuffer>, memory: Box<HaMemoryAbstract>, spaces: Vec<vk::DeviceSize>) -> HaBufferRepository {

        let offsets = spaces_to_offsets(&spaces);

        HaBufferRepository { buffers, memory: Some(memory), offsets, }
    }

    pub fn data_uploader(&mut self, device: &HaLogicalDevice) -> Result<BufferDataUploader, AllocatorError> {

        if let Some(ref mut memory) = self.memory {

            BufferDataUploader::new(device, memory, &self.offsets)
        } else {
            Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }
    }

    pub fn data_updater(&mut self) -> Result<BufferDataUpdater, AllocatorError> {

        if let Some(ref mut memory) = self.memory {

            let updater = BufferDataUpdater::new(memory, &self.offsets);
            Ok(updater)
        } else {
            Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }
    }

    pub fn ready_update(&mut self, device: &HaLogicalDevice) -> Result<(), AllocatorError> {

        if let Some(ref mut memory) = self.memory {
            match memory.memory_type() {
                | HaMemoryType::HostMemory => {
                    memory.enable_map(device, true)?;
                },
                | HaMemoryType::DeviceMemory => {
                    // currently nothing to do.
                },
            }
        } else {
            return Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }

        Ok(())
    }

    pub fn shut_update(&mut self, device: &HaLogicalDevice) -> Result<(), AllocatorError> {

        if let Some(ref mut memory) = self.memory {
            match memory.memory_type() {
                | HaMemoryType::HostMemory => {
                    memory.enable_map(device, false)?;
                },
                | HaMemoryType::DeviceMemory => {
                    // currently nothing to do.
                },
            }
        } else {
            return Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }

        Ok(())
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
    pub fn copy_buffers_to_buffers(device: &HaLogicalDevice, from_items: &[BufferSubItem], to_items: &[BufferSubItem])
        -> Result<(), AllocatorError> {

        let mut transfer = device.transfer();
        {
            let command_buffer = transfer.command()?;

            let recorder = command_buffer.setup_record(device);
            recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?;

            for (from, to) in from_items.iter().zip(to_items.iter()) {
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
