
use crate::assets::glTF::error::GltfError;
use crate::assets::glTF::data::IntermediateglTFData;
use crate::utils::types::{ Point3F, Point2F, Vector3F, Vector4F };

use gsvk::buffer::instance::GsVertexBlock;
use gsvk::memory::transfer::GsBufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::types::vkbytes;
use gsma::data_size;

type Vector4U = nalgebra::Vector4<u16>;

use std::ops::{ BitAnd, BitOr, BitOrAssign, BitAndAssign };


// --------------------------------------------------------------------------------------
/// glTF Primitive attributes flags.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GsglTFAttrFlag(u32);

impl GsglTFAttrFlag {
    pub const NONE      : GsglTFAttrFlag = GsglTFAttrFlag(0b0);
    pub const POSITION  : GsglTFAttrFlag = GsglTFAttrFlag(0b1);
    pub const NORMAL    : GsglTFAttrFlag = GsglTFAttrFlag(0b10);
    pub const TANGENT   : GsglTFAttrFlag = GsglTFAttrFlag(0b100);
    pub const TEXCOORD_0: GsglTFAttrFlag = GsglTFAttrFlag(0b1000);
    pub const TEXCOORD_1: GsglTFAttrFlag = GsglTFAttrFlag(0b10000);
    pub const COLOR_0   : GsglTFAttrFlag = GsglTFAttrFlag(0b100000);
    pub const JOINTS_0  : GsglTFAttrFlag = GsglTFAttrFlag(0b1000000);
    pub const WEIGHTS_0 : GsglTFAttrFlag = GsglTFAttrFlag(0b10000000);

    // POSITION.
    pub const GPAP       : GsglTFAttrFlag = GsglTFAttrFlag(0b1);
    // POSITION, NORMAL.
    pub const GPAPN      : GsglTFAttrFlag = GsglTFAttrFlag(0b11);
    // POSITION, NORMAL, TEXCOORD_0.
    pub const GPAPNTE0   : GsglTFAttrFlag = GsglTFAttrFlag(0b1101);
    // POSITION, NORMAL, TANGENT, TEXCOORD_0, TEXCOORD_1, COLOR_0, JOINTS_0, WEIGHTS_0.
    pub const GPAULTIMATE: GsglTFAttrFlag = GsglTFAttrFlag(0b11111111);

    pub fn vertex_size(&self) -> Option<vkbytes> {
        match *self {
            | GsglTFAttrFlag::GPAP        => Some(data_size!(GPAP)),
            | GsglTFAttrFlag::GPAPN       => Some(data_size!(GPAPN)),
            | GsglTFAttrFlag::GPAPNTE0    => Some(data_size!(GPAPNTe0)),
            | GsglTFAttrFlag::GPAULTIMATE => Some(data_size!(GPAUltimate)),
            | _ => None,
        }
    }
}

impl BitAnd for GsglTFAttrFlag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        GsglTFAttrFlag(self.0 & rhs.0)
    }
}

impl BitAndAssign for GsglTFAttrFlag {

    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for GsglTFAttrFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        GsglTFAttrFlag(self.0 | rhs.0)
    }
}

impl BitOrAssign for GsglTFAttrFlag {

    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
// --------------------------------------------------------------------------------------

// --------------------------------------------------------------------------------------
/// glTF Primitive attribute.
pub(crate) trait GPAttribute {

    fn extend(&mut self, primitive: &gltf::Primitive, source: &IntermediateglTFData) -> Result<usize, GltfError>;

    fn data_length(&self) -> usize;

    fn upload(&self, to: &GsVertexBlock, by: &mut GsBufferDataUploader) -> Result<(), AllocatorError>;
}

macro_rules! read_attribute {
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, position) => {
        if $target.data.len() == $origin_length {
            let new_vertexs = $reader.read_positions()
                .ok_or(GltfError::ModelContentMissing)?
                .map(|pos| {
                    let position = Point3F::from(pos);
                    $VertexType { position, ..Default::default() }
                }).collect::<Vec<_>>();
            $target.data.extend(new_vertexs);
        } else {
            let pos_iter = $reader.read_positions()
                .ok_or(GltfError::ModelContentMissing)?;
            for (i, pos) in pos_iter.enumerate() {
                $target.data[i + $origin_length].position = Point3F::from(pos);
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, normal) => {
        if $target.data.len() == $origin_length {
            let new_vertexs = $reader.read_normals()
                .ok_or(GltfError::ModelContentMissing)?
                .map(|nor| {
                    let normal = Vector3F::from(nor);
                    $VertexType { normal, ..Default::default() }
                }).collect::<Vec<_>>();
            $target.data.extend(new_vertexs);
        } else {
            let normal_iter = $reader.read_normals()
                .ok_or(GltfError::ModelContentMissing)?;
            for (i, normal) in normal_iter.enumerate() {
                $target.data[i + $origin_length].normal = Vector3F::from(normal);
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, tangents) => {
        if $target.data.len() == $origin_length {
            let new_vertexs = $reader.read_tangents()
                .ok_or(GltfError::ModelContentMissing)?
                .map(|tan| {
                    let tangents = Vector4F::from(tan);
                    $VertexType { tangents, ..Default::default() }
                }).collect::<Vec<_>>();
            $target.data.extend(new_vertexs);
        } else {
            let tangents_iter = $reader.read_tangents()
                .ok_or(GltfError::ModelContentMissing)?;
            for (i, tangent) in tangents_iter.enumerate() {
                $target.data[i + $origin_length].tangents = Vector4F::from(tangent);
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, texcoord_0) => {
        if $target.data.len() == $origin_length {
            let new_vertexs = $reader.read_tex_coords(0)
                .ok_or(GltfError::ModelContentMissing)?
                .into_f32()
                .map(|texcoord| {
                    let texcoord_0 = Point2F::from(texcoord);
                    $VertexType { texcoord_0, ..Default::default() }
                }).collect::<Vec<_>>();
            $target.data.extend(new_vertexs);
        } else {
            let texcoord_0_iter = $reader.read_tex_coords(0)
                .ok_or(GltfError::ModelContentMissing)?
                .into_f32();
            for (i, texcoord_0) in texcoord_0_iter.enumerate() {
                $target.data[i + $origin_length].texcoord_0 = Point2F::from(texcoord_0);
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, texcoord_1) => {
        if $target.data.len() == $origin_length {
            let new_vertexs = $reader.read_tex_coords(1)
                .ok_or(GltfError::ModelContentMissing)?
                .into_f32()
                .map(|texcoord| {
                    let texcoord_1 = Point2F::from(texcoord);
                    $VertexType { texcoord_1, ..Default::default() }
                }).collect::<Vec<_>>();
            $target.data.extend(new_vertexs);
        } else {
            let texcoord_1_iter = $reader.read_tex_coords(1)
                .ok_or(GltfError::ModelContentMissing)?
                .into_f32();
            for (i, texcoord_1) in texcoord_1_iter.enumerate() {
                $target.data[i + $origin_length].texcoord_1 = Point2F::from(texcoord_1);
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, color_0) => {
        if $target.data.len() == $origin_length {
            let new_vertexs = $reader.read_colors(0)
                .ok_or(GltfError::ModelContentMissing)?
                .into_rgba_f32()
                .map(|color| {
                    let color_0 = Vector4F::from(color);
                    $VertexType { color_0, ..Default::default() }
                }).collect::<Vec<_>>();
            $target.data.extend(new_vertexs);
        } else {
            let color_0_iter = $reader.read_colors(0)
                .ok_or(GltfError::ModelContentMissing)?
                .into_rgba_f32();
            for (i, color_0) in color_0_iter.enumerate() {
                $target.data[i + $origin_length].color_0 = Vector4F::from(color_0);
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, joints_0) => {
        if $target.data.len() == $origin_length {
            let new_vertexs = $reader.read_joints(0)
                .ok_or(GltfError::ModelContentMissing)?
                .into_u16()
                .map(|joint| {
                    let joints_0 = Vector4U::from(joint);
                    $VertexType { joints_0, ..Default::default() }
                }).collect::<Vec<_>>();
            $target.data.extend(new_vertexs);
        } else {
            let joints_0_iter = $reader.read_joints(0)
                .ok_or(GltfError::ModelContentMissing)?
                .into_u16();
            for (i, joints_0) in joints_0_iter.enumerate() {
                $target.data[i + $origin_length].joints_0 = Vector4U::from(joints_0);
            }
        }
    };
    ($target:ident, $reader:ident, $origin_length:ident, $VertexType:ident, weights_0) => {
        if $target.data.len() == $origin_length {
            let new_vertexs = $reader.read_weights(0)
                .ok_or(GltfError::ModelContentMissing)?
                .into_f32()
                .map(|weight| {
                    let weights_0 = Vector4F::from(weight);
                    $VertexType { weights_0, ..Default::default() }
                }).collect::<Vec<_>>();
            $target.data.extend(new_vertexs);
        } else {
            let weights_0_iter = $reader.read_weights(0)
                .ok_or(GltfError::ModelContentMissing)?
                .into_f32();
            for (i, weights_0) in weights_0_iter.enumerate() {
                $target.data[i + $origin_length].weights_0 = Vector4F::from(weights_0);
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

        #[derive(Default)]
        pub(crate) struct $name_gpa {
            data: Vec<$name_vertex>,
        }

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

        impl GPAttribute for $name_gpa {

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

            fn upload(&self, to: &GsVertexBlock, by: &mut GsBufferDataUploader) -> Result<(), AllocatorError> {

                let _  = by.upload(to, &self.data)?;
                Ok(())
            }
        }
    };
}

// glTF Primitive with only position attribute.
define_gpa!(GPAP, GPAPVertex, {
    position,
});

/// glTF Primitive with position and normal attributes.
define_gpa!(GPAPN, GPAPNVertex, {
    position, normal,
});

/// glTF Primitive with position, normal and texcoord_0 attributes.
define_gpa!(GPAPNTe0, GPAPNTe0Vertex, {
    position, normal, texcoord_0,
});

/// glTF Primitive with all attributes.
define_gpa!(GPAUltimate, GPAUltimateVertex, {
    position, normal, tangents, texcoord_0, texcoord_1, color_0, joints_0, weights_0,
});
// --------------------------------------------------------------------------------------
