
use vk::core::device::HaDevice;

use vk::resources::image::HaImageView;
use vk::resources::memory::HaMemoryAbstract;
use vk::resources::error::{ AllocatorError, ImageError };

use resources::image::sample::{ HaSampleImage, SampleImageInfo };
use resources::image::depth::{ HaDepthStencilAttachment, DepthStencilAttachmentInfo };

use resources::image::ImageBranchInfoAbs;
use resources::allocator::image::ImageAllocateInfo;
use resources::repository::HaImageRepository;

pub struct HaImageDistributor {

    device: HaDevice,
    memory: Box<HaMemoryAbstract>,

    views: Vec<HaImageView>,
    infos: Vec<ImageAllocateInfo>,
}

impl HaImageDistributor {

    pub(super) fn new(device: HaDevice, infos: Vec<ImageAllocateInfo>, memory: Box<HaMemoryAbstract>) -> Result<HaImageDistributor, AllocatorError> {

        let mut views = vec![];
        for info in infos.iter() {
            let view = HaImageView::config(&device, &info.image, &info.view_desc, info.storage.format)?;
            views.push(view);
        }

        let distributor = HaImageDistributor {
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

    pub fn into_repository(self) -> HaImageRepository {

        let images = self.infos.into_iter()
            .map(|info| info.image).collect();

        HaImageRepository::store(self.device, images, self.views, self.memory)
    }
}
