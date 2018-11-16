
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::error::SyncError;
use utils::marker::Handles;

use std::ptr;

pub struct HaSemaphore {

    device: HaDevice,
    pub(crate) handle: vk::Semaphore,
}

impl HaSemaphore {

    pub fn setup(device: &HaDevice) -> Result<HaSemaphore, SyncError> {

        let create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SemaphoreCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let handle = unsafe {
            device.handle.create_semaphore(&create_info, None)
                .or(Err(SyncError::SemaphoreCreationError))?
        };

        let semaphore = HaSemaphore {
            device: device.clone(),
            handle,
        };
        Ok(semaphore)
    }

    #[inline]
    pub fn null_handle() -> vk::Semaphore {
        vk::Semaphore::null()
    }

    pub fn cleanup(&self) {
        unsafe {
            self.device.handle.destroy_semaphore(self.handle, None);
        }
    }
}

impl<'re> Handles for [&'re HaSemaphore] {
    type HandleType = vk::Semaphore;

    #[inline]
    fn handles(&self) -> Vec<Self::HandleType> {
        self.iter().map(|s| s.handle).collect()
    }
}
