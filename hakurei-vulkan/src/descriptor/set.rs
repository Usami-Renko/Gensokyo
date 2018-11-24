
use ash::vk;

use core::device::HaDevice;

use descriptor::layout::{ HaDescriptorSetLayout, DescriptorSetLayoutInfo };
use descriptor::entity::DescriptorSetEntity;
use descriptor::binding::DescriptorBindingInfo;
use descriptor::binding::{ DescriptorBufferBindableTarget, DescriptorImageBindableTarget };

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


#[derive(Default)]
pub struct DescriptorSetConfig {

    layout_flags: vk::DescriptorSetLayoutCreateFlags,

    bindings    : Vec<Box<dyn DescriptorBindingInfo>>,
    stage_flags : Vec<vk::ShaderStageFlags>,
}

impl DescriptorSetConfig {

    pub fn init(layout_flags: vk::DescriptorSetLayoutCreateFlags) -> DescriptorSetConfig {
        DescriptorSetConfig {
            layout_flags,
            ..Default::default()
        }
    }

    pub fn add_buffer_binding(&mut self, bind_target: &'static impl DescriptorBufferBindableTarget, stages: vk::ShaderStageFlags) {

        let binding_info = bind_target.binding_info(None);
        self.add_binding(Box::new(binding_info), stages);
    }

    pub fn add_image_binding(&mut self, bind_target: &'static impl DescriptorImageBindableTarget, stages: vk::ShaderStageFlags) {

        let binding_info = bind_target.binding_info();
        self.add_binding(Box::new(binding_info), stages);
    }

    fn add_binding(&mut self, binding: Box<dyn DescriptorBindingInfo>, stages: vk::ShaderStageFlags) {

        self.bindings.push(binding);
        self.stage_flags.push(stages);
    }

    pub fn to_layout_info(&self) -> DescriptorSetLayoutInfo {

        let mut layout_info = DescriptorSetLayoutInfo::setup(self.layout_flags);
        for (i, info) in self.bindings.iter().enumerate() {
            layout_info.add_binding(info, self.stage_flags[i]);
        }

        layout_info
    }

    pub fn iter_binding(&self) -> Iter<Box<dyn DescriptorBindingInfo>> {
        self.bindings.iter()
    }
}


pub struct DescriptorSet {

    entity: DescriptorSetEntity,
    layout: HaDescriptorSetLayout,
    set_index: usize,
}

impl DescriptorSet {

    pub fn new(from: &HaDescriptorSet, config: &DescriptorSetConfig, set_index: usize) -> DescriptorSet {

        let binding_indices = config.iter_binding()
            .map(|b| b.binding_content().binding)
            .collect();

        DescriptorSet {
            entity: DescriptorSetEntity::new(from, binding_indices),
            layout: from.layout(),
            set_index,
        }
    }
}
