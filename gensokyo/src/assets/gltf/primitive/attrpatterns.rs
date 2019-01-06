
use crate::assets::gltf::error::GltfError;
use crate::utils::types::{ Point3F, Point2F, Vector3F, Vector4F };

use gsvk::buffer::instance::GsVertexBlock;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::types::vkbytes;
use gsma::data_size;

type Vector4U = nalgebra::Vector4<u16>;

use std::ops::{ BitAnd, BitOr, BitOrAssign, BitAndAssign };

#[derive(Eq, PartialEq)]
pub struct GPAFlag(u32);

impl GPAFlag {
    pub const NONE      : GPAFlag = GPAFlag(0b0);
    pub const POSITION  : GPAFlag = GPAFlag(0b1);
    pub const NORMAL    : GPAFlag = GPAFlag(0b10);
    pub const TANGENT   : GPAFlag = GPAFlag(0b100);
    pub const TEXCOORD_0: GPAFlag = GPAFlag(0b1000);
    pub const TEXCOORD_1: GPAFlag = GPAFlag(0b10000);
    pub const COLOR_0   : GPAFlag = GPAFlag(0b100000);
    pub const JOINTS_0  : GPAFlag = GPAFlag(0b1000000);
    pub const WEIGHTS_0 : GPAFlag = GPAFlag(0b10000000);

    pub const GPAP       : GPAFlag = GPAFlag(0b1);
    pub const GPAPN      : GPAFlag = GPAFlag(0b11);
    pub const GPAPNTE0   : GPAFlag = GPAFlag(0b1101);
    pub const GPAULTIMATE: GPAFlag = GPAFlag(0b11111111);
}

impl BitAnd for GPAFlag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        GPAFlag(self.0 & rhs.0)
    }
}

impl BitAndAssign for GPAFlag {

    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for GPAFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        GPAFlag(self.0 | rhs.0)
    }
}

impl BitOrAssign for GPAFlag {

    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

pub(super) trait GPAttribute {

    fn load<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
        where Self: Sized, F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>;

    fn attribute_size(&self) -> vkbytes;

    fn upload(&self, to: &GsVertexBlock, by: &mut BufferDataUploader) -> Result<(), AllocatorError>;
    fn update_transform(&mut self, transform: &nalgebra::Matrix4<f32>);
}


macro_rules! read_attribute {
    ($target:ident, $reader:ident, $VertexType:ident, position) => {
        $target.data = $reader.read_positions()
            .ok_or(GltfError::ModelContentMissing)?
            .map(|pos| {
                let position = Point3F::from(pos);
                $VertexType { position, ..Default::default() }
            }).collect();
    };
    ($target:ident, $reader:ident, $VertexType:ident, normal) => {
        let normal_iter = $reader.read_normals()
            .ok_or(GltfError::ModelContentMissing)?;
        for (i, normal) in normal_iter.enumerate() {
            $target.data[i].normal = Vector3F::from(normal);
        }
    };
    ($target:ident, $reader:ident, $VertexType:ident, tangents) => {
        let tangents_iter = $reader.read_tangents()
            .ok_or(GltfError::ModelContentMissing)?;
        for (i, tangent) in tangents_iter.enumerate() {
            $target.data[i].tangents = Vector4F::from(tangent);
        }
    };
    ($target:ident, $reader:ident, $VertexType:ident, texcoord_0) => {
        let texcoord_0_iter = $reader.read_tex_coords(0)
            .ok_or(GltfError::ModelContentMissing)?
            .into_f32();
        for (i, texcoord_0) in texcoord_0_iter.enumerate() {
            $target.data[i].texcoord_0 = Point2F::from(texcoord_0);
        }
    };
    ($target:ident, $reader:ident, $VertexType:ident, texcoord_1) => {
        let texcoord_1_iter = $reader.read_tex_coords(1)
            .ok_or(GltfError::ModelContentMissing)?
            .into_f32();
        for (i, texcoord_1) in texcoord_1_iter.enumerate() {
            $target.data[i].texcoord_1 = Point2F::from(texcoord_1);
        }
    };
    ($target:ident, $reader:ident, $VertexType:ident, color_0) => {
        let color_0_iter = $reader.read_colors(0)
            .ok_or(GltfError::ModelContentMissing)?
            .into_rgba_f32();
        for (i, color_0) in color_0_iter.enumerate() {
            $target.data[i].color_0 = Vector4F::from(color_0);
        }
    };
    ($target:ident, $reader:ident, $VertexType:ident, joints_0) => {
        let joints_0_iter = $reader.read_joints(0)
            .ok_or(GltfError::ModelContentMissing)?
            .into_u16();
        for (i, joints_0) in joints_0_iter.enumerate() {
            $target.data[i].joints_0 = Vector4U::from(joints_0);
        }
    };
    ($target:ident, $reader:ident, $VertexType:ident, weights_0) => {
        let weights_0_iter = $reader.read_weights(0)
            .ok_or(GltfError::ModelContentMissing)?
            .into_f32();
        for (i, weights_0) in weights_0_iter.enumerate() {
            $target.data[i].weights_0 = Vector4F::from(weights_0);
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
        pub(super) struct $name_gpa {
            data: Vec<$name_vertex>
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

            fn load<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
                where Self: Sized, F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

                let mut target = $name_gpa::default();

                $(
                    read_attribute!(target, reader, $name_vertex, $attribute);
                )*

                Ok(target)
            }

            fn attribute_size(&self) -> vkbytes {
                data_size!(self.data, $name_vertex)
            }

            fn upload(&self, to: &GsVertexBlock, by: &mut BufferDataUploader) -> Result<(), AllocatorError> {

                let _  = by.upload(to, &self.data)?;
                Ok(())
            }

            fn update_transform(&mut self, transform: &nalgebra::Matrix4<f32>) {

                self.data.iter_mut().for_each(|vertex| {
                    vertex.position = transform.transform_point(&vertex.position);
                });
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
