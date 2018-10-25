
use ash::vk;

#[derive(Debug, Clone)]
pub struct ImageViewItem {

    pub(crate) handles   : Option<ImageObjHandles>,
    pub(crate) view_index: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct ImageObjHandles {

    image: vk::Image,
    view : vk::ImageView,
}

impl ImageViewItem {

    pub fn from_unallocate(index: usize) -> ImageViewItem {
        ImageViewItem {
            handles: None,
            view_index: index,
        }
    }

    pub fn set_handles(&mut self, image: vk::Image, view: vk::ImageView) {
        let handles = ImageObjHandles {
            image, view,
        };
        self.handles = Some(handles)
    }

    pub fn get_view_handle(&self) -> Option<vk::ImageView> {
        self.handles.as_ref()
            .and_then(|handles| Some(handles.view.clone()))
    }
}

pub trait HaImageBranchAbs {

    fn view_index(&self) -> usize;
    fn fill_handles(&mut self, image: vk::Image, view: vk::ImageView);
}
