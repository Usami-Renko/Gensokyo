
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::device::SharingMode;

use pipeline::state::multisample::SampleCountType;
use resources::image::enums::{ ImageType, ImageTiling, ImageLayout };
use resources::image::flag::{ ImageCreateFlag, ImageUsageFlag };
use resources::image::traits::ImageHandleEntity;
use resources::memory::MemoryDstEntity;
use resources::error::ImageError;

use utils::types::{ vkint, vkformat, vkMemorySize, vkDimension3D };
use utils::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

/// Images represent all kind of ‘pixel-like’ arrays.
///
/// HaImage is a wrapper class for vk::Image.
pub struct HaImage {

    pub(crate) handle: vk::Image,
    requirement: vk::MemoryRequirements,
}

impl HaImage {

    pub(crate) fn from_swapchain(handle: vk::Image) -> HaImage {

        HaImage {
            handle,
            requirement: vk::MemoryRequirements {
                size: 0, alignment: 0, memory_type_bits: 0
            }
        }
    }

    pub(crate) fn new(device: &HaDevice, handle: vk::Image) -> HaImage {

        let requirement = device.handle.get_image_memory_requirements(handle);

        HaImage {
            handle, requirement,
        }
    }

    pub fn config(device: &HaDevice, desc: &ImageDescInfo, dimension: vkDimension3D, format: vkformat)
        -> Result<HaImage, ImageError> {

        let info = vk::ImageCreateInfo {
            s_type: vk::StructureType::ImageCreateInfo,
            p_next: ptr::null(),
            flags : desc.flags.flags(),
            format: format.value(),
            extent: dimension,
            tiling: desc.tiling.value(),
            usage : desc.usages.flags(),
            samples       : desc.sample_count.value(),
            image_type    : desc.image_type.value(),
            mip_levels    : desc.mip_levels,
            array_layers  : desc.array_layers,
            initial_layout: desc.initial_layout.value(),
            sharing_mode  : desc.sharing.value(),
            queue_family_index_count: desc.queue_family_indices.len() as vkint,
            p_queue_family_indices  : desc.queue_family_indices.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_image(&info, None)
                .or(Err(ImageError::ImageCreationError))?
        };

        let requirement = device.handle.get_image_memory_requirements(handle);

        let image = HaImage {
            handle, requirement,
        };

        Ok(image)
    }

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_image(self.handle, None);
        }
    }
}

impl ImageHandleEntity for HaImage {

    fn handle(&self) -> vk::Image {
        self.handle
    }
}

impl MemoryDstEntity for HaImage {

    fn type_bytes(&self) -> vkint {
        self.requirement.memory_type_bits
    }

    fn aligment_size(&self) -> vkMemorySize {

        use utils::memory::bind_to_alignment;
        bind_to_alignment(self.requirement.size, self.requirement.alignment)
    }
}

#[derive(Debug, Clone)]
pub struct ImageDescInfo {

    pub flags: Vec<ImageCreateFlag>,
    /// tiling specifies the tiling arrangement of the data elements in memory.
    pub tiling: ImageTiling,
    /// usage describes the intended usage of the image.
    pub usages: Vec<ImageUsageFlag>,
    /// sample_count is the number of sub-data element samples in the image used in multisampling.
    pub sample_count: SampleCountType,
    /// image_type specifies the basic dimensionality of the image.
    ///
    /// Layers in array textures do not count as a dimension for the purposes of the image type.
    pub image_type: ImageType,
    /// mip_levels describes the number of levels of detail available for minified sampling of the image.
    pub mip_levels: vkint,
    /// array_layers is the number of layers in the image.
    pub array_layers: vkint,
    /// initial_layout specifies the initial vk::ImageLayout of all image subresources of the image.
    pub initial_layout: ImageLayout,

    /// sharing specifies the sharing mode of the image when it will be accessed by multiple queue families.
    pub sharing: SharingMode,
    /// queue_family_indices is a list of queue families that will access this image.
    ///
    /// ignored if sharingMode is not vk::SharingMode::Concurrent.
    pub queue_family_indices: Vec<vkint>,
}

impl ImageDescInfo {

    pub fn init(img_type: ImageType, tiling: ImageTiling, usages: &[ImageUsageFlag], initial_layout: ImageLayout) -> ImageDescInfo {
        ImageDescInfo {
            tiling, usages: usages.to_vec(), image_type: img_type, initial_layout,
            ..Default::default()
        }
    }
}

impl Default for ImageDescInfo {

    fn default() -> ImageDescInfo {
        ImageDescInfo {
            flags : vec![],
            tiling: ImageTiling::Optimal,
            usages: [ImageUsageFlag::ColorAttachmentBit].to_vec(),
            sample_count: SampleCountType::Count1Bit,
            image_type  : ImageType::Type2d,
            mip_levels  : 1,
            array_layers: 1,
            initial_layout: ImageLayout::Undefined,

            sharing: SharingMode::Exclusive,
            queue_family_indices: vec![],
        }
    }
}
