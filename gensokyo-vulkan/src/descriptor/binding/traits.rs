
use crate::descriptor::set::GsDescriptorSet;
use crate::descriptor::types::GsDescriptorType;

use crate::types::vkuint;

/// The mirror meta data of a specific descriptor.
#[derive(Debug, Clone)]
pub struct DescriptorMetaMirror {

    /// the binding index used in shader for the descriptor.
    pub binding: vkuint,
    /// the total element count of each descriptor.
    pub count: vkuint,
    /// the type of descriptor.
    pub descriptor_type: GsDescriptorType,
}

pub trait DescriptorBindingCI {
    type DescriptorWriteType;

    fn meta_mirror(&self) -> DescriptorMetaMirror;
    fn write_info(&self, set: &GsDescriptorSet) -> Self::DescriptorWriteType;
}

/// The meta data of a specific descriptor.
#[derive(Debug, Clone)]
pub struct DescriptorMeta {

    pub binding: vkuint,
    pub descriptor_type: GsDescriptorType,
}

impl From<DescriptorMeta> for DescriptorMetaMirror {

    fn from(meta: DescriptorMeta) -> DescriptorMetaMirror {
        DescriptorMetaMirror {
            binding: meta.binding,
            count  : 1,
            descriptor_type: meta.descriptor_type.clone(),
        }
    }
}

/// The meta data of a specific descriptor array.
#[derive(Debug, Clone)]
pub struct DescriptorArrayMeta {

    pub binding: vkuint,
    pub count  : vkuint,
    pub descriptor_type: GsDescriptorType,
}

impl From<DescriptorArrayMeta> for DescriptorMetaMirror {

    fn from(meta: DescriptorArrayMeta) -> DescriptorMetaMirror {
        DescriptorMetaMirror {
            binding: meta.binding,
            count  : meta.count,
            descriptor_type: meta.descriptor_type,
        }
    }
}
