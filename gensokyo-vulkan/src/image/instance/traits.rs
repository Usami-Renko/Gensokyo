
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::view::ImageSubRange;
use crate::image::allocator::ImageAllotCI;
use crate::memory::transfer::DataCopyer;

use crate::error::VkResult;
use crate::types::vkuint;

pub trait ImageCIAbstract<R>: Sized {

    fn check_physical_support(&self, device: &GsDevice) -> VkResult<()>;
    fn build(&self, device: &GsDevice) -> VkResult<GsImage>;
    fn refactor(self, device: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, R)>;
}

pub trait ImageTgtCIAbs: Sized {

    // image property.
    fn with_tiling(self, tiling: vk::ImageTiling) -> Self;
    fn with_initial_layout(self, layout: vk::ImageLayout) -> Self;
    fn with_samples(self, count: vk::SampleCountFlags, mip_levels: vkuint, array_layers: vkuint) -> Self;
    fn with_share_queues(self, queue_family_indices: Vec<vkuint>) -> Self;
}

pub trait ImageViewCIAbs: Sized {

    // image view property.
    fn with_mapping_component(self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle) -> Self;

    /// Select the set of mipmap levels and array layers to be accessible to the view.
    ///
    /// aspect_mask is a bitmask of vk::ImageAspectFlagBits specifying which aspect(s) of the image are included in the view.
    ///
    /// base_mip_level is the first mipmap level accessible to the view.
    ///
    /// level_count is the number of mipmap levels (starting from base_mip_level) accessible to the view.
    ///
    /// base_array_layer is the first array layer accessible to the view.
    ///
    /// layer_count is the number of array layers (starting from baseArrayLayer) accessible to the view.
    fn with_subrange(self, value: ImageSubRange) -> Self;
}

/// Image Barrier Bundle Abstract.
pub trait ImageBarrierBundleAbs {

    fn make_barrier_transform(&mut self, device: &GsDevice, copyer: &DataCopyer, allot_cis: &mut Vec<ImageAllotCI>) -> VkResult<()>;
}
