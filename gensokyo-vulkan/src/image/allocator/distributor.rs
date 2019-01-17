
use crate::core::device::GsDevice;

use crate::image::view::GsImageView;
use crate::image::allocator::ImageAllocateInfo;
use crate::image::instance::sample::{ GsSampleImage, SampleImageInfo };
use crate::image::instance::depth::{ GsDepthStencilAttachment, DepthStencilAttachmentInfo };
use crate::image::instance::ImageInstanceInfoAbs;
use crate::image::repository::GsImageRepository;

use crate::memory::instance::GsImageMemory;
use crate::error::VkResult;

use std::marker::PhantomData;

pub struct GsImageDistributor<M> {

    phantom_type: PhantomData<M>,

    device: GsDevice,
    memory: GsImageMemory,

    views: Vec<GsImageView>,
    infos: Vec<ImageAllocateInfo>,
}

impl<M> GsImageDistributor<M> {

    pub(super) fn new(phantom_type: PhantomData<M>, device: GsDevice, infos: Vec<ImageAllocateInfo>, memory: GsImageMemory) -> VkResult<GsImageDistributor<M>> {

        let mut views = vec![];
        for info in infos.iter() {

            let view = info.view_desc.build(&device, &info.image, &info.image_desc.specific)?;
            views.push(view);
        }

        let distributor = GsImageDistributor {
            phantom_type, device, memory, infos, views,
        };

        Ok(distributor)
    }

    pub fn acquire_sample_image(&self, info: SampleImageInfo) -> VkResult<GsSampleImage> {

        // TODO: Simplified unwrap() here.
        let allocate_index = info.allocate_index().unwrap();
        let sampler = info.gen_sample(&self.device)?;

        let image = GsSampleImage::setup(
            info, sampler,
            &self.infos[allocate_index],
            &self.views[allocate_index]
        );

        Ok(image)
    }

    pub fn acquire_depth_stencil_image(&self, info: DepthStencilAttachmentInfo) -> VkResult<GsDepthStencilAttachment> {

        // TODO: Simplified unwrap() here.
        let allocate_index = info.allocate_index().unwrap();

        let image = GsDepthStencilAttachment::setup(
            info,
            &self.infos[allocate_index],
            &self.views[allocate_index]
        );

        Ok(image)
    }

    pub fn into_repository(self) -> GsImageRepository<M> {

        let images = self.infos.into_iter()
            .map(|info| info.image).collect();

        GsImageRepository::store(self.phantom_type, self.device, images, self.views, self.memory)
    }
}
