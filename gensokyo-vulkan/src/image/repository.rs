
use core::device::GsDevice;

use memory::instance::GsImageMemory;
use image::target::GsImage;
use image::view::GsImageView;

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

    pub fn cleanup(&mut self) {

        self.images.iter()
            .for_each(|image| image.cleanup(&self.device));
        self.views.iter()
            .for_each(|view| view.cleanup(&self.device));

        self.memory.cleanup(&self.device);

        self.views.clear();
        self.images.clear();
    }
}
