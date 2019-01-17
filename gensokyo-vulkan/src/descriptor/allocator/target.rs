
use ash::vk;

use crate::core::device::GsDevice;

use crate::descriptor::DescriptorSetConfig;
use crate::descriptor::DescriptorPoolInfo;
use crate::descriptor::GsDescriptorType;
use crate::descriptor::allocator::index::IDescriptorSet;
use crate::descriptor::allocator::distributor::GsDescriptorDistributor;

use crate::error::VkResult;
use crate::utils::assign::GsAssignIndex;
use crate::types::vkuint;

use std::collections::HashMap;

pub struct GsDescriptorAllocator {

    device: GsDevice,
    pool_flag: vk::DescriptorPoolCreateFlags,

    set_configs: Vec<DescriptorSetConfig>,
}

impl GsDescriptorAllocator {

    pub fn new(device: &GsDevice, flags: vk::DescriptorPoolCreateFlags) -> GsDescriptorAllocator {

        GsDescriptorAllocator {
            device   : device.clone(),
            pool_flag: flags,

            set_configs: vec![],
        }
    }

    pub fn append_set(&mut self, config: DescriptorSetConfig) -> GsAssignIndex<IDescriptorSet> {

        let set_index = self.set_configs.len();
        let dst_index = GsAssignIndex {
            allot_info: IDescriptorSet {
                set_index,
            },
            assign_index: set_index,
        };

        self.set_configs.push(config);

        dst_index
    }

    pub fn allocate(self) -> VkResult<GsDescriptorDistributor> {

        // descriptor pool
        let pool_sizes = self.pool_sizes();
        let mut pool_info = DescriptorPoolInfo::new(self.pool_flag);

        for pool_size in pool_sizes.iter() {
            pool_info.add_pool_size(pool_size.0, pool_size.1);
        }
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

        let repository = GsDescriptorDistributor::new(self.device, pool, sets, self.set_configs);
        Ok(repository)
    }

    fn pool_sizes(&self) -> Vec<(GsDescriptorType, vkuint)> {

        let mut map = HashMap::new();
        for config in self.set_configs.iter() {
            for info in config.iter_binding() {

                let count = map.entry(info.borrow_binding_content().descriptor_type)
                    .or_insert(0);
                *count += 1;
            }
        }

        let result = map.into_iter().collect();
        result
    }

    pub fn reset(&mut self, flags: vk::DescriptorPoolCreateFlags) {

        self.pool_flag = flags;
        self.set_configs.clear();
    }
}
