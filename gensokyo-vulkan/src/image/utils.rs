
use ash::vk;

use crate::image::view::ImageSubRange;
use crate::types::{ vkuint, vkDim3D };

pub struct ImageCopyInfo {

    /// `handle` is the handle of image whose data is copied from or copy to.
    pub(crate) handle: vk::Image,
    /// `layout` is the destination layout of image, if the image is as the data destination.
    ///
    /// `layout` is the current layout of image, if the image is as the data source.
    pub(crate) layout: vk::ImageLayout,
    /// `extent` is the dimension of image, if the image is as the data destination, or `extent` will be ignored.
    pub(crate) extent: vkDim3D,
    /// `sub_resource` is the subresources of the image used for the source or destination image data.
    pub(crate) sub_resource_layers: ImageCopySubrange,
}

pub struct ImageCopySubrange(pub(crate) vk::ImageSubresourceLayers);

impl ImageCopySubrange {

    // indicate to copy the base mip level.
    pub fn base_copy(r#for: &ImageSubRange) -> ImageCopySubrange {

        let value = vk::ImageSubresourceLayers {
            aspect_mask      : r#for.0.aspect_mask,
            mip_level        : 0, // the base mip-level is 0.
            base_array_layer : 0, // TODO: array level is not cover yet.
            layer_count      : 1,
        };
        ImageCopySubrange(value)
    }

    // indicate to copy mipmap at level `mipmap_level`.
    pub fn copy(r#for: &ImageSubRange, mipmap_level: vkuint) -> ImageCopySubrange {

        let value = vk::ImageSubresourceLayers {
            aspect_mask      : r#for.0.aspect_mask,
            mip_level        : mipmap_level,
            base_array_layer : 0, // TODO: array level is not cover yet.
            layer_count      : 1,
        };
        ImageCopySubrange(value)
    }
}
