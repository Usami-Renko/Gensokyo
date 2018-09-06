
use ash::vk;

use core::device::{ HaLogicalDevice, QueueSubmitBundle, DeviceQueueIdentifier };

use resources::buffer::{HaBuffer, BufferSubItem};
use resources::command::{ CommandBufferUsageFlag, CommandBufferUsage };
use resources::memory::device::HaDeviceMemory;
use resources::memory::traits::HaMemoryAbstract;
use resources::error::AllocatorError;

use utility::memory::spaces_to_offsets;

pub struct HaBufferRepository {

    buffers: Vec<HaBuffer>,
    memory : Option<HaDeviceMemory>,

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

    pub(crate) fn store(buffers: Vec<HaBuffer>, memory: HaDeviceMemory, spaces: Vec<vk::DeviceSize>) -> HaBufferRepository {

        let offsets = spaces_to_offsets(&spaces);

        HaBufferRepository { buffers, memory: Some(memory), offsets, }
    }

    pub fn tranfer_data<D: Copy>(&self, device: &HaLogicalDevice, data: &Vec<D>, item: &BufferSubItem) -> Result<(), AllocatorError> {

        let memory = self.memory.as_ref()
            .ok_or(AllocatorError::MemoryNotYetAllocated)?;

        let offset = self.offsets[item.buffer_index] + item.offset;

        let data_ptr = memory.map(device, offset, item.size)
            .map_err(|e| AllocatorError::Memory(e))?;

        self.buffers[item.buffer_index].copy_data(data_ptr, item.size, data);

        // FIXME: No need to unmap size every time.
        memory.unmap(device, offset, item.size)
            .map_err(|e| AllocatorError::Memory(e))?;

        Ok(())
    }

    pub fn copy_buffer_to_buffer(&self, device: &HaLogicalDevice, from_item: &BufferSubItem, to_item: &BufferSubItem) -> Result<(), AllocatorError> {

        let mut command_buffers = device.transfer_command_pool.allocate (device, CommandBufferUsage::UnitaryCommand, 1)?;
        let command_buffer = command_buffers.pop().unwrap();

        // TODO: Only support one region.
        let copy_regions = [
            vk::BufferCopy {
                src_offset: from_item.offset,
                dst_offset: to_item.offset,
                size      : to_item.size,
            },
        ];

        let recorder = command_buffer.setup_record(device);
        recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?
            .copy_buffer(
                from_item.handle,
                to_item.handle,
                &copy_regions)
            .finish()?;

        let submit_infos = [
            QueueSubmitBundle {
                wait_semaphores: &[],
                sign_semaphores: &[],
                wait_stages    : &[],
                commands       : &[&command_buffer],
            },
        ];

        device.submit(&submit_infos, None, DeviceQueueIdentifier::Transfer)?;
        // FIXME: Use fence would allow you to schedule multiple transfers simultaneously and wait for all of them complete, instead of executing one at a time.
        let _ = device.wait_idle();

        device.transfer_command_pool.free(device, &[&command_buffer]);

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
        self.buffers.clear();

        if let Some(ref memory) = self.memory {
            memory.cleanup(device);
        }
        self.memory = None;

        self.offsets.clear();
    }
}
