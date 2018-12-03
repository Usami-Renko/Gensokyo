
use ash::vk;

use core::device::GsDevice;

use descriptor::layout::{ GsDescriptorSetLayout, DescriptorSetLayoutInfo };
use descriptor::entity::DescriptorSetEntity;
use descriptor::binding::DescriptorBindingInfo;
use descriptor::binding::{ DescriptorBufferBindableTarget, DescriptorImageBindableTarget };

use std::slice::Iter;

pub struct GsDescriptorSet {

    pub(crate) handle: vk::DescriptorSet,
    layout: GsDescriptorSetLayout,
}

impl GsDescriptorSet {

    pub(crate) fn new(handle: vk::DescriptorSet, layout: GsDescriptorSetLayout) -> GsDescriptorSet {

        GsDescriptorSet {
            handle, layout,
        }
    }

    pub fn layout(&self) -> GsDescriptorSetLayout {
        self.layout.clone()
    }

    pub fn cleanup(&self, device: &GsDevice) {
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

    pub fn add_buffer_binding(&mut self, bind_target: &impl DescriptorBufferBindableTarget, stages: vk::ShaderStageFlags) {

        let binding_info = bind_target.binding_info(None);
        self.add_binding(Box::new(binding_info), stages);
    }

    pub fn add_image_binding(&mut self, bind_target: &impl DescriptorImageBindableTarget, stages: vk::ShaderStageFlags) {

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

    pub(crate) entity: DescriptorSetEntity,
    pub(crate) layout: GsDescriptorSetLayout,
    _set_index: usize,
}

impl DescriptorSet {

    pub fn new(from: &GsDescriptorSet, config: &DescriptorSetConfig, set_index: usize) -> DescriptorSet {

        let binding_indices = config.iter_binding()
            .map(|b| b.binding_content().binding)
            .collect();

        DescriptorSet {
            entity: DescriptorSetEntity::new(from, binding_indices),
            layout: from.layout(),
            _set_index: set_index,
        }
    }
}
