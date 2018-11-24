
use ash::vk;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use image::target::HaImage;
use image::storage::ImageStorageInfo;
use image::allocator::ImageAllocateInfo;
use image::error::ImageError;
use memory::transfer::DataCopyer;
use memory::AllocatorError;

use types::vkuint;

pub trait ImageInstanceInfoAbs {

    fn build_image(&self, device: &HaDevice) -> Result<HaImage, ImageError>;
    fn allocate_index(&self) -> Option<usize>;
    fn set_allocate_index(&mut self, value: usize);
    fn allocate_info(&self, image: HaImage, storage: ImageStorageInfo) -> ImageAllocateInfo;
}

pub trait HaImageDescAbs {

    // image property.
    fn with_tiling(&mut self, tiling: vk::ImageTiling);
    fn with_initial_layout(&mut self, layout: vk::ImageLayout);
    fn with_samples(&mut self, count: vk::SampleCountFlags, mip_levels: vkuint, array_layers: vkuint);
    fn with_share_queues(&mut self, queue_family_indices: Vec<vkuint>);
}

pub trait HaImageViewDescAbs {

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

    fn make_transfermation(&mut self, physical: &HaPhyDevice, device: &HaDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllocateInfo>) -> Result<(), AllocatorError>;
    fn cleanup(&mut self);
}
