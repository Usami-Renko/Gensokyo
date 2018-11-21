
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use descriptor::set::HaDescriptorSet;
use descriptor::layout::HaDescriptorSetLayout;
use descriptor::HaDescriptorType;
use descriptor::error::DescriptorError;

use types::vkuint;

use std::ptr;

#[derive(Default)]
pub struct DescriptorPoolInfo {

    flags     : vk::DescriptorPoolCreateFlags,
    pool_sizes: Vec<vk::DescriptorPoolSize>,
    max_sets  : vkuint,
}

impl DescriptorPoolInfo {

    // TODO: Add configuration for vk::DescriptorPoolCreateFlags.
    pub fn new(flags: vk::DescriptorPoolCreateFlags) -> DescriptorPoolInfo {

        DescriptorPoolInfo {
            flags,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn set_pool_size_max(&mut self, max_size: vkuint) {
        self.max_sets = max_size;
    }

    pub fn add_pool_size(&mut self, desc_type: HaDescriptorType, count: vkuint) {

        self.pool_sizes.push(vk::DescriptorPoolSize {
            ty: desc_type.to_raw(),
            descriptor_count: count,
        });
    }

    pub fn build(&self, device: &HaDevice) -> Result<HaDescriptorPool, DescriptorError> {

        let max_sets = if self.max_sets == 0 { self.pool_sizes.len() as vkuint } else { self.max_sets };

        let info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags : self.flags,
            max_sets,
            pool_size_count: self.pool_sizes.len() as vkuint,
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

#[derive(Default)]
pub struct HaDescriptorPool {

    handle: vk::DescriptorPool,
}

impl HaDescriptorPool {

    pub fn allocate(&self, device: &HaDevice, layouts: Vec<HaDescriptorSetLayout>) -> Result<Vec<HaDescriptorSet>, DescriptorError> {

        let layout_handles: Vec<vk::DescriptorSetLayout> = collect_handle!(layouts);

        let allocate_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool: self.handle,
            descriptor_set_count: layout_handles.len() as vkuint,
            p_set_layouts       : layout_handles.as_ptr(),
        };

        let handles = unsafe {
            device.handle.allocate_descriptor_sets(&allocate_info)
                .or(Err(DescriptorError::SetAllocateError))?
        };

        let sets = layouts.into_iter().zip(handles.into_iter())
            .map(|(layout, handle)|
                HaDescriptorSet::new(handle, layout)
        ).collect();

        Ok(sets)
    }

    pub fn cleanup(&self, device: &HaDevice) {

        unsafe {
            device.handle.destroy_descriptor_pool(self.handle, None);
        }
    }
}
