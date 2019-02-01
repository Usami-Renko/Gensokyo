
use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::view::GsImageView;
use crate::image::instance::sampler::GsSamplerMirror;
use crate::image::allocator::types::ImageMemoryTypeAbs;

use crate::memory::instance::GsImageMemory;

use std::marker::PhantomData;
use std::collections::HashSet;

pub struct GsImageRepository<M>
    where
        M: ImageMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device : GsDevice,

    images  : Vec<GsImage>,
    views   : Vec<GsImageView>,
    samplers: HashSet<GsSamplerMirror>,

    memory : GsImageMemory,
}

impl<M> GsImageRepository<M>
    where
        M: ImageMemoryTypeAbs {

    pub(crate) fn store(_: PhantomData<M>, device: GsDevice, images: Vec<GsImage>, views: Vec<GsImageView>, samplers: HashSet<GsSamplerMirror>, memory: GsImageMemory)
        -> GsImageRepository<M> {

        GsImageRepository {
            phantom_type: PhantomData,
            device, images, views, samplers, memory,
        }
    }
}

impl<M> Drop for GsImageRepository<M>
    where
        M: ImageMemoryTypeAbs {

    fn drop(&mut self) {

        self.images.iter().for_each(|image| image.destroy(&self.device));
        self.views.iter().for_each(|view| view.destroy(&self.device));
        self.samplers.iter().for_each(|sampler| sampler.destroy(&self.device));

        self.memory.destroy(&self.device);
    }
}
