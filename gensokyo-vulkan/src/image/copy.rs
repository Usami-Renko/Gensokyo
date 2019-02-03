
use ash::vk;

use crate::types::vkDim3D;

pub struct ImageFullCopyInfo {

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

pub struct ImageRangesCopyInfo {

    /// `handle` is the handle of image whose data is copied from or copy to.
    pub(crate) handle: vk::Image,
    /// `layout` is the destination layout of image, if the image is as the data destination.
    ///
    /// `layout` is the current layout of image, if the image is as the data source.
    pub(crate) layout: vk::ImageLayout,
    /// `ranges` specified the sub ranges of image to be copied.
    pub(crate) ranges: Vec<ImageCopyRange>,
}

pub struct ImageCopyRange {

    /// `extent` is the dimension of image, if the image is as the data destination, or `extent` will be ignored.
    pub(crate) extent: vkDim3D,
    /// `sub_resource` is the subresources of the image used for the source or destination image data.
    pub(crate) sub_resource_layers: vk::ImageSubresourceLayers,
}
