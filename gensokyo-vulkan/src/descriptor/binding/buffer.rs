
use ash::vk;

use crate::descriptor::set::GsDescriptorSet;
use crate::descriptor::binding::traits::DescriptorMeta;
use crate::descriptor::binding::traits::{ DescriptorBindingCI, DescriptorMetaMirror };

use crate::utils::wrapper::VKWrapperPair;
use crate::types::vkbytes;

use std::ptr;

/// Descriptor buffer binding target.
pub trait DescriptorBindingBufTgt {

    fn binding_info(&self) -> DescriptorBindingBufInfo;
}

pub struct DescriptorBindingBufInfo {

    pub meta: DescriptorMeta,
    /// the size of each element of descriptor.
    pub element_size: vkbytes,
    /// the handle of buffer where the descriptor data stores.
    pub buffer_handle: vk::Buffer,
}

impl DescriptorBindingCI for DescriptorBindingBufInfo {
    type DescriptorWriteType = VKWrapperPair<Vec<vk::DescriptorBufferInfo>, vk::WriteDescriptorSet>;

    fn meta_mirror(&self) -> DescriptorMetaMirror {
        self.meta.clone().into()
    }

    fn write_info(&self, set: &GsDescriptorSet) -> Self::DescriptorWriteType {

        let contents = vec![
            vk::DescriptorBufferInfo {
                buffer: self.buffer_handle,
                offset: 0,
                // TODO: check maxUniformBufferRange or maxStorageBufferRange in physical device limit.
                range: self.element_size,
            }
        ];

        let write_set = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set    : set.handle,
            dst_binding: self.meta.binding,
            dst_array_element  : 0,
            descriptor_count   : 1,
            descriptor_type    : self.meta.descriptor_type.into(),
            p_image_info       : ptr::null(),
            p_buffer_info      : contents.as_ptr(),
            p_texel_buffer_view: ptr::null(),
        };

        VKWrapperPair {
            content: contents,
            info   : write_set,
        }
    }
}
