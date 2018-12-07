
use ash::vk;
use ash::version::DeviceV1_0;

use gsma::collect_handle;

use crate::core::device::{ GsDevice, DeviceQueueIdentifier };
use crate::command::buffer::{ GsCommandBuffer, CmdBufferUsage };
use crate::command::error::CommandError;

use crate::types::vkuint;

use std::ptr;

pub struct GsCommandPool {

    device: GsDevice,
    handle: vk::CommandPool,
}

impl GsCommandPool {

    // TODO: Add configuration for vk::CommandPoolCreateFlags.
    pub fn setup(device: &GsDevice, queue: DeviceQueueIdentifier, flags: vk::CommandPoolCreateFlags)
        -> Result<GsCommandPool, CommandError> {

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

        let pool = GsCommandPool {
            device: device.clone(),
            handle,
        };
        Ok(pool)
    }

    /// Allocate vk::CommandBuffer from the vk::CommandPool.
    ///
    /// usage indicates the type of command buffer.
    pub fn allocate(&self, usage: CmdBufferUsage, count: usize) -> Result<Vec<GsCommandBuffer>, CommandError> {

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
            .map(|&handle| GsCommandBuffer::new(handle, usage))
            .collect();
        Ok(buffers)
    }

    pub fn free(&self, buffers_to_free: &[GsCommandBuffer]) {

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
