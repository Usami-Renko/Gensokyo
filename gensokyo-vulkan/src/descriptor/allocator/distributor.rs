
use crate::core::device::GsDevice;

use crate::descriptor::{ GsDescriptorPool, GsDescriptorSet, DescriptorSetConfig };

use crate::descriptor::set::DescriptorSet;
use crate::descriptor::repository::GsDescriptorRepository;
use crate::descriptor::allocator::index::DescriptorSetIndex;

pub struct GsDescriptorDistributor {

    device : GsDevice,
    pool   : GsDescriptorPool,
    sets   : Vec<GsDescriptorSet>,
    configs: Vec<DescriptorSetConfig>,

    update_sets: Vec<DescriptorSetIndex>,
}

impl GsDescriptorDistributor {

    pub(super) fn new(device: GsDevice, pool: GsDescriptorPool, sets: Vec<GsDescriptorSet>, configs: Vec<DescriptorSetConfig>) -> GsDescriptorDistributor {

        GsDescriptorDistributor {
            device, pool, sets, configs,
            update_sets: vec![],
        }
    }

    pub fn acquire_set(&mut self, index: DescriptorSetIndex) -> DescriptorSet {

        let set = &self.sets[index.value];
        let config = &self.configs[index.value];
        self.update_sets.push(index.clone());

        DescriptorSet::new(set, config, index.value)
    }

    pub fn into_repository(self) -> GsDescriptorRepository {

        self.update_descriptors();

        GsDescriptorRepository::store(self.device, self.pool, self.sets)
    }

    fn update_descriptors(&self) {

        let mut write_infos = Vec::with_capacity(self.update_sets.len());

        for set_index in self.update_sets.iter() {

            let config = &self.configs[set_index.value];
            let update_set = &self.sets[set_index.value];

            let set_write_infos: Vec<_> = config.iter_binding()
                .map(|binding| binding.write_set(update_set))
                .collect();
            write_infos.extend(set_write_infos);
        }

        self.device.update_descriptor_sets(write_infos);
    }
}
