
use ash::vk;

pub struct ImageViewItem {

    pub(crate) image_handle: vk::Image,
    pub(crate) view_handle: vk::ImageView,
    pub(crate) view_index: usize,
}

