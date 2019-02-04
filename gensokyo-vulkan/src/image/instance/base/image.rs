
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::{ GsImage, ImageTgtCI, ImagePropertyCI, ImageSpecificCI };
use crate::image::view::{ ImageViewCI, ImageSubRange };
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::traits::{ ImageCICommonApi, ImageTgtCIApi, ImageViewCIApi };
use crate::image::mipmap::MipmapMethod;
use crate::image::compress::ImageCompressType;

use crate::error::{ VkResult, VkError };
use crate::types::vkuint;

pub struct GsBackendImage {

    pub image_ci: ImageTgtCI,
    pub view_ci : ImageViewCI,

    pub storage: ImageStorageInfo,
}

impl From<ImageStorageInfo> for GsBackendImage {

    fn from(storage: ImageStorageInfo) -> GsBackendImage {

        let property = ImagePropertyCI::default();

        let mut specific = ImageSpecificCI::default();
        specific.format    = storage.format;
        specific.dimension = storage.dimension;

        GsBackendImage {
            storage,
            image_ci: ImageTgtCI { property, specific },
            view_ci: ImageViewCI::default(),
        }
    }
}

impl ImageCICommonApi for GsBackendImage {

    fn set_mipmap(&mut self, method: MipmapMethod) {

        self.image_ci.property.mipmap = method;

        match method {
            | MipmapMethod::Disable => {
                self.image_ci.property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
                self.image_ci.property.mip_levels = 1;
                self.view_ci.subrange.0.base_mip_level = 0;
                self.view_ci.subrange.0.level_count    = 1;
            },
            | MipmapMethod::StepBlit
            | MipmapMethod::BaseLevelBlit => {
                let mip_level = self.estimate_mip_levels();

                self.image_ci.property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
                self.image_ci.property.mip_levels = mip_level;
                self.view_ci.subrange.0.base_mip_level = 0;
                self.view_ci.subrange.0.level_count    = mip_level;
            },
        }
    }

    fn estimate_mip_levels(&self) -> vkuint {

        use std::cmp::max;
        let max_extent = max(self.image_ci.specific.dimension.width, self.image_ci.specific.dimension.height) as f32;
        (max_extent.log2().floor() as vkuint) + 1
    }

    fn build(&self, device: &GsDevice) -> VkResult<GsImage> {
        self.image_ci.build(device)
    }
}

// Property setting for vk::Image.
impl ImageTgtCIApi for GsBackendImage {

    fn set_tiling(&mut self, tiling: vk::ImageTiling) {
        self.image_ci.property.tiling = tiling;
    }

    fn set_initial_layout(&mut self, layout: vk::ImageLayout) {
        self.image_ci.property.initial_layout = layout;
    }

    fn set_samples(&mut self, count: vk::SampleCountFlags, mip_levels: vkuint, array_layers: vkuint) {
        self.image_ci.property.sample_count = count;
        self.image_ci.property.mip_levels = mip_levels;
        self.image_ci.property.array_layers = array_layers;
    }

    fn set_share_queues(&mut self, queue_family_indices: Vec<vkuint>) {
        self.image_ci.specific.share_queue_families(Some(queue_family_indices));
    }

    fn set_compression(&mut self, compression: ImageCompressType) {
        self.image_ci.specific.compression = compression;
    }
}

// Property setting for vk::ImageView.
impl ImageViewCIApi for GsBackendImage {

    fn set_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle) {
        self.view_ci.components = vk::ComponentMapping { r, g, b, a };
    }

    fn set_subrange(&mut self, value: ImageSubRange) {
        self.view_ci.subrange = value;
    }
}

impl GsBackendImage {

    pub fn check_mipmap_support(&self, device: &GsDevice) -> VkResult<()> {

        if self.image_ci.property.mipmap.is_support_by_device(device, &self.image_ci)? == false {
            return Err(VkError::other(format!("vk::Format: {:?} is not support for mipmap generation", self.image_ci.specific.format)))
        }

        Ok(())
    }
}
