
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::descriptor::HaDescriptorPool;
use resources::descriptor::{ DescriptorItem, DescriptorSetItem };
use resources::descriptor::HaDescriptorSetLayout;
use resources::descriptor::{ DescriptorSetConfig, HaDescriptorSet };

pub struct CmdDescriptorBindingInfos {

    pub(crate) handles: Vec<vk::DescriptorSet>,
}


pub struct HaDescriptorRepository {

    device : Option<HaDevice>,
    pool   : HaDescriptorPool,
    sets   : Vec<HaDescriptorSet>,
    configs: Vec<DescriptorSetConfig>,
}

impl HaDescriptorRepository {

    pub fn empty() -> HaDescriptorRepository {
        HaDescriptorRepository {
            device : None,
            pool   : HaDescriptorPool::uninitialize(),
            sets   : vec![],
            configs: vec![],
        }
    }

    pub(crate) fn store(device: &HaDevice, pool: HaDescriptorPool, sets: Vec<HaDescriptorSet>, configs: Vec<DescriptorSetConfig>)
        -> HaDescriptorRepository {

        HaDescriptorRepository {
            device: Some(device.clone()),
            pool, sets, configs,
        }
    }

    // TODO: Currently only support descriptors in the same Buffer Repository.
    // TODO: Redesign the API, if items is not buffer items, the function will crash.
    pub fn update_descriptors(&self, items: &[DescriptorItem]) {

        let mut write_sets = vec![];

        for item in items.iter() {

            let binding_info = &self.configs[item.set_index].bindings[item.binding_index];
            let write_set = binding_info.write_set(&self.sets[item.set_index]);
            write_sets.push(write_set);
        }

        unsafe {
            self.device.as_ref().unwrap().handle.update_descriptor_sets(&write_sets, &[]);
        }
    }

    pub fn set_layout_at(&self, set_item: &DescriptorSetItem) -> &HaDescriptorSetLayout {
        &self.sets[set_item.set_index].layout
    }

    pub fn descriptor_binding_infos(&self, sets: &[&DescriptorSetItem]) -> CmdDescriptorBindingInfos {

        let handles = sets.iter()
            .map(|set_item| self.sets[set_item.set_index].handle).collect();
        CmdDescriptorBindingInfos {
            handles,
        }
    }

    pub fn cleanup(&mut self) {

        if let Some(ref device) = self.device {
            for config in self.configs.iter() {
                config.cleanup(&device);
            }

            self.pool.cleanup(&device);
            self.pool = HaDescriptorPool::uninitialize();

            for set in self.sets.iter() {
                set.cleanup(&device);
            }
        }

        self.sets.clear();
        self.configs.clear();
    }
}

