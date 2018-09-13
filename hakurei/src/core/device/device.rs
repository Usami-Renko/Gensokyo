
use ash;
use ash::vk;
use ash::vk::uint32_t;
use ash::version::{ V1_0, DeviceV1_0 };

use core::device::queue::QueueSubmitBundle;
use core::device::queue::{ HaQueueAbstract, HaGraphicsQueue, HaPresentQueue, HaTransferQueue, HaTransfer };
use core::device::queue::{ HaQueue, QueueContainer };
use core::error::LogicalDeviceError;

use resources::error::CommandError;

use sync::fence::HaFence;
use sync::error::SyncError;
use utility::time::TimePeriod;
use utility::marker::Handles;

use std::ptr;

pub struct HaLogicalDevice {

    pub(crate) handle: ash::Device<V1_0>,
    pub(super) queue_container: QueueContainer,

    pub(crate) graphics_queue: HaGraphicsQueue,
    pub(crate) present_queue : HaPresentQueue,
    pub(super) transfer_queue: HaTransferQueue,
}

pub enum DeviceQueueIdentifier {
    Graphics,
    Present,
    Transfer,
    Custom(Box<DeviceQueueIdentifier>, usize),
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

    pub fn transfer(&self) -> HaTransfer {

        self.transfer_queue.transfer(&self)
    }

    pub fn submit(&self, bundles: &[QueueSubmitBundle], fence: Option<&HaFence>, queue_ident: DeviceQueueIdentifier)
        -> Result<(), CommandError> {

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
        self.handle.device_wait_idle()
            .or(Err(LogicalDeviceError::WaitIdleError))
    }

    pub(crate) fn cleanup(&self) {

        unsafe {
            self.transfer_queue.clean(self);
            self.handle.destroy_device(None);
        }
    }

    pub(crate) fn queue_handle_by_identifier(&self, identifier: DeviceQueueIdentifier) -> &HaQueue {
        match identifier {
            | DeviceQueueIdentifier::Graphics => &self.graphics_queue.queue,
            | DeviceQueueIdentifier::Present  => &self.present_queue.queue,
            | DeviceQueueIdentifier::Transfer => &self.transfer_queue.queue,
            | DeviceQueueIdentifier::Custom(ident, queue_index) => {
                self.queue_container.queue(*ident, queue_index)
            },
        }
    }
}
