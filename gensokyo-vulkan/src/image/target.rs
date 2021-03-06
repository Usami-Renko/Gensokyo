
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::image::mipmap::MipmapMethod;
use crate::image::format::GsImageFormat;
use crate::memory::MemoryDstEntity;

use crate::error::{ VkResult, VkError };
use crate::types::{ vkuint, vkbytes, vkDim3D };
use crate::types::format::Format;

use std::ptr;

/// Images represent all kind of ‘pixel-like’ arrays.
///
/// GsImage is a wrapper class for vk::Image.
pub struct GsImage {

    pub(crate) handle: vk::Image,
    requirement: vk::MemoryRequirements,
}

impl GsImage {

    #[inline(always)]
    fn build(device: &GsDevice, image_ci: vk::ImageCreateInfo) -> VkResult<GsImage> {

        let handle = unsafe {
            device.logic.handle.create_image(&image_ci, None)
                .or(Err(VkError::create("Image View")))?
        };

        let requirement = unsafe {
            device.logic.handle.get_image_memory_requirements(handle)
        };

        let image = GsImage { handle, requirement };
        Ok(image)
    }

    pub fn discard(&self, device: &GsDevice) {

        unsafe {
            device.logic.handle.destroy_image(self.handle, None);
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

impl MemoryDstEntity for GsImage {

    fn type_bytes(&self) -> vkuint {
        self.requirement.memory_type_bits
    }

    fn aligned_size(&self) -> vkbytes {

        use crate::utils::memory::bound_to_alignment;
        bound_to_alignment(self.requirement.size, self.requirement.alignment)
    }
}


#[derive(Debug, Clone)]
pub struct ImageTgtCI {

    pub property: ImagePropertyCI,
    pub specific: ImageSpecificCI,
}

impl ImageTgtCI {

    pub fn build(&self, device: &GsDevice) -> VkResult<GsImage> {

        let image_ci = vk::ImageCreateInfo {
            s_type : vk::StructureType::IMAGE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : self.property.flags,
            format : self.specific.format.clone().into(),
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

        GsImage::build(device, image_ci)
    }
}

#[derive(Debug, Clone)]
pub struct ImagePropertyCI {

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
    /// `mipmap` specifies how the program generate mipmap for the image.
    pub mipmap: MipmapMethod,
}

#[derive(Debug, Clone)]
pub struct ImageSpecificCI {

    /// `dimension` describes the number of data elements in each dimension of the base level.
    pub dimension: vkDim3D,
    /// `format` describes the format and type of the data elements that will be contained in the image.
    pub format: GsImageFormat,
    /// `sharing` specifies the sharing mode of the image when it will be accessed by multiple queue families.
    ///
    /// Default is vk::SharingMode::Exclusive.
    sharing: vk::SharingMode,
    /// `queue_family_indices` is a list of queue families that will access this image.
    ///
    /// ignored if sharingMode is not vk::SharingMode::Concurrent.
    queue_family_indices: Vec<vkuint>,
}

impl Default for ImagePropertyCI {

    fn default() -> ImagePropertyCI {

        ImagePropertyCI {
            flags : vk::ImageCreateFlags::empty(),
            tiling: vk::ImageTiling::OPTIMAL,
            usages: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            sample_count: vk::SampleCountFlags::TYPE_1,
            image_type  : vk::ImageType::TYPE_2D,
            mip_levels  : 1,
            array_layers: 1,
            initial_layout: vk::ImageLayout::UNDEFINED,
            mipmap: MipmapMethod::Disable,
        }
    }
}

impl ImageSpecificCI {

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

impl Default for ImageSpecificCI {

    fn default() -> ImageSpecificCI {

        ImageSpecificCI {
            format: GsImageFormat::Uncompressed(Format::UNDEFINED.into()),
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
