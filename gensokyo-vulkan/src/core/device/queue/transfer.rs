
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::GsDevice;
use core::device::device::{ GsLogicalDevice, DeviceConfig };
use core::device::queue::{ GsQueue, GsQueueAbstract };
use core::error::LogicalDeviceError;

use sync::{ GsFence, SyncError };
use command::{ GsCommandBuffer, CmdBufferUsage };
use command::CommandError;

use types::{ vklint, vkuint };

use std::rc::Rc;
use std::ptr;

pub struct GsTransferQueue {

    queue: Rc<GsQueue>,
    pool: TransferCommandPool,

    transfer_wait_time: vklint,
}

impl GsQueueAbstract for GsTransferQueue {

    fn new(device: &ash::Device, queue: &Rc<GsQueue>, config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let pool = TransferCommandPool::setup(device, queue)?;

        let transfer_queue = GsTransferQueue {
            queue: queue.clone(),
            pool,
            transfer_wait_time: config.transfer_wait_time,
        };
        Ok(transfer_queue)
    }

    fn queue(&self) -> &Rc<GsQueue> {
        &self.queue
    }

    fn cleanup(&self, device: &GsLogicalDevice) {
        self.pool.cleanup(device);
    }
}

impl GsTransferQueue {

    pub fn transfer(&self, device: &GsDevice) -> GsTransfer {

        GsTransfer {
            device: device.clone(),
            command_buffers: vec![],
            // make sign to false, since the fence will be reset whenever transfer start.
            // TODO: handle unwrap().
            fence: GsFence::setup(device, false).unwrap(),
            transfer_wait_time: self.transfer_wait_time,
        }
    }
}

pub struct GsTransfer {

    device: GsDevice,
    fence: GsFence,
    command_buffers: Vec<GsCommandBuffer>,
    transfer_wait_time: vklint,
}

impl GsTransfer {

    pub fn commands(&self, count: usize) -> Result<Vec<GsCommandBuffer>, CommandError> {

        // just use a single primary command buffer for transferation.
        let transfer_queue = self.device.transfer_queue();
        let commands = transfer_queue.pool.allocate(&self.device, count)?;
        Ok(commands)
    }

    pub fn commits(&mut self, commands: Vec<GsCommandBuffer>) {

        self.command_buffers.extend(commands);
    }

    pub fn command(&self) -> Result<GsCommandBuffer, CommandError> {

        let transfer_queue = self.device.transfer_queue();
        let mut commands = transfer_queue.pool.allocate(&self.device, 1)?;
        Ok(commands.pop().unwrap())
    }

    pub fn commit(&mut self, command: GsCommandBuffer) {
        self.command_buffers.push(command);
    }

    pub fn excute(&mut self) -> Result<(), SyncError> {

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
            command_buffer_count  : submit_commands.len() as vkuint,
            p_command_buffers     : submit_commands.as_ptr(),
            signal_semaphore_count: 0,
            p_signal_semaphores   : ptr::null(),
        };

        let transfer_queue = self.device.transfer_queue();

        unsafe {
            self.device.handle.queue_submit(transfer_queue.queue.handle, &[submit_info], self.fence.handle)
                .or(Err(SyncError::QueueSubmitError))?;
        }

        self.fence.wait(self.transfer_wait_time)?;
        transfer_queue.pool.free(&self.device, &self.command_buffers);
        self.command_buffers.clear();

        Ok(())
    }
}

impl Drop for GsTransfer {

    fn drop(&mut self) {

        self.fence.cleanup();
    }
}


struct TransferCommandPool {

    handle: vk::CommandPool,
}

impl TransferCommandPool {

    fn setup(device: &ash::Device, queue: &GsQueue) -> Result<TransferCommandPool, CommandError> {

        let info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            // TODO: Consider CommandPoolFlag::ResetCommandBufferBit.
            // the command buffer will be short-live, so use TransientBit.
            flags: vk::CommandPoolCreateFlags::TRANSIENT,
            queue_family_index: queue.family_index,
        };

        let handle = unsafe {
            device.create_command_pool(&info, None)
                .or(Err(CommandError::PoolCreationError))?
        };

        let pool = TransferCommandPool { handle };
        Ok(pool)
    }

    fn allocate(&self, device: &GsDevice, count: usize) -> Result<Vec<GsCommandBuffer>, CommandError> {

        let allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: self.handle,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: count as vkuint,
        };

        let handles = unsafe {
            device.handle.allocate_command_buffers(&allocate_info)
                .or(Err(CommandError::BufferAllocateError))?
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
            device.handle.free_command_buffers(self.handle, &buffer_handles);
        }
    }

    fn cleanup(&self, device: &GsLogicalDevice) {

        unsafe {
            device.handle.destroy_command_pool(self.handle, None);
        }
    }
}
