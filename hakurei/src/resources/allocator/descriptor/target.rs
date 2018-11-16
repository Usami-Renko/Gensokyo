
use vk::core::device::HaDevice;

use vk::resources::descriptor::{ DescriptorSetConfig, DescriptorSetLayoutInfo };
use vk::resources::descriptor::{ DescriptorPoolInfo, DescriptorPoolFlag };
use vk::resources::descriptor::HaDescriptorType;
use vk::resources::error::AllocatorError;
use vk::utils::types::vkint;

use resources::allocator::descriptor::index::DescriptorSetIndex;
use resources::allocator::descriptor::HaDescriptorDistributor;

use std::collections::HashMap;

pub struct HaDescriptorAllocator {

    device: HaDevice,
    pool_flag: Vec<DescriptorPoolFlag>,

    set_configs: Vec<DescriptorSetConfig>,
}

impl HaDescriptorAllocator {

    pub(crate) fn new(device: &HaDevice, flags: &[DescriptorPoolFlag]) -> HaDescriptorAllocator {

        HaDescriptorAllocator {
            device   : device.clone(),
            pool_flag: flags.into(),

            set_configs: vec![],
        }
    }

    pub fn append_set(&mut self, config: DescriptorSetConfig) -> DescriptorSetIndex {

        let index = DescriptorSetIndex {
            value: self.set_configs.len(),
        };
        self.set_configs.push(config);

        index
    }

    pub fn allocate(self) -> Result<HaDescriptorDistributor, AllocatorError> {

        // descriptor pool
        let pool_sizes = self.pool_sizes();
        let mut pool_info = DescriptorPoolInfo::new(&self.pool_flag);

        pool_sizes.iter().for_each(|pool_size| {
            pool_info.add_pool_size(pool_size.0, pool_size.1);
        });
        let pool = pool_info.build(&self.device)?;

        // descriptor layout
        let mut layouts = vec![];
        for config in self.set_configs.iter() {
            let layout_info = config.to_layout_info();
            let layout = layout_info.build(&self.device)?;
            layouts.push(layout);
        }

        // descriptor sets
        let sets = pool.allocate(&self.device, layouts)?;

        let repository = HaDescriptorDistributor::new(self.device, pool, sets, self.set_configs);
        Ok(repository)
    }

    fn pool_sizes(&self) -> Vec<(HaDescriptorType, vkint)> {

        let mut map = HashMap::new();
        for config in self.set_configs.iter() {
            for info in config.iter_binding() {

                let count = map.entry(info.binding_content().descriptor_type)
                    .or_insert(0 as vkint);
                *count += 1;
            }
        }

        let result = map.into_iter().collect();
        result
    }

    pub fn reset(&mut self, flags: &[DescriptorPoolFlag]) {

        self.pool_flag = flags.into();
        self.set_configs.clear();
    }
}
