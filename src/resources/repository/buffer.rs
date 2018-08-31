
use ash::vk;

use core::device::{ HaLogicalDevice, QueueSubmitBundle, DeviceQueueIdentifier };

use resources::buffer::HaBuffer;
use resources::command::{ CommandBufferUsageFlag, CommandBufferUsage };
use resources::memory::device::HaDeviceMemory;
use resources::memory::traits::HaMemoryAbstract;
use resources::error::AllocatorError;

pub struct HaBufferRepository {

    buffers: Vec<HaBuffer>,
    memory : Option<HaDeviceMemory>,

    /// The size of each buffer occupy.
    spaces : Vec<vk::DeviceSize>,
    /// The offset of each buffer in meomory.
    offsets: Vec<vk::DeviceSize>,
}

pub struct BufferBindingInfos {

    pub(crate) handles: Vec<vk::Buffer>,
    pub(crate) offsets: Vec<vk::DeviceSize>,
}


impl HaBufferRepository {

    pub fn empty() -> HaBufferRepository {
        HaBufferRepository {
            buffers: vec![],
            memory : None,

            spaces : vec![],
            offsets: vec![],
        }
    }

    pub(crate) fn store(buffers: Vec<HaBuffer>, memory: HaDeviceMemory, spaces: Vec<vk::DeviceSize>) -> HaBufferRepository {

        let mut current: vk::DeviceSize = 0;
        let mut offsets = vec![];
        for space in spaces.iter() {
            offsets.push(current);
            current += space;
        }

        HaBufferRepository {
            buffers,
            memory: Some(memory),

            spaces,
            offsets,
        }
    }

    pub fn tranfer_data<D: Copy>(&self, device: &HaLogicalDevice, data: &Vec<D>, buffer_index: usize) -> Result<(), AllocatorError> {

        let memory = self.memory.as_ref().ok_or(AllocatorError::MemoryNotYetAllocated)?;

        let size = self.spaces[buffer_index];
        let offset = self.offsets[buffer_index];

        let data_ptr = memory.map(device, offset, size)
            .map_err(|e| AllocatorError::Memory(e))?;

        self.buffers[buffer_index].copy_data(data_ptr, size, data);

        memory.unmap(device, offset, size)
            .map_err(|e| AllocatorError::Memory(e))?;

        Ok(())
    }

    pub fn copy_data(&self, device: &HaLogicalDevice, from_repository: &HaBufferRepository, from_buffer_index: usize, to_buffer_index: usize) -> Result<(), AllocatorError> {

        let mut command_buffers = device.transfer_command_pool.allocate(device, CommandBufferUsage::UnitaryCommand, 1)?;
        let command_buffer = command_buffers.pop().unwrap();

        let copy_regions = [
            vk::BufferCopy {
                src_offset : from_repository.offsets[from_buffer_index],
                dst_offset : self.offsets[to_buffer_index],
                size       : from_repository.spaces[from_buffer_index],
            },
        ];

        let recorder = command_buffer.setup_record(device);
        recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?
            .copy_buffer(
                from_repository.buffer_at(from_buffer_index),
                self.buffer_at(to_buffer_index),
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

    fn buffer_at(&self, index: usize) -> &HaBuffer {
        &self.buffers[index]
    }

    pub fn binding_infos(&self) -> BufferBindingInfos {

        let handles: Vec<vk::Buffer> = self.buffers.iter().map(|b| b.handle).collect();
        let offsets = self.offsets.clone();

        BufferBindingInfos {
            handles,
            offsets,
        }
    }

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        for buffer in self.buffers.iter() {
            buffer.cleanup(device);
        }

        if let Some(ref memory) = self.memory {
            memory.cleanup(device);
        }
    }
}
