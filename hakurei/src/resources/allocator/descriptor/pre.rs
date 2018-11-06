
use ash::vk;
use ash::vk::uint32_t;

use core::device::HaDevice;

use resources::allocator::HaDescriptorDistributor;
use resources::descriptor::DescriptorSetConfig;
use resources::descriptor::{ DescriptorSetIndex, DescriptorSetLayoutInfo };
use resources::descriptor::{ DescriptorPoolInfo, DescriptorPoolFlag };
use resources::error::AllocatorError;

use utility::marker::VulkanFlags;

use std::collections::HashMap;

pub struct HaDescriptorPreAllocator {

    device: HaDevice,
    pool_flag: vk::DescriptorPoolCreateFlags,

    set_configs: Vec<DescriptorSetConfig>,
}

impl HaDescriptorPreAllocator {

    pub(crate) fn new(device: &HaDevice, flags: &[DescriptorPoolFlag]) -> HaDescriptorPreAllocator {

        HaDescriptorPreAllocator {
            device   : device.clone(),
            pool_flag: flags.flags(),

            set_configs: vec![],
        }
    }

    pub fn append_set(&mut self, config: DescriptorSetConfig) -> DescriptorSetIndex {

        let index = DescriptorSetIndex(self.set_configs.len());
        self.set_configs.push(config);

        index
    }

    pub fn allocate(self) -> Result<HaDescriptorDistributor, AllocatorError> {

        // descriptor pool
        let pool_sizes = self.pool_sizes();
        let mut pool_info = DescriptorPoolInfo::new(self.pool_flag);
        pool_sizes.iter().for_each(|pool_size| {
            pool_info.add_pool_size(pool_size.0, pool_size.1);
        });
        let pool = pool_info.build(&self.device)?;

        // descriptor layout
        let mut layouts = vec![];
        for config in self.set_configs.iter() {
            let mut layout_info = DescriptorSetLayoutInfo::setup(config.layout_flags);
            for (i, info) in config.bindings.iter().enumerate() {
                layout_info.add_binding(info, config.stage_flags[i]);
            }
            let layout = layout_info.build(&self.device)?;
            layouts.push(layout);
        }

        // descriptor sets
        let sets = pool.allocate(&self.device, layouts)?;

        let repository = HaDescriptorDistributor::new(self.device, pool, sets, self.set_configs);
        Ok(repository)
    }

    fn pool_sizes(&self) -> Vec<(vk::DescriptorType, uint32_t)> {

        let mut map = HashMap::new();
        for config in self.set_configs.iter() {
            for info in config.bindings.iter() {

                let count = map.entry(info.descriptor_type())
                    .or_insert(0 as uint32_t);
                *count += 1;
            }
        }

        let result = map.into_iter().collect();
        result
    }

    pub fn reset(&mut self, flags: &[DescriptorPoolFlag]) {

        self.pool_flag = flags.flags();
        self.set_configs.clear();
    }
}
