
use ash::vk;

use core::device::HaDevice;

use types::{ vkbytes, vkDim3D };

use memory::{ HaMemoryType, MemorySelector, HaMemoryAbstract };
use memory::MemoryError;

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

pub trait ImageHandleEntity {

    fn handle(&self) -> vk::Image;
}

pub trait ImageInstance: ImageCopiable {}

pub trait ImageCopiable {

    fn copy_info(&self) -> ImageCopyInfo;
}

pub trait ImageMemoryTypeAbs {

    fn memory_type(&self) -> HaMemoryType;
    fn allot_memory(&self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<Box<dyn HaMemoryAbstract>, MemoryError>;
}
