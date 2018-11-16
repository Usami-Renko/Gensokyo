
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use resources::image::image::HaImage;
use resources::image::flag::ImageAspectFlag;
use resources::image::enums::{ ImageViewType, ComponentSwizzle };
use resources::error::ImageError;

use utils::types::{ vkint, vkformat };
use utils::format::VKFormat;
use utils::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

/// Wrapper class for vk::ImageView.
///
/// Images aren't directly accessed in Vulkan, but rather through views described by a subresource range.
///
/// This allows for multiple views of one image with differing ranges (e.g. for different layers).
pub struct HaImageView {

    pub(crate) handle: vk::ImageView,
}

impl HaImageView {

    pub fn config(device: &HaDevice, for_image: &HaImage, desc: &ImageViewDescInfo, format: vkformat) -> Result<HaImageView, ImageError> {

        let view_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::ImageViewCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::ImageViewCreateFlags::empty(),
            image: for_image.handle,
            view_type: desc.view_type.value(),
            format   : format.value(),
            components: vk::ComponentMapping {
                r: desc.components.0.value(),
                g: desc.components.0.value(),
                b: desc.components.0.value(),
                a: desc.components.0.value(),
            },
            subresource_range: desc.subrange.gen_subrange(),
        };

        let handle = unsafe {
            device.handle.create_image_view(&view_info, None)
                .or(Err(ImageError::ViewCreationError))?
        };

        let view = HaImageView {
            handle,
        };
        Ok(view)
    }

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_image_view(self.handle, None);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageViewDescInfo {

    /// usage specifies the type of the image view.
    pub view_type: ImageViewType,
    /// components specifies a remapping of color components (or of depth or stencil components after they have been converted into color components).
    pub components: (ComponentSwizzle, ComponentSwizzle, ComponentSwizzle, ComponentSwizzle),
    /// subrange selects the set of mipmap levels and array layers to be accessible to the view.
    pub subrange: ImageSubresourceRange,
}

impl ImageViewDescInfo {

    pub fn init(view_type: ImageViewType, aspect_mask: &[ImageAspectFlag]) -> ImageViewDescInfo {
        ImageViewDescInfo {
            view_type,
            subrange : ImageSubresourceRange(
                vk::ImageSubresourceRange {
                    // aspect_mask specifies which aspect(s) of the image are included in the view
                    aspect_mask: aspect_mask.flags(),
                    base_mip_level  : 0,
                    level_count     : 1,
                    base_array_layer: 0,
                    layer_count     : 1,
                }
            ),
            ..Default::default()
        }
    }

    pub fn reset_depth_image_aspect_mask(&mut self, format: vkformat) {

        self.subrange.0.aspect_mask = match format {
            | VKFormat::D32Sfloat => [
                ImageAspectFlag::DepthBit,
            ].flags(),
            | VKFormat::D24UnormS8Uint
            | VKFormat::D32SfloatS8Uint => [
                ImageAspectFlag::DepthBit,
                ImageAspectFlag::StencilBit,
            ].flags(),
            | _ => panic!("This format is not available for DepthStencil Image.")
        };
    }
}

impl Default for ImageViewDescInfo {

    fn default() -> ImageViewDescInfo {

        ImageViewDescInfo {
            view_type : ImageViewType::Type2d,
            components: (
                ComponentSwizzle::Identity,
                ComponentSwizzle::Identity,
                ComponentSwizzle::Identity,
                ComponentSwizzle::Identity,
            ),
            subrange: ImageSubresourceRange::default(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct ImageSubresourceRange(vk::ImageSubresourceRange);

impl Default for ImageSubresourceRange {

    fn default() -> ImageSubresourceRange {

        let sub_range = vk::ImageSubresourceRange {
            aspect_mask      : [ImageAspectFlag::ColorBit].flags(),
            base_mip_level   : 0,
            level_count      : 1,
            base_array_layer : 0,
            layer_count      : 1,
        };

        ImageSubresourceRange(sub_range)
    }
}

impl ImageSubresourceRange {

    pub fn swapchain_subrange() -> ImageSubresourceRange {
        Default::default()
    }

    pub fn set(&mut self, base_mip_level: vkint, level_count: vkint, base_array_layer: vkint, layer_count: vkint) {

        self.0.base_mip_level   = base_mip_level;
        self.0.level_count      = level_count;
        self.0.base_array_layer = base_array_layer;
        self.0.layer_count      = layer_count;
    }

    pub(super) fn gen_sublayers(&self) -> vk::ImageSubresourceLayers {

        vk::ImageSubresourceLayers {
            aspect_mask      : self.0.aspect_mask,
            mip_level        : self.0.base_mip_level,
            base_array_layer : self.0.base_array_layer,
            layer_count      : self.0.layer_count,
        }
    }

    pub(super) fn gen_subrange(&self) -> vk::ImageSubresourceRange {
        self.0.clone()
    }
}
