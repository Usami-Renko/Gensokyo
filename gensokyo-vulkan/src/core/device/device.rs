
use ash::vk;
use ash::version::DeviceV1_0;

use gsma::collect_handle;

use crate::core::device::GsDevice;
use crate::core::device::enums::{ DeviceQueueIdentifier, QueueRequestStrategy };
use crate::core::device::queue::{ GsGraphicsQueue, GsPresentQueue, GsTransferQueue, GsTransfer };
use crate::core::device::queue::{ GsQueue, QueueSubmitBundle };

use crate::sync::GsFence;

use crate::error::{ VkResult, VkError };
use crate::types::vklint;

use std::ptr;

pub struct GsLogicalDevice {

    pub(crate) handle: ash::Device,

    graphics_queue: GsGraphicsQueue,
    present_queue : GsPresentQueue,
    transfer_queue: GsTransferQueue,
}

impl GsLogicalDevice {

    pub(super) fn new(handle: ash::Device, graphics: GsGraphicsQueue, present: GsPresentQueue, transfer: GsTransferQueue) -> GsLogicalDevice {

        GsLogicalDevice {
            handle,
            graphics_queue: graphics,
            present_queue : present,
            transfer_queue: transfer,
        }
    }

    /// Tell device to wait for a group of fences.
    ///
    /// To wait for a single fence, use GsFence::wait() method instead.
    pub fn wait_fences(&self, fences: &[&GsFence], wait_all: bool, timeout: vklint) -> VkResult<()> {

        let handles: Vec<vk::Fence> = collect_handle!(fences);

        unsafe {
            self.handle.wait_for_fences(&handles, wait_all, timeout)
                .or(Err(VkError::device("Failed to reset fence.")))?
        }
        Ok(())
    }

    pub fn reset_fences(&self, fences: &[&GsFence]) -> VkResult<()> {

        let handles: Vec<vk::Fence> = collect_handle!(fences);

        unsafe {
            self.handle.reset_fences(&handles)
                .or(Err(VkError::device("Failed to reset fence.")))?
        }

        Ok(())
    }

    /// Submit a singe command bundle to Device.
    pub fn submit_single(&self, bundle: &QueueSubmitBundle, fence: Option<&GsFence>, queue_ident: DeviceQueueIdentifier) -> VkResult<()> {

        // TODO: Add configuration to select submit queue family
        // TODO: Add Speed test to this function.
        let wait_semaphores: Vec<vk::Semaphore> = collect_handle!(bundle.wait_semaphores);
        let sign_semaphores: Vec<vk::Semaphore> = collect_handle!(bundle.sign_semaphores);
        let commands: Vec<vk::CommandBuffer> = collect_handle!(bundle.commands);

        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            // an array of semaphores upon which to wait before the command buffers for this batch begin execution.
            wait_semaphore_count   : wait_semaphores.len() as _,
            p_wait_semaphores      : wait_semaphores.as_ptr(),
            // an array of pipeline stages at which each corresponding semaphore wait will occur.
            p_wait_dst_stage_mask  : bundle.wait_stages.as_ptr(),
            // an array of command buffers to execute in the batch.
            command_buffer_count   : commands.len() as _,
            p_command_buffers      : commands.as_ptr(),
            // an array of semaphores which will be signaled when the command buffers for this batch have completed execution.
            signal_semaphore_count : sign_semaphores.len() as _,
            p_signal_semaphores    : sign_semaphores.as_ptr(),
        };

        let queue = self.queue_handle_by_identifier(queue_ident);
        let fence = fence
            .and_then(|f| Some(f.handle))
            .unwrap_or(vk::Fence::null());

        unsafe {
            self.handle.queue_submit(queue.handle, &[submit_info], fence)
                .or(Err(VkError::device("Failed to submit command to device.")))?
        }

        Ok(())
    }

    pub fn wait_idle(&self) -> VkResult<()> {
        unsafe {
            self.handle.device_wait_idle()
                .or(Err(VkError::device("Device failed to wait idle.")))
        }
    }

    pub fn destroy(&self) {

        unsafe {
            self.graphics_queue.destroy();
            self.present_queue.destroy();
            self.transfer_queue.destroy(self);

            self.handle.destroy_device(None);
        }
    }

    pub fn transfer(device: &GsDevice) -> VkResult<GsTransfer> {

        device.transfer_queue.transfer(device)
    }

    pub fn queue_handle_by_identifier(&self, identifier: DeviceQueueIdentifier) -> &GsQueue {
        match identifier {
            | DeviceQueueIdentifier::Graphics => &self.graphics_queue.queue(),
            | DeviceQueueIdentifier::Present  => &self.present_queue.queue(),
            | DeviceQueueIdentifier::Transfer => &self.transfer_queue.queue(),
        }
    }

    pub fn update_descriptor_sets(&self, write_contents: &Vec<vk::WriteDescriptorSet>) {

        unsafe {
            self.handle.update_descriptor_sets(&write_contents, &[]);
        }
    }

    pub(crate) fn transfer_queue(&self) -> &GsTransferQueue {
        &self.transfer_queue
    }
}

#[derive(Debug, Clone)]
pub struct DeviceConfig {

    pub queue_request_strategy: QueueRequestStrategy,
    pub transfer_wait_time: vklint,
}
