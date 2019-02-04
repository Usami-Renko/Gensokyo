
use crate::core::GsDevice;

use crate::descriptor::GsDescriptorPool;
use crate::descriptor::GsDescriptorSet;

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
}

impl Drop for GsDescriptorRepository {

    fn drop(&mut self) {

        self.pool.discard(&self.device);

        self.sets.iter().for_each(|set| set.discard(&self.device));
    }
}
