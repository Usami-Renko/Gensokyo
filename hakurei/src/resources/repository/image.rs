
use core::device::HaDevice;

use resources::memory::HaMemoryAbstract;
use resources::image::{ HaImage, HaImageView };

#[derive(Default)]
pub struct HaImageRepository {

    device : Option<HaDevice>,
    images : Vec<HaImage>,
    views  : Vec<HaImageView>,
    memory : Option<Box<HaMemoryAbstract>>,
}

impl HaImageRepository {

    pub fn empty() -> HaImageRepository {
        HaImageRepository::default()
    }

    pub(crate) fn store(device: HaDevice, images: Vec<HaImage>, views: Vec<HaImageView>, memory: Box<HaMemoryAbstract>)
        -> HaImageRepository {

        HaImageRepository {
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
