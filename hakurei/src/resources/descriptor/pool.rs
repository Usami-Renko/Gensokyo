
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::descriptor::set::HaDescriptorSet;
use resources::descriptor::layout::HaDescriptorSetLayout;
use resources::error::DescriptorError;

use utility::marker::{ VulkanFlags, Handles };

use std::ptr;

pub(crate) struct DescriptorPoolInfo {

    flags     : vk::DescriptorPoolCreateFlags,
    max_sets  : uint32_t,
    pool_sizes: Vec<vk::DescriptorPoolSize>,
}

impl DescriptorPoolInfo {

    pub fn new(flags: vk::DescriptorPoolCreateFlags) -> DescriptorPoolInfo {
        DescriptorPoolInfo {
            flags,
            max_sets  : 0,
            pool_sizes: vec![],
        }
    }

    pub fn _set_pool_size_max(&mut self, max_size: uint32_t) {
        self.max_sets = max_size;
    }
    pub fn add_pool_size(&mut self, desc_type: vk::DescriptorType, count: uint32_t) {
        self.pool_sizes.push(vk::DescriptorPoolSize {
            typ: desc_type,
            descriptor_count: count,
        });
    }

    pub fn build(&self, device: &HaDevice) -> Result<HaDescriptorPool, DescriptorError> {
        let max_sets = if self.max_sets == 0 { self.pool_sizes.len() as uint32_t } else { self.max_sets };

        let info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DescriptorPoolCreateInfo,
            p_next: ptr::null(),
            flags : self.flags,
            max_sets,
            pool_size_count: self.pool_sizes.len() as uint32_t,
            p_pool_sizes   : self.pool_sizes.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_descriptor_pool(&info, None)
                .or(Err(DescriptorError::PoolCreationError))?
        };

        let descriptor_pool = HaDescriptorPool {
            handle,
        };
        Ok(descriptor_pool)
    }
}

pub(crate) struct HaDescriptorPool {

    handle: vk::DescriptorPool,
}

impl HaDescriptorPool {

    pub fn uninitialize() -> HaDescriptorPool {
        HaDescriptorPool {
            handle: vk::DescriptorPool::null(),
        }
    }

    pub fn allocator(&self, device: &HaDevice, layouts: Vec<HaDescriptorSetLayout>)
        -> Result<Vec<HaDescriptorSet>, DescriptorError> {
        let handles = layouts.handles();

        let allocate_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DescriptorSetAllocateInfo,
            p_next: ptr::null(),
            descriptor_pool: self.handle,
            descriptor_set_count: handles.len() as uint32_t,
            p_set_layouts       : handles.as_ptr(),
        };

        let handles = unsafe {
            device.handle.allocate_descriptor_sets(&allocate_info)
                .or(Err(DescriptorError::SetAllocateError))?
        };

        let mut sets = vec![];

        for (index, layout) in layouts.into_iter().enumerate() {
            let set = HaDescriptorSet {
                handle: handles[index],
                layout,
            };
            sets.push(set);
        }
        Ok(sets)
    }

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_descriptor_pool(self.handle, None);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DescriptorPoolFlag {
    /// FreeDescriptorSetBit specifies that descriptor sets can return their individual allocations to the pool.
    ///
    /// i.e. all of vkAllocateDescriptorSets, vkFreeDescriptorSets, and vkResetDescriptorPool are allowed.
    ///
    /// Otherwise, descriptor sets allocated from the pool must not be individually freed back to the pool.
    ///
    /// i.e. only vkAllocateDescriptorSets and vkResetDescriptorPool are allowed.
    FreeDescriptorSetBit,
}

impl VulkanFlags for [DescriptorPoolFlag] {
    type FlagType = vk::DescriptorPoolCreateFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::DescriptorPoolCreateFlags::empty(), |acc, flag| {
            match *flag {
                | DescriptorPoolFlag::FreeDescriptorSetBit => acc | vk::DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT,
            }
        })
    }
}
