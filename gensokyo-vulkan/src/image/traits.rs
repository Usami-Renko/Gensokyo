
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::utils::{ ImageFullCopyInfo, ImageCopySubrange };
use crate::image::instance::traits::ImageInstanceInfoDesc;

pub trait ImageHandleEntity: Sized {

    fn handle(&self) -> vk::Image;
}

pub trait ImageInstance<I>: ImageCopiable {

    fn build(img: I, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized;
}

pub trait ImageCopiable: Sized {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageFullCopyInfo;
}
