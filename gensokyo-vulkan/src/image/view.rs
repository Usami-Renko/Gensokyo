
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;
use crate::image::target::{ GsImage, ImageSpecificCI };
use crate::error::{ VkResult, VkError };
use crate::types::format::GsFormat;
use crate::types::vkuint;

use std::ptr;

/// Wrapper class for vk::ImageView.
///
/// Images aren't directly accessed in Vulkan, but rather through views described by a subresource range.
///
/// This allows for multiple views of one image with differing ranges (e.g. for different layers).
pub struct GsImageView {

    pub(crate) handle: vk::ImageView,
}

impl GsImageView {

    pub fn destroy(&self, device: &GsDevice) {
        unsafe {
            device.logic.handle.destroy_image_view(self.handle, None);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageViewCI {

    /// `view_type` specifies the type of the image view.
    pub view_type: vk::ImageViewType,
    /// `components` specifies a remapping of color components (or of depth or stencil components after they have been converted into color components).
    pub components: vk::ComponentMapping,
    /// `subrange` selects the set of mipmap levels and array layers to be accessible to the view.
    pub subrange: ImageSubRange,
}

impl ImageViewCI {

    pub fn new(view_type: vk::ImageViewType, aspect_mask: vk::ImageAspectFlags) -> ImageViewCI {

        ImageViewCI {
            view_type,
            subrange: ImageSubRange(vk::ImageSubresourceRange {
                aspect_mask, // aspect_mask specifies which aspect(s) of the image are included in the view
                base_mip_level  : 0,
                level_count     : 1,
                base_array_layer: 0,
                layer_count     : 1,
            }),
            ..Default::default()
        }
    }

    pub fn with_subrange(mut self, value: ImageSubRange) -> ImageViewCI {
        self.subrange = value;
        self
    }

    pub fn build(&self, device: &GsDevice, image: &GsImage, specific: &ImageSpecificCI) -> VkResult<GsImageView> {

        let image_view_ci = vk::ImageViewCreateInfo {
            s_type     : vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next     : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags      : vk::ImageViewCreateFlags::empty(),
            image      : image.handle,
            view_type  : self.view_type,
            format     : specific.format.0,
            components : self.components,
            subresource_range : self.subrange.0,
        };

        let handle = unsafe {
            device.logic.handle.create_image_view(&image_view_ci, None)
                .or(Err(VkError::create("Image View")))?
        };

        let view = GsImageView { handle };
        Ok(view)
    }

    pub(crate) fn build_for_swapchain(&self, device: &GsDevice, image: &GsImage, format: GsFormat) -> VkResult<GsImageView> {

        let mut specific = ImageSpecificCI::default();
        specific.format = format;
        self.build(device, image, &specific)
    }
}

impl Default for ImageViewCI {

    fn default() -> ImageViewCI {

        ImageViewCI {
            view_type : vk::ImageViewType::TYPE_2D,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::IDENTITY,
                g: vk::ComponentSwizzle::IDENTITY,
                b: vk::ComponentSwizzle::IDENTITY,
                a: vk::ComponentSwizzle::IDENTITY,
            },
            subrange: ImageSubRange::default(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct ImageSubRange(pub(super) vk::ImageSubresourceRange);

impl Default for ImageSubRange {

    fn default() -> ImageSubRange {

        let subrange = vk::ImageSubresourceRange {
            aspect_mask      : vk::ImageAspectFlags::COLOR,
            base_mip_level   : 0,
            level_count      : 1,
            base_array_layer : 0,
            layer_count      : 1,
        };
        ImageSubRange(subrange)
    }
}

impl ImageSubRange {

    pub fn new() -> ImageSubRange {
        ImageSubRange::default()
    }

    pub fn with_aspect_mask(mut self, value: vk::ImageAspectFlags) -> ImageSubRange {
        self.0.aspect_mask = value;
        self
    }

    pub fn with_layer(mut self, base_array_layer: vkuint, layer_count: vkuint) -> ImageSubRange {
        self.0.base_array_layer = base_array_layer;
        self.0.layer_count = layer_count;
        self
    }

    pub fn with_mip_level(mut self, base_mip_level: vkuint, level_count: vkuint) -> ImageSubRange {
        self.0.base_mip_level = base_mip_level;
        self.0.level_count = level_count;
        self
    }
}
