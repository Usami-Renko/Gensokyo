
use ash::vk;

use image::utils::ImageCopyInfo;

pub trait ImageHandleEntity {

    fn handle(&self) -> vk::Image;
}

pub trait ImageInstance: ImageCopiable {}

pub trait ImageCopiable {

    fn copy_info(&self) -> ImageCopyInfo;
}
