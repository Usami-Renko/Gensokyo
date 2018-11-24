
use core::device::HaDevice;

use image::view::HaImageView;
use image::allocator::ImageAllocateInfo;
use image::instance::sample::{ HaSampleImage, SampleImageInfo };
use image::instance::depth::{ HaDepthStencilAttachment, DepthStencilAttachmentInfo };
use image::instance::ImageInstanceInfoAbs;
use image::repository::HaImageRepository;
use image::ImageError;

use memory::HaMemoryAbstract;
use memory::AllocatorError;

use std::marker::PhantomData;

pub struct HaImageDistributor<M> {

    phantom_type: PhantomData<M>,

    device: HaDevice,
    memory: Box<dyn HaMemoryAbstract>,

    views: Vec<HaImageView>,
    infos: Vec<ImageAllocateInfo>,
}

impl<M> HaImageDistributor<M> {

    pub(super) fn new(_: PhantomData<M>, device: HaDevice, infos: Vec<ImageAllocateInfo>, memory: Box<dyn HaMemoryAbstract>) -> Result<HaImageDistributor<M>, AllocatorError> {

        let mut views = vec![];
        for info in infos.iter() {

            let view = info.view_desc.build(&device, &info.image, &info.image_desc.specific)?;
            views.push(view);
        }

        let distributor = HaImageDistributor {
            phantom_type: PhantomData,
            device, memory, infos, views,
        };

        Ok(distributor)
    }

    pub fn acquire_sample_image(&self, info: SampleImageInfo) -> Result<HaSampleImage, AllocatorError> {

        let allocate_index = info.allocate_index()
            .ok_or(AllocatorError::Image(ImageError::NotYetAllocateError))?;
        let sampler = info.gen_sample(&self.device)?;

        let image = HaSampleImage::setup(
            info, sampler,
            &self.infos[allocate_index],
            &self.views[allocate_index]
        );

        Ok(image)
    }

    pub fn acquire_depth_stencil_image(&self, info: DepthStencilAttachmentInfo) -> Result<HaDepthStencilAttachment, AllocatorError> {

        let allocate_index = info.allocate_index()
            .ok_or(AllocatorError::Image(ImageError::NotYetAllocateError))?;

        let image = HaDepthStencilAttachment::setup(
            info,
            &self.infos[allocate_index],
            &self.views[allocate_index],
        );

        Ok(image)
    }

    pub fn into_repository(self) -> HaImageRepository<M> {

        let images = self.infos.into_iter()
            .map(|info| info.image).collect();

        HaImageRepository::store(self.phantom_type, self.device, images, self.views, self.memory)
    }
}
