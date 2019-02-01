
use crate::core::GsDevice;

use crate::image::entity::ImageEntity;
use crate::image::view::GsImageView;
use crate::image::traits::ImageInstance;
use crate::image::allocator::ImageAllotCI;
use crate::image::allocator::types::ImageMemoryTypeAbs;
use crate::image::instance::sampler::GsSamplerMirror;
use crate::image::repository::GsImageRepository;

use crate::memory::instance::GsImageMemory;
use crate::error::VkResult;
use crate::utils::allot::{ GsAssignIndex, GsDistributeApi, GsDistIntoRepository };

use std::marker::PhantomData;
use std::collections::HashSet;

pub struct GsImageDistributor<M>
    where
        M: ImageMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device: GsDevice,
    memory: GsImageMemory,

    views: Vec<GsImageView>,
    samplers: HashSet<GsSamplerMirror>,
    image_allot_cis: Vec<ImageAllotCI>,
}

impl<M, R, T> GsDistributeApi<R, T, GsImageRepository<M>> for GsImageDistributor<M>
    where
        T: ImageInstance<R>,
        M: ImageMemoryTypeAbs {

    fn acquire(&self, index: GsAssignIndex<R>) -> T {

        let image_index = index.assign_index;
        let image_allot_ci = &self.image_allot_cis[image_index];
        let image_entity = ImageEntity::new(&image_allot_ci.image, &self.views[image_index]);
        let desc = self.image_allot_cis[image_index].gen_desc();

        T::build(index.take_info(), image_entity, desc)
    }
}

impl<M> GsDistIntoRepository<GsImageRepository<M>> for GsImageDistributor<M>
    where
        M: ImageMemoryTypeAbs {

    fn into_repository(self) -> GsImageRepository<M> {

        let images = self.image_allot_cis.into_iter()
            .map(|info| info.image).collect();

        GsImageRepository::store(self.phantom_type, self.device, images, self.views, self.samplers, self.memory)
    }
}

impl<M> GsImageDistributor<M>
    where
        M: ImageMemoryTypeAbs {

    pub(super) fn new(phantom_type: PhantomData<M>, device: GsDevice, image_allot_cis: Vec<ImageAllotCI>, samplers: HashSet<GsSamplerMirror>, memory: GsImageMemory) -> VkResult<GsImageDistributor<M>> {

        let mut views = Vec::with_capacity(image_allot_cis.len());
        for info in image_allot_cis.iter() {

            let view = info.view_ci.build(&device, &info.image, &info.image_ci.specific)?;
            views.push(view);
        }

        let distributor = GsImageDistributor { phantom_type, device, memory, image_allot_cis, samplers, views };
        Ok(distributor)
    }
}

