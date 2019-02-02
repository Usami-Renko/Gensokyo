
use ash::vk;

use crate::core::GsDevice;

use crate::descriptor::types::GsDescriptorType;
use crate::descriptor::layout::{ GsDescriptorSetLayout, DescriptorSetLayoutCI };
use crate::descriptor::binding::{ DescriptorBindingBufInfo, DescriptorBindingBufTgt };
use crate::descriptor::binding::{ DescriptorBindingImgInfo, DescriptorBindingImgTgt };

use crate::pipeline::target::GsPipelineStage;
use crate::descriptor::binding::DescriptorBindingCI;

use crate::utils::wrapper::VKWrapperInfo;
use crate::types::vkuint;

use std::collections::HashMap;

pub struct GsDescriptorSet {

    pub(crate) handle: vk::DescriptorSet,
    pub(crate) layout: GsDescriptorSetLayout,
}

impl GsDescriptorSet {

    pub(crate) fn new(handle: vk::DescriptorSet, layout: GsDescriptorSetLayout) -> GsDescriptorSet {

        GsDescriptorSet { handle, layout }
    }

    pub fn destroy(&self, device: &GsDevice) {
        self.layout.destroy(device);
    }
}


#[derive(Default)]
pub struct DescriptorSetConfig {

    // flags is reserved for future use in API version 1.1.92.
    layout_flags: vk::DescriptorSetLayoutCreateFlags,

    binding_bufs: Vec<BindingContent<DescriptorBindingBufInfo>>,
    binding_imgs: Vec<BindingContent<DescriptorBindingImgInfo>>,
}

struct BindingContent<T> {
    info: T,
    stage_flags: vk::ShaderStageFlags,
}

impl DescriptorSetConfig {

    pub fn new() -> DescriptorSetConfig {
        DescriptorSetConfig::default()
    }

    pub fn add_buffer_binding(&mut self, bind_target: &impl DescriptorBindingBufTgt, stage: GsPipelineStage) {

        let binding = BindingContent {
            info: bind_target.binding_info(),
            stage_flags: stage.0,
        };
        self.binding_bufs.push(binding);
    }

    pub fn add_image_binding(&mut self, bind_target: &impl DescriptorBindingImgTgt, stage: GsPipelineStage) {

        let binding = BindingContent {
            info: bind_target.binding_info(),
            stage_flags: stage.0,
        };
        self.binding_imgs.push(binding);
    }

    pub fn to_layout_ci(&self) -> DescriptorSetLayoutCI {

        let binding_count = self.binding_bufs.len() + self.binding_imgs.len();
        let mut layout_info = GsDescriptorSetLayout::new(binding_count);
        layout_info.set_flags(self.layout_flags);

        for binding in self.binding_bufs.iter() {
            layout_info.add_binding(&binding.info, binding.stage_flags);
        }

        for binding in self.binding_imgs.iter() {
            layout_info.add_binding(&binding.info, binding.stage_flags);
        }

        layout_info
    }

    pub(super) fn add_pool_size(&self, pool_map: &mut HashMap<GsDescriptorType, vkuint>) {

        for binding in self.binding_bufs.iter() {

            let meta = binding.info.meta_mirror();
            let count = pool_map.entry(meta.descriptor_type)
                .or_insert(0);
            *count += meta.count;
        }

        for binding in self.binding_imgs.iter() {

            let meta = binding.info.meta_mirror();
            let count = pool_map.entry(meta.descriptor_type)
                .or_insert(0);
            *count += meta.count;
        }
    }

    pub(super) fn add_write_set(&self, set: &GsDescriptorSet,
        buffers: &mut VKWrapperInfo<Vec<vk::DescriptorBufferInfo>, vk::WriteDescriptorSet>,
        images : &mut VKWrapperInfo<Vec<vk::DescriptorImageInfo>,  vk::WriteDescriptorSet>
    ) {


        let write_iter = self.binding_bufs.iter()
            .map(|binding| binding.info.write_info(set));
        buffers.extend(write_iter);

        let write_iter = self.binding_imgs.iter()
            .map(|binding| binding.info.write_info(set));
        images.extend(write_iter);
    }

    // TODO: Add configuration for vk::DescriptorSetLayoutCreateFlags.
    pub fn set_flags(&mut self, flags: vk::DescriptorSetLayoutCreateFlags) {
        self.layout_flags = flags;
    }
}


pub struct DescriptorSet {

    pub(crate) handle: vk::DescriptorSet,
    pub(crate) layout: GsDescriptorSetLayout,

    /// `set_index` is the `set` value used in shader code, like the following example shader snippet:
    ///
    /// layout (set = 1, binding = 0) uniform UniformBlock { mat4 projection; }
    set_index: usize,
}

impl DescriptorSet {

    pub fn new(from: &GsDescriptorSet, set_index: usize) -> DescriptorSet {

        // let mut binding_indices = Vec::with_capacity(config.binding_bufs.len() + config.binding_imgs.len());
        //
        // let buf_binding_iter = config.binding_bufs.iter()
        //     .map(|b| b.info.meta.binding);
        // binding_indices.extend(buf_binding_iter);
        //
        // let img_binding_iter = config.binding_imgs.iter()
        //     .map(|b| b.info.meta.binding);
        // binding_indices.extend(img_binding_iter);

        DescriptorSet {
            handle: from.handle,
            layout: from.layout.clone(),
            set_index,
        }
    }

    pub fn set_index(&self) -> usize {
        self.set_index.clone()
    }
}
