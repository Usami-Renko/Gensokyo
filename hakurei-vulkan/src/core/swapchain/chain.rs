
use ash;
use ash::vk;

use core::device::HaDevice;
use core::device::queue::{ HaGraphicsQueue, HaQueueAbstract };
use core::swapchain::enums::{ ColorSpace, PresentMode };
use core::swapchain::error::SwapchainRuntimeError;

use resources::image::{ HaImage, HaImageView };
use resources::sync::{ HaFence, HaSemaphore };

use utils::marker::Handles;
use utils::types::{ vkint, vklint, vkformat, vkDimension2D };

use std::ptr;

/// Wrapper class for `vk::SwapchainKHR` object.
pub struct HaSwapchain {

    /// handle of `vk::SwapchainKHR`.
    handle: vk::SwapchainKHR,
    /// the extension loader provides functions for creation and destruction of `vk::SwapchainKHR` object.
    loader: ash::extensions::Swapchain,

    /// the presentable image objects associated with the swapchain.
    ///
    /// These images are created in `loader.create_swapchain_khr(..)` call.
    _images: Vec<HaImage>,
    /// the corresponding image views associated with the presentable images created by swapchain.
    views  : Vec<HaImageView>,

    /// the format of presentable images.
    format: vkformat,
    /// the dimension of presentable images.
    extent: vkDimension2D,

    /// the maximum duration to wait in `device.acquire_next_image_khr(..)` call, in nanoseconds.
    image_acquire_time: vklint,
}

impl HaSwapchain {

    pub(crate) fn new(handle: vk::SwapchainKHR, loader: ash::extensions::Swapchain, images: Vec<HaImage>, views: Vec<HaImageView>, format: vkformat, extent: vkDimension2D, image_acquire_time: vklint) -> HaSwapchain {

        HaSwapchain {
            handle, loader, _images: images, views, format, extent, image_acquire_time
        }
    }

    /// Acquire an available presentable image to use, and retrieve the index of that image.
    ///
    /// `sign_semaphore` is the semaphore to signal during this function, or None for no semaphore to signal.
    ///
    /// `sign_fence` is the fence to signal during this function, or None for no fence to signal.
    pub fn next_image(&self, sign_semaphore: Option<&HaSemaphore>, sign_fence: Option<&HaFence>) -> Result<vkint, SwapchainRuntimeError> {

        // the the handle of semaphore and fence
        let semaphore = sign_semaphore.and_then(|s| Some(s.handle))
            .unwrap_or(HaSemaphore::null_handle());
        let fence = sign_fence.and_then(|f| Some(f.handle))
            .unwrap_or(HaFence::null_handle());

        // execute next image acquire operation.
        let result = unsafe {
            self.loader.acquire_next_image_khr(self.handle, self.image_acquire_time, semaphore, fence)
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
    pub fn present(&self, wait_semaphores: &[&HaSemaphore], image_index: vkint, queue: &HaGraphicsQueue)
        -> Result<(), SwapchainRuntimeError> {

        let semaphores = wait_semaphores.handles();

        // Currently only support single swapchain and single image index.
        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PresentInfoKhr,
            p_next: ptr::null(),
            wait_semaphore_count: semaphores.len() as vkint,
            p_wait_semaphores   : semaphores.as_ptr(),
            swapchain_count     : 1,
            p_swapchains        : &self.handle,
            p_image_indices     : &image_index,
            // VKResult of each swapchain
            p_results: ptr::null_mut(),
        };

        unsafe {
            self.loader.queue_present_khr(queue.queue().handle, &present_info)
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

    // TODO: Remove the following function.
    pub fn extent(&self) -> vkDimension2D {
        self.extent
    }
    // TODO: Remove the following function.
    pub(crate) fn views(&self) -> &Vec<HaImageView> {
        &self.views
    }
    // TODO: Remove the following function.
    pub(crate) fn handle(&self) -> vk::SwapchainKHR {
        self.handle
    }
}

#[derive(Debug, Clone)]
pub struct SwapchainConfig {

    pub image_count: vkint,
    /// the value of layers property in vk::Framebuffer.
    pub framebuffer_layers: vkint,

    pub prefer_surface_format      : vkformat,
    pub prefer_surface_color_space : ColorSpace,

    pub prefer_primary_present_mode   : PresentMode,
    pub prefer_secondary_present_mode : PresentMode,

    pub acquire_image_time_out: vklint,
}
