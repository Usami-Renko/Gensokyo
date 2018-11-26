
use core::device::HaDevice;

use memory::instance::HaImageMemory;
use image::target::HaImage;
use image::view::HaImageView;

use std::marker::PhantomData;

#[derive(Default)]
pub struct HaImageRepository<M> {

    phantom_type: PhantomData<M>,

    device : Option<HaDevice>,
    images : Vec<HaImage>,
    views  : Vec<HaImageView>,
    memory : Option<HaImageMemory>,
}

impl<M> HaImageRepository<M> {

    pub(crate) fn store(_: PhantomData<M>, device: HaDevice, images: Vec<HaImage>, views: Vec<HaImageView>, memory: HaImageMemory)
        -> HaImageRepository<M> {

        HaImageRepository {
            phantom_type: PhantomData,
            device: Some(device),
            images,
            views,
            memory: Some(memory),
        }
    }

    pub fn cleanup(&mut self) {

        if let Some(ref device) = self.device {

            self.images.iter()
                .for_each(|image| image.cleanup(device));
            self.views.iter()
                .for_each(|view| view.cleanup(device));

            if let Some(ref mut memory) = self.memory {
                memory.cleanup(device);
            }
        }

        self.views.clear();
        self.images.clear();
    }
}
