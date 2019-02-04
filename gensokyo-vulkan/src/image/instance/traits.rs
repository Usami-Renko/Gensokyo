
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::instance::sampler::GsSamplerMirror;
use crate::image::view::ImageSubRange;
use crate::image::entity::ImageEntity;
use crate::image::copy::ImageCopiable;
use crate::image::mipmap::MipmapMethod;
use crate::image::allocator::ImageAllotCI;
use crate::memory::transfer::DataCopyer;

use crate::error::VkResult;
use crate::types::{ vkuint, vkDim3D };

pub trait ImageCIApi: ImageCICommonApi + ImageCISpecificApi {}

pub trait ImageCICommonApi: ImageTgtCIApi + ImageViewCIApi {

    fn set_mipmap(&mut self, method: MipmapMethod);

    fn estimate_mip_levels(&self) -> vkuint;

    fn build(&self, device: &GsDevice) -> VkResult<GsImage>;
}

pub trait ImageCISpecificApi: Sized {
    type IConveyor: IImageConveyor;

    fn check_physical_support(&self, device: &GsDevice) -> VkResult<()>;

    fn refactor(self, device: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, Self::IConveyor)>;
}

pub trait ImageTgtCIApi: Sized {

    // image property.
    fn set_tiling(&mut self, tiling: vk::ImageTiling);
    fn set_initial_layout(&mut self, layout: vk::ImageLayout);
    fn set_samples(&mut self, count: vk::SampleCountFlags, mip_levels: vkuint, array_layers: vkuint);
    fn set_share_queues(&mut self, queue_family_indices: Vec<vkuint>);
}

pub trait ImageViewCIApi: Sized {

    // image view property.
    fn set_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle);

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
    fn set_subrange(&mut self, value: ImageSubRange);
}

/// Image Barrier Bundle Abstract.
pub trait ImageBarrierBundleAbs {

    fn make_barrier_transform(&mut self, device: &GsDevice, copyer: &DataCopyer, allot_cis: &mut Vec<ImageAllotCI>) -> VkResult<()>;
}

pub trait IImageConveyor {

    fn sampler_mirror(&self) -> Option<GsSamplerMirror>;
}

pub trait ImageInstance<I>: ImageCopiable {

    fn build(img: I, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized;
}

#[derive(Debug, Default)]
pub struct ImageInstanceInfoDesc {

    pub current_layout: vk::ImageLayout,
    pub dimension: vkDim3D,
    pub subrange: ImageSubRange,
}
