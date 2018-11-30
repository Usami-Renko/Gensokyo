
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::{ HaDevice, DeviceQueueIdentifier };
use command::buffer::{ HaCommandBuffer, CmdBufferUsage };
use command::error::CommandError;

use types::vkuint;

use std::ptr;

pub struct HaCommandPool {

    device: HaDevice,
    handle: vk::CommandPool,
}

impl HaCommandPool {

    // TODO: Add configuration for vk::CommandPoolCreateFlags.
    pub fn setup(device: &HaDevice, queue: DeviceQueueIdentifier, flags: vk::CommandPoolCreateFlags)
        -> Result<HaCommandPool, CommandError> {

        let queue = device.queue_handle_by_identifier(queue);

        let command_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags,
            queue_family_index: queue.family_index,
        };

        let handle = unsafe {
            device.handle.create_command_pool(&command_info, None)
                .or(Err(CommandError::PoolCreationError))?
        };

        let pool = HaCommandPool {
            device: device.clone(),
            handle,
        };
        Ok(pool)
    }

    /// Allocate vk::CommandBuffer from the vk::CommandPool.
    ///
    /// usage indicates the type of command buffer.
    pub fn allocate(&self, usage: CmdBufferUsage, count: usize) -> Result<Vec<HaCommandBuffer>, CommandError> {

        let allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: self.handle,
            level: usage.level(),
            command_buffer_count: count as vkuint,
        };

        let handles = unsafe {
            self.device.handle.allocate_command_buffers(&allocate_info)
                .or(Err(CommandError::BufferAllocateError))?
        };

        let buffers = handles.iter()
            .map(|&handle| HaCommandBuffer::new(handle, usage))
            .collect();
        Ok(buffers)
    }

    pub fn free(&self, buffers_to_free: &[HaCommandBuffer]) {

        let buffer_handles = collect_handle!(buffers_to_free);

        unsafe {
            self.device.handle.free_command_buffers(self.handle, &buffer_handles);
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            self.device.handle.destroy_command_pool(self.handle, None);
        }
    }
}
