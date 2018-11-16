
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;

use vk::pipeline::state::multisample::SampleCountType;

use vk::resources::image::{ HaImage, ImageDescInfo };
use vk::resources::image::{ ImageLayout, ImageTiling };
use vk::resources::image::ImageCopyInfo;
use vk::resources::image::ComponentSwizzle;
use vk::resources::error::{ AllocatorError, ImageError };
use vk::resources::transfer::DataCopyer;

use resources::allocator::image::ImageAllocateInfo;
use resources::image::io::{ ImageLoadConfig, ImageStorageInfo };

use vk::utils::types::vkint;

pub trait ImageBranchInfoAbs {

    fn storage(&mut self, physical: &HaPhyDevice, config: &ImageLoadConfig) ->  Result<ImageStorageInfo, ImageError>;
    fn view_desc(&self) -> &ImageDescInfo;
    fn allocate_index(&self) -> Option<usize>;
    fn set_allocate_index(&mut self, value: usize);
    fn allocate_info(&self, image: HaImage, storage: ImageStorageInfo) -> ImageAllocateInfo;
}

pub trait HaImageDescAbs {

    // image property.
    fn set_tiling(&mut self, tiling: ImageTiling);
    fn set_initial_layout(&mut self, layout: ImageLayout);
    fn set_samples(&mut self, count: SampleCountType, mip_levels: vkint, array_layers: vkint);
    fn set_share_queues(&mut self, queue_family_indices: Vec<vkint>);
}

pub trait HaImageViewDescAbs {

    // image view property.
    fn set_mapping_component(&mut self, r: ComponentSwizzle, g: ComponentSwizzle, b: ComponentSwizzle, a: ComponentSwizzle);

    /// Select the set of mipmap levels and array layers to be accessible to the view.
    ///
    /// base_mip_level is the first mipmap level accessible to the view.
    ///
    /// level_count is the number of mipmap levels (starting from base_mip_level) accessible to the view.
    ///
    /// base_array_layer is the first array layer accessible to the view.
    ///
    /// layer_count is the number of array layers (starting from baseArrayLayer) accessible to the view.
    fn set_subrange(&mut self, base_mip_level: vkint, level_count: vkint, base_array_layer: vkint, layer_count: vkint);
}

/// Image Barrier Bundle Abstract.
pub trait ImageBarrierBundleAbs {

    fn make_transfermation(&mut self, physical: &HaPhyDevice, device: &HaDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllocateInfo>) -> Result<(), AllocatorError>;
    fn cleanup(&mut self);
}
