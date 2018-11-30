
use core::device::HaDevice;

use descriptor::HaDescriptorPool;
use descriptor::HaDescriptorSet;

pub struct HaDescriptorRepository {

    device: HaDevice,
    pool  : HaDescriptorPool,
    sets  : Vec<HaDescriptorSet>,
}

impl HaDescriptorRepository {

    pub(crate) fn store(device: HaDevice, pool: HaDescriptorPool, sets: Vec<HaDescriptorSet>)
        -> HaDescriptorRepository {

        HaDescriptorRepository {
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

