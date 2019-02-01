
use ash::vk;

use crate::core::GsDevice;

use crate::descriptor::DescriptorSetConfig;
use crate::descriptor::GsDescriptorPool;
use crate::descriptor::GsDescriptorType;
use crate::descriptor::allocator::distributor::GsDescriptorDistributor;
use crate::descriptor::allocator::index::IDescriptorSet;

use crate::error::VkResult;
use crate::utils::allot::{ GsAssignIndex, GsAllocatorApi, GsAllotIntoDistributor };
use crate::types::vkuint;

use std::collections::HashMap;

pub struct GsDescriptorAllocator {

    device: GsDevice,
    pool_flag: vk::DescriptorPoolCreateFlags,

    set_configs: Vec<DescriptorSetConfig>,
    update_sets: Vec<usize>,
}

impl GsAllocatorApi<DescriptorSetConfig, IDescriptorSet, GsDescriptorDistributor> for GsDescriptorAllocator {
    type AssignResult = GsAssignIndex<IDescriptorSet>;

    fn assign(&mut self, config: DescriptorSetConfig) -> Self::AssignResult {

        let set_index = self.set_configs.len();
        let dst_index = GsAssignIndex {
            convey_info: IDescriptorSet {
                set_index,
            },
            assign_index: set_index,
        };

        self.set_configs.push(config);
        self.update_sets.push(set_index);

        dst_index
    }
}

impl GsAllotIntoDistributor<GsDescriptorDistributor> for GsDescriptorAllocator {

    fn allocate(self) -> VkResult<GsDescriptorDistributor> {

        // descriptor pool
        let pool_sizes = self.pool_sizes();
        let mut pool_info = GsDescriptorPool::new(self.pool_flag);

        for pool_size in pool_sizes.iter() {
            pool_info.add_pool_size(pool_size.0, pool_size.1);
        }
        let pool = pool_info.build(&self.device)?;

        // descriptor layout
        let mut layouts = Vec::with_capacity(self.set_configs.len());
        for config in self.set_configs.iter() {
            let layout_info = config.to_layout_ci();
            let layout = layout_info.build(&self.device)?;
            layouts.push(layout);
        }

        // descriptor sets
        let sets = pool.allocate(&self.device, layouts)?;

        let repository = GsDescriptorDistributor::new(self.device, pool, sets, self.set_configs, self.update_sets);
        Ok(repository)
    }

    fn reset(&mut self) {
        self.set_configs.clear();
    }
}

impl GsDescriptorAllocator {

    pub fn create(device: &GsDevice) -> GsDescriptorAllocator {

        GsDescriptorAllocator {
            device   : device.clone(),
            pool_flag: vk::DescriptorPoolCreateFlags::empty(),

            set_configs: vec![],
            update_sets: vec![],
        }
    }

    pub fn with_flags(&mut self, flags: vk::DescriptorPoolCreateFlags) {
        self.pool_flag = flags;
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
}
