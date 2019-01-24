
use crate::assets::glTF::data::IntermediateglTFData;
use crate::utils::types::{ Point3F, Point2F, Vector3F, Vector4F };
use crate::assets::error::GltfError;

use gsvk::buffer::instance::{ GsVertexBuffer, GsBufVertexInfo };
use gsvk::memory::transfer::GsBufferDataUploader;
use gsvk::types::vkbytes;
use gsvk::error::VkResult;
use gsma::data_size;

use std::ops::{ BitAnd, BitOr, BitOrAssign, BitAndAssign };

type Vector4U = nalgebra::Vector4<u16>;

pub(crate) struct GsglTFAttributesData {

    vertex_size: vkbytes,
    content: Box<dyn GPAttributes>,
}

impl GsglTFAttributesData {

    pub fn new(flag: GsglTFAttrFlags) -> Result<GsglTFAttributesData, GltfError> {

        let content = match flag {
            | GsglTFAttrFlags::GPA_P        => Box::new(GPA_P::default())        as Box<dyn GPAttributes>,
            | GsglTFAttrFlags::GPA_PN       => Box::new(GPA_PN::default())       as Box<dyn GPAttributes>,
            | GsglTFAttrFlags::GPA_PNTE0    => Box::new(GPA_PNTe0::default())    as Box<dyn GPAttributes>,
            | GsglTFAttrFlags::GPA_ULTIMATE => Box::new(GPA_Ultimate::default()) as Box<dyn GPAttributes>,
            | _ => return Err(GltfError::loading("Unsupported glTF primitive attributes combination."))
        };

        let attributes = GsglTFAttributesData {
            vertex_size: flag.vertex_size()
                .ok_or(GltfError::loading("Unsupported glTF primitive attributes combination."))?,
            content,
        };
        Ok(attributes)
    }

    pub fn data_size(&self) -> vkbytes {
        (self.content.data_length() as vkbytes) * self.vertex_size
    }

    pub fn vertex_info(&self) -> GsBufVertexInfo {

        GsBufVertexInfo::new(self.vertex_size, self.content.data_length())
    }

    pub fn data_content(&self) -> &Box<dyn GPAttributes> {
        &self.content
    }

    pub fn data_content_mut(&mut self) -> &mut Box<dyn GPAttributes> {
        &mut self.content
    }
}


// --------------------------------------------------------------------------------------
/// glTF Primitive attributes flags.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GsglTFAttrFlags(u32);

impl GsglTFAttrFlags {
    pub const NONE      : GsglTFAttrFlags = GsglTFAttrFlags(0b0);
    pub const POSITION  : GsglTFAttrFlags = GsglTFAttrFlags(0b1);
    pub const NORMAL    : GsglTFAttrFlags = GsglTFAttrFlags(0b10);
    pub const TANGENT   : GsglTFAttrFlags = GsglTFAttrFlags(0b100);
    pub const TEXCOORD_0: GsglTFAttrFlags = GsglTFAttrFlags(0b1000);
    pub const TEXCOORD_1: GsglTFAttrFlags = GsglTFAttrFlags(0b10000);
    pub const COLOR_0   : GsglTFAttrFlags = GsglTFAttrFlags(0b100000);
    pub const JOINTS_0  : GsglTFAttrFlags = GsglTFAttrFlags(0b1000000);
    pub const WEIGHTS_0 : GsglTFAttrFlags = GsglTFAttrFlags(0b10000000);

    // POSITION.
    pub const GPA_P       : GsglTFAttrFlags = GsglTFAttrFlags(0b1);
    // POSITION, NORMAL.
    pub const GPA_PN      : GsglTFAttrFlags = GsglTFAttrFlags(0b11);
    // POSITION, NORMAL, TEXCOORD_0.
    pub const GPA_PNTE0   : GsglTFAttrFlags = GsglTFAttrFlags(0b1101);
    // POSITION, NORMAL, TANGENT, TEXCOORD_0, TEXCOORD_1, COLOR_0, JOINTS_0, WEIGHTS_0.
    pub const GPA_ULTIMATE: GsglTFAttrFlags = GsglTFAttrFlags(0b11111111);

    fn vertex_size(&self) -> Option<vkbytes> {
        match *self {
            | GsglTFAttrFlags::GPA_P        => Some(data_size!(GPA_P)),
            | GsglTFAttrFlags::GPA_PN       => Some(data_size!(GPA_PN)),
            | GsglTFAttrFlags::GPA_PNTE0    => Some(data_size!(GPA_PNTe0)),
            | GsglTFAttrFlags::GPA_ULTIMATE => Some(data_size!(GPA_Ultimate)),
            | _ => None,
        }
    }
}

impl BitAnd for GsglTFAttrFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        GsglTFAttrFlags(self.0 & rhs.0)
    }
}

impl BitAndAssign for GsglTFAttrFlags {

    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for GsglTFAttrFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        GsglTFAttrFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for GsglTFAttrFlags {

    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
// --------------------------------------------------------------------------------------

// --------------------------------------------------------------------------------------
/// glTF Primitive attributes.
pub(crate) trait GPAttributes {

    fn extend(&mut self, primitive: &gltf::Primitive, source: &IntermediateglTFData) -> Result<usize, GltfError>;

    fn data_length(&self) -> usize;

    fn upload(&self, to: &GsVertexBuffer, by: &mut GsBufferDataUploader) -> VkResult<()>;
}

macro_rules! read_attribute {
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, position) => {

        if let Some(pos_iter) = $reader.read_positions() {

            if $target.data.len() == $origin_length {
                let vertex_iter = pos_iter.map(|pos| {
                    let position = Point3F::from(pos);
                    $VertexType { position, ..Default::default() }
                });
                $target.data.extend(vertex_iter);
            } else {
                for (i, pos) in pos_iter.enumerate() {
                    $target.data[i + $origin_length].position = Point3F::from(pos);
                }
            }
        }

    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, normal) => {

        if let Some(normal_iter) = $reader.read_normals() {

            if $target.data.len() == $origin_length {
                let vertex_iter = normal_iter.map(|nor| {
                    let normal = Vector3F::from(nor);
                    $VertexType { normal, ..Default::default() }
                });
                $target.data.extend(vertex_iter);
            } else {
                for (i, normal) in normal_iter.enumerate() {
                    $target.data[i + $origin_length].normal = Vector3F::from(normal);
                }
            }
        }

    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, tangents) => {

        if let Some(tangents_iter) = $reader.read_tangents() {

            if $target.data.len() == $origin_length {
                let vertex_iter = tangents_iter.map(|tan| {
                    let tangents = Vector4F::from(tan);
                    $VertexType { tangents, ..Default::default() }
                });
                $target.data.extend(vertex_iter);
            } else {
                for (i, tangent) in tangents_iter.enumerate() {
                    $target.data[i + $origin_length].tangents = Vector4F::from(tangent);
                }
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, texcoord_0) => {

        if let Some(texcoord_0_iter) = $reader.read_tex_coords(0) {

            if $target.data.len() == $origin_length {
                let vertex_iter = texcoord_0_iter.into_f32().map(|texcoord| {
                    let texcoord_0 = Point2F::from(texcoord);
                    $VertexType { texcoord_0, ..Default::default() }
                });
                $target.data.extend(vertex_iter);
            } else {
                for (i, texcoord_0) in texcoord_0_iter.into_f32().enumerate() {
                    $target.data[i + $origin_length].texcoord_0 = Point2F::from(texcoord_0);
                }
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, texcoord_1) => {

        if let Some(texcoord_1_iter) = $reader.read_tex_coords(1) {

            if $target.data.len() == $origin_length {
                let vertex_iter = texcoord_1_iter.into_f32().map(|texcoord| {
                    let texcoord_1 = Point2F::from(texcoord);
                    $VertexType { texcoord_1, ..Default::default() }
                });
                $target.data.extend(vertex_iter);
            } else {
                for (i, texcoord_1) in texcoord_1_iter.into_f32().enumerate() {
                    $target.data[i + $origin_length].texcoord_1 = Point2F::from(texcoord_1);
                }
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, color_0) => {

        if let Some(color_0_iter) = $reader.read_colors(0) {

            if $target.data.len() == $origin_length {
                let vertex_iter = color_0_iter.into_rgba_f32().map(|color| {
                    let color_0 = Vector4F::from(color);
                    $VertexType { color_0, ..Default::default() }
                });
                $target.data.extend(vertex_iter);
            } else {
                for (i, color_0) in color_0_iter.into_rgba_f32().enumerate() {
                    $target.data[i + $origin_length].color_0 = Vector4F::from(color_0);
                }
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, joints_0) => {

        if let Some(joints_0_iter) = $reader.read_joints(0) {

            if $target.data.len() == $origin_length {
                let vertex_iter = joints_0_iter.into_u16().map(|joint| {
                    let joints_0 = Vector4U::from(joint);
                    $VertexType { joints_0, ..Default::default() }
                });
                $target.data.extend(vertex_iter);
            } else {
                for (i, joints_0) in joints_0_iter.into_u16().enumerate() {
                    $target.data[i + $origin_length].joints_0 = Vector4U::from(joints_0);
                }
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, weights_0) => {

        if let Some(weights_0_iter) = $reader.read_weights(0) {

            if $target.data.len() == $origin_length {
                let vertex_iter = weights_0_iter.into_f32().map(|weight| {
                    let weights_0 = Vector4F::from(weight);
                    $VertexType { weights_0, ..Default::default() }
                });
                $target.data.extend(vertex_iter);
            } else {
                for (i, weights_0) in weights_0_iter.into_f32().enumerate() {
                    $target.data[i + $origin_length].weights_0 = Vector4F::from(weights_0);
                }
            }
        }
    };
}

macro_rules! attribute_type {
    (position)   => (Point3F);
    (normal)     => (Vector3F);
    (tangents)   => (Vector4F);
    (texcoord_0) => (Point2F);
    (texcoord_1) => (Point2F);
    (color_0)    => (Vector4F);
    (joints_0)   => (Vector4U);
    (weights_0)  => (Vector4F);
}

macro_rules! attribute_default {
    (position)   => { Point3F::new(0.0, 0.0, 0.0) };
    (normal)     => { nalgebra::zero() };
    (tangents)   => { nalgebra::zero() };
    (texcoord_0) => { Point2F::new(0.0, 0.0) };
    (texcoord_1) => { Point2F::new(0.0, 0.0) };
    (color_0)    => { nalgebra::zero() };
    (joints_0)   => { nalgebra::zero() };
    (weights_0)  => { nalgebra::zero() };
}

macro_rules! define_gpa {
    ($name_gpa:ident, $name_vertex:ident, {
        $(
            $attribute:ident,
        )*
    }) => {

        #[allow(non_camel_case_types)]
        #[derive(Default)]
        pub(crate) struct $name_gpa {
            data: Vec<$name_vertex>,
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy)]
        struct $name_vertex {
            $(
                $attribute: attribute_type!($attribute),
            )*
        }

        impl Default for $name_vertex {

            fn default() -> $name_vertex {
                $name_vertex {
                    $(
                        $attribute: attribute_default!($attribute),
                    )*
                }
            }
        }

        impl GPAttributes for $name_gpa {

            fn extend(&mut self, primitive: &gltf::Primitive, source: &IntermediateglTFData) -> Result<usize, GltfError> {

                let reader = primitive.reader(|b| Some(&source.data_buffer[b.index()]));
                let origin_length = self.data.len();
                $(
                    read_attribute!(self, reader, origin_length, $name_vertex, $attribute);
                )*

                let extend_length = self.data.len() - origin_length;
                Ok(extend_length)
            }

            fn data_length(&self) -> usize {
                self.data.len()
            }

            fn upload(&self, to: &GsVertexBuffer, by: &mut GsBufferDataUploader) -> VkResult<()> {

                // println!("{:?}", &self.data);
                let _  = by.upload(to, &self.data)?;
                Ok(())
            }
        }
    };
}

// glTF Primitive with only position attribute.
define_gpa!(GPA_P, GPA_P_Vertex, {
    position,
});

/// glTF Primitive with position and normal attributes.
define_gpa!(GPA_PN, GPA_PN_Vertex, {
    position, normal,
});

/// glTF Primitive with position, normal and texcoord_0 attributes.
define_gpa!(GPA_PNTe0, GPA_PNTe0_Vertex, {
    position, normal, texcoord_0,
});

/// glTF Primitive with all attributes.
define_gpa!(GPA_Ultimate, GPA_Ultimate_Vertex, {
    position, normal, tangents, texcoord_0, texcoord_1, color_0, joints_0, weights_0,
});
// --------------------------------------------------------------------------------------
