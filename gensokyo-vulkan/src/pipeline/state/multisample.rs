
use ash::vk;

use std::ptr;

use crate::types::{ vkuint, vkbool, vkfloat, VK_FALSE, VK_TRUE };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MultisamplePrefab {
    /// Disable multisample configuration.
    Disable,
}

impl  MultisamplePrefab {

    fn generate(&self) -> GsMultisampleState {
        match self {
            | MultisamplePrefab::Disable => GsMultisampleState { ..Default::default() },
        }
    }
}

pub struct GsMultisampleState {

    /// The number of samples per pixel used in rasterization.
    sample_count  : vk::SampleCountFlags,
    /// Sample shading can be used to specify a minimum number of unique samples to process for each fragment.
    sample_shading: SampleShading,
    /// Controls whether a temporary coverage value is generated based on the alpha component of the fragment’s first color output.
    alpha_to_coverage_enable: vkbool,
    /// Controls whether the alpha component of the fragment’s first color output is replaced with one.
    alpha_to_one_enalbe     : vkbool,
}

impl GsMultisampleState {

    pub fn setup(prefab: MultisamplePrefab) -> GsMultisampleState {
        prefab.generate()
    }

    pub(crate) fn info(&self) -> vk::PipelineMultisampleStateCreateInfo {

        vk::PipelineMultisampleStateCreateInfo {
            s_type : vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags  : vk::PipelineMultisampleStateCreateFlags::empty(),

            rasterization_samples    : self.sample_count,
            sample_shading_enable    : self.sample_shading.enable,
            min_sample_shading       : self.sample_shading.min_sample,
            p_sample_mask            : &self.sample_shading.sample_masks,
            alpha_to_coverage_enable : self.alpha_to_coverage_enable,
            alpha_to_one_enable      : self.alpha_to_one_enalbe,
        }
    }

    pub fn set_sample_count(&mut self, count: vk::SampleCountFlags) {
        self.sample_count = count;
    }
    pub fn set_sample_shading(&mut self, sample_shading: SampleShading) {
        self.sample_shading = sample_shading;
    }
    pub fn set_alpha_to_coverage_enable(&mut self, enable: bool) {
        self.alpha_to_coverage_enable = if enable { VK_TRUE } else { VK_FALSE };
    }
    pub fn set_alpha_to_one_enalbe(&mut self, enable: bool) {
        self.alpha_to_one_enalbe = if enable { VK_TRUE } else { VK_FALSE };
    }
}

impl Default for GsMultisampleState {

    fn default() -> GsMultisampleState {
        GsMultisampleState {
            sample_count   : vk::SampleCountFlags::TYPE_1,
            sample_shading : SampleShading::disable(),
            alpha_to_coverage_enable : VK_FALSE,
            alpha_to_one_enalbe      : VK_FALSE,
        }
    }
}


pub struct SampleShading {

    enable      : vkbool,
    min_sample  : vkfloat,
    sample_masks: vkuint,
}

impl SampleShading {

    pub fn disable() -> SampleShading {
        SampleShading {
            enable       : VK_FALSE,
            min_sample   : 0.0,
            sample_masks : 0,
        }
    }

    pub fn setup(min_sample: vkfloat, sample_masks: vkuint) -> SampleShading {
        SampleShading { enable: VK_TRUE, min_sample, sample_masks, }
    }
}
