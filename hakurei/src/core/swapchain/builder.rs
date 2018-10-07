
use ash;
use ash::vk;
use ash::vk::{ uint32_t, uint64_t };

use config::engine::EngineConfig;
use core::instance::HaInstance;
use core::physical::HaPhyDevice;
use core::device::HaDevice;
use core::surface::HaSurface;

use core::swapchain::chain::HaSwapchain;
use core::swapchain::support::SwapchainSupport;
use core::swapchain::error::SwapchainInitError;

use resources::image::{ HaImage, HaImageView, ImageViewDescInfo, ImageAspectFlag, ImageViewType };

use utility::marker::VulkanFlags;

use std::ptr;

// FIXME: Add configuration for this flag and remove #[allow(dead_code)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SwapchainCreateFlag {
    SplitInstanceBindRegionsBit,
    ProtectedBitKHR,
}

impl VulkanFlags for [SwapchainCreateFlag] {
    type FlagType = vk::SwapchainCreateFlagsKHR;

    // TODO: These flags were introduced in Vulkan 1.1, but ash crate had not covered yet.
    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::SwapchainCreateFlagsKHR::empty(), |acc, flag| {
            match *flag {
                | SwapchainCreateFlag::SplitInstanceBindRegionsBit => acc | vk::SwapchainCreateFlagsKHR::empty(),
                | SwapchainCreateFlag::ProtectedBitKHR             => acc | vk::SwapchainCreateFlagsKHR::empty(),
            }
        })
    }
}

pub struct SwapchainBuilder<'vk, 'win: 'vk> {

    device:  HaDevice,
    surface: &'vk HaSurface<'win>,

    support: SwapchainSupport,
    image_share_info: SwapchainImageShaingInfo,
    image_count: uint32_t,
    acquire_image_time: uint64_t,
}

impl<'vk, 'win: 'vk> SwapchainBuilder<'vk, 'win> {

    pub fn init(config: &EngineConfig, physical: &HaPhyDevice, device: &HaDevice, surface: &'vk HaSurface<'win>)
        -> Result<SwapchainBuilder<'vk, 'win>, SwapchainInitError> {

        let support = SwapchainSupport::query_support(surface, physical.handle, config)
            .map_err(|e| SwapchainInitError::SurfacePropertiesQuery(e))?;

        let image_share_info = sharing_mode(device);

        let builder = SwapchainBuilder {
            device: device.clone(),
            surface,

            support,
            image_share_info,
            image_count       : config.core.swapchain.image_count,
            acquire_image_time: config.core.swapchain.acquire_image_time_out.vulkan_time(),
        };

        Ok(builder)
    }

    pub fn build(&self, instance: &HaInstance, old_chain: Option<&HaSwapchain>)
        -> Result<HaSwapchain, SwapchainInitError> {

        let prefer_format = self.support.optimal_format();
        let prefer_present_mode = self.support.optimal_present_mode();
        let prefer_extent = self.support.optimal_extent(self.surface);

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SwapchainCreateInfoKhr,
            p_next: ptr::null(),
            // TODO: Vulkan 1.1 introduced flags for SwapchainCreateInfoKHR, add flags selection in future.
            flags : vk::SwapchainCreateFlagsKHR::empty(),

            surface           : self.surface.handle,
            min_image_count   : self.image_count,
            image_format      : prefer_format.format,
            image_color_space : prefer_format.color_space,
            image_extent      : prefer_extent,
            // the number of views in a multiview/stereo surface.
            // this value must be greater than 0.
            // for non-stereoscopic-3D applications, this value is 1
            image_array_layers: 1,
            // what kind of operations we'll use the images in the swap chain for.
            image_usage: vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            // for range or image subresources accessing,
            // use exclusize mode in single queue family or concurrent mode in multiple queue families.
            image_sharing_mode       : self.image_share_info.mode,
            // only use this field in concurrent mode.
            queue_family_index_count : self.image_share_info.queue_family_indices.len() as uint32_t,
            // only use this field in concurrent mode.
            p_queue_family_indices   : self.image_share_info.queue_family_indices.as_ptr(),
            pre_transform            : self.support.current_transform(),
            // indicating the alpha usage when blending with other window system.
            composite_alpha          : vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR, // ignore the alpha value
            present_mode             : prefer_present_mode,
            // set this true to discard the pixels out of surface
            clipped                  : vk::VK_TRUE,
            // pass the old swapchain may help vulkan to reuse some resources.
            old_swapchain            : if let Some(chain) = old_chain { chain.handle } else { vk::SwapchainKHR::null() },
        };

        let loader = ash::extensions::Swapchain::new(&instance.handle, &self.device.handle)
            .or(Err(SwapchainInitError::ExtensionLoadError))?;

        let handle = unsafe {
            loader.create_swapchain_khr(&swapchain_create_info, None)
                .or(Err(SwapchainInitError::SwapchianCreationError))?
        };

        let images = loader.get_swapchain_images_khr(handle)
            .or(Err(SwapchainInitError::SwapchainImageGetError))?
            .iter().map(|&img_handle| HaImage::from_swapchain(img_handle)).collect::<Vec<_>>();

        let mut view_desc = ImageViewDescInfo::init(
            ImageViewType::Type2d,
            &[ImageAspectFlag::ColorBit],
        );

        view_desc.subrange = vk::ImageSubresourceRange {
            aspect_mask: [ImageAspectFlag::ColorBit].flags(),
            base_mip_level: 0, level_count: 1, base_array_layer: 0, layer_count: 1
        };

        let mut views = vec![];
        for image in images.iter() {
            let view = HaImageView::config(&self.device, image, &view_desc, prefer_format.format)
                .or(Err(SwapchainInitError::ImageViewCreationError))?;
            views.push(view);
        }

        let swapchain = HaSwapchain {
            handle, loader, views,
            _images: images,
            format: prefer_format.format,
            extent: prefer_extent,

            image_acquire_time: self.acquire_image_time
        };
        Ok(swapchain)
    }
}


struct SwapchainImageShaingInfo {
    mode: vk::SharingMode,
    queue_family_indices: Vec<uint32_t>,
}
fn sharing_mode(device: &HaDevice) -> SwapchainImageShaingInfo {

    if device.graphics_queue.queue.family_index == device.present_queue.queue.family_index {
        SwapchainImageShaingInfo {
            mode: vk::SharingMode::Exclusive,
            queue_family_indices: vec![],
        }
    } else {
        SwapchainImageShaingInfo {
            mode: vk::SharingMode::Concurrent,
            queue_family_indices: vec![
                device.graphics_queue.queue.family_index,
                device.present_queue.queue.family_index,
            ],
        }
    }

}
