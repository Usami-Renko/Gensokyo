
use ash::vk;

use image::{ GsImage, GsImageView };
use image::traits::ImageHandleEntity;

#[derive(Debug, Clone, Default)]
pub struct ImageEntity {

    pub image: vk::Image,
    pub view : vk::ImageView,
}

impl ImageEntity {

    pub fn new(image: &GsImage, view: &GsImageView) -> ImageEntity {
        ImageEntity {
            image: image.handle,
            view : view.handle,
        }
    }
}

impl ImageHandleEntity for ImageEntity {

    fn handle(&self) -> vk::Image {
        self.image
    }
}
