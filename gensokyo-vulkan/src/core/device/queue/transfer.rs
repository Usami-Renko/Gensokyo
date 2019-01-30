
use ash::vk;
use ash::version::DeviceV1_0;

use gsma::collect_handle;

use crate::core::GsDevice;
use crate::core::device::device::{ GsLogicalDevice, DeviceConfig };
use crate::core::device::queue::GsQueue;

use crate::command::{ GsCommandBuffer, CmdBufferUsage };
use crate::sync::GsFence;

use crate::error::{ VkResult, VkError };
use crate::types::vklint;

use std::ptr;

pub struct GsTransferQueue {

    queue: GsQueue,
    pool: TransferCommandPool,

    transfer_wait_time: vklint,
}

impl GsTransferQueue {

    pub fn new(device: &ash::Device, queue: GsQueue, config: &DeviceConfig) -> VkResult<Self> {

        let pool = TransferCommandPool::setup(device, &queue)?;

        let transfer_queue = GsTransferQueue {
            queue, pool,
            transfer_wait_time: config.transfer_wait_time,
        };
        Ok(transfer_queue)
    }

    pub fn queue(&self) -> &GsQueue {
        &self.queue
    }

    pub fn destroy(&self, device: &GsLogicalDevice) {
        self.pool.destroy(device);
    }
}

impl GsTransferQueue {

    pub fn transfer(&self, device: &GsDevice) -> VkResult<GsTransfer> {

        let transfer = GsTransfer {
            device: device.clone(),
            command_buffers: vec![],
            // make sign to false, since the fence will be reset whenever transfer start.
            fence: GsFence::create(device, false)?,
            transfer_wait_time: self.transfer_wait_time,
        };
        Ok(transfer)
    }
}

pub struct GsTransfer {

    device: GsDevice,
    fence: GsFence,
    command_buffers: Vec<GsCommandBuffer>,
    transfer_wait_time: vklint,
}

impl GsTransfer {

    pub fn commands(&self, count: usize) -> VkResult<Vec<GsCommandBuffer>> {

        // just use a single primary command buffer for transfer.
        let transfer_queue = self.device.logic.transfer_queue();
        let commands = transfer_queue.pool.allocate(&self.device, count)?;
        Ok(commands)
    }

    pub fn commits(&mut self, commands: Vec<GsCommandBuffer>) {

        self.command_buffers.extend(commands);
    }

    pub fn command(&self) -> VkResult<GsCommandBuffer> {

        let transfer_queue = self.device.logic.transfer_queue();
        let mut commands = transfer_queue.pool.allocate(&self.device, 1)?;
        Ok(commands.pop().unwrap())
    }

    pub fn commit(&mut self, command: GsCommandBuffer) {
        self.command_buffers.push(command);
    }

    pub fn execute(&mut self) -> VkResult<()> {

        if self.command_buffers.is_empty() {
            return Ok(())
        }

        self.fence.reset()?;

        let submit_commands: Vec<vk::CommandBuffer> = collect_handle!(self.command_buffers);

        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count  : 0,
            p_wait_semaphores     : ptr::null(),
            p_wait_dst_stage_mask : ptr::null(),
            command_buffer_count  : submit_commands.len() as _,
            p_command_buffers     : submit_commands.as_ptr(),
            signal_semaphore_count: 0,
            p_signal_semaphores   : ptr::null(),
        };

        let transfer_queue = self.device.logic.transfer_queue();

        unsafe {
            self.device.logic.handle.queue_submit(transfer_queue.queue.handle, &[submit_info], self.fence.handle)
                .or(Err(VkError::device("Failed to submit command to device.")))?
        };

        self.fence.wait(self.transfer_wait_time)?;
        transfer_queue.pool.free(&self.device, &self.command_buffers);
        self.command_buffers.clear();

        Ok(())
    }
}

struct TransferCommandPool {

    handle: vk::CommandPool,
}

impl TransferCommandPool {

    fn setup(device: &ash::Device, queue: &GsQueue) -> VkResult<TransferCommandPool> {

        let command_pool_ci = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            // TODO: Consider CommandPoolFlag::ResetCommandBufferBit.
            // the command buffer will be short-live, so use TransientBit.
            flags: vk::CommandPoolCreateFlags::TRANSIENT,
            queue_family_index: queue.family_index,
        };

        let handle = unsafe {
            device.create_command_pool(&command_pool_ci, None)
                .or(Err(VkError::create("Command Pool")))?
        };

        let pool = TransferCommandPool { handle };
        Ok(pool)
    }

    fn allocate(&self, device: &GsDevice, count: usize) -> VkResult<Vec<GsCommandBuffer>> {

        let allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: self.handle,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: count as _,
        };

        let handles = unsafe {
            device.logic.handle.allocate_command_buffers(&allocate_info)
                .or(Err(VkError::create("Command Buffer")))?
        };

        let buffers = handles.iter()
            .map(|&handle|
                GsCommandBuffer::new(handle, CmdBufferUsage::UnitaryCommand)
            ).collect();

        Ok(buffers)
    }

    fn free(&self, device: &GsDevice, buffers_to_free: &[GsCommandBuffer]) {

        let buffer_handles: Vec<vk::CommandBuffer> = collect_handle!(buffers_to_free);

        unsafe {
            device.logic.handle.free_command_buffers(self.handle, &buffer_handles);
        }
    }

    fn destroy(&self, device: &GsLogicalDevice) {

        unsafe {
            device.handle.destroy_command_pool(self.handle, None);
        }
    }
}
