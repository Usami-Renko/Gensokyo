
use crate::assets::glTF::levels::GsglTFNodeEntity;
use crate::assets::glTF::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::buffer::instance::{ GsUniformBuffer, GsBufUniformInfo };
use gsvk::memory::transfer::GsBufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::types::{ vkuint, vkbytes };
use gsma::data_size;

use std::ops::{ BitAnd, BitOr, BitOrAssign, BitAndAssign };

// --------------------------------------------------------------------------------------
pub(crate) struct GsglTFNodesData {

    element_size: vkbytes,
    content: Box<dyn GNProperties>,
}

impl GsglTFNodesData {

    pub fn new(flag: GsglTFNodeUniformFlags) -> Result<GsglTFNodesData, GltfError> {

        let content = match flag {
            | GsglTFNodeUniformFlags::GPN_T => {
                let properties = GPN_T::default();
                Box::new(properties) as Box<dyn GNProperties>
            },
            | _ => return Err(GltfError::UnsupportNodeProperties)
        };

        let nodes_data = GsglTFNodesData {
            element_size: flag.element_size()
                .ok_or(GltfError::UnsupportNodeProperties)?,
            content,
        };
        Ok(nodes_data)
    }

    pub fn uniform_info(&self, uniform_binding: vkuint) -> GsBufUniformInfo {

        GsBufUniformInfo::new_dyn(uniform_binding, 1, self.element_size, self.content.data_length())
    }

    pub fn data_content_mut(&mut self) -> &mut Box<dyn GNProperties> {
        &mut self.content
    }

    pub fn data_content(&self) -> &Box<dyn GNProperties> {
        &self.content
    }
}
// --------------------------------------------------------------------------------------

// --------------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GsglTFNodeUniformFlags(u32);

impl GsglTFNodeUniformFlags {
    pub const NONE            : GsglTFNodeUniformFlags = GsglTFNodeUniformFlags(0b0);
    pub const TRANSFORM_MATRIX: GsglTFNodeUniformFlags = GsglTFNodeUniformFlags(0b1);
    // pub const JOINT_MATRIX    : GsglTFNodeUniformFlags = GsglTFNodeUniformFlags(0b10);

    pub const GPN_T: GsglTFNodeUniformFlags = GsglTFNodeUniformFlags(0b1);

    fn element_size(&self) -> Option<vkbytes> {
        match *self {
            | GsglTFNodeUniformFlags::GPN_T => Some(data_size!(GPN_T)),
            | _ => None,
        }
    }
}

impl BitAnd for GsglTFNodeUniformFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        GsglTFNodeUniformFlags(self.0 & rhs.0)
    }
}

impl BitAndAssign for GsglTFNodeUniformFlags {

    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for GsglTFNodeUniformFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        GsglTFNodeUniformFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for GsglTFNodeUniformFlags {

    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
// --------------------------------------------------------------------------------------

// --------------------------------------------------------------------------------------
/// glTF Node properties.
pub(crate) trait GNProperties {

    fn upload(&self, to: &GsUniformBuffer, by: &mut GsBufferDataUploader, alignment: vkbytes) -> Result<(), AllocatorError>;

    fn extend(&mut self, node: &GsglTFNodeEntity);

    fn data_length(&self) -> usize;
}

macro_rules! read_transform {
    ($target:ident, $node:ident, $uniform_type:ident, transform) => {
        let new_uniforms = $uniform_type {
            transform: $node.local_transform.clone(),
            ..Default::default()
        };
        $target.data.push(new_uniforms);
    };
}

macro_rules! gn_property_type {
    (transform) => (Matrix4F);
}

macro_rules! gn_property_default {
    (transform) => { Matrix4F::identity() };
}

macro_rules! define_gnp {
    ($name_gnp:ident, $name_uniform:ident, {
        $(
            $attribute:ident,
        )*
    }) => {

        #[allow(non_camel_case_types)]
        #[derive(Default)]
        pub(crate) struct $name_gnp {
            data: Vec<$name_uniform>,
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy)]
        struct $name_uniform {
            $(
                $attribute: gn_property_type!($attribute),
            )*
        }

        impl Default for $name_uniform {

            fn default() -> $name_uniform {
                $name_uniform {
                    $(
                        $attribute: gn_property_default!($attribute),
                    )*
                }

            }
        }

        impl GNProperties for $name_gnp {

            fn extend(&mut self, node: &GsglTFNodeEntity) {

                $(
                    read_transform!(self, node, $name_uniform, $attribute);
                )*
            }

            fn data_length(&self) -> usize {
                self.data.len()
            }

            fn upload(&self, to: &GsUniformBuffer, by: &mut GsBufferDataUploader, alignment: vkbytes) -> Result<(), AllocatorError> {

                let _ = by.upload_align(to, &self.data, alignment)?;
                Ok(())
            }
        }
    };
}

define_gnp!(GPN_T, GPN_T_VERTEX, {
    transform,
});
// --------------------------------------------------------------------------------------
