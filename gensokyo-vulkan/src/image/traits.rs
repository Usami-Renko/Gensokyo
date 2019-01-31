
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::utils::{ ImageCopyInfo, ImageCopySubrange };
use crate::image::instance::desc::ImageInstanceInfoDesc;

pub trait ImageHandleEntity: Sized {

    fn handle(&self) -> vk::Image;
}

pub trait ImageInstance<I>: ImageCopiable {

    fn build(img: I, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized;
}

pub trait ImageCopiable: Sized {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageCopyInfo;
}
