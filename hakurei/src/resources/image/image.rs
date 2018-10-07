
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use resources::image::flag::ImageUsageFlag;
use resources::error::ImageError;

use utility::dimension::Dimension3D;
use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

/// Images represent all kind of ‘pixel-like’ arrays.
///
/// HaImage is a wrapper class for vk::Image.
pub(crate) struct HaImage {

    pub(crate) handle: vk::Image,
    pub(crate) requirement: vk::MemoryRequirements,
}

impl HaImage {

    pub fn from_swapchain(handle: vk::Image) -> HaImage {
        HaImage {
            handle,
            requirement: vk::MemoryRequirements {
                size: 0, alignment: 0, memory_type_bits: 0
            }
        }
    }

    pub fn config(device: &HaDevice, desc: &ImageDescInfo, dimension: Dimension3D, format: vk::Format)
        -> Result<HaImage, ImageError> {

        let info = vk::ImageCreateInfo {
            s_type: vk::StructureType::ImageCreateInfo,
            p_next: ptr::null(),
            flags : desc.flags,
            format,
            extent: dimension,
            tiling: desc.tiling,
            usage : desc.usage,
            samples       : desc.sample_count,
            image_type    : desc.image_type,
            mip_levels    : desc.mip_levels,
            array_layers  : desc.array_layers,
            initial_layout: desc.initial_layout,
            sharing_mode  : desc.sharing,
            queue_family_index_count: desc.queue_family_indices.len() as uint32_t,
            p_queue_family_indices  : desc.queue_family_indices.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_image(&info, None)
                .or(Err(ImageError::ImageCreationError))?
        };

        let requirement = device.handle.get_image_memory_requirements(handle);

        let image = HaImage {
            handle,
            requirement,
        };
        Ok(image)
    }

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_image(self.handle, None);
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ImageDescInfo {

    pub flags     : vk::ImageCreateFlags,
    /// tiling specifies the tiling arrangement of the data elements in memory.
    pub tiling    : vk::ImageTiling,
    /// usage describes the intended usage of the image.
    pub usage     : vk::ImageUsageFlags,
    /// sample_count is the number of sub-data element samples in the image used in multisampling.
    pub sample_count: vk::SampleCountFlags,
    /// image_type specifies the basic dimensionality of the image.
    ///
    /// Layers in array textures do not count as a dimension for the purposes of the image type.
    pub image_type: vk::ImageType,
    /// mip_levels describes the number of levels of detail available for minified sampling of the image.
    pub mip_levels: uint32_t,
    /// array_layers is the number of layers in the image.
    pub array_layers: uint32_t,
    /// initial_layout specifies the initial vk::ImageLayout of all image subresources of the image.
    pub initial_layout: vk::ImageLayout,

    /// sharing specifies the sharing mode of the image when it will be accessed by multiple queue families.
    pub sharing: vk::SharingMode,
    /// queue_family_indices is a list of queue families that will access this image.
    ///
    /// ignored if sharingMode is not vk::SharingMode::Concurrent.
    pub queue_family_indices: Vec<uint32_t>,
}

impl ImageDescInfo {

    pub fn init(img_type: super::ImageType, tiling: super::ImageTiling, usages: &[ImageUsageFlag], initial_layout: super::ImageLayout) -> ImageDescInfo {
        ImageDescInfo {
            tiling: tiling.value(), usage: usages.flags(), image_type: img_type.value(), initial_layout: initial_layout.value(),
            ..Default::default()
        }
    }
}

impl Default for ImageDescInfo {

    fn default() -> ImageDescInfo {
        ImageDescInfo {
            flags: vk::ImageCreateFlags::empty(),
            tiling: vk::ImageTiling::Optimal,
            usage : vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            sample_count: vk::SAMPLE_COUNT_1_BIT,
            image_type  : vk::ImageType::Type2d,
            mip_levels  : 1,
            array_layers: 1,
            initial_layout: vk::ImageLayout::Undefined,

            sharing: vk::SharingMode::Exclusive,
            queue_family_indices: vec![],
        }
    }
}
