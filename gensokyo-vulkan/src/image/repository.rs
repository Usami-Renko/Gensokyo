
use crate::core::device::GsDevice;

use crate::memory::instance::GsImageMemory;
use crate::image::target::GsImage;
use crate::image::view::GsImageView;

use std::marker::PhantomData;

pub struct GsImageRepository<M> {

    phantom_type: PhantomData<M>,

    device : GsDevice,
    images : Vec<GsImage>,
    views  : Vec<GsImageView>,
    memory : GsImageMemory,
}

impl<M> GsImageRepository<M> {

    pub(crate) fn store(_: PhantomData<M>, device: GsDevice, images: Vec<GsImage>, views: Vec<GsImageView>, memory: GsImageMemory)
        -> GsImageRepository<M> {

        GsImageRepository {
            phantom_type: PhantomData,
            device, images, views, memory,
        }
    }
}

impl<M> Drop for GsImageRepository<M> {

    fn drop(&mut self) {

        self.images.iter()
            .for_each(|image| image.destroy(&self.device));
        self.views.iter()
            .for_each(|view| view.destroy(&self.device));

        self.memory.destroy(&self.device);
    }
}
