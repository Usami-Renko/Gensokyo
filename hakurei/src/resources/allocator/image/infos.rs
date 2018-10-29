
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::image::HaImage;
use resources::image::{ ImageDescInfo, ImageViewDescInfo, ImageStorageInfo };
use resources::image::ImageLayout;
use resources::image::ImageBlockEntity;
use resources::image::{ ImageCopiable, ImageCopyInfo };
use resources::image::{ ImageBranchType, ImageBranchInfoDesc };

use utility::marker::VulkanEnum;

pub(crate) struct ImageAllocateInfo {

    pub(super) typ: ImageBranchType,

    pub(crate) image: HaImage,
    pub(crate) image_desc: ImageDescInfo,
    pub(crate) view_desc : ImageViewDescInfo,

    pub(crate) storage: ImageStorageInfo,
    pub(crate) space  : vk::DeviceSize,

    pub(crate) final_layout: ImageLayout,
}

impl ImageAllocateInfo {

    pub fn new(typ: ImageBranchType, storage: ImageStorageInfo, image: HaImage, image_desc: ImageDescInfo, view_desc: ImageViewDescInfo) -> ImageAllocateInfo {

        use utility::memory::bind_to_alignment;
        let space = bind_to_alignment(image.requirement.size, image.requirement.alignment);

        ImageAllocateInfo {
            typ, image, image_desc, view_desc, storage, space,
            final_layout: ImageLayout::Undefined,
        }
    }

    pub fn gen_desc(&self) -> ImageBranchInfoDesc {
        ImageBranchInfoDesc {
            current_layout: self.final_layout,
            dimension: self.storage.dimension,
            sub_range: self.view_desc.subrange.clone(),
        }
    }

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_image(self.image.handle, None);
        }
    }
}

impl ImageBlockEntity for ImageAllocateInfo {

}

impl ImageCopiable for ImageAllocateInfo {

    fn copy_info(&self) -> ImageCopyInfo {
        ImageCopyInfo {
            handle: self.image.handle,
            // the destination layout after data copy.
            // This value should be vk::TransferDstOptimal.
            layout: self.final_layout.value(),
            extent: self.storage.dimension,
            sub_resource: vk::ImageSubresourceLayers {
                aspect_mask     : self.view_desc.subrange.aspect_mask,
                mip_level       : self.view_desc.subrange.base_mip_level,
                base_array_layer: self.view_desc.subrange.base_array_layer,
                layer_count     : self.view_desc.subrange.layer_count,
            }
        }
    }
}
