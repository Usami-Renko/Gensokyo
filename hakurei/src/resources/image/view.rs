
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use resources::image::{ HaImage, ImageAspectFlag };
use resources::error::ImageError;

use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

/// Wrapper class for vk::ImageView.
///
/// Images aren't directly accessed in Vulkan, but rather through views described by a subresource range.
///
/// This allows for multiple views of one image with differing ranges (e.g. for different layers).
pub(crate) struct HaImageView {

    pub handle: vk::ImageView,
}

impl HaImageView {

    pub fn config(device: &HaDevice, for_image: &HaImage, desc: &ImageViewDescInfo, format: vk::Format) -> Result<HaImageView, ImageError> {

        let view_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::ImageViewCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::ImageViewCreateFlags::empty(),
            image: for_image.handle,
            view_type: desc.view_type,
            format,
            components: desc.components,
            subresource_range: desc.subrange.clone(),
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
pub(crate) struct ImageViewDescInfo {

    // TODO: Make these fields private.
    /// usage specifies the type of the image view.
    pub view_type : vk::ImageViewType,
    /// components specifies a remapping of color components (or of depth or stencil components after they have been converted into color components).
    pub components: vk::ComponentMapping,
    /// subrange selects the set of mipmap levels and array layers to be accessible to the view.
    pub subrange  : vk::ImageSubresourceRange,
}

impl ImageViewDescInfo {

    pub fn init(view_type: super::ImageViewType, aspect_mask: &[ImageAspectFlag]) -> ImageViewDescInfo {
        ImageViewDescInfo {
            view_type: view_type.value(),
            subrange : vk::ImageSubresourceRange {
                // aspect_mask specifies which aspect(s) of the image are included in the view.
                aspect_mask: aspect_mask.flags(),
                base_mip_level  : 0,
                level_count     : 1,
                base_array_layer: 0,
                layer_count     : 1,
            },
            ..Default::default()
        }
    }

    pub fn reset_depth_image_aspect_mask(&mut self, format: vk::Format) {

        self.subrange.aspect_mask = match format {
            | vk::Format::D32Sfloat => [
                ImageAspectFlag::DepthBit,
            ].flags(),
            | vk::Format::D24UnormS8Uint
            | vk::Format::D32SfloatS8Uint => [
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
            view_type : vk::ImageViewType::Type2d,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::Identity,
                g: vk::ComponentSwizzle::Identity,
                b: vk::ComponentSwizzle::Identity,
                a: vk::ComponentSwizzle::Identity,
            },
            subrange: vk::ImageSubresourceRange {
                aspect_mask     : [ImageAspectFlag::ColorBit].flags(),
                base_mip_level  : 0,
                level_count     : 1,
                base_array_layer: 0,
                layer_count     : 1,
            }
        }
    }
}
