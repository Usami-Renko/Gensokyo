
use ash::vk;

use resources::image::ImageLayout;
use resources::image::view::ImageSubresourceRange;

use utils::marker::VulkanEnum;
use utils::types::vkDimension3D;

pub struct ImageCopyInfo {

    /// `handle` is the handle of image whose data is copied from or copy to.
    pub(crate) handle: vk::Image,
    /// `layout` is the destination layout of image, if the image is as the data destination.
    ///
    /// `layout` is the current layout of image, if the image is as the data source.
    pub(crate) layout: vk::ImageLayout,
    /// `extent` is the dimension of image, if the image is as the data destination, or `extent` will be ignored.
    pub(crate) extent: vkDimension3D,
    /// `sub_resource` is the subresources of the image used for the source or destination image data.
    pub(crate) sub_resource: vk::ImageSubresourceLayers,
}

impl ImageCopyInfo {

    pub fn new(image: &impl ImageHandleEntity, subrange: &ImageSubresourceRange, layout: ImageLayout, dimension: vkDimension3D) -> ImageCopyInfo {

        ImageCopyInfo {
            handle: image.handle(),
            layout: layout.value(),
            extent: dimension,
            sub_resource: subrange.gen_sublayers(),
        }
    }
}

pub trait ImageHandleEntity {

    fn handle(&self) -> vk::Image;
}

pub trait ImageBlockEntity: ImageCopiable {

}

pub trait ImageCopiable {

    fn copy_info(&self) -> ImageCopyInfo;
}
