
use crate::core::device::GsDevice;

use crate::image::entity::ImageEntity;
use crate::image::view::GsImageView;
use crate::image::traits::ImageInstance;
use crate::image::allocator::ImageAllotInfo;
use crate::image::allocator::types::ImageMemoryTypeAbs;
use crate::image::repository::GsImageRepository;

use crate::memory::instance::GsImageMemory;
use crate::error::VkResult;
use crate::utils::api::{ GsAssignIndex, GsDistributeApi, GsDistIntoRepository };

use std::marker::PhantomData;

pub struct GsImageDistributor<M> where M: ImageMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device: GsDevice,
    memory: GsImageMemory,

    views: Vec<GsImageView>,
    image_infos: Vec<ImageAllotInfo>,
}

impl<M, R, T> GsDistributeApi<R, T, GsImageRepository<M>> for GsImageDistributor<M>
    where T: ImageInstance<R>,
          M: ImageMemoryTypeAbs {

    fn acquire(&self, index: GsAssignIndex<R>) -> T {

        let image_index = index.assign_index;
        let image_info = &self.image_infos[image_index];
        let image_entity = ImageEntity::new(&image_info.image, &self.views[image_index]);
        let desc = self.image_infos[image_index].gen_desc();

        T::new(index.take_info(), image_entity, desc)
    }
}

impl<M> GsDistIntoRepository<GsImageRepository<M>> for GsImageDistributor<M>
    where M: ImageMemoryTypeAbs {

    fn into_repository(self) -> GsImageRepository<M> {

        let images = self.image_infos.into_iter()
            .map(|info| info.image).collect();

        GsImageRepository::store(self.phantom_type, self.device, images, self.views, self.memory)
    }
}

impl<M> GsImageDistributor<M> where M: ImageMemoryTypeAbs {

    pub(super) fn new(phantom_type: PhantomData<M>, device: GsDevice, image_infos: Vec<ImageAllotInfo>, memory: GsImageMemory) -> VkResult<GsImageDistributor<M>> {

        let mut views = vec![];
        for info in image_infos.iter() {

            let view = info.view_desc.build(&device, &info.image, &info.image_desc.specific)?;
            views.push(view);
        }

        let distributor = GsImageDistributor {
            phantom_type, device, memory, image_infos, views,
        };

        Ok(distributor)
    }
}
