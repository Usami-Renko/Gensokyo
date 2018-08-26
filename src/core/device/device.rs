
use ash;
use ash::vk;
use ash::vk::uint32_t;
use ash::version::V1_0;
use ash::version::DeviceV1_0;

use core::device::queue::QueueInfo;
use core::device::queue::QueueSubmitBundle;
use core::error::LogicalDeviceError;

use sync::HaFence;
use sync::error::SyncError;
use utility::time::TimePeriod;
use utility::marker::Handles;

use std::ptr;

pub struct HaLogicalDevice {

    pub handle: ash::Device<V1_0>,
    queues: Vec<QueueInfo>,

    pub graphics_queue_index: Option<usize>,
    pub present_queue_index:  Option<usize>,
}

impl<'resource> HaLogicalDevice {

    pub fn new(handle: ash::Device<V1_0>, queues: Vec<QueueInfo>, graphics_queue_index: Option<usize>, present_queue_index:  Option<usize>) -> HaLogicalDevice {
        HaLogicalDevice {
            handle,
            queues,
            graphics_queue_index,
            present_queue_index,
        }
    }

    pub fn graphics_queue(&self) -> Option<&QueueInfo> {
        Some(&self.queues[self.graphics_queue_index?])
    }

    pub fn present_queue(&self) -> Option<&QueueInfo> {
        Some(&self.queues[self.present_queue_index?])
    }

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

    pub fn submit(&self, bundles: &[QueueSubmitBundle], fence: Option<&HaFence>)
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

        let graphics_queue = self.graphics_queue()
            .ok_or(LogicalDeviceError::GraphicsQueueUnavailable)?
            .handle;
        let fence = fence
            .and_then(|f| Some(f.handle))
            .unwrap_or(HaFence::null_handle());
        unsafe {
            self.handle.queue_submit(graphics_queue, &submit_infos, fence)
                .or(Err(LogicalDeviceError::QueueSubmitError))?;
        }

        Ok(())
    }

    pub fn wait_idle(&self) -> Result<(), LogicalDeviceError> {
        self.handle.device_wait_idle()
            .or(Err(LogicalDeviceError::WaitIdleError))
    }

    pub fn cleanup(&self) {

        unsafe {
            self.handle.destroy_device(None);
        }
    }
}