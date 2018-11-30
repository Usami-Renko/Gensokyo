
use core::device::HaDevice;

use memory::instance::HaImageMemory;
use image::target::HaImage;
use image::view::HaImageView;

use std::marker::PhantomData;

pub struct HaImageRepository<M> {

    phantom_type: PhantomData<M>,

    device : HaDevice,
    images : Vec<HaImage>,
    views  : Vec<HaImageView>,
    memory : HaImageMemory,
}

impl<M> HaImageRepository<M> {

    pub(crate) fn store(_: PhantomData<M>, device: HaDevice, images: Vec<HaImage>, views: Vec<HaImageView>, memory: HaImageMemory)
        -> HaImageRepository<M> {

        HaImageRepository {
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
