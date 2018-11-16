
use ash::vk;

use core::device::HaDevice;

use resources::descriptor::layout::{ HaDescriptorSetLayout, DescriptorSetLayoutFlag, DescriptorSetLayoutInfo };
use resources::descriptor::binding::DescriptorBindingInfo;
use resources::descriptor::binding::{ DescriptorBufferBindableTarget, DescriptorImageBindableTarget };
use pipeline::shader::ShaderStageFlag;

use utils::marker::VulkanFlags;

use std::slice::Iter;

pub struct HaDescriptorSet {

    pub(crate) handle: vk::DescriptorSet,
    layout: HaDescriptorSetLayout,
}

impl HaDescriptorSet {

    pub(crate) fn new(handle: vk::DescriptorSet, layout: HaDescriptorSetLayout) -> HaDescriptorSet {

        HaDescriptorSet {
            handle, layout,
        }
    }

    pub fn layout(&self) -> HaDescriptorSetLayout {
        self.layout.clone()
    }

    pub fn cleanup(&self, device: &HaDevice) {
        self.layout.cleanup(device);
    }
}


pub struct DescriptorSetConfig {

    bindings    : Vec<Box<DescriptorBindingInfo>>,
    stage_flags : Vec<vk::ShaderStageFlags>,
    layout_flags: vk::DescriptorSetLayoutCreateFlags,
}

impl DescriptorSetConfig {

    pub fn init(flags: &[DescriptorSetLayoutFlag]) -> DescriptorSetConfig {
        DescriptorSetConfig {
            bindings    : vec![],
            stage_flags : vec![],
            layout_flags: flags.flags(),
        }
    }

    pub fn add_buffer_binding(&mut self, bind_target: &'static impl DescriptorBufferBindableTarget, stages: &[ShaderStageFlag]) {

        let binding_info = bind_target.binding_info(None);
        self.add_binding(Box::new(binding_info), stages);
    }

    pub fn add_image_binding(&mut self, bind_target: &'static impl DescriptorImageBindableTarget, stages: &[ShaderStageFlag]) {

        let binding_info = bind_target.binding_info();
        self.add_binding(Box::new(binding_info), stages);
    }

    fn add_binding(&mut self, binding: Box<DescriptorBindingInfo>, stages: &[ShaderStageFlag]) {

        self.bindings.push(binding);
        self.stage_flags.push(stages.flags());
    }

    pub fn to_layout_info(&self) -> DescriptorSetLayoutInfo {

        let mut layout_info = DescriptorSetLayoutInfo::setup(self.layout_flags);
        for (i, info) in self.bindings.iter().enumerate() {
            layout_info.add_binding(info, self.stage_flags[i]);
        }

        layout_info
    }

    pub fn iter_binding(&self) -> Iter<Box<DescriptorBindingInfo>> {
        self.bindings.iter()
    }
}
