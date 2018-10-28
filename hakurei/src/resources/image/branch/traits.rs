
use ash::vk;
use ash::vk::uint32_t;

use resources::allocator::ImageAllocateInfo;
use resources::image::{ ImageTiling, ImageLayout };
use resources::repository::DataCopyer;
use resources::error::AllocatorError;

use pipeline::state::SampleCountType;

pub(crate) trait HaImageDescAbs {

    // image property.
    fn set_tiling(&mut self, tiling: ImageTiling);
    fn set_initial_layout(&mut self, layout: ImageLayout);
    fn set_samples(&mut self, count: SampleCountType, mip_levels: uint32_t, array_layers: uint32_t);
    fn set_share_queues(&mut self, queue_family_indices: Vec<uint32_t>);
}

pub(crate) trait HaImageViewDescAbs {

    // image view property.
    fn set_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle);

    /// Select the set of mipmap levels and array layers to be accessible to the view.
    ///
    /// base_mip_level is the first mipmap level accessible to the view.
    ///
    /// level_count is the number of mipmap levels (starting from base_mip_level) accessible to the view.
    ///
    /// base_array_layer is the first array layer accessible to the view.
    ///
    /// layer_count is the number of array layers (starting from baseArrayLayer) accessible to the view.
    fn set_subrange(&mut self, base_mip_level: uint32_t, level_count: uint32_t, base_array_layer: uint32_t, layer_count: uint32_t);
}

/// Image Barrier Bundle Abstract.
pub(crate) trait ImageBarrierBundleAbs {

    fn make_transfermation(&mut self, copyer: &DataCopyer, infos: &Vec<ImageAllocateInfo>) -> Result<(), AllocatorError>;
    fn cleanup(&mut self);
}

pub trait ImageBlockEntity: ImageCopiable {

}

pub trait ImageCopiable {

    fn copy_info(&self) -> ImageCopyInfo;
}

pub struct ImageCopyInfo {

    /// `handle` is the handle of image whose data is copied from or copy to.
    pub(crate) handle: vk::Image,
    /// `layout` is the destination layout of image, if the image is as the data destination.
    ///
    /// `layout` is the current layout of image, if the image is as the data source.
    pub(crate) layout: vk::ImageLayout,
    /// `extent` is the dimension of image, if the image is as the data destination, or `extent` will be ignored.
    pub(crate) extent: vk::Extent3D,
    /// `sub_resource` is the subresources of the image used for the source or destination image data.
    pub(crate) sub_resource: vk::ImageSubresourceLayers,
}
