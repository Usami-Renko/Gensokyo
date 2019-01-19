
use ash::vk;

use gsma::collect_handle;

use crate::core::device::{ GsDevice, DeviceQueueIdentifier };
use crate::error::{ VkResult, VkError };

use crate::image::{ GsImage, GsImageView };
use crate::sync::{ GsFence, GsSemaphore };

use crate::types::{ vkuint, vklint, vkDim2D };
use crate::types::format::GsFormat;

use std::ptr;

/// Wrapper class for `vk::SwapchainKHR` object.
pub struct GsSwapchain {

    /// handle of `vk::SwapchainKHR`.
    pub(crate) handle: vk::SwapchainKHR,
    /// the extension loader provides functions for creation and destruction of `vk::SwapchainKHR` object.
    loader: ash::extensions::khr::Swapchain,

    /// the presentable image objects associated with the swapchain.
    ///
    /// These images are created in `loader.create_swapchain_khr(..)` call and are destroyed automatically when `vk::SwapchainKHR` is destroyed.
    #[allow(dead_code)]
    images: Vec<GsImage>,
    /// the corresponding image views associated with the presentable images created by swapchain.
    views: Vec<GsImageView>,
    /// the format of presentable images.
    format: GsFormat,
    /// the dimension of presentable images.
    extent: vkDim2D,

    /// the maximum duration to wait in `device.acquire_next_image_khr(..)` call, in nanoseconds.
    image_acquire_time: vklint,
}

impl GsSwapchain {

    pub(crate) fn new(handle: vk::SwapchainKHR, loader: ash::extensions::khr::Swapchain, images: Vec<GsImage>, views: Vec<GsImageView>, format: GsFormat, extent: vkDim2D, image_acquire_time: vklint) -> GsSwapchain {

        GsSwapchain {
            handle, loader, images, views, format, extent, image_acquire_time
        }
    }

    /// Acquire an available presentable image to use, and retrieve the index of that image.
    ///
    /// `sign_semaphore` is the semaphore to signal during this function, or None for no semaphore to signal.
    ///
    /// `sign_fence` is the fence to signal during this function, or None for no fence to signal.
    pub fn next_image(&self, sign_semaphore: Option<&GsSemaphore>, sign_fence: Option<&GsFence>) -> VkResult<vkuint> {

        // the the handle of semaphore and fence.
        let semaphore = sign_semaphore.and_then(|s| Some(s.handle))
            .unwrap_or(vk::Semaphore::null());
        let fence = sign_fence.and_then(|f| Some(f.handle))
            .unwrap_or(vk::Fence::null());

        // execute next image acquire operation.
        let (image_index, is_sub_optimal) = unsafe {
            self.loader.acquire_next_image(self.handle, self.image_acquire_time, semaphore, fence)
                .map_err(|error| match error {
                    | vk::Result::TIMEOUT               => VkError::swapchain_sync(SwapchainSyncError::TimeOut),
                    | vk::Result::ERROR_OUT_OF_DATE_KHR => VkError::swapchain_sync(SwapchainSyncError::SurfaceOutDate),
                    | _ => VkError::swapchain_sync(SwapchainSyncError::Unknown),
                })?
        };

        if is_sub_optimal {
            Err(VkError::swapchain_sync(SwapchainSyncError::SubOptimal))
        } else {
            Ok(image_index)
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
    pub fn present(&self, device: &GsDevice, wait_semaphores: &[&GsSemaphore], image_index: vkuint, queue: DeviceQueueIdentifier)
        -> VkResult<()> {

        let semaphores: Vec<vk::Semaphore> = collect_handle!(wait_semaphores);

        // Currently only support single swapchain and single image index.
        let present_info = vk::PresentInfoKHR {
            s_type              : vk::StructureType::PRESENT_INFO_KHR,
            p_next              : ptr::null(),
            wait_semaphore_count: semaphores.len() as _,
            p_wait_semaphores   : semaphores.as_ptr(),
            swapchain_count     : 1,
            p_swapchains        : &self.handle,
            p_image_indices     : &image_index,
            // VKResult of each swapchain.
            p_results           : ptr::null_mut(),
        };

        let is_sub_optimal = unsafe {
            self.loader.queue_present(device.queue_handle_by_identifier(queue).handle, &present_info)
                .or(Err(VkError::device("Get unknown error when acquiring image.")))?
        };

        if is_sub_optimal {
            Err(VkError::swapchain_sync(SwapchainSyncError::SubOptimal))
        } else {
            Ok(())
        }
    }

    /// Destroy the `vk::SwapchainKHR` object.
    ///
    /// The application must not destroy `vk::SwapchainKHR` until after completion of all outstanding operations on images that were acquired from the `vk::SwapchainKHR`.
    pub fn destroy(&self, device: &GsDevice) {

        // destroy all the presentable images created by this swapchain.
        self.views.iter()
            .for_each(|v| v.destroy(device));

        // destroy the swapchain itself.
        unsafe {
            self.loader.destroy_swapchain(self.handle, None);
        }
    }

    // TODO: Remove the following function.
    pub fn extent(&self) -> vkDim2D {
        self.extent
    }
    pub fn format(&self) -> GsFormat {
        self.format
    }
    // TODO: Remove the following function.
    pub(crate) fn views(&self) -> &Vec<GsImageView> {
        &self.views
    }
}

#[derive(Debug, Clone)]
pub struct SwapchainConfig {

    pub image_count: vkuint,
    /// the value of layers property in vk::Framebuffer.
    pub framebuffer_layers: vkuint,

    pub prefer_surface_format      : GsFormat,
    pub prefer_surface_color_space : vk::ColorSpaceKHR,

    pub prefer_primary_present_mode   : vk::PresentModeKHR,
    pub prefer_secondary_present_mode : vk::PresentModeKHR,

    pub acquire_image_time_out: vklint,
}


#[derive(Debug, Fail)]
pub enum SwapchainSyncError {
    #[fail(display = "No image became available within the time allowed.")]
    TimeOut,
    #[fail(display = "Swapchain does not match the surface properties exactly.")]
    SubOptimal,
    #[fail(display = "Surface has changed and is not compatible with the swapchain.")]
    SurfaceOutDate,
    #[fail(display = "Get unknown error when acquiring image.")]
    Unknown,
}
