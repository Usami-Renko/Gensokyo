
use ash;
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::instance::Instance;
use core::physical::PhysicalDevice;
use core::device::LogicalDevice;
use core::surface::Surface;

use swapchain::chain::Swapchain;
use swapchain::support::SwapchainSupport;
use swapchain::error::SwapchainInitError;

use constant::swapchain::SWAPCHAIN_IMAGE_COUNT;
use utility::marker::VulkanFlags;

use std::ptr;

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

    instance: &'vk Instance,
    device:   &'vk LogicalDevice,
    surface:  &'vk Surface<'win>,

    support: SwapchainSupport,
    image_share_info: SwapchainImageShaingInfo,
    image_count: uint32_t,
}

impl<'builder, 'vk: 'builder, 'win: 'vk> SwapchainBuilder<'vk, 'win> {

    pub fn init(instance: &'vk Instance, physical: &PhysicalDevice, device: &'vk LogicalDevice, surface: &'vk Surface<'win>)
        -> Result<SwapchainBuilder<'vk, 'win>, SwapchainInitError> {

        let support = SwapchainSupport::query_support(surface, physical.handle)
            .or_else(|error| {
                println!("[Error] {}", error.to_string());
                Err(SwapchainInitError::SurfacePropertiesQueryError)
            }
        )?;

        let image_share_info = sharing_mode(device)?;

        let swapchain = SwapchainBuilder {
            instance,
            surface,
            device,

            support,
            image_share_info,
            image_count: SWAPCHAIN_IMAGE_COUNT,
        };

        Ok(swapchain)
    }

    pub fn set_image_count(&'builder mut self, image_count: uint32_t) -> &'builder mut SwapchainBuilder<'vk, 'win> {
        self.image_count = image_count;
        self
    }

    pub fn build(&self) -> Result<Swapchain, SwapchainInitError> {

        let prefer_format = self.support.optimal_format();
        let prefer_present_mode = self.support.optimal_present_mode();
        let extent = self.support.extent(self.surface);

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SwapchainCreateInfoKhr,
            p_next: ptr::null(),
            // TODO: Vulkan 1.1 introduced flags for SwapchainCreateInfoKHR, add flags selection in future.
            flags   : vk::SwapchainCreateFlagsKHR::empty(),

            surface           : self.surface.handle,
            min_image_count   : self.image_count,
            image_format      : prefer_format.format,
            image_color_space : prefer_format.color_space,
            image_extent      : extent,
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
            // because this is the first creation of swapchain, no need to set this field
            old_swapchain            : vk::SwapchainKHR::null(),
        };

        let loader = ash::extensions::Swapchain::new(&self.instance.handle, &self.device.handle)
            .or(Err(SwapchainInitError::ExtensionLoadError))?;

        let handle = unsafe {
            loader.create_swapchain_khr(&swapchain_create_info, None)
                .or(Err(SwapchainInitError::SwapchianCreationError))?
        };

        let images = loader.get_swapchain_images_khr(handle)
            .or(Err(SwapchainInitError::SwapchainImageGetError))?;

        let views = generate_imageviews(self.device, prefer_format.format, &images)?;

        let swapchain = Swapchain::new(handle, loader, images, views, prefer_format.format, extent);
        Ok(swapchain)
    }
}


struct SwapchainImageShaingInfo {
    mode: vk::SharingMode,
    queue_family_indices: Vec<uint32_t>,
}
fn sharing_mode(device: &LogicalDevice) -> Result<SwapchainImageShaingInfo, SwapchainInitError> {

    let graphics_queue = device.graphics_queue()
        .ok_or(SwapchainInitError::GraphicsQueueNotAvailable)?;
    let present_queue  = device.present_queue()
        .ok_or(SwapchainInitError::PresentQueueNotAvailable)?;

    let share_info = if graphics_queue.family_index == present_queue.family_index {
        SwapchainImageShaingInfo {
            mode: vk::SharingMode::Exclusive,
            queue_family_indices: vec![],
        }
    } else {
        SwapchainImageShaingInfo {
            mode: vk::SharingMode::Concurrent,
            queue_family_indices: vec![
                graphics_queue.family_index,
                present_queue.family_index,
            ],
        }
    };

    Ok(share_info)
}

fn generate_imageviews(device: &LogicalDevice, format: vk::Format, images: &Vec<vk::Image>)
    -> Result<Vec<vk::ImageView>, SwapchainInitError> {

    let mut imageviews = vec![];

    // TODO: Wrap this logical in future image section.
    for &image in images.iter() {

        let imageview_create_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::ImageViewCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.0.82
            flags: vk::ImageViewCreateFlags::empty(),
            image,
            view_type: vk::ImageViewType::Type2d,
            format,
            // specifies a remapping of color components
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::Identity,
                g: vk::ComponentSwizzle::Identity,
                b: vk::ComponentSwizzle::Identity,
                a: vk::ComponentSwizzle::Identity,
            },
            // selecting the set of mipmap levels and array layers to be accessible to the view
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask      : vk::IMAGE_ASPECT_COLOR_BIT,
                base_mip_level   : 0,
                level_count      : 1,
                base_array_layer : 0,
                layer_count      : 1,
            }
        };

        let imageview = unsafe {
            device.handle.create_image_view(&imageview_create_info, None)
                .or(Err(SwapchainInitError::ImageViewCreationError))?
        };

        imageviews.push(imageview);
    }

    Ok(imageviews)
}
