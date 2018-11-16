
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::error::SyncError;
use utils::types::vklint;
use utils::marker::{ Handles, VulkanFlags };

use std::ptr;

pub struct HaFence {

    device: HaDevice,
    pub(crate) handle: vk::Fence,
}

impl HaFence {

    pub fn setup(device: &HaDevice, is_sign: bool) -> Result<HaFence, SyncError> {

        let flags = if is_sign {
            [&FenceCreateFlag::Signaled].flags()
        } else {
            vk::FenceCreateFlags::empty()
        };

        let create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FenceCreateInfo,
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

    /// reset a single fence
    pub fn reset(&self) -> Result<(), SyncError> {
        unsafe {
            self.device.handle.reset_fences(&[self.handle])
                .or(Err(SyncError::FenceResetError))?;
        }
        Ok(())
    }

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

impl<'re> Handles for [&'re HaFence] {
    type HandleType = vk::Fence;

    #[inline]
    fn handles(&self) -> Vec<Self::HandleType> {
        self.iter().map(|f| f.handle).collect()
    }
}
impl Handles for [HaFence] {
    type HandleType = vk::Fence;

    #[inline]
    fn handles(&self) -> Vec<Self::HandleType> {
        self.iter().map(|f| f.handle).collect()
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FenceCreateFlag {
    /// Signaled specifies that the fence object is created in the signaled state.
    ///
    /// Otherwise, it is created in the unsignaled state.
    Signaled,
}

impl<'re> VulkanFlags for [&'re FenceCreateFlag] {
    type FlagType = vk::FenceCreateFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::FenceCreateFlags::empty(), |acc, flag| {
            match flag {
                | FenceCreateFlag::Signaled => acc | vk::FENCE_CREATE_SIGNALED_BIT,
            }
        })
    }
}
