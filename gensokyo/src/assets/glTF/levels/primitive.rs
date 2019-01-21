
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::model::GsglTFRenderParams;
use crate::assets::glTF::levels::traits::{ GsglTFLevelEntity, GsglTFArchitecture };
use crate::assets::glTF::material::material::GsglTFMaterialData;
use crate::assets::glTF::primitive::attributes::GsglTFAttrFlags;
use crate::assets::glTF::primitive::transforms::GsglTFNodeUniformFlags;
use crate::assets::error::GltfError;

use gsvk::pipeline::target::GsPipelineStage;
use gsvk::command::{ GsCmdRecorder, GsCmdGraphicsApi };
use gsvk::utils::phantom::Graphics;
use gsvk::types::{ vkbytes, vkuint };

// --------------------------------------------------------------------------------------
/// A wrapper class for primitive level in glTF, containing the render parameters read from glTF file.
pub(super) struct GsglTFPrimitiveEntity {

    /// the draw parameters for rendering.
    method: DrawMethod,
    /// the starting offset of attributes in vertex buffer.
    offset: vkbytes,
    /// the render information of material data.
    material: Vec<u8>,
}

impl<'a> GsglTFLevelEntity<'a> for GsglTFPrimitiveEntity {
    type GltfArchLevel = gltf::Primitive<'a>;
    type GltfDataLevel = gltf::Primitive<'a>;

    fn read_architecture(level: Self::GltfArchLevel) -> Result<GsglTFArchitecture<Self>, GltfError> {

        if level.mode() != gltf::mesh::Mode::Triangles {
            // Currently only support Triangle topology.
            return Err(GltfError::loading("Unsupported glTF primitive render mode."))
        }

        let mut attr_flag = GsglTFAttrFlags::NONE;
        for (attribute, _accessor) in level.attributes() {
            match attribute {
                | gltf::Semantic::Positions    => attr_flag |= GsglTFAttrFlags::POSITION,
                | gltf::Semantic::Normals      => attr_flag |= GsglTFAttrFlags::NORMAL,
                | gltf::Semantic::Tangents     => attr_flag |= GsglTFAttrFlags::TANGENT,
                | gltf::Semantic::Colors(0)    => attr_flag |= GsglTFAttrFlags::COLOR_0,
                | gltf::Semantic::TexCoords(0) => attr_flag |= GsglTFAttrFlags::TEXCOORD_0,
                | gltf::Semantic::TexCoords(1) => attr_flag |= GsglTFAttrFlags::TEXCOORD_1,
                | gltf::Semantic::Joints(0)    => attr_flag |= GsglTFAttrFlags::JOINTS_0,
                | gltf::Semantic::Weights(0)   => attr_flag |= GsglTFAttrFlags::WEIGHTS_0,
                | _ => return Err(GltfError::loading("Unsupported glTF primitive attributes combination."))
            }
        }

        // the draw parameters will be set in `Self::read_data` method, so fill 0 here.
        let draw_method = match level.indices() {
            | Some(_) => DrawMethod::DrawIndex {  index_count: 0,  first_index: 0 },
            | None    => DrawMethod::DrawArray { vertex_count: 0, first_vertex: 0 },
        };

        let arch_target = GsglTFArchitecture {
            arch: GsglTFPrimitiveEntity {
                method: draw_method,
                // the following properties will be set in `Self::read_data` method.
                offset: 0,
                material: Vec::new(),
            },
            attr_flags: attr_flag,
            node_flags: GsglTFNodeUniformFlags::NONE,
        };
        Ok(arch_target)
    }

    fn read_data(&mut self, level: Self::GltfDataLevel, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError> {

        // load attributes data.
        let vertex_extend_info = data.extend_attributes(&level, source)?;
        self.offset = vertex_extend_info.start_offset;

        // load indices.
        let reader = level.reader(|b| Some(&source.data_buffer[b.index()]));
        let indices_extend_info = data.extend_indices(&reader)?;

        // load material.
        let raw_material = GsglTFMaterialData::from(&level.material());
        self.material = raw_material.into_data(&source.limits)?;

        // set the draw parameter.
        self.method = match self.method {
            | DrawMethod::DrawArray { .. } => {
                DrawMethod::DrawArray {
                    vertex_count: vertex_extend_info.start_index,
                    first_vertex: vertex_extend_info.extend_vertex_count,
                }
            },
            | DrawMethod::DrawIndex { .. } => {
                DrawMethod::DrawIndex {
                    index_count: indices_extend_info.extend_indices_count,
                    first_index: indices_extend_info.start_index,
                }
            },
        };

        Ok(())
    }
}

impl GsglTFPrimitiveEntity {

    pub(super) fn record_command(&self, recorder: &GsCmdRecorder<Graphics>, params: &GsglTFRenderParams) {

        if params.is_push_materials {
            recorder.push_constants(GsPipelineStage::FRAGMENT, 0, &self.material);
        }

        match self.method {
            | DrawMethod::DrawArray { vertex_count, first_vertex } => {
                recorder.draw(vertex_count, 1, first_vertex, 0);
            },
            | DrawMethod::DrawIndex { index_count, first_index } => {
                recorder.draw_indexed(index_count, 1, first_index, 0, 0);
            },
        }
    }
}
// --------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------
#[derive(Debug, Clone, Eq, PartialEq)]
enum DrawMethod {
    DrawArray { vertex_count: vkuint, first_vertex: vkuint },
    DrawIndex {  index_count: vkuint,  first_index: vkuint },
}
// --------------------------------------------------------------------------------------
