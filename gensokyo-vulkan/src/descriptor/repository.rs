
use core::device::GsDevice;

use descriptor::GsDescriptorPool;
use descriptor::GsDescriptorSet;

pub struct GsDescriptorRepository {

    device: GsDevice,
    pool  : GsDescriptorPool,
    sets  : Vec<GsDescriptorSet>,
}

impl GsDescriptorRepository {

    pub(crate) fn store(device: GsDevice, pool: GsDescriptorPool, sets: Vec<GsDescriptorSet>)
        -> GsDescriptorRepository {

        GsDescriptorRepository {
            device: device.clone(),
            pool, sets,
        }
    }

    pub fn cleanup(&mut self) {

        self.pool.cleanup(&self.device);

        self.sets.iter()
            .for_each(|set| set.cleanup(&self.device));

        self.sets.clear();
    }
}

