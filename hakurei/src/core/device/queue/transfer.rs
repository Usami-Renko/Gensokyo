
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use config::core::CoreConfig;
use core::DeviceV1;
use core::device::{ HaLogicalDevice, HaQueue };
use core::error::LogicalDeviceError;

use resources::command::{ HaCommandPool, CommandPoolFlag };
use resources::command::{ HaCommandBuffer, CommandBufferUsage };
use resources::error::{ CommandError, AllocatorError };

use sync::fence::HaFence;

use utility::marker::Handles;
use utility::time::TimePeriod;

use std::ptr;

pub struct TransferQueue {

    queue: HaQueue,
    pool : HaCommandPool,

    transfer_wait_time: TimePeriod,
}

impl<'re> TransferQueue {

    pub fn new(device: &DeviceV1, queue: HaQueue, config: &CoreConfig) -> Result<TransferQueue, LogicalDeviceError> {

        let pool = HaCommandPool::setup_from_handle(device, &queue, &[
            // the command buffer will be short-live, so use TransientBit.
            CommandPoolFlag::TransientBit,
            // TODO: Consider CommandPoolFlag::ResetCommandBufferBit.
        ])?;

        let transfer_queue = TransferQueue { queue, pool, transfer_wait_time: config.transfer_wait_time, };
        Ok(transfer_queue)
    }

    pub fn transfer(&'re self, device: &'re HaLogicalDevice) -> HaTransfer<'re> {

        // make sign to false, since the fence will be reset whenever transfer start.
        let fence = HaFence::setup(device, false).unwrap();
        let commands = vec![];

        HaTransfer {
            device,
            command_buffers: commands, fence,
            queue: &self.queue,
            pool : &self.pool,
            transfer_wait_time: self.transfer_wait_time,
        }
    }

    pub fn clean(&self, device: &HaLogicalDevice) {
        self.pool.cleanup(device);
    }
}

pub struct HaTransfer<'re> {

    device: &'re HaLogicalDevice,
    queue : &'re HaQueue,
    pool  : &'re HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    fence : HaFence,
    transfer_wait_time: TimePeriod,
}

impl<'re> HaTransfer<'re> {

    pub fn commands(&mut self, count: usize) -> Result<&[HaCommandBuffer], CommandError> {

        // just use a single primary command buffer for transferation.
        let mut new_commands = self.pool.allocate(self.device, CommandBufferUsage::UnitaryCommand, count)?;
        let start_index = self.command_buffers.len();
        self.command_buffers.append(&mut new_commands);

        let commands = &self.command_buffers[start_index..];
        Ok(commands)
    }

    pub fn command(&mut self) -> Result<&HaCommandBuffer, CommandError> {

        let mut new_commands = self.pool.allocate(self.device, CommandBufferUsage::UnitaryCommand, 1)?;
        self.command_buffers.append(&mut new_commands);
        Ok(&self.command_buffers.last().unwrap())
    }

    pub fn excute(&mut self) -> Result<(), AllocatorError> {

        if self.command_buffers.is_empty() {
            return Err(CommandError::NoCommandAvailable)?;
        }

        self.fence.reset(self.device)?;

        let submit_commands = self.command_buffers.handles();

        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SubmitInfo,
            p_next: ptr::null(),
            wait_semaphore_count  : 0,
            p_wait_semaphores     : ptr::null(),
            p_wait_dst_stage_mask : ptr::null(),
            command_buffer_count  : submit_commands.len() as uint32_t,
            p_command_buffers     : submit_commands.as_ptr(),
            signal_semaphore_count: 0,
            p_signal_semaphores   : ptr::null(),
        };

        unsafe {
            self.device.handle.queue_submit(self.queue.handle, &[submit_info], self.fence.handle)
                .or(Err(CommandError::QueueSubmitError))?;
        }

        self.fence.wait(self.device, self.transfer_wait_time)?;
        self.pool.free(self.device, &self.command_buffers);
        self.command_buffers.clear();

        Ok(())
    }
}

impl<'re> Drop for HaTransfer<'re> {

    fn drop(&mut self) {

        unsafe {
            self.device.handle.destroy_fence(self.fence.handle, None);
        }
    }
}
