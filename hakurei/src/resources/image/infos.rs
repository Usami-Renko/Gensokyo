
use vk::core::device::HaDevice;

use vk::resources::image::{ HaImage, ImageDescInfo };
use vk::resources::image::{ ImageViewDescInfo, ImageSubresourceRange };
use vk::resources::image::ImageLayout;
use vk::resources::image::{ ImageBlockEntity, ImageCopiable, ImageCopyInfo };

use resources::image::io::ImageStorageInfo;
use resources::image::enums::ImageBranchType;

use vk::utils::types::{ vkDimension3D, vkMemorySize };

pub struct ImageBranchInfoDesc {

    pub current_layout: ImageLayout,
    pub dimension: vkDimension3D,
    pub sub_range: ImageSubresourceRange,
}

impl ImageBranchInfoDesc {

    pub fn unset() -> ImageBranchInfoDesc {

        ImageBranchInfoDesc {
            current_layout: ImageLayout::Undefined,
            dimension: vkDimension3D {
                width: 0, height: 0, depth: 0,
            },
            sub_range: ImageSubresourceRange::default(),
        }
    }
}
