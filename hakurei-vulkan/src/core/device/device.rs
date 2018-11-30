
use ash;
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::device::enums::{ DeviceQueueIdentifier, QueueRequestStrategy };
use core::device::queue::{ HaQueueAbstract, HaGraphicsQueue, HaPresentQueue, HaTransferQueue, HaTransfer };
use core::device::queue::{ HaQueue, QueueContainer, QueueSubmitBundle };
use core::error::LogicalDeviceError;

use sync::HaFence;
use descriptor::DescriptorWriteInfo;
use command::CommandError;
use sync::SyncError;

use types::{ vklint, vkuint };

use std::rc::Rc;
use std::ptr;

pub struct HaLogicalDevice {

    pub(crate) handle: ash::Device,

    queue_container: QueueContainer,

    graphics_queue: HaGraphicsQueue,
    present_queue : HaPresentQueue,
    transfer_queue: HaTransferQueue,
}

impl HaLogicalDevice {

    pub(super) fn new(handle: ash::Device, container: QueueContainer, graphics: HaGraphicsQueue, present: HaPresentQueue, transfer: HaTransferQueue) -> HaLogicalDevice {

        HaLogicalDevice {
            handle,
            queue_container: container,
            graphics_queue: graphics,
            present_queue : present,
            transfer_queue: transfer,
        }
    }

    /// Tell device to wait for a group of fences.
    ///
    /// To wait for a single fence, use HaFence::wait() method instead.
    pub fn wait_fences(&self, fences: &[HaFence], wait_all: bool, timeout: vklint) -> Result<(), SyncError> {

        let handles: Vec<vk::Fence> = collect_handle!(fences);

        unsafe {
            self.handle.wait_for_fences(&handles, wait_all, timeout)
                .or(Err(SyncError::FenceTimeOutError))?;
        }
        Ok(())
    }

    pub fn reset_fences(&self, fences: &[HaFence]) -> Result<(), SyncError> {

        let handles: Vec<vk::Fence> = collect_handle!(fences);

        unsafe {
            self.handle.reset_fences(&handles)
                .or(Err(SyncError::FenceResetError))?;
        }

        Ok(())
    }

    pub fn transfer(device: &HaDevice) -> HaTransfer {

        device.transfer_queue.transfer(device)
    }

    pub fn submit(&self, bundles: &[QueueSubmitBundle], fence: Option<&HaFence>, queue_ident: DeviceQueueIdentifier) -> Result<(), CommandError> {

        // TODO: Add configuration to select submit queue family
        // TODO: Add Speed test to this function.
        let mut submit_infos = vec![];
        for bundle in bundles.iter() {

            let wait_semaphores: Vec<vk::Semaphore> = collect_handle!(bundle.wait_semaphores);
            let sign_semaphores: Vec<vk::Semaphore> = collect_handle!(bundle.sign_semaphores);
            let commands: Vec<vk::CommandBuffer> = collect_handle!(bundle.commands);

            let submit_info = vk::SubmitInfo {
                s_type: vk::StructureType::SUBMIT_INFO,
                p_next: ptr::null(),
                // an array of semaphores upon which to wait before the command buffers for this batch begin execution.
                wait_semaphore_count   : wait_semaphores.len() as vkuint,
                p_wait_semaphores      : wait_semaphores.as_ptr(),
                // an array of pipeline stages at which each corresponding semaphore wait will occur.
                p_wait_dst_stage_mask  : bundle.wait_stages.as_ptr(),
                // an array of command buffers to execute in the batch.
                command_buffer_count   : commands.len() as vkuint,
                p_command_buffers      : commands.as_ptr(),
                // an array of semaphores which will be signaled when the command buffers for this batch have completed execution.
                signal_semaphore_count : sign_semaphores.len() as vkuint,
                p_signal_semaphores    : sign_semaphores.as_ptr(),
            };

            submit_infos.push(submit_info);
        }

        let queue = self.queue_handle_by_identifier(queue_ident);
        let fence = fence
            .and_then(|f| Some(f.handle))
            .unwrap_or(HaFence::null_handle());

        unsafe {
            self.handle.queue_submit(queue.handle, &submit_infos, fence)
                .or(Err(CommandError::QueueSubmitError))?;
        }

        Ok(())
    }

    pub fn wait_idle(&self) -> Result<(), LogicalDeviceError> {
        unsafe {
            self.handle.device_wait_idle()
                .or(Err(LogicalDeviceError::WaitIdleError))
        }
    }

    pub fn cleanup(&self) {

        unsafe {
            self.graphics_queue.cleanup(self);
            self.present_queue.cleanup(self);
            self.transfer_queue.cleanup(self);

            self.handle.destroy_device(None);
        }
    }

    pub fn queue_handle_by_identifier(&self, identifier: DeviceQueueIdentifier) -> &Rc<HaQueue> {
        match identifier {
            | DeviceQueueIdentifier::Graphics => &self.graphics_queue.queue(),
            | DeviceQueueIdentifier::Present  => &self.present_queue.queue(),
            | DeviceQueueIdentifier::Transfer => &self.transfer_queue.queue(),
            | DeviceQueueIdentifier::Custom { identifier, queue_index } => {
                self.queue_container.queue(*identifier, queue_index)
            },
        }
    }

    pub fn update_descriptor_sets(&self, write_infos: Vec<DescriptorWriteInfo>) {

        let write_sets: Vec<vk::WriteDescriptorSet> = write_infos.into_iter()
            .map(|info| info.info)
            .collect();

        unsafe {
            self.handle.update_descriptor_sets(&write_sets, &[]);
        }
    }

    pub(crate) fn transfer_queue(&self) -> &HaTransferQueue {
        &self.transfer_queue
    }
}

#[derive(Debug, Clone)]
pub struct DeviceConfig {

    pub queue_request_strategy: QueueRequestStrategy,
    pub transfer_wait_time: vklint,
}
