
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::error::{ VkError, VkResult };
use crate::types::vklint;

use std::ptr;

pub struct GsFence {

    pub(crate) handle: vk::Fence,
    device: GsDevice,
}

impl GsFence {

    pub fn create(device: &GsDevice, is_sign: bool) -> VkResult<GsFence> {

        let flags = if is_sign {
            vk::FenceCreateFlags::SIGNALED
        } else {
            vk::FenceCreateFlags::empty()
        };

        let fence_ci = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags,
        };

        let handle = unsafe {
            device.logic.handle.create_fence(&fence_ci, None)
                .map_err(|_| VkError::create("Fence"))?
        };

        let fence = GsFence {
            device: device.clone(),
            handle,
        };
        Ok(fence)
    }

    /// Tell device to wait for this fence.
    ///
    /// To wait for a group of fences, use LogicalDevice::wait_fences() method instead.
    pub fn wait(&self, timeout: vklint) -> VkResult<()> {
        self.device.logic.wait_fences(&[self], true, timeout)
    }

    /// reset a single fence.
    pub fn reset(&self) -> VkResult<()> {
        self.device.logic.reset_fences(&[self])
    }
}

impl Drop for GsFence {

    fn drop(&mut self) {
        unsafe {
            self.device.logic.handle.destroy_fence(self.handle, None);
        }
    }
}
