
use ash::vk;

use resources::image::ImageLayout;

pub struct ImageCopyInfo {

    /// `handle` is the handle of image whose data is copied from or copy to.
    pub(crate) handle: vk::Image,
    /// `layout` is the destination layout of image, if the image is as the data destination.
    ///
    /// `layout` is the current layout of image, if the image is as the data source.
    pub(crate) layout: vk::ImageLayout,
    /// `extent` is the dimension of image, if the image is as the data destination, or `extent` will be ignored.
    pub(crate) extent: vk::Extent3D,
    /// `sub_resource` is the subresources of the image used for the source or destination image data.
    pub(crate) sub_resource: vk::ImageSubresourceLayers,
}

pub(crate) struct ImageBranchInfoDesc {

    pub current_layout: ImageLayout,
    pub dimension: vk::Extent3D,
    pub sub_range: vk::ImageSubresourceRange,
}

impl ImageBranchInfoDesc {

    pub fn unset() -> ImageBranchInfoDesc {
        ImageBranchInfoDesc {
            current_layout: ImageLayout::Undefined,
            dimension: vk::Extent3D {
                width: 0, height: 0, depth: 0,
            },
            sub_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::empty(),
                base_mip_level  : 0,
                level_count     : 0,
                base_array_layer: 0,
                layer_count     : 0,
            }
        }
    }

    pub fn gen_sublayers(&self) -> vk::ImageSubresourceLayers {
        vk::ImageSubresourceLayers {
            aspect_mask     : self.sub_range.aspect_mask,
            mip_level       : self.sub_range.base_mip_level,
            base_array_layer: self.sub_range.base_array_layer,
            layer_count     : self.sub_range.layer_count,
        }
    }
}
