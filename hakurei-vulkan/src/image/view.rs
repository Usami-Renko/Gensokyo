
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use image::target::{ HaImage, ImageSpecificInfo };
use image::error::ImageError;

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

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_image_view(self.handle, None);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageViewDescInfo {

    /// `view_type` specifies the type of the image view.
    pub view_type: vk::ImageViewType,
    /// `components` specifies a remapping of color components (or of depth or stencil components after they have been converted into color components).
    pub components: vk::ComponentMapping,
    /// `subrange` selects the set of mipmap levels and array layers to be accessible to the view.
    pub subrange: vk::ImageSubresourceRange,
}

impl ImageViewDescInfo {

    pub fn new(view_type: vk::ImageViewType, aspect_mask: vk::ImageAspectFlags) -> ImageViewDescInfo {

        ImageViewDescInfo {
            view_type,
            subrange: vk::ImageSubresourceRange {
                // aspect_mask specifies which aspect(s) of the image are included in the view
                aspect_mask,
                base_mip_level  : 0,
                level_count     : 1,
                base_array_layer: 0,
                layer_count     : 1,
            },
            ..Default::default()
        }
    }

    pub fn build(&self, device: &HaDevice, image: &HaImage, specific: &ImageSpecificInfo) -> Result<HaImageView, ImageError> {

        let view_info = vk::ImageViewCreateInfo {
            s_type     : vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next     : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags      : vk::ImageViewCreateFlags::empty(),
            image      : image.handle,
            view_type  : self.view_type,
            format     : specific.format,
            components : self.components,
            subresource_range : self.subrange,
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

    pub(crate) fn build_for_swapchain(&self, device: &HaDevice, image: &HaImage, format: vk::Format) -> Result<HaImageView, ImageError> {

        let mut specific = ImageSpecificInfo::default();
        specific.format = format;
        self.build(device, image, &specific)
    }
}

impl Default for ImageViewDescInfo {

    fn default() -> ImageViewDescInfo {

        ImageViewDescInfo {
            view_type : vk::ImageViewType::TYPE_2D,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::IDENTITY,
                g: vk::ComponentSwizzle::IDENTITY,
                b: vk::ComponentSwizzle::IDENTITY,
                a: vk::ComponentSwizzle::IDENTITY,
            },
            subrange: vk::ImageSubresourceRange {
                aspect_mask      : vk::ImageAspectFlags::COLOR,
                base_mip_level   : 0,
                level_count      : 1,
                base_array_layer : 0,
                layer_count      : 1,
            },
        }
    }
}
