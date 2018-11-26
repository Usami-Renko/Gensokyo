
use core::device::HaDevice;

use descriptor::HaDescriptorPool;
use descriptor::HaDescriptorSet;

pub struct HaDescriptorRepository {

    device: Option<HaDevice>,
    pool  : HaDescriptorPool,
    sets  : Vec<HaDescriptorSet>,
}

impl HaDescriptorRepository {

    pub(crate) fn store(device: HaDevice, pool: HaDescriptorPool, sets: Vec<HaDescriptorSet>)
        -> HaDescriptorRepository {

        HaDescriptorRepository {
            device: Some(device.clone()),
            pool, sets,
        }
    }

    pub fn cleanup(&mut self) {

        if let Some(ref device) = self.device {

            self.pool.cleanup(&device);

            self.sets.iter()
                .for_each(|set| set.cleanup(&device));
        }

        self.sets.clear();
    }
}

