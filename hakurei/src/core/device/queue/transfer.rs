
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use config::core::DeviceConfig;
use core::DeviceV1;
use core::device::{ HaLogicalDevice, HaDevice, HaQueue };
use core::device::queue::HaQueueAbstract;
use core::error::LogicalDeviceError;

use resources::command::{ HaCommandBuffer, CommandBufferUsage, CommandPoolFlag };
use resources::error::{ CommandError, AllocatorError };

use sync::fence::HaFence;

use utility::marker::{ VulkanFlags, Handles };
use utility::time::TimePeriod;

use std::rc::Rc;
use std::ptr;

pub struct HaTransferQueue {

    pub queue: Rc<HaQueue>,
    pool : TransferCommandPool,

    transfer_wait_time: TimePeriod,
}

impl HaQueueAbstract for HaTransferQueue {

    fn new(device: &DeviceV1, queue: &Rc<HaQueue>, config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let pool = TransferCommandPool::setup(device, queue)?;

        let transfer_queue = HaTransferQueue {
            queue: queue.clone(),
            pool,
            transfer_wait_time: config.transfer_wait_time,
        };
        Ok(transfer_queue)
    }

    fn handle(&self) -> vk::Queue {
        self.queue.handle
    }

    fn cleanup(&self, device: &HaLogicalDevice) {
        self.pool.cleanup(device);
    }
}

impl HaTransferQueue {

    pub fn transfer(&self, device: &HaDevice) -> HaTransfer {

        // make sign to false, since the fence will be reset whenever transfer start.
        let fence = HaFence::setup(device, false).unwrap();
        let commands = vec![];

        HaTransfer {
            device: device.clone(),
            command_buffers: commands, fence,
            transfer_wait_time: self.transfer_wait_time,
        }
    }
}

pub struct HaTransfer {

    device: HaDevice,
    command_buffers: Vec<HaCommandBuffer>,

    fence : HaFence,
    transfer_wait_time: TimePeriod,
}

impl HaTransfer {

    pub fn commands(&mut self, count: usize) -> Result<&[HaCommandBuffer], CommandError> {

        // just use a single primary command buffer for transferation.
        let mut new_commands = self.device.transfer_queue.pool.allocate(&self.device, count)?;
        let start_index = self.command_buffers.len();
        self.command_buffers.append(&mut new_commands);

        let commands = &self.command_buffers[start_index..];
        Ok(commands)
    }

    pub fn command(&mut self) -> Result<&HaCommandBuffer, CommandError> {

        let mut new_commands = self.device.transfer_queue.pool.allocate(&self.device, 1)?;
        self.command_buffers.append(&mut new_commands);
        Ok(&self.command_buffers.last().unwrap())
    }

    pub fn excute(&mut self) -> Result<(), AllocatorError> {

        if self.command_buffers.is_empty() {
            return Err(CommandError::NoCommandAvailable)?;
        }

        self.fence.reset()?;

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
            self.device.handle.queue_submit(self.device.transfer_queue.queue.handle, &[submit_info], self.fence.handle)
                .or(Err(CommandError::QueueSubmitError))?;
        }

        self.fence.wait(self.transfer_wait_time)?;
        self.device.transfer_queue.pool.free(&self.device, &self.command_buffers);
        self.command_buffers.clear();

        Ok(())
    }
}

impl Drop for HaTransfer {

    fn drop(&mut self) {

        self.fence.cleanup();
    }
}





struct TransferCommandPool {

    handle: vk::CommandPool,
}

impl TransferCommandPool {

    fn setup(device: &DeviceV1, queue: &HaQueue) -> Result<TransferCommandPool, CommandError> {

        let info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::CommandPoolCreateInfo,
            p_next: ptr::null(),
            // TODO: Consider CommandPoolFlag::ResetCommandBufferBit.
            // the command buffer will be short-live, so use TransientBit.
            flags: [CommandPoolFlag::TransientBit].flags(),
            queue_family_index: queue.family_index,
        };

        let handle = unsafe {
            device.create_command_pool(&info, None)
                .or(Err(CommandError::PoolCreationError))?
        };

        let pool = TransferCommandPool { handle, };
        Ok(pool)
    }

    fn allocate(&self, device: &HaDevice, count: usize) -> Result<Vec<HaCommandBuffer>, CommandError> {

        let allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::CommandBufferAllocateInfo,
            p_next: ptr::null(),
            command_pool: self.handle,
            level: vk::CommandBufferLevel::Primary,
            command_buffer_count: count as uint32_t,
        };

        let handles = unsafe {
            device.handle.allocate_command_buffers(&allocate_info)
                .or(Err(CommandError::BufferAllocateError))?
        };

        let buffers = handles.iter()
            .map(|&handle|
                HaCommandBuffer::new(&device, handle, CommandBufferUsage::UnitaryCommand)
            ).collect();

        Ok(buffers)
    }

    fn free(&self, device: &HaDevice, buffers_to_free: &[HaCommandBuffer]) {
        let buffer_handles = buffers_to_free.handles();

        unsafe {
            device.handle.free_command_buffers(self.handle, &buffer_handles);
        }
    }

    fn cleanup(&self, device: &HaLogicalDevice) {

        unsafe {
            device.handle.destroy_command_pool(self.handle, None);
        }
    }
}
