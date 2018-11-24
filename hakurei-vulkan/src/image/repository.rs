
use core::device::HaDevice;

use memory::HaMemoryAbstract;
use image::target::HaImage;
use image::view::HaImageView;

use std::marker::PhantomData;

#[derive(Default)]
pub struct HaImageRepository<M> {

    phantom_type: PhantomData<M>,

    device : Option<HaDevice>,
    images : Vec<HaImage>,
    views  : Vec<HaImageView>,
    memory : Option<Box<dyn HaMemoryAbstract>>,
}

impl<M> HaImageRepository<M> {

    pub(crate) fn store(_: PhantomData<M>, device: HaDevice, images: Vec<HaImage>, views: Vec<HaImageView>, memory: Box<dyn HaMemoryAbstract>)
        -> HaImageRepository<M> {

        HaImageRepository {
            phantom_type: PhantomData,
            device: Some(device),
            memory: Some(memory),
            images, views,
        }
    }

    pub fn cleanup(&mut self) {

        if let Some(ref device) = self.device {

            self.images.iter().for_each(|image| image.cleanup(device));
            self.views.iter().for_each(|view| view.cleanup(device));

            if let Some(ref memory) = self.memory {
                memory.cleanup(device);
            }
        }

        self.views.clear();
        self.images.clear();
    }
}
