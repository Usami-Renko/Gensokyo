
use ash;
use ash::vk;
use ash::vk::uint32_t;
use ash::version::V1_0;
use ash::version::DeviceV1_0;

use core::device::queue::HaQueue;
use core::device::queue::QueueSubmitBundle;
use core::error::LogicalDeviceError;

use sync::fence::HaFence;
use sync::error::SyncError;
use utility::time::TimePeriod;
use utility::marker::Handles;

use std::ptr;

pub struct HaLogicalDevice {

    pub(crate) handle: ash::Device<V1_0>,
    pub(crate) queues: Vec<HaQueue>,

    pub(crate) graphics_queue: HaQueue,
    pub(crate) present_queue : HaQueue,
    pub(crate) transfer_queue: HaQueue,
}

pub enum DeviceQueueIdentifier {
    Graphics,
    Present,
    Transfer,
    Custom(usize),
}

impl<'resource> HaLogicalDevice {

    /// Tell device to wait for a group of fences.
    ///
    /// To wait for a single fence, use HaFence::wait() method instead.
    pub fn wait_fences(&self, fences: &[HaFence], wait_all: bool, timeout: TimePeriod) -> Result<(), SyncError> {
        let handles = fences.handles();
        unsafe {
            self.handle.wait_for_fences(&handles, wait_all, timeout.vulkan_time())
                .or(Err(SyncError::FenceTimeOutError))?;
        }
        Ok(())
    }

    pub fn reset_fences(&self, fences: &[HaFence]) -> Result<(), SyncError> {
        let handles = fences.handles();
        unsafe {
            self.handle.reset_fences(&handles)
                .or(Err(SyncError::FenceResetError))?;
        }

        Ok(())
    }

    pub fn submit(&self, bundles: &[QueueSubmitBundle], fence: Option<&HaFence>, queue: DeviceQueueIdentifier)
        -> Result<(), LogicalDeviceError> {

        // TODO: Add configuration to select submit queue family
        // TODO: Add Speed test to this function.
        let mut submit_infos = vec![];
        for bundle in bundles.iter() {

            let wait_semaphores = bundle.wait_semaphores.handles();
            let sign_semaphores = bundle.sign_semaphores.handles();
            let stages = bundle.wait_stages.handles();
            let commands = bundle.commands.handles();

            let submit_info = vk::SubmitInfo {
                s_type: vk::StructureType::SubmitInfo,
                p_next: ptr::null(),
                // an array of semaphores upon which to wait before the command buffers for this batch begin execution.
                wait_semaphore_count: wait_semaphores.len() as uint32_t,
                p_wait_semaphores   : wait_semaphores.as_ptr(),
                // an array of pipeline stages at which each corresponding semaphore wait will occur.
                p_wait_dst_stage_mask: stages.as_ptr(),
                // an array of command buffers to execute in the batch.
                command_buffer_count: commands.len() as uint32_t,
                p_command_buffers   : commands.as_ptr(),
                // an array of semaphores which will be signaled when the command buffers for this batch have completed execution.
                signal_semaphore_count: sign_semaphores.len() as uint32_t,
                p_signal_semaphores   : sign_semaphores.as_ptr(),
            };

            submit_infos.push(submit_info);
        }

        let queue = self.queue_by_identifier(queue);
        let fence = fence
            .and_then(|f| Some(f.handle))
            .unwrap_or(HaFence::null_handle());
        unsafe {
            self.handle.queue_submit(queue.handle, &submit_infos, fence)
                .or(Err(LogicalDeviceError::QueueSubmitError))?;
        }

        Ok(())
    }

    pub fn wait_idle(&self) -> Result<(), LogicalDeviceError> {
        self.handle.device_wait_idle()
            .or(Err(LogicalDeviceError::WaitIdleError))
    }

    pub(crate) fn cleanup(&self) {

        unsafe {
            self.handle.destroy_device(None);
        }
    }

    fn queue_by_identifier(&self, identifier: DeviceQueueIdentifier) -> &HaQueue {
        match identifier {
            | DeviceQueueIdentifier::Graphics => &self.graphics_queue,
            | DeviceQueueIdentifier::Present  => &self.present_queue,
            | DeviceQueueIdentifier::Transfer => &self.transfer_queue,
            | DeviceQueueIdentifier::Custom(queue_index) => &self.queues[queue_index],
        }
    }
}
