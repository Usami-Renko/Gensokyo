
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;
use crate::error::{ VkResult, VkError };

use std::ptr;

pub struct GsSemaphore {

    pub(crate) handle: vk::Semaphore,
    device: GsDevice,
}

impl GsSemaphore {

    pub fn create(device: &GsDevice) -> VkResult<GsSemaphore> {

        let semaphore_ci = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let handle = unsafe {
            device.logic.handle.create_semaphore(&semaphore_ci, None)
                .or(Err(VkError::create("Semaphore")))?
        };

        let semaphore = GsSemaphore {
            device: device.clone(),
            handle,
        };
        Ok(semaphore)
    }
}

impl Drop for GsSemaphore {

    fn drop(&mut self) {
        unsafe {
            self.device.logic.handle.destroy_semaphore(self.handle, None);
        }
    }
}
