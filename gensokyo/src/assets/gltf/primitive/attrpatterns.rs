
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
    ($target:ident, $reader:ident, normal) => {
        let normal_iter = $reader.read_normals()
            .ok_or(GltfError::ModelContentMissing)?;
        for (i, normal) in normal_iter.enumerate() {
            $target.data[i].normal = Vector3F::from(normal);
        }
    };
    ($target:ident, $reader:ident, tangents) => {
        let tangents_iter = $reader.read_tangents()
            .ok_or(GltfError::ModelContentMissing)?;
        for (i, tangent) in tangents_iter.enumerate() {
            $target.data[i].tangents = Vector4F::from(tangent);
        }
    };
    ($target:ident, $reader:ident, texcoord_0) => {
        let texcoord_0_iter = $reader.read_tex_coords(0)
            .ok_or(GltfError::ModelContentMissing)?
            .into_f32();
        for (i, texcoord_0) in texcoord_0_iter.enumerate() {
            $target.data[i].texcoord_0 = Point2F::from(texcoord_0);
        }
    };
    ($target:ident, $reader:ident, texcoord_1) => {
        let texcoord_1_iter = $reader.read_tex_coords(1)
            .ok_or(GltfError::ModelContentMissing)?
            .into_f32();
        for (i, texcoord_1) in texcoord_1_iter.enumerate() {
            $target.data[i].texcoord_1 = Point2F::from(texcoord_1);
        }
    };
    ($target:ident, $reader:ident, color_0) => {
        let color_0_iter = $reader.read_colors(0)
            .ok_or(GltfError::ModelContentMissing)?
            .into_rgba_f32();
        for (i, color_0) in color_0_iter.enumerate() {
            $target.data[i].color_0 = Vector4F::from(color_0);
        }
    };
    ($target:ident, $reader:ident, joints_0) => {
        let joints_0_iter = $reader.read_joints(0)
            .ok_or(GltfError::ModelContentMissing)?
            .into_u16();
        for (i, joints_0) in joints_0_iter.enumerate() {
            $target.data[i].joints_0 = Vector4U::from(joints_0);
        }
    };
    ($target:ident, $reader:ident, weights_0) => {
        let weights_0_iter = $reader.read_weights(0)
            .ok_or(GltfError::ModelContentMissing)?
            .into_f32();
        for (i, weights_0) in weights_0_iter.enumerate() {
            $target.data[i].weights_0 = Vector4F::from(weights_0);
        }

    };
}

// ---------------------------------------------------------------------------------
/// glTF Primitive with only position attribute.
#[derive(Default)]
pub(super) struct GPAP {
    data: Vec<GPAPVertex>,
}

#[derive(Debug, Clone, Copy)]
struct GPAPVertex {
    position: Point3F, // POSITION property.
}

impl Default for GPAPVertex {

    fn default() -> GPAPVertex {
        GPAPVertex {
            position: Point3F::new(0.0, 0.0, 0.0),
        }
    }
}

impl GPAttribute for GPAP {

    fn load<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
        where Self: Sized, F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let mut target = GPAP::default();
        read_attribute!(target, reader, GPAPVertex, position);

        Ok(target)
    }

    fn attribute_size(&self) -> vkbytes {
        data_size!(self.data, GPAPVertex)
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
// ---------------------------------------------------------------------------------

// ---------------------------------------------------------------------------------
/// glTF Primitive with position and normal attribute.
#[derive(Default)]
pub(super) struct GPAPN {
    data: Vec<GPAPNVertex>,
}

#[derive(Debug, Clone, Copy)]
struct GPAPNVertex {
    position: Point3F,  // POSITION property.
    normal  : Vector3F, // NORMAL   property.
}

impl Default for GPAPNVertex {

    fn default() -> GPAPNVertex {
        GPAPNVertex {
            position: Point3F::new(0.0, 0.0, 0.0),
            normal  : nalgebra::zero(),
        }
    }
}

impl GPAttribute for GPAPN {

    fn load<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
        where Self: Sized, F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let mut target = GPAPN::default();
        read_attribute!(target, reader, GPAPNVertex, position);
        read_attribute!(target, reader, normal);

        Ok(target)
    }

    fn attribute_size(&self) -> vkbytes {
        data_size!(self.data, GPAPNVertex)
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
// ---------------------------------------------------------------------------------

// ---------------------------------------------------------------------------------
/// glTF Primitive with position, normal and texcoord_0 attribute.
#[derive(Default)]
pub(super) struct GPAPNTe0 {
    data: Vec<GPAPNTe0Vertex>,
}

#[derive(Debug, Clone, Copy)]
struct GPAPNTe0Vertex {
    position  : Point3F,  // POSITION.
    normal    : Vector3F, // NORMAL.
    texcoord_0: Point2F,  // TEXCOORD_0.
}

impl Default for GPAPNTe0Vertex {

    fn default() -> GPAPNTe0Vertex {
        GPAPNTe0Vertex {
            position  : Point3F::new(0.0, 0.0, 0.0),
            normal    : nalgebra::zero(),
            texcoord_0: Point2F::new(0.0, 0.0),
        }
    }
}

impl GPAttribute for GPAPNTe0 {

    fn load<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
        where Self: Sized, F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let mut target = GPAPNTe0::default();
        read_attribute!(target, reader, GPAPNTe0Vertex, position);
        read_attribute!(target, reader, normal);
        read_attribute!(target, reader, texcoord_0);

        Ok(target)
    }

    fn attribute_size(&self) -> vkbytes {
        data_size!(self.data, GPAPNTe0Vertex)
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
// ---------------------------------------------------------------------------------

// ---------------------------------------------------------------------------------
/// glTF Primitive with all attribute.
#[derive(Default)]
pub(super) struct GPAUltimate {
    data: Vec<GPAUltimateVertex>,
}

#[derive(Debug, Clone, Copy)]
struct GPAUltimateVertex {
    position  : Point3F,  // POSITION.
    normal    : Vector3F, // NORMAL.
    tangents  : Vector4F, // TANGENT.
    texcoord_0: Point2F,  // TEXCOORD_0.
    texcoord_1: Point2F,  // TEXCOORD_1.
    color_0   : Vector4F, // COLOR_0.
    joints_0  : Vector4U, // JOINTS_0.
    weights_0 : Vector4F, // WEIGHTS_0.
}

impl Default for GPAUltimateVertex {

    fn default() -> GPAUltimateVertex {
        GPAUltimateVertex {
            position  : Point3F::new(0.0, 0.0, 0.0),
            normal    : nalgebra::zero(),
            tangents  : nalgebra::zero(),
            texcoord_0: Point2F::new(0.0, 0.0),
            texcoord_1: Point2F::new(0.0, 0.0),
            color_0   : nalgebra::zero(),
            joints_0  : nalgebra::zero(),
            weights_0 : nalgebra::zero(),
        }
    }
}

impl GPAttribute for GPAUltimate {

    fn load<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
        where Self: Sized, F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let mut target = GPAUltimate::default();
        read_attribute!(target, reader, GPAUltimateVertex, position);
        read_attribute!(target, reader, normal);
        read_attribute!(target, reader, tangents);
        read_attribute!(target, reader, texcoord_0);
        read_attribute!(target, reader, texcoord_1);
        read_attribute!(target, reader, color_0);
        read_attribute!(target, reader, joints_0);
        read_attribute!(target, reader, weights_0);

        Ok(target)
    }

    fn attribute_size(&self) -> vkbytes {
        data_size!(self.data, GPAUltimateVertex)
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
// ---------------------------------------------------------------------------------
