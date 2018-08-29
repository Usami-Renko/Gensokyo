
use ash;
use ash::vk;
use ash::vk::uint32_t;

use core::device::HaLogicalDevice;
use core::device::HaQueue;

use swapchain::error::SwapchainRuntimeError;

use resources::image::{ HaImage, HaImageView };
use resources::framebuffer::HaFramebuffer;
use constant::swapchain::ACQUIRE_IMAGE_TIME_OUT;
use utility::marker::Handles;

use sync::fence::HaFence;
use sync::semaphore::HaSemaphore;

use std::ptr;

pub struct HaSwapchain {

    pub(super) handle: vk::SwapchainKHR,
    pub(super) loader: ash::extensions::Swapchain,

    pub(super) _images     : Vec<HaImage>,
    pub(super) views       : Vec<HaImageView>,
    pub(crate) framebuffers: Vec<HaFramebuffer>,

    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

impl HaSwapchain {

    pub fn next_image(&self, sign_semaphore: Option<&HaSemaphore>, sign_fence: Option<&HaFence>)
        -> Result<uint32_t, SwapchainRuntimeError> {

        let semaphore = sign_semaphore
            .and_then(|s| Some(s.handle))
            .unwrap_or(HaSemaphore::null_handle());
        let fence = sign_fence
            .and_then(|f| Some(f.handle))
            .unwrap_or(HaFence::null_handle());

        let result = unsafe {
            self.loader.acquire_next_image_khr(
                self.handle,
                ACQUIRE_IMAGE_TIME_OUT.vulkan_time(),
                semaphore, fence)
        };

        match result {
            | Ok(image_index) => Ok(image_index),
            | Err(vk_result) => {
                let err = match vk_result {
                    | vk::Result::ErrorSurfaceLostKhr    => SwapchainRuntimeError::SurfaceUnAvailableError,
                    | vk::Result::NotReady               => SwapchainRuntimeError::ImageNotReadyError,
                    | vk::Result::Timeout                => SwapchainRuntimeError::AcquireTimeOut,
                    | vk::Result::SuboptimalKhr          => SwapchainRuntimeError::SurfaceSubOptimalError,
                    | vk::Result::ErrorOutOfDateKhr      => SwapchainRuntimeError::SurfaceOutOfDateError,
                    | vk::Result::ErrorDeviceLost        => SwapchainRuntimeError::DeviceUnAvailableError,
                    | vk::Result::ErrorOutOfHostMemory   => SwapchainRuntimeError::OutOfHostMemory,
                    | vk::Result::ErrorOutOfDeviceMemory => SwapchainRuntimeError::OutOfDeviceMemory,
                    | _ => SwapchainRuntimeError::Unkndow,
                };

                Err(err)
            }
        }
    }

    pub fn present(&self, wait_semaphores: &[&HaSemaphore], image_index: uint32_t, queue: &HaQueue)
        -> Result<(), SwapchainRuntimeError> {

        let semaphores = wait_semaphores.handles();

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PresentInfoKhr,
            p_next: ptr::null(),
            /// wait_semaphores specifies the semaphores to wait for before issuing the present request.
            wait_semaphore_count: semaphores.len() as uint32_t,
            p_wait_semaphores   : semaphores.as_ptr(),
            // Currently just support a single swapchain.
            swapchain_count: 1,
            p_swapchains   : [self.handle].as_ptr(),
            p_image_indices: &image_index,
            // VKResult of each swapchain
            p_results: ptr::null_mut(),
        };

        unsafe {
            self.loader.queue_present_khr(queue.handle, &present_info)
                .map_err(|err| {
                    match err {
                        | vk::Result::SuboptimalKhr          => SwapchainRuntimeError::SurfaceSubOptimalError,
                        | vk::Result::ErrorOutOfDateKhr      => SwapchainRuntimeError::SurfaceOutOfDateError,
                        | vk::Result::ErrorSurfaceLostKhr    => SwapchainRuntimeError::SurfaceUnAvailableError,
                        | vk::Result::ErrorOutOfHostMemory   => SwapchainRuntimeError::OutOfHostMemory,
                        | vk::Result::ErrorOutOfDeviceMemory => SwapchainRuntimeError::OutOfDeviceMemory,
                        | _ => SwapchainRuntimeError::Unkndow,
                    }
                })?;
        }

        Ok(())
    }

    pub fn recreate(&mut self) {
        // TODO: Need Implementation
        unimplemented!()
    }

    pub fn cleanup(&self, device: &HaLogicalDevice) {

        self.framebuffers.iter().for_each(|f| f.cleanup(device));
        self.views.iter().for_each(|v| v.cleanup(device));

        unsafe {
            self.loader.destroy_swapchain_khr(self.handle, None);
        }
    }
}


