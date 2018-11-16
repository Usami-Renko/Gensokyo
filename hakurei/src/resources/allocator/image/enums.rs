
use vk::core::device::HaDevice;

use vk::resources::memory::HaMemoryType;
use vk::resources::image::{ HaImage, ImageDescInfo };
use vk::resources::image::{ ImageViewDescInfo, ImageSubresourceRange };
use vk::resources::image::ImageLayout;
use vk::resources::image::{ ImageBlockEntity, ImageCopiable, ImageCopyInfo };
use vk::resources::memory::MemoryDstEntity;
use vk::utils::types::vkMemorySize;

use resources::image::{ ImageBranchInfoDesc, ImageBranchType };
use resources::image::io::ImageStorageInfo;

pub struct ImageAllocateInfo {

    pub typ: ImageBranchType,

    pub image: HaImage,
    pub image_desc: ImageDescInfo,
    pub view_desc : ImageViewDescInfo,

    pub storage: ImageStorageInfo,
    pub space  : vkMemorySize,

    pub final_layout: ImageLayout,
}

impl ImageAllocateInfo {

    pub fn new(typ: ImageBranchType, storage: ImageStorageInfo, image: HaImage, image_desc: ImageDescInfo, view_desc: ImageViewDescInfo) -> ImageAllocateInfo {

        let space = image.aligment_size();

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

        self.image.cleanup(device);
    }
}

impl ImageBlockEntity for ImageAllocateInfo {

}

impl ImageCopiable for ImageAllocateInfo {

    fn copy_info(&self) -> ImageCopyInfo {

        // The layout paramater is the destination layout after data copy.
        // This value should be vk::TransferDstOptimal.
        ImageCopyInfo::new(&self.image, &self.view_desc.subrange, self.final_layout, self.storage.dimension)
    }
}
