
use ash::vk;

use crate::image::utils::ImageCopyInfo;

pub trait ImageHandleEntity: Sized {

    fn handle(&self) -> vk::Image;
}

pub trait ImageInstance: ImageCopiable {}

pub trait ImageCopiable: Sized {

    fn copy_info(&self) -> ImageCopyInfo;
}
