
use ash::vk;

use core::device::GsDevice;

use image::target::{ GsImage, ImageDescInfo, ImagePropertyInfo, ImageSpecificInfo };
use image::view::ImageViewDescInfo;
use image::sampler::{ GsSampler, SamplerDescInfo };
use image::enums::{ ImageInstanceType, ImagePipelineStage };
use image::storage::ImageStorageInfo;
use image::instance::traits::{ ImageInstanceInfoAbs, GsImageDescAbs ,GsImageViewDescAbs };
use image::allocator::ImageAllocateInfo;
use image::error::ImageError;

use descriptor::{ DescriptorBindingContent, GsDescriptorType, ImageDescriptorType };

use types::vkuint;

pub struct SampleImageInfo {

    pipeline_stage: ImagePipelineStage,

    image_desc  : ImageDescInfo,
    view_desc   : ImageViewDescInfo,
    sampler_desc: SamplerDescInfo,

    allocate_index: Option<usize>,
    storage: Option<ImageStorageInfo>,

    binding: DescriptorBindingContent,
}

impl SampleImageInfo {

    pub fn new(binding: vkuint, count: vkuint, storage: ImageStorageInfo, pipeline_stage: ImagePipelineStage) -> SampleImageInfo {

        let mut property = ImagePropertyInfo::default();
        property.image_type = vk::ImageType::TYPE_2D;
        property.tiling = vk::ImageTiling::OPTIMAL;
        property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;

        let mut specific = ImageSpecificInfo::default();
        specific.format    = storage.format;
        specific.dimension = storage.dimension;

        let binding = DescriptorBindingContent {
            binding, count,
            descriptor_type: GsDescriptorType::Image(ImageDescriptorType::CombinedImageSampler)
        };

        SampleImageInfo {
            image_desc: ImageDescInfo { property, specific },
            view_desc: ImageViewDescInfo::new(vk::ImageViewType::TYPE_2D, vk::ImageAspectFlags::COLOR),
            sampler_desc: SamplerDescInfo::default(),
            storage: Some(storage),
            pipeline_stage, allocate_index: None, binding,
        }
    }

    pub(crate) fn gen_sample(&self, device: &GsDevice) -> Result<GsSampler, ImageError> {
        self.sampler_desc.build(device)
    }

    pub(crate) fn take_storage(&mut self) -> Option<ImageStorageInfo> {
        self.storage.take()
    }

    pub fn binding(&self) -> DescriptorBindingContent {
        self.binding.clone()
    }
}

impl ImageInstanceInfoAbs for SampleImageInfo {

    fn build_image(&self, device: &GsDevice) -> Result<GsImage, ImageError> {
        self.image_desc.build(device)
    }

    fn allocate_index(&self) -> Option<usize> {
        self.allocate_index
    }

    fn set_allocate_index(&mut self, value: usize) {
        self.allocate_index = Some(value);
    }

    fn allocate_info(&self, image: GsImage, storage: ImageStorageInfo) -> ImageAllocateInfo {

        ImageAllocateInfo::new(
            ImageInstanceType::SampleImage { stage: self.pipeline_stage },
            storage, image,
            self.image_desc.clone(),
            self.view_desc.clone()
        )
    }
}

impl_image_desc_info_abs!(SampleImageInfo);
