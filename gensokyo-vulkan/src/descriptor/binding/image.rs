
use ash::vk;

use crate::descriptor::set::GsDescriptorSet;
use crate::descriptor::binding::traits::{ DescriptorMeta, DescriptorArrayMeta };
use crate::descriptor::binding::traits::{ DescriptorBindingCI, DescriptorMetaMirror };

use crate::utils::wrapper::VKWrapperPair;

use std::ptr;

/// Descriptor image binding target.
pub trait DescriptorBindingImgTgt {

    fn binding_info(&self) -> DescriptorBindingImgInfo;
}

/// The information to generate vk::DescriptorImgInfo.
pub struct DescriptorBindingImgInfo {

    pub meta: DescriptorMeta,
    /// the handle of sampler.
    pub sampler_handle: vk::Sampler,
    /// the handle of image view where the descriptor data stores.
    pub view_handle: vk::ImageView,
    /// what the layout is for this descriptor in shader.
    pub dst_layout: vk::ImageLayout,
}

impl DescriptorBindingCI for DescriptorBindingImgInfo {
    type DescriptorWriteType = VKWrapperPair<Vec<vk::DescriptorImageInfo>, vk::WriteDescriptorSet>;

    fn meta_mirror(&self) -> DescriptorMetaMirror {
        self.meta.clone().into()
    }

    fn write_info(&self, set: &GsDescriptorSet) -> Self::DescriptorWriteType {

        let contents = vec![
            vk::DescriptorImageInfo {
                sampler     : self.sampler_handle,
                image_view  : self.view_handle,
                image_layout: self.dst_layout,
            },
        ];

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.meta.binding,
            dst_array_element  : 0,
            descriptor_count   : 1,
            descriptor_type    : self.meta.descriptor_type.into(),
            p_image_info       : contents.as_ptr(),
            p_buffer_info      : ptr::null(),
            p_texel_buffer_view: ptr::null(),
        };

        VKWrapperPair {
            content: contents,
            info   : write_set,
        }
    }
}

/// Array version of DescriptorBindingImgTgt.
pub trait DescriptorBindingImgArrayTgt {

    fn binding_info(&self) -> DescriptorBindingImgArrayInfo;
}

/// Array version of DescriptorBindingImgTgt.
pub struct DescriptorBindingImgArrayInfo {

    pub meta: DescriptorArrayMeta,
    pub content: BindingImgArrayContent,
}

pub enum BindingImgArrayContent {

    MultiSamplers {
        /// the handle of multiple sampler.
        /// The count of `samplers` should be equal to `content.count` in `DescriptorBindingArray`.
        samplers: Vec<vk::Sampler>,
        /// the handle of image view where the descriptor data stores.
        view_handle: vk::ImageView,
        /// what the layout is for this descriptor in shader.
        dst_layout: vk::ImageLayout,
    },
}

impl DescriptorBindingCI for DescriptorBindingImgArrayInfo {
    type DescriptorWriteType = VKWrapperPair<Vec<vk::DescriptorImageInfo>, vk::WriteDescriptorSet>;

    fn meta_mirror(&self) -> DescriptorMetaMirror {
        self.meta.clone().into()
    }

    fn write_info(&self, set: &GsDescriptorSet) -> Self::DescriptorWriteType {

        let contents: Vec<vk::DescriptorImageInfo> = match &self.content {
            | BindingImgArrayContent::MultiSamplers { samplers, view_handle, dst_layout } => {

                debug_assert_eq!(samplers.len(), self.meta.count as usize);

                samplers.iter().map(|sampler_handle| {
                    vk::DescriptorImageInfo {
                        sampler     : sampler_handle.clone(),
                        image_view  : view_handle.clone(),
                        image_layout: dst_layout.clone(),
                    }
                }).collect()
            },
        };

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.meta.binding,
            // TODO: Currently dst_array_element filed is not configurable.
            dst_array_element  : 0,
            descriptor_count   : self.meta.count,
            descriptor_type    : self.meta.descriptor_type.into(),
            p_image_info       : contents.as_ptr(),
            p_buffer_info      : ptr::null(),
            p_texel_buffer_view: ptr::null(),
        };

        VKWrapperPair {
            content: contents,
            info   : write_set,
        }
    }
}

