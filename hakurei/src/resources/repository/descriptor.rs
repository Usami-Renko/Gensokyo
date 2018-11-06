
use core::device::HaDevice;

use resources::descriptor::HaDescriptorPool;
use resources::descriptor::HaDescriptorSet;

pub struct HaDescriptorRepository {

    device: Option<HaDevice>,
    pool  : HaDescriptorPool,
    sets  : Vec<HaDescriptorSet>,
}

impl HaDescriptorRepository {

    pub fn empty() -> HaDescriptorRepository {
        HaDescriptorRepository {
            device: None,
            pool  : HaDescriptorPool::uninitialize(),
            sets  : vec![],
        }
    }

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
            self.pool = HaDescriptorPool::uninitialize();

            self.sets.iter()
                .for_each(|set| set.cleanup(&device));
        }

        self.sets.clear();
    }
}

