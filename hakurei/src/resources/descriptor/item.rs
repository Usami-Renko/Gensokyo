
use ash::vk;
use ash::vk::uint32_t;

use resources::descriptor::layout::DescriptorType;
use resources::descriptor::layout::DescriptorSetLayoutFlag;
use resources::buffer::BufferItem;

use pipeline::shader::module::ShaderStageFlag;

use utility::marker::VulkanFlags;

pub struct DescriptorBindingInfo {

    /// the binding index used in shader for the descriptor.
    pub binding: uint32_t,
    /// the type of descriptor.
    pub type_  : DescriptorType,
    /// the element count of each descriptor.
    pub count  : uint32_t,
    /// the size of each element of descriptor.
    pub element_size: vk::DeviceSize,
    /// the reference to buffer where the descriptor data stores.
    pub buffer : BufferItem,
}

pub struct DescriptorSetConfig {

    pub(crate) bindings    : Vec<DescriptorBindingInfo>,
    pub(crate) stage_flags : Vec<vk::ShaderStageFlags>,
    pub(crate) layout_flags: vk::DescriptorSetLayoutCreateFlags,
}

impl DescriptorSetConfig {

    pub fn init(flags: &[DescriptorSetLayoutFlag]) -> DescriptorSetConfig {
        DescriptorSetConfig {
            bindings    : vec![],
            stage_flags : vec![],
            layout_flags: flags.flags(),
        }
    }

    pub fn add_binding(&mut self, item: DescriptorBindingInfo, stages: &[ShaderStageFlag]) -> usize {
        let descriptor_index = self.bindings.len();
        self.bindings.push(item);
        self.stage_flags.push(stages.flags());
        descriptor_index
    }
}


#[derive(Debug, Clone)]
pub struct DescriptorItem {

    pub(crate) set_index    : usize,
    pub(crate) binding_index: usize,
}

impl DescriptorItem {

    pub fn unset() -> DescriptorItem {
        DescriptorItem {
            set_index    : 0,
            binding_index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DescriptorSetItem {

    pub(crate) set_index: usize,
}

impl DescriptorSetItem {

    pub fn unset() -> DescriptorSetItem {
        DescriptorSetItem {
            set_index    : 0,
        }
    }
}
