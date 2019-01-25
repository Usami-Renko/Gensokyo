
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::image::traits::ImageHandleEntity;
use crate::memory::MemoryDstEntity;

use crate::error::{ VkResult, VkError };
use crate::types::{ vkuint, vkbytes, vkDim3D };
use crate::types::format::GsFormat;

use std::ptr;

/// Images represent all kind of ‘pixel-like’ arrays.
///
/// GsImage is a wrapper class for vk::Image.
pub struct GsImage {

    pub(crate) handle: vk::Image,
    requirement: vk::MemoryRequirements,
}

impl GsImage {

    fn new(device: &GsDevice, handle: vk::Image) -> GsImage {

        let requirement = unsafe {
            device.handle.get_image_memory_requirements(handle)
        };

        GsImage {
            handle, requirement,
        }
    }

    pub fn destroy(&self, device: &GsDevice) {

        unsafe {
            device.handle.destroy_image(self.handle, None);
        }
    }
}

impl From<vk::Image> for GsImage {

    fn from(handle: vk::Image) -> GsImage {
        GsImage {
            handle,
            requirement: vk::MemoryRequirements {
                size: 0,
                alignment: 0,
                memory_type_bits: 0,
            }
        }
    }
}

impl ImageHandleEntity for GsImage {

    fn handle(&self) -> vk::Image {
        self.handle
    }
}

impl MemoryDstEntity for GsImage {

    fn type_bytes(&self) -> vkuint {
        self.requirement.memory_type_bits
    }

    fn alignment_size(&self) -> vkbytes {

        use crate::utils::memory::bound_to_alignment;
        bound_to_alignment(self.requirement.size, self.requirement.alignment)
    }
}


#[derive(Debug, Clone)]
pub struct ImageDescInfo {

    pub property: ImagePropertyInfo,
    pub specific: ImageSpecificInfo,
}

impl ImageDescInfo {

    pub fn build(&self, device: &GsDevice) -> VkResult<GsImage> {

        let image_ci = vk::ImageCreateInfo {
            s_type : vk::StructureType::IMAGE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : self.property.flags,
            format : self.specific.format.0,
            extent : self.specific.dimension,
            tiling : self.property.tiling,
            usage  : self.property.usages,
            samples        : self.property.sample_count,
            image_type     : self.property.image_type,
            mip_levels     : self.property.mip_levels,
            array_layers   : self.property.array_layers,
            initial_layout : self.property.initial_layout,
            sharing_mode   : self.specific.sharing,
            queue_family_index_count: self.specific.queue_family_indices.len() as _,
            p_queue_family_indices  : self.specific.queue_family_indices.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_image(&image_ci, None)
                .or(Err(VkError::create("Image View")))?
        };

        let image = GsImage::new(device, handle);
        Ok(image)
    }
}

#[derive(Debug, Clone)]
pub struct ImagePropertyInfo {

    /// `flags` describing additional parameters of the image.
    pub flags: vk::ImageCreateFlags,
    /// `tiling` specifies the tiling arrangement of the data elements in memory.
    pub tiling: vk::ImageTiling,
    /// `usages` describes the intended usage of the image.
    pub usages: vk::ImageUsageFlags,
    /// `sample_count` is the number of sub-data element samples in the image used in multisampling.
    pub sample_count: vk::SampleCountFlags,
    /// `image_type` specifies the basic dimensionality of the image.
    ///
    /// `Layers` in array textures do not count as a dimension for the purposes of the image type.
    pub image_type: vk::ImageType,
    /// `mip_levels` describes the number of levels of detail available for minified sampling of the image.
    pub mip_levels: vkuint,
    /// `array_layers` is the number of layers in the image.
    pub array_layers: vkuint,
    /// `initial_layout` specifies the initial vk::ImageLayout of all image subresources of the image.
    pub initial_layout: vk::ImageLayout,
}

#[derive(Debug, Clone)]
pub struct ImageSpecificInfo {

    /// `dimension` describes the number of data elements in each dimension of the base level.
    pub dimension: vkDim3D,
    /// `format` describes the format and type of the data elements that will be contained in the image.
    pub format: GsFormat,
    /// `sharing` specifies the sharing mode of the image when it will be accessed by multiple queue families.
    ///
    /// Default is vk::SharingMode::Exclusive.
    sharing: vk::SharingMode,
    /// `queue_family_indices` is a list of queue families that will access this image.
    ///
    /// ignored if sharingMode is not vk::SharingMode::Concurrent.
    queue_family_indices: Vec<vkuint>,
}

impl Default for ImagePropertyInfo {

    fn default() -> ImagePropertyInfo {

        ImagePropertyInfo {
            flags : vk::ImageCreateFlags::empty(),
            tiling: vk::ImageTiling::OPTIMAL,
            usages: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            sample_count: vk::SampleCountFlags::TYPE_1,
            image_type  : vk::ImageType::TYPE_2D,
            mip_levels  : 1,
            array_layers: 1,
            initial_layout: vk::ImageLayout::UNDEFINED,
        }
    }
}

impl ImageSpecificInfo {

    pub fn share_queue_families(&mut self, family_indices: Option<Vec<vkuint>>) {

        if let Some(family_indices) = family_indices {
            self.sharing = vk::SharingMode::CONCURRENT;
            self.queue_family_indices = family_indices;
        } else {
            self.sharing = vk::SharingMode::EXCLUSIVE;
            self.queue_family_indices.clear();
        }
    }
}

impl Default for ImageSpecificInfo {

    fn default() -> ImageSpecificInfo {

        ImageSpecificInfo {
            format: GsFormat::UNDEFINED,
            dimension: vkDim3D {
                width : 0,
                height: 0,
                depth : 0,
            },
            sharing: vk::SharingMode::EXCLUSIVE,
            queue_family_indices: vec![],
        }
    }
}
