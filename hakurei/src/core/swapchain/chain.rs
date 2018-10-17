
use ash;
use ash::vk;
use ash::vk::uint32_t;

use core::device::{ HaDevice, HaGraphicsQueue, HaQueueAbstract };
use core::swapchain::error::SwapchainRuntimeError;

use resources::image::{ HaImage, HaImageView };
use utility::marker::Handles;

use sync::fence::HaFence;
use sync::semaphore::HaSemaphore;

use std::ptr;

/// Wrapper class for `vk::SwapchainKHR` object.
pub struct HaSwapchain {

    /// handle of `vk::SwapchainKHR`.
    pub(super) handle: vk::SwapchainKHR,
    /// the extension loader provides functions for creation and destruction of `vk::SwapchainKHR` object.
    pub(super) loader: ash::extensions::Swapchain,

    /// the presentable image objects associated with the swapchain.
    ///
    /// These images are created in `loader.create_swapchain_khr(..)` call.
    pub(super) _images     : Vec<HaImage>,
    /// the corresponding image views associated with the presentable images created by swapchain.
    pub(crate) views       : Vec<HaImageView>,

    /// the format of presentable images.
    pub format: vk::Format,
    /// the dimension of presentable images.
    pub extent: vk::Extent2D,

    /// the maximum duration to wait in `device.acquire_next_image_khr(..)` call, in nanoseconds.
    pub(crate) image_acquire_time: vk::uint64_t,
}

impl HaSwapchain {

    /// Acquire an available presentable image to use, and retrieve the index of that image.
    ///
    /// `sign_semaphore` is the semaphore to signal during this function, or None for no semaphore to signal.
    ///
    /// `sign_fence` is the fence to signal during this function, or None for no fence to signal.
    pub fn next_image(&self, sign_semaphore: Option<&HaSemaphore>, sign_fence: Option<&HaFence>)
        -> Result<uint32_t, SwapchainRuntimeError> {

        // the the handle of semaphore and fence
        let semaphore = sign_semaphore
            .and_then(|s| Some(s.handle))
            .unwrap_or(HaSemaphore::null_handle());
        let fence = sign_fence
            .and_then(|f| Some(f.handle))
            .unwrap_or(HaFence::null_handle());

        // execute next image acquire operation.
        let result = unsafe {
            self.loader.acquire_next_image_khr(
                self.handle,
                self.image_acquire_time,
                semaphore, fence)
        };

        // handle several specific errors.
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

    /// Queue an image for presentation.
    ///
    /// `wait_semaphores` specifies the semaphores to wait for before issuing the present request.
    ///
    /// `queue` is a queue that is capable of presentation to the target surface’s platform on the same device as the image’s swapchain.
    /// Generally it's a `vk::Queue` that is support `vk::QUEUE_GRAPHICS_BIT`.
    ///
    /// `image_index` is the index of swapchain’s presentable images.
    pub fn present(&self, wait_semaphores: &[&HaSemaphore], image_index: uint32_t, queue: &HaGraphicsQueue)
        -> Result<(), SwapchainRuntimeError> {

        let semaphores = wait_semaphores.handles();

        // Currently only support single swapchain and single image index.
        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PresentInfoKhr,
            p_next: ptr::null(),
            wait_semaphore_count: semaphores.len() as uint32_t,
            p_wait_semaphores   : semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains   : &self.handle,
            p_image_indices: &image_index,
            // VKResult of each swapchain
            p_results: ptr::null_mut(),
        };

        unsafe {
            self.loader.queue_present_khr(queue.handle(), &present_info)
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

    /// Some cleaning operations before this object was uninitialized.
    pub fn cleanup(&self, device: &HaDevice) {

        // destroy all the presentable images created by this swapchain.
        self.views.iter().for_each(|v| v.cleanup(device));

        // destroy the swapchain itself.
        unsafe {
            self.loader.destroy_swapchain_khr(self.handle, None);
        }
    }
}
