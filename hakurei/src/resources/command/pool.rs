
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::{ HaDevice, DeviceQueueIdentifier };

use resources::command::{ HaCommandBuffer, CommandBufferUsage };
use resources::error::CommandError;

use utility::marker::VulkanFlags;
use utility::marker::Handles;

use std::ptr;

pub struct HaCommandPool {

    device: Option<HaDevice>,
    pub(super) handle: vk::CommandPool,
}

impl HaCommandPool {

    pub fn uninitialize() -> HaCommandPool {
        HaCommandPool {
            device: None,
            handle: vk::CommandPool::null(),
        }
    }

    pub(crate) fn setup(device: &HaDevice, queue: DeviceQueueIdentifier, flags: &[CommandPoolFlag])
        -> Result<HaCommandPool, CommandError> {

        let queue = device.queue_handle_by_identifier(queue);

        let info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::CommandPoolCreateInfo,
            p_next: ptr::null(),
            flags: flags.flags(),
            queue_family_index: queue.family_index,
        };

        let handle = unsafe {
            device.handle.create_command_pool(&info, None)
                .or(Err(CommandError::PoolCreationError))?
        };

        let pool = HaCommandPool {
            device: Some(device.clone()),
            handle,
        };
        Ok(pool)
    }

    /// Allocate vk::CommandBuffer from the vk::CommandPool.
    ///
    /// usage indicates the type of command buffer.
    pub fn allocate(&self, usage: CommandBufferUsage, count: usize) -> Result<Vec<HaCommandBuffer>, CommandError> {

        let allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::CommandBufferAllocateInfo,
            p_next: ptr::null(),
            command_pool: self.handle,
            level: usage.level(),
            command_buffer_count: count as uint32_t,
        };

        let device = self.device.as_ref().unwrap();

        let handles = unsafe {
            device.handle.allocate_command_buffers(&allocate_info)
                .or(Err(CommandError::BufferAllocateError))?
        };

        let buffers = handles.iter()
            .map(|&handle| HaCommandBuffer { handle, usage }).collect();
        Ok(buffers)
    }

    pub fn free(&self, buffers_to_free: &[HaCommandBuffer]) {
        let buffer_handles = buffers_to_free.handles();

        unsafe {
            self.device.as_ref().unwrap().handle
                .free_command_buffers(self.handle, &buffer_handles);
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            self.device.as_ref().unwrap().handle
                .destroy_command_pool(self.handle, None);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommandPoolFlag {
    /// TransientBit specifies that command buffers allocated from the pool will be short-lived,
    /// meaning that they will be reset or freed in a relatively short timeframe.
    ///
    /// This flag may be used by the implementation to control memory allocation behavior within the pool.
    TransientBit,
    /// ResetCommandBufferBit allows any command buffer allocated from a pool to be individually reset to the initial state; either by calling vkResetCommandBuffer, or via the implicit reset when calling vkBeginCommandBuffer.
    ///
    /// If this flag is not set on a pool, then vkResetCommandBuffer must not be called for any command buffer allocated from that pool.
    ResetCommandBufferBit,
}

impl VulkanFlags for [CommandPoolFlag] {
    type FlagType = vk::CommandPoolCreateFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::CommandPoolCreateFlags::empty(), |acc, flag| {
            match *flag {
                | CommandPoolFlag::TransientBit          => acc | vk::COMMAND_POOL_CREATE_TRANSIENT_BIT,
                | CommandPoolFlag::ResetCommandBufferBit => acc | vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            }
        })
    }
}
