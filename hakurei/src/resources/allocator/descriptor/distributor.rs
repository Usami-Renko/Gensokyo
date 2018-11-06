
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::repository::HaDescriptorRepository;
use resources::descriptor::{ HaDescriptorPool, HaDescriptorSet, DescriptorSetConfig };
use resources::descriptor::{ DescriptorSet, DescriptorSetIndex };
use resources::descriptor::DescriptorSetItem;

pub struct HaDescriptorDistributor {

    device : HaDevice,
    pool   : HaDescriptorPool,
    sets   : Vec<HaDescriptorSet>,
    configs: Vec<DescriptorSetConfig>,

    update_sets: Vec<DescriptorSetIndex>,
}

impl HaDescriptorDistributor {

    pub(crate) fn new(device: HaDevice, pool: HaDescriptorPool, sets: Vec<HaDescriptorSet>, configs: Vec<DescriptorSetConfig>) -> HaDescriptorDistributor {

        HaDescriptorDistributor {
            device, pool, sets, configs,
            update_sets: vec![],
        }
    }

    pub fn acquire_set(&mut self, index: DescriptorSetIndex) -> DescriptorSet {

        let set = &self.sets[index.0];
        let binding_indices = self.configs[index.0].bindings.iter()
            .map(|binding_info| binding_info.binding_index()).collect();

        let set = DescriptorSet {
            item: DescriptorSetItem {
                handle   : set.handle,
                set_index: index.0,
                binding_indices,
            },
            layout: set.layout.handle,
        };

        self.update_sets.push(index);

        set
    }

    pub fn into_repository(self) -> HaDescriptorRepository {

        self.update_descriptors();

        HaDescriptorRepository::store(self.device, self.pool, self.sets)
    }

    fn update_descriptors(&self) {

        let mut write_infos = Vec::with_capacity(self.update_sets.len());

        for set_index in self.update_sets.iter() {

            for binding_info in self.configs[set_index.0].bindings.iter() {
                let write_info = binding_info.write_set(&self.sets[set_index.0]);
                write_infos.push(write_info);
            }
        }

        let write_sets = write_infos.into_iter()
            .map(|info| info.info).collect::<Vec<_>>();

        unsafe {
            self.device.handle.update_descriptor_sets(&write_sets, &[]);
        }
    }
}
