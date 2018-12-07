
use ash::vk;

use image::utils::ImageCopyInfo;

pub trait ImageHandleEntity where Self: Sized {

    fn handle(&self) -> vk::Image;
}

pub trait ImageInstance: ImageCopiable {}

pub trait ImageCopiable where Self: Sized {

    fn copy_info(&self) -> ImageCopyInfo;
}
