
use winit;
use ash::vk;

use crate::core::instance::GsInstance;
use crate::core::physical::GsPhyDevice;
use crate::core::device::{ GsDevice, DeviceQueueIdentifier };
use crate::core::surface::GsSurface;

use crate::core::swapchain::GsChain;
use crate::core::swapchain::chain::{ GsSwapchain, SwapchainConfig };
use crate::core::swapchain::support::SwapchainSupport;
use crate::core::swapchain::error::SwapchainInitError;

use crate::image::{ GsImage, ImageViewDescInfo };

use crate::types::{ vkuint, vklint, VK_TRUE };

use std::ptr;

pub struct SwapchainBuilder<'s> {

    device : GsDevice,
    surface: &'s GsSurface,

    support: SwapchainSupport,
    image_share_info: SwapchainImageSharingInfo,
    image_count: vkuint,
    acquire_image_time: vklint,
}

impl<'s> SwapchainBuilder<'s> {

    pub fn init(config: &SwapchainConfig, physical: &GsPhyDevice, device: &GsDevice, surface: &'s GsSurface)
        -> Result<SwapchainBuilder<'s>, SwapchainInitError> {

        let support = SwapchainSupport::query_support(surface, physical.handle, config)
            .map_err(|e| SwapchainInitError::SurfacePropertiesQuery(e))?;

        let image_share_info = sharing_mode(device);

        let builder = SwapchainBuilder {
            device: device.clone(),
            surface,

            support,
            image_share_info,
            image_count       : config.image_count,
            acquire_image_time: config.acquire_image_time_out,
        };

        Ok(builder)
    }

    pub fn build(self, instance: &GsInstance, old_chain: Option<&GsChain>, window: &winit::Window)
        -> Result<GsSwapchain, SwapchainInitError> {

        let prefer_format = self.support.optimal_format();
        let prefer_extent = self.support.optimal_extent(window)?;

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type                   : vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next                   : ptr::null(),
            // TODO: Add configuration for SwapchainCreateFlagsKHR.
            flags                    : vk::SwapchainCreateFlagsKHR::empty(),
            surface                  : self.surface.handle,
            min_image_count          : self.image_count,
            image_format             : prefer_format.format,
            image_color_space        : prefer_format.color_space,
            image_extent             : prefer_extent,
            // the number of views in a multiview/stereo surface.
            // this value must be greater than 0.
            // for non-stereoscopic-3D applications, this value is 1.
            image_array_layers       : 1,
            // what kind of operations we'll use the images in the swap chain for.
            image_usage              : vk::ImageUsageFlags::COLOR_ATTACHMENT,
            // for range or image subresources accessing,
            // use exclusive mode in single queue family or concurrent mode in multiple queue families.
            image_sharing_mode       : self.image_share_info.mode,
            // only use this field in concurrent mode.
            queue_family_index_count : self.image_share_info.queue_family_indices.len() as _,
            // only use this field in concurrent mode.
            p_queue_family_indices   : self.image_share_info.queue_family_indices.as_ptr(),
            pre_transform            : self.support.current_transform(),
            // indicating the alpha usage when blending with other window system.
            composite_alpha          : vk::CompositeAlphaFlagsKHR::OPAQUE, // ignore the alpha value
            present_mode             : self.support.optimal_present_mode(),
            // set this true to discard the pixels out of surface
            clipped                  : VK_TRUE,
            // pass the old swapchain may help vulkan to reuse some resources.
            old_swapchain: old_chain.and_then(|c| Some(c.handle)).unwrap_or(vk::SwapchainKHR::null()),
        };

        let loader = ash::extensions::khr::Swapchain::new(&instance.handle, &self.device.handle);

        let handle = unsafe {
            loader.create_swapchain(&swapchain_create_info, None)
                .or(Err(SwapchainInitError::SwapchianCreationError))?
        };

        let images: Vec<GsImage> = unsafe {
            loader.get_swapchain_images(handle)
                .or(Err(SwapchainInitError::SwapchainImageGetError))?
                .iter().map(|&img_handle| GsImage::from_swapchain(img_handle))
                .collect()
        };

        let view_desc = ImageViewDescInfo::new(
            vk::ImageViewType::TYPE_2D,
            vk::ImageAspectFlags::COLOR,
        );

        let mut views = vec![];
        for image in images.iter() {
            let view = view_desc.build_for_swapchain(&self.device, image, prefer_format.format)
                .or(Err(SwapchainInitError::ImageViewCreationError))?;
            views.push(view);
        }

        let swapchain = GsSwapchain::new(handle, loader, images, views, prefer_format.format, prefer_extent, self.acquire_image_time);
        Ok(swapchain)
    }
}


struct SwapchainImageSharingInfo {

    mode: vk::SharingMode,
    queue_family_indices: Vec<vkuint>,
}

fn sharing_mode(device: &GsDevice) -> SwapchainImageSharingInfo {

    let graphics_queue = device.queue_handle_by_identifier(DeviceQueueIdentifier::Graphics);
    let present_queue = device.queue_handle_by_identifier(DeviceQueueIdentifier::Present);

    if graphics_queue.family_index == present_queue.family_index {
        SwapchainImageSharingInfo {
            mode: vk::SharingMode::EXCLUSIVE,
            queue_family_indices: vec![],
        }
    } else {
        SwapchainImageSharingInfo {
            mode: vk::SharingMode::CONCURRENT,
            queue_family_indices: vec![
                graphics_queue.family_index,
                present_queue.family_index,
            ],
        }
    }
}
