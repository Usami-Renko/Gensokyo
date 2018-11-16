
use vk::core::device::HaDevice;

use vk::resources::descriptor::{ HaDescriptorPool, HaDescriptorSet, DescriptorSetConfig };
use vk::resources::descriptor::DescriptorSetItem;

use resources::descriptor::DescriptorSet;
use resources::repository::HaDescriptorRepository;
use resources::allocator::descriptor::index::DescriptorSetIndex;

pub struct HaDescriptorDistributor {

    device : HaDevice,
    pool   : HaDescriptorPool,
    sets   : Vec<HaDescriptorSet>,
    configs: Vec<DescriptorSetConfig>,

    update_sets: Vec<DescriptorSetIndex>,
}

impl HaDescriptorDistributor {

    pub(super) fn new(device: HaDevice, pool: HaDescriptorPool, sets: Vec<HaDescriptorSet>, configs: Vec<DescriptorSetConfig>) -> HaDescriptorDistributor {

        HaDescriptorDistributor {
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

    pub fn into_repository(self) -> HaDescriptorRepository {

        self.update_descriptors();

        HaDescriptorRepository::store(self.device, self.pool, self.sets)
    }

    fn update_descriptors(&self) {

        let mut write_infos = Vec::with_capacity(self.update_sets.len());

        for set_index in self.update_sets.iter() {

            let config = &self.configs[set_index.value];
            let update_set = &self.sets[set_index.value];

            let set_write_infos = config.iter_binding()
                .map(|binding| binding.write_set(update_set))
                .collect::<Vec<_>>();
            write_infos.extend(set_write_infos);
        }

        self.device.update_descriptor_sets(write_infos);
    }
}
