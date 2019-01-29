
use ash::vk;
use ash::version::DeviceV1_0;

use gsma::collect_handle;

use crate::core::GsDevice;
use crate::core::device::DeviceQueueIdentifier;
use crate::command::buffer::{ GsCommandBuffer, CmdBufferUsage };
use crate::error::{ VkResult, VkError };

use std::ptr;

pub struct GsCommandPool {

    device: GsDevice,
    handle: vk::CommandPool,
}

impl GsCommandPool {

    // TODO: Add configuration for vk::CommandPoolCreateFlags.
    pub fn setup(device: &GsDevice, queue: DeviceQueueIdentifier, flags: vk::CommandPoolCreateFlags)
        -> VkResult<GsCommandPool> {

        let queue = device.logic.queue_handle_by_identifier(queue);

        let command_pool_ci = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags,
            queue_family_index: queue.family_index,
        };

        let handle = unsafe {
            device.logic.handle.create_command_pool(&command_pool_ci, None)
                .or(Err(VkError::create("Command Pool")))?
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
    pub fn allocate(&self, usage: CmdBufferUsage, count: usize) -> VkResult<Vec<GsCommandBuffer>> {

        let allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: self.handle,
            level: usage.level(),
            command_buffer_count: count as _,
        };

        let handles = unsafe {
            self.device.logic.handle.allocate_command_buffers(&allocate_info)
                .or(Err(VkError::device("Failed to allocate Command Buffer.")))?
        };

        let buffers = handles.iter()
            .map(|&handle| GsCommandBuffer::new(handle, usage))
            .collect();
        Ok(buffers)
    }

    pub fn free(&self, buffers_to_free: &[GsCommandBuffer]) {

        let buffer_handles: Vec<vk::CommandBuffer> = collect_handle!(buffers_to_free);

        unsafe {
            self.device.logic.handle.free_command_buffers(self.handle, &buffer_handles);
        }
    }
}

impl Drop for GsCommandPool {

    fn drop(&mut self) {
        unsafe {
            self.device.logic.handle.destroy_command_pool(self.handle, None);
        }
    }
}
