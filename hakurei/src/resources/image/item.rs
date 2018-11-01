
use ash::vk;

#[derive(Debug, Clone)]
pub(crate) struct ImageViewItem {

    pub(crate) image_handle: vk::Image,
    pub(crate) view_handle : vk::ImageView,
}

impl ImageViewItem {

    pub fn unset() -> ImageViewItem {
        ImageViewItem {
            image_handle: vk::Image::null(),
            view_handle : vk::ImageView::null(),
        }
    }

    pub fn new(image: vk::Image, view: vk::ImageView) -> ImageViewItem {
        ImageViewItem {
            image_handle: image,
            view_handle : view,
        }
    }
}
