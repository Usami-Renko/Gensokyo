
use ash::vk;

use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::image::target::GsImage;
use crate::image::storage::ImageStorageInfo;
use crate::image::allocator::ImageAllocateInfo;
use crate::image::error::ImageError;
use crate::memory::transfer::DataCopyer;
use crate::memory::AllocatorError;

use crate::types::vkuint;

pub trait ImageInstanceInfoAbs where Self: Sized {

    fn build_image(&self, device: &GsDevice) -> Result<GsImage, ImageError>;
    fn allocate_index(&self) -> Option<usize>;
    fn set_allocate_index(&mut self, value: usize);
    fn allocate_info(&self, image: GsImage, storage: ImageStorageInfo) -> ImageAllocateInfo;
}

pub trait GsImageDescAbs where Self: Sized {

    // image property.
    fn with_tiling(&mut self, tiling: vk::ImageTiling);
    fn with_initial_layout(&mut self, layout: vk::ImageLayout);
    fn with_samples(&mut self, count: vk::SampleCountFlags, mip_levels: vkuint, array_layers: vkuint);
    fn with_share_queues(&mut self, queue_family_indices: Vec<vkuint>);
}

pub trait GsImageViewDescAbs where Self: Sized {

    // image view property.
    fn with_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle);

    /// Select the set of mipmap levels and array layers to be accessible to the view.
    ///
    /// base_mip_level is the first mipmap level accessible to the view.
    ///
    /// level_count is the number of mipmap levels (starting from base_mip_level) accessible to the view.
    ///
    /// base_array_layer is the first array layer accessible to the view.
    ///
    /// layer_count is the number of array layers (starting from baseArrayLayer) accessible to the view.
    fn with_subrange(&mut self, base_mip_level: vkuint, level_count: vkuint, base_array_layer: vkuint, layer_count: vkuint);
}

/// Image Barrier Bundle Abstract.
pub trait ImageBarrierBundleAbs {

    fn make_transfermation(&mut self, physical: &GsPhyDevice, device: &GsDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllocateInfo>) -> Result<(), AllocatorError>;
    fn cleanup(&mut self);
}
