
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use sync::error::SyncError;
use types::vklint;

use std::ptr;

pub struct HaFence {

    pub(crate) handle: vk::Fence,
    device: HaDevice,
}

impl HaFence {

    pub fn setup(device: &HaDevice, is_sign: bool) -> Result<HaFence, SyncError> {

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
                .or(Err(SyncError::FenceCreationError))?
        };

        let fence = HaFence {
            device: device.clone(),
            handle,
        };
        Ok(fence)
    }

    /// Tell device to wait for this fence.
    ///
    /// To wait for a group of fences, use LogicalDevice::wait_fences() method instead.
    pub fn wait(&self, timeout: vklint) -> Result<(), SyncError> {
        unsafe {
            self.device.handle.wait_for_fences(&[self.handle], true, timeout)
                .or(Err(SyncError::FenceTimeOutError))?;
        }
        Ok(())
    }

    /// reset a single fence.
    pub fn reset(&self) -> Result<(), SyncError> {
        unsafe {
            self.device.handle.reset_fences(&[self.handle])
                .or(Err(SyncError::FenceResetError))?;
        }
        Ok(())
    }

    // TODO: Remove this function.
    #[inline]
    pub fn null_handle() -> vk::Fence {
        vk::Fence::null()
    }

    pub fn cleanup(&self) {
        unsafe {
            self.device.handle.destroy_fence(self.handle, None);
        }
    }
}
