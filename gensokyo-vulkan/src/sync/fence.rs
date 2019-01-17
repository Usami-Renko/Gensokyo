
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::error::{ VkError, VkResult };
use crate::types::vklint;

use std::ptr;

pub struct GsFence {

    pub(crate) handle: vk::Fence,
    device: GsDevice,
}

impl GsFence {

    pub fn setup(device: &GsDevice, is_sign: bool) -> VkResult<GsFence> {

        let flags = if is_sign {
            vk::FenceCreateFlags::SIGNALED
        } else {
            vk::FenceCreateFlags::empty()
        };

        let create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags,
        };

        let handle = unsafe {
            device.handle.create_fence(&create_info, None)
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
        self.device.wait_fences(&[self], true, timeout)
    }

    /// reset a single fence.
    pub fn reset(&self) -> VkResult<()> {
        self.device.reset_fences(&[self])
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.handle.destroy_fence(self.handle, None);
        }
    }
}
