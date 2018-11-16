
use ash::vk;

use resources::image::{ HaImage, HaImageView };
use resources::image::traits::ImageHandleEntity;

#[derive(Debug, Clone)]
pub struct ImageViewItem {

    pub image_handle: vk::Image,
    pub view_handle : vk::ImageView,
}

impl ImageViewItem {

    pub fn unset() -> ImageViewItem {
        ImageViewItem {
            image_handle: vk::Image::null(),
            view_handle : vk::ImageView::null(),
        }
    }

    pub fn new(image: &HaImage, view: &HaImageView) -> ImageViewItem {
        ImageViewItem {
            image_handle: image.handle,
            view_handle : view.handle,
        }
    }
}

impl ImageHandleEntity for ImageViewItem {

    fn handle(&self) -> vk::Image {
        self.image_handle
    }
}
