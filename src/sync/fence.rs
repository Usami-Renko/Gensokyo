
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use sync::error::SyncError;
use utility::marker::VulkanFlags;

use structures::time::TimePeriod;
use utility::marker::Handles;

use std::ptr;

pub struct HaFence {

    pub(crate) handle: vk::Fence,
}

impl HaFence {

    pub fn setup(device: &HaLogicalDevice, sign: bool) -> Result<HaFence, SyncError> {

        let flags = if sign {
            vk::FENCE_CREATE_SIGNALED_BIT
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
            handle,
        };
        Ok(fence)
    }

    /// Tell device to wait for this fence.
    ///
    /// To wait for a group of fences, use LogicalDevice::wait_fences() method instead.
    pub fn wait(&self, device: &HaLogicalDevice, timeout: TimePeriod) -> Result<(), SyncError> {
        unsafe {
            device.handle.wait_for_fences(&[self.handle], true, timeout.vulkan_time())
                .or(Err(SyncError::FenceTimeOutError))?;
        }
        Ok(())
    }

    /// reset a single fence
    pub fn reset(&self, device: &HaLogicalDevice) -> Result<(), SyncError> {
        unsafe {
            device.handle.reset_fences(&[self.handle])
                .or(Err(SyncError::FenceResetError))?;
        }
        Ok(())
    }

    #[inline]
    pub fn null_handle() -> vk::Fence {
        vk::Fence::null()
    }

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.destroy_fence(self.handle, None);
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


#[allow(dead_code)]
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
            match *flag {
                | FenceCreateFlag::Signaled => acc | vk::FENCE_CREATE_SIGNALED_BIT,
            }
        })
    }
}
