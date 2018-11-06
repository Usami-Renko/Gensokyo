
use ash::vk;

use core::device::HaDevice;

use resources::descriptor::HaDescriptorSetLayout;
use resources::descriptor::{ DescriptorBindingInfo, DescriptorSetLayoutFlag };
use resources::descriptor::{ DescriptorBufferBindableTarget, DescriptorImageBindableTarget };
use pipeline::shader::ShaderStageFlag;

use utility::marker::VulkanFlags;

pub(crate) struct HaDescriptorSet {

    pub(crate) handle: vk::DescriptorSet,
    pub(crate) layout: HaDescriptorSetLayout,
}

impl HaDescriptorSet {

    pub(crate) fn cleanup(&self, device: &HaDevice) {
        self.layout.cleanup(device);
    }
}


pub struct DescriptorSetConfig {

    pub(crate) bindings    : Vec<Box<DescriptorBindingInfo>>,
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

    pub fn add_buffer_binding(&mut self, bind_target: &impl DescriptorBufferBindableTarget, stages: &[ShaderStageFlag]) {

        let binding_info = bind_target.binding_info(None);
        self.add_binding(Box::new(binding_info), stages);
    }

    pub fn add_image_binding(&mut self, bind_target: &impl DescriptorImageBindableTarget, stages: &[ShaderStageFlag]) {

        let binding_info = bind_target.binding_info();
        self.add_binding(Box::new(binding_info), stages);
    }

    fn add_binding(&mut self, binding: Box<DescriptorBindingInfo>, stages: &[ShaderStageFlag]) {

        self.bindings.push(binding);
        self.stage_flags.push(stages.flags());
    }
}
