
use crate::core::GsDevice;

use crate::descriptor::{ GsDescriptorPool, GsDescriptorSet, DescriptorSetConfig };
use crate::descriptor::set::DescriptorSet;
use crate::descriptor::repository::GsDescriptorRepository;
use crate::descriptor::allocator::index::IDescriptorSet;

use crate::utils::allot::{ GsAssignIndex, GsDistributeApi, GsDistIntoRepository };

use crate::utils::wrapper::VKWrapperInfo;

pub struct GsDescriptorDistributor {

    device : GsDevice,
    pool   : GsDescriptorPool,
    sets   : Vec<GsDescriptorSet>,
    configs: Vec<DescriptorSetConfig>,

    update_sets: Vec<usize>,
}

impl GsDistributeApi<IDescriptorSet, DescriptorSet, GsDescriptorRepository> for GsDescriptorDistributor {

    fn acquire(&self, index: GsAssignIndex<IDescriptorSet>) -> DescriptorSet {

        let set_index = index.assign_index;
        let set = &self.sets[set_index];
        let config = &self.configs[set_index];

        DescriptorSet::new(set, config, set_index)
    }
}

impl GsDistIntoRepository<GsDescriptorRepository> for GsDescriptorDistributor {

    fn into_repository(self) -> GsDescriptorRepository {

        self.update_descriptors();

        GsDescriptorRepository::store(self.device, self.pool, self.sets)
    }
}

impl GsDescriptorDistributor {

    pub(super) fn new(device: GsDevice, pool: GsDescriptorPool, sets: Vec<GsDescriptorSet>, configs: Vec<DescriptorSetConfig>, update_sets: Vec<usize>) -> GsDescriptorDistributor {

        GsDescriptorDistributor { device, pool, sets, configs, update_sets }
    }

    fn update_descriptors(&self) {

        let mut write_infos = VKWrapperInfo::new();

        for &set_index in self.update_sets.iter() {

            let config = &self.configs[set_index];
            let update_set = &self.sets[set_index];

            for binding in config.iter_binding() {
                let write_pair = binding.write_set(update_set);
                write_infos.push(write_pair);
            }
        }

        self.device.logic.update_descriptor_sets(write_infos.borrow_info());
    }
}
