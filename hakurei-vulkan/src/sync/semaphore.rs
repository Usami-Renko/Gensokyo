
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use sync::error::SyncError;

use std::ptr;

pub struct HaSemaphore {

    pub(crate) handle: vk::Semaphore,
    device: HaDevice,
}

impl HaSemaphore {

    pub fn setup(device: &HaDevice) -> Result<HaSemaphore, SyncError> {

        let create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
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

    pub fn cleanup(&self) {
        unsafe {
            self.device.handle.destroy_semaphore(self.handle, None);
        }
    }
}
