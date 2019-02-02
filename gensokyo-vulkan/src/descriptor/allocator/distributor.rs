
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

        DescriptorSet::new(set, set_index)
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

        let mut buffer_write_infos = VKWrapperInfo::new();
        let mut  image_write_infos = VKWrapperInfo::new();

        for &set_index in self.update_sets.iter() {

            let config = &self.configs[set_index];
            let update_set = &self.sets[set_index];

            config.add_write_set(update_set, &mut buffer_write_infos, &mut image_write_infos);
        }

        if buffer_write_infos.is_empty() == false {
            self.device.logic.update_descriptor_sets(buffer_write_infos.borrow_info());
        }

        if image_write_infos.is_empty() == false {
            self.device.logic.update_descriptor_sets(image_write_infos.borrow_info());
        }
    }
}
