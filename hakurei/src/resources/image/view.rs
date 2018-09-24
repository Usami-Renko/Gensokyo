
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use resources::image::{ HaImage, ImageAspectFlag };
use resources::error::ImageError;

use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

pub(crate) struct HaImageView {

    pub handle: vk::ImageView,
}

impl HaImageView {

    pub fn config(device: &HaDevice, image: &HaImage, desc: &ImageViewDescInfo, format: vk::Format) -> Result<HaImageView, ImageError> {

        let view_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::ImageViewCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::ImageViewCreateFlags::empty(),
            image : image.handle,
            view_type : desc.view_type,
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

pub struct ImageViewDescInfo {

    // TODO: Make these fields private.
    /// usage specifies the type of the image view.
    pub(super) view_type : vk::ImageViewType,
    /// components specifies a remapping of color components (or of depth or stencil components after they have been converted into color components).
    pub(super) components: vk::ComponentMapping,
    /// subrange selects the set of mipmap levels and array layers to be accessible to the view.
    pub(crate) subrange: vk::ImageSubresourceRange,
}

impl ImageViewDescInfo {

    pub fn init(view_type: super::ImageViewType) -> ImageViewDescInfo {
        ImageViewDescInfo {
            view_type: view_type.value(),
            ..Default::default()
        }
    }

    pub fn set_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle) {
        self.components = vk::ComponentMapping { r, g, b, a };
    }

    /// Select the set of mipmap levels and array layers to be accessible to the view.
    ///
    /// aspect_mask specifies which aspect(s) of the image are included in the view.
    ///
    /// base_mip_level is the first mipmap level accessible to the view.
    ///
    /// level_count is the number of mipmap levels (starting from base_mip_level) accessible to the view.
    ///
    /// base_array_layer is the first array layer accessible to the view.
    ///
    /// layer_count is the number of array layers (starting from baseArrayLayer) accessible to the view.
    pub fn set_subrange(&mut self, aspects: &[ImageAspectFlag], base_mip_level: uint32_t, level_count: uint32_t, base_array_layer: uint32_t, layer_count: uint32_t) {
        self.subrange = vk::ImageSubresourceRange {
            aspect_mask: aspects.flags(),
            base_mip_level, level_count, base_array_layer, layer_count
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
