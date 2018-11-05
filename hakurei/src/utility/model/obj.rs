
use tobj;

use utility::model::{ ModelLoadingErr, ModelObjLoadingError };

use std::path::Path;

/// An Wavefront Obj file loader.
pub struct ModelObjLoader {}

impl ModelObjLoader {

    pub(crate) fn new() -> ModelObjLoader {
        ModelObjLoader {}
    }

    pub fn load_model(&self, from: impl AsRef<Path>, data_entity: &mut impl ObjDataEntity) -> Result<(), ModelLoadingErr> {

        let (models, _materials) = tobj::load_obj(from.as_ref())
            .map_err(|e| ModelLoadingErr::Obj(ModelObjLoadingError::Loading(e)))?;

        // TODO: Currently only support loading first model in obj file.
        let first_model = &models[0];
        let mesh = &first_model.mesh;

        let total_vertices_count = mesh.positions.len() / 3;
        data_entity.init_vertices_capacity(total_vertices_count);

        for i in 0..total_vertices_count {

            data_entity.fill_vertex(
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
                mesh.texcoords[i * 2],
                mesh.texcoords[i * 2 + 1],
            );
        }

        data_entity.fill_indices(mesh.indices.clone());

        Ok(())
    }
}

pub trait ObjDataEntity {

    fn init_vertices_capacity(&mut self, vertex_amount: usize);
    fn fill_vertex(&mut self, pos_x: f32, pos_y: f32, pos_z: f32, tex_x: f32, tex_y: f32);
    fn fill_indices(&mut self, indices: Vec<u32>);
}
