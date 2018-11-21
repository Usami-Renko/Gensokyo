
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::device::device::{ HaLogicalDevice, DeviceConfig };
use core::device::queue::{ HaQueue, HaQueueAbstract };
use core::error::LogicalDeviceError;

use sync::HaFence;
use command::{ HaCommandBuffer, CmdBufferUsage };
use command::CommandError;

use types::{ vklint, vkuint };

use std::rc::Rc;
use std::ptr;

pub struct HaTransferQueue {

    queue: Rc<HaQueue>,
    pool: TransferCommandPool,

    transfer_wait_time: vklint,
}

impl HaQueueAbstract for HaTransferQueue {

    fn new(device: &ash::Device, queue: &Rc<HaQueue>, config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let pool = TransferCommandPool::setup(device, queue)?;

        let transfer_queue = HaTransferQueue {
            queue: queue.clone(),
            pool,
            transfer_wait_time: config.transfer_wait_time,
        };
        Ok(transfer_queue)
    }

    fn queue(&self) -> &Rc<HaQueue> {
        &self.queue
    }

    fn cleanup(&self, device: &HaLogicalDevice) {
        self.pool.cleanup(device);
    }
}

impl HaTransferQueue {

    pub fn transfer(&self, device: &HaDevice) -> HaTransfer {

        // make sign to false, since the fence will be reset whenever transfer start.
        // TODO: handle unwrap().
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
    fence: HaFence,
    command_buffers: Vec<HaCommandBuffer>,
    transfer_wait_time: vklint,
}

impl HaTransfer {

    pub fn commands(&self, count: usize) -> Result<Vec<HaCommandBuffer>, CommandError> {

        // just use a single primary command buffer for transferation.
        let transfer_queue = self.device.transfer_queue();
        let commands = transfer_queue.pool.allocate(&self.device, count)?;
        Ok(commands)
    }

    pub fn commits(&mut self, commands: Vec<HaCommandBuffer>) {
        commands.into_iter()
            .for_each(|command| self.command_buffers.push(command));
    }

    pub fn command(&self) -> Result<HaCommandBuffer, CommandError> {

        let transfer_queue = self.device.transfer_queue();
        let mut commands = transfer_queue.pool.allocate(&self.device, 1)?;
        Ok(commands.pop().unwrap())
    }

    pub fn commit(&mut self, command: HaCommandBuffer) {
        self.command_buffers.push(command);
    }

    pub fn excute(&mut self) -> Result<(), CommandError> {

        if self.command_buffers.is_empty() {
            return Err(CommandError::NoCommandAvailable)?;
        }

        // TODO: handle unwrap().
        self.fence.reset().unwrap();

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
                .or(Err(CommandError::QueueSubmitError))?;
        }

        // TODO: handle unwrap().
        self.fence.wait(self.transfer_wait_time).unwrap();
        transfer_queue.pool.free(&self.device, &self.command_buffers);
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

    fn setup(device: &ash::Device, queue: &HaQueue) -> Result<TransferCommandPool, CommandError> {

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

    fn allocate(&self, device: &HaDevice, count: usize) -> Result<Vec<HaCommandBuffer>, CommandError> {

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
                HaCommandBuffer::new(handle, CmdBufferUsage::UnitaryCommand)
            ).collect();

        Ok(buffers)
    }

    fn free(&self, device: &HaDevice, buffers_to_free: &[HaCommandBuffer]) {

        let buffer_handles: Vec<vk::CommandBuffer> = collect_handle!(buffers_to_free);

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
