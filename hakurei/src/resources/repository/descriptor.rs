
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::descriptor::HaDescriptorPool;
use resources::descriptor::{ DescriptorItem, DescriptorSetItem };
use resources::descriptor::{ DescriptorType, HaDescriptorSetLayout };
use resources::descriptor::{ DescriptorSetConfig, HaDescriptorSet };
use resources::repository::HaBufferRepository;

use utility::marker::VulkanEnum;

use std::ptr;

pub struct CmdDescriptorBindingInfos {

    pub(crate) handles: Vec<vk::DescriptorSet>,
}


pub struct HaDescriptorRepository {

    pool   : HaDescriptorPool,
    sets   : Vec<HaDescriptorSet>,
    configs: Vec<DescriptorSetConfig>,
}

impl HaDescriptorRepository {

    pub fn empty() -> HaDescriptorRepository {
        HaDescriptorRepository {
            pool   : HaDescriptorPool::uninitialize(),
            sets   : vec![],
            configs: vec![],
        }
    }

    pub(crate) fn store(pool: HaDescriptorPool, sets: Vec<HaDescriptorSet>, configs: Vec<DescriptorSetConfig>)
        -> HaDescriptorRepository {

        HaDescriptorRepository {
            pool,
            sets,
            configs,
        }
    }

    // TODO: Currently only support descriptors in the same Buffer Repository.
    pub fn update_descriptors(&self, device: &HaLogicalDevice, buffer_repository: &HaBufferRepository, items: &[DescriptorItem]) {

        let mut write_sets = vec![];

        for item in items.iter() {

            let binding_info = &self.configs[item.set_index].bindings[item.binding_index];
            let buffer = &binding_info.buffer;

            let write_set = match binding_info.type_ {
                | DescriptorType::Sampler
                | DescriptorType::SampledImage
                | DescriptorType::CombinedImageSampler
                | DescriptorType::StorageImage
                | DescriptorType::InputAttachment => {
                    unimplemented!()
                },
                | DescriptorType::UniformBuffer
                | DescriptorType::UniformBufferDynamic
                | DescriptorType::StorageBuffer
                | DescriptorType::StorageBufferDynamic => {
                    let mut buffer_infos = vec![];
                    for i in 0..(binding_info.count as vk::DeviceSize) {
                        let buffer_info = vk::DescriptorBufferInfo {
                            // buffer is the buffer resource.
                            buffer: buffer_repository.buffer_at(buffer.buffer_index).handle,
                            // offset is the offset in bytes from the start of buffer. Access to buffer memory via this descriptor uses addressing that is relative to this starting offset.
                            offset: buffer.offset + i * binding_info.element_size,
                            // range is the size in bytes that is used for this descriptor update, or VK_WHOLE_SIZE to use the range from offset to the end of the buffer.
                            range : binding_info.element_size,
                        };
                        buffer_infos.push(buffer_info);
                    }

                    vk::WriteDescriptorSet {
                        s_type              : vk::StructureType::WriteDescriptorSet,
                        p_next              : ptr::null(),
                        dst_set             : self.sets[item.set_index].handle,
                        dst_binding         : binding_info.binding,
                        // TODO: Currently dst_array_element filed is not configurable
                        dst_array_element   : 0,
                        descriptor_count    : binding_info.count,
                        descriptor_type     : binding_info.type_.value(),
                        p_image_info        : ptr::null(),
                        p_buffer_info       : buffer_infos.as_ptr(),
                        p_texel_buffer_view : ptr::null(),
                    }
                },
                | DescriptorType::UniformTexelBuffer
                | DescriptorType::StorageTexelBuffer => {
                    unimplemented!()
                }
            };

            write_sets.push(write_set);
        }

        unsafe {
            device.handle.update_descriptor_sets(&write_sets, &[]);
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

    pub fn clean(&self, device: &HaLogicalDevice) {
        self.pool.clean(device);
        for set in self.sets.iter() {
            set.cleanup(device);
        }
    }
}

