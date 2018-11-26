
use ash::vk;

use image::traits::ImageHandleEntity;
use types::vkDim3D;

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
    pub(crate) sub_resource_layers: vk::ImageSubresourceLayers,
}

impl ImageCopyInfo {

    pub fn new(image: &impl ImageHandleEntity, subrange_layers: vk::ImageSubresourceLayers, layout: vk::ImageLayout, dimension: vkDim3D) -> ImageCopyInfo {

        ImageCopyInfo {
            handle: image.handle(),
            layout,
            extent: dimension,
            sub_resource_layers: subrange_layers,
        }
    }
}

pub fn image_subrange_to_layers(subrange: &vk::ImageSubresourceRange) -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers {
        aspect_mask      : subrange.aspect_mask,
        mip_level        : subrange.base_mip_level,
        base_array_layer : subrange.base_array_layer,
        layer_count      : subrange.layer_count,
    }
}
