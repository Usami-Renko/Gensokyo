
use core::device::HaDevice;

use resources::image::HaImageView;
use resources::image::{ HaSampleImage, SampleImageInfo };
use resources::image::{ HaDepthStencilImage, DepthStencilImageInfo };
use resources::image::ImageBranchInfoAbs;
use resources::memory::HaMemoryAbstract;
use resources::allocator::ImageAllocateInfo;
use resources::repository::HaImageRepository;
use resources::error::AllocatorError;

pub struct HaImageDistributor {

    device: HaDevice,
    memory: Box<HaMemoryAbstract>,

    views: Vec<HaImageView>,
    infos: Vec<ImageAllocateInfo>,
}

impl HaImageDistributor {

    pub(crate) fn new(device: HaDevice, infos: Vec<ImageAllocateInfo>, memory: Box<HaMemoryAbstract>) -> Result<HaImageDistributor, AllocatorError> {

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

        let sampler = info.gen_sample(&self.device)?;
        let allocate_index = info.allocate_index();

        let image = HaSampleImage::setup(
            info, sampler,
            &self.infos[allocate_index],
            self.views[allocate_index].handle,
        );

        Ok(image)
    }

    pub fn acquire_depth_stencil_image(&self, info: DepthStencilImageInfo) -> Result<HaDepthStencilImage, AllocatorError> {

        let allocate_index = info.allocate_index();
        let format = info.format;

        let image = HaDepthStencilImage::setup(
            info, format,
            &self.infos[allocate_index],
            self.views[allocate_index].handle,
        );

        Ok(image)
    }

    pub fn into_repository(self) -> HaImageRepository {

        let images = self.infos.into_iter()
            .map(|info| info.image).collect();

        HaImageRepository::store(self.device, images, self.views, self.memory)
    }
}
