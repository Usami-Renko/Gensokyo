
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::descriptor::HaDescriptorPool;
use resources::descriptor::{ DescriptorItem, DescriptorSetItem };
use resources::descriptor::HaDescriptorSetLayout;
use resources::descriptor::{ DescriptorSetConfig, HaDescriptorSet };
use resources::error::AllocatorError;

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

    // TODO: Currently only support descriptors in the same HaDescriptorRepository.
    // TODO: Redesign the API, if items is not buffer items, the function will crash.
    pub fn update_descriptors(&self, items: &[DescriptorItem]) -> Result<(), AllocatorError> {

        let mut write_infos = Vec::with_capacity(items.len());

        for item in items.iter() {

            let binding_info = &self.configs[item.set_index].bindings[item.binding_index];
            let write_info = binding_info.write_set(&self.sets[item.set_index])?;

            write_infos.push(write_info);
        }

        let write_sets = write_infos.into_iter()
            .map(|info| info.set).collect::<Vec<_>>();

        unsafe {
            self.device.as_ref().unwrap().handle
                .update_descriptor_sets(&write_sets, &[]);
        }

        Ok(())
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

