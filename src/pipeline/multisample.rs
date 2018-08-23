
use ash::vk;
use ash::vk::{ Bool32, uint32_t };

use std::ptr;
use std::os::raw::c_float;

pub struct HaMultisample {

    /// The number of samples per pixel used in rasterization.
    sample_count: SampleCountType,
    /// Sample shading can be used to specify a minimum number of unique samples to process for each fragment.
    sample_shading: SampleShading,
    /// Controls whether a temporary coverage value is generated based on the alpha component of the fragment’s first color output.
    alpha_to_coverage_enable : Bool32,
    /// Controls whether the alpha component of the fragment’s first color output is replaced with one.
    alpha_to_one_enalbe      : Bool32,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MultisamplePrefab {
    /// Disable multisample configuration.
    Disable,
}

impl MultisamplePrefab {
    fn generate(&self) -> HaMultisample {
        match *self {
            | MultisamplePrefab::Disable => HaMultisample { ..Default::default() },
        }
    }
}

impl HaMultisample {

    pub fn init() -> HaMultisample {
        HaMultisample { ..Default::default() }
    }

    pub fn setup(prefab: MultisamplePrefab) -> HaMultisample {
        prefab.generate()
    }

    #[inline]
    pub fn info(&self) -> vk::PipelineMultisampleStateCreateInfo {
        vk::PipelineMultisampleStateCreateInfo {
            s_type : vk::StructureType::PipelineMultisampleStateCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags  : vk::PipelineMultisampleStateCreateFlags::empty(),

            rasterization_samples : self.sample_count.flag(),
            sample_shading_enable : self.sample_shading.enable,
            min_sample_shading    : self.sample_shading.min_sample,
            p_sample_mask         : &self.sample_shading.sample_masks,
            alpha_to_coverage_enable : self.alpha_to_coverage_enable,
            alpha_to_one_enable      : self.alpha_to_one_enalbe,
        }
    }

    pub fn set_sample_count(&mut self, count: SampleCountType) {
        self.sample_count = count;
    }
    pub fn set_sample_shading(&mut self, sample_shading: SampleShading) {
        self.sample_shading = sample_shading;
    }
    pub fn set_alpha_to_coverage_enable(&mut self, enable: bool) {
        self.alpha_to_coverage_enable = if enable { 1 } else { 0 };
    }
    pub fn set_alpha_to_one_enalbe(&mut self, enable: bool) {
        self.alpha_to_one_enalbe = if enable { 1 } else { 0 };
    }
}

impl Default for HaMultisample {

    fn default() -> HaMultisample {
        HaMultisample {
            sample_count   : SampleCountType::Count1Bit,
            sample_shading : SampleShading::disable(),
            alpha_to_coverage_enable : vk::VK_FALSE,
            alpha_to_one_enalbe      : vk::VK_FALSE,
        }
    }
}


pub struct SampleShading {
    enable       : Bool32,
    min_sample   : c_float,
    sample_masks : uint32_t,
}

impl SampleShading {

    pub fn disable() -> SampleShading {
        SampleShading {
            enable       : vk::VK_FALSE,
            min_sample   : 0.0,
            sample_masks : 0,
        }
    }

    pub fn setup(min_sample: c_float, sample_masks: uint32_t) -> SampleShading {
        SampleShading { enable: vk::VK_TRUE, min_sample, sample_masks, }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SampleCountType {
    Count1Bit, Count2Bit, Count4Bit, Count8Bit, Count16Bit, Count32Bit, Count64Bit,
}
impl SampleCountType {
    fn flag(&self) -> vk::SampleCountFlags {
        match *self {
            | SampleCountType::Count1Bit  => vk::SAMPLE_COUNT_1_BIT,
            | SampleCountType::Count2Bit  => vk::SAMPLE_COUNT_2_BIT,
            | SampleCountType::Count4Bit  => vk::SAMPLE_COUNT_4_BIT,
            | SampleCountType::Count8Bit  => vk::SAMPLE_COUNT_8_BIT,
            | SampleCountType::Count16Bit => vk::SAMPLE_COUNT_16_BIT,
            | SampleCountType::Count32Bit => vk::SAMPLE_COUNT_32_BIT,
            | SampleCountType::Count64Bit => vk::SAMPLE_COUNT_64_BIT,
        }
    }
}
