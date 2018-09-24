
use ash::vk;
use ash::vk::uint32_t;

use core::device::HaDevice;

use resources::descriptor::{ DescriptorSetConfig, DescriptorItem, DescriptorSetItem };
use resources::descriptor::{ DescriptorSetLayoutInfo };
use resources::descriptor::{ DescriptorPoolInfo, DescriptorPoolFlag };
use resources::repository::HaDescriptorRepository;
use resources::error::DescriptorError;

use utility::marker::VulkanFlags;

use std::collections::HashMap;

pub struct HaDescriptorAllocator {

    device: HaDevice,
    pool_flag: vk::DescriptorPoolCreateFlags,

    set_configs: Vec<DescriptorSetConfig>,
}

impl HaDescriptorAllocator {

    pub(crate) fn new(device: &HaDevice, flags: &[DescriptorPoolFlag]) -> HaDescriptorAllocator {

        HaDescriptorAllocator {
            device: device.clone(),
            pool_flag: flags.flags(),

            set_configs: vec![],
        }
    }

    pub fn attach_descriptor_set(&mut self, config: DescriptorSetConfig) -> (DescriptorSetItem, Vec<DescriptorItem>) {
        let set_index = self.set_configs.len();

        let mut items = vec![];
        for i in 0..config.bindings.len() {
            items.push(DescriptorItem {
                set_index,
                binding_index: i
            });
        }
        let set = DescriptorSetItem {
            set_index,
        };

        self.set_configs.push(config);

        (set, items)
    }

    pub fn allocate(&mut self) -> Result<HaDescriptorRepository, DescriptorError> {

        // descriptor pool
        let pool_sizes = self.pool_sizes();
        let mut pool_info = DescriptorPoolInfo::new(self.pool_flag);
        pool_sizes.iter().for_each(|&pool_size| {
            pool_info.add_pool_size(pool_size.0, pool_size.1);
        });
        let pool = pool_info.build(&self.device)?;

        // descriptor layout
        let mut layouts = vec![];
        for (i, config) in self.set_configs.iter().enumerate() {
            let mut layout_info = DescriptorSetLayoutInfo::setup(config.layout_flags);
            for info in config.bindings.iter() {
                layout_info.add_binding(info, config.stage_flags[i]);
            }
            let layout = layout_info.build(&self.device)?;
            layouts.push(layout);
        }

        // descriptor sets
        let sets = pool.allocator(&self.device, layouts)?;
        let configs = self.set_configs.drain(..).collect();

        let repository = HaDescriptorRepository::store(&self.device, pool, sets, configs);
        Ok(repository)
    }

    fn pool_sizes(&self) -> Vec<(vk::DescriptorType, uint32_t)> {

        let mut map = HashMap::new();
        for config in self.set_configs.iter() {
            for info in config.bindings.iter() {

                let count = map.entry(info.descriptor_type()).or_insert(0 as uint32_t);
                *count += 1;
            }
        }


        let result = map.drain().collect();
        result
    }

    pub fn reset(&mut self, flags: &[DescriptorPoolFlag]) {
        self.pool_flag = flags.flags();
        self.set_configs.clear();
    }
}

