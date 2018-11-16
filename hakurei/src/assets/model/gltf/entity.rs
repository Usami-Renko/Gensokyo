
use gltf;

use vk::utils::types::vkint;
use vk::resources::buffer::BufferStorageType;
use vk::resources::command::{ HaCommandRecorder, CmdBufferBindingInfo };
use vk::pipeline::shader::{ VertexInputDescription, HaVertexInputAttribute, VertexInputRate, HaVertexInputBinding };
use vk::utils::types::vkformat;
use vk::utils::format::VKFormat;
use vk::resources::error::AllocatorError;

use assets::model::{ GltfResources, GltfRawData };
use assets::model::GltfScene;
use assets::model::GltfHierarchyAbstract;
use assets::model::{ ModelLoadingErr, ModelGltfLoadingError };

use resources::buffer::{ HaIndexBlock, HaVertexBlock };
use resources::repository::HaBufferRepository;
use toolkit::AllocatorKit;

use std::path::Path;

#[derive(Default)]
pub struct GltfEntity {

    _scenes: Vec<GltfScene>,
    resources: GltfResources,

    allo_res: Option<AllocateResource>,
}

impl GltfEntity {

    pub(crate) fn load(path: impl AsRef<Path>) -> Result<GltfEntity, ModelLoadingErr> {

        let mut resources = GltfResources::default();

        let (document, buffers, images) = gltf::import(path)
            .map_err(|e| ModelLoadingErr::Gltf(ModelGltfLoadingError::Gltf(e)))?;
        let raw_data = GltfRawData {
            document, buffers, images,
        };

        let mut scenes = vec![];
        for raw_scene in raw_data.document.scenes() {
            let scene = GltfScene::from_hierarchy(raw_scene, &mut resources, &raw_data)
                .map_err(|e| ModelLoadingErr::Gltf(e))?;
            scenes.push(scene);
        }

        let entity = GltfEntity {
            _scenes: scenes, resources, allo_res: None,
        };

        Ok(entity)
    }

    pub fn config_buffer(&mut self, kit: &AllocatorKit, storage: BufferStorageType) -> Result<(), AllocatorError> {

        let mut allocator = kit.buffer(storage);

        let mut vertex_indices = vec![];
        let mut index_indices  = vec![];

        // create a buffer for each primitive.
        for mesh in self.resources.meshes.iter() {
            for primitive in mesh.primitives.iter() {

                let block_info = primitive.block_info();
                let vertex_index = allocator.append_buffer(block_info)?;
                vertex_indices.push(vertex_index);

                let index_info = primitive.index_info();
                let index_index = allocator.append_buffer(index_info)?;
                index_indices.push(index_index);
            }
        }

        let distributor = allocator.allocate()?;

        let mut vertex_buffers = vec![];
        let mut index_buffers = vec![];
        let mut index_counts = vec![];

        for vertex in vertex_indices.into_iter() {
            let vertex_buffer = distributor.acquire_vertex(vertex);
            vertex_buffers.push(vertex_buffer);
        }
        for index in index_indices.into_iter() {
            let index_buffer = distributor.acquire_index(index);
            index_buffers.push(index_buffer);
        }

        let mut repository = distributor.into_repository();

        {
            let mut uploader = repository.data_uploader()?;

            let mut buffer_index = 0;
            for mesh in self.resources.meshes.iter() {
                for primitive in mesh.primitives.iter() {

                    primitive.upload_vertex_data(&vertex_buffers[buffer_index], &mut uploader)?;
                    primitive.upload_index_data(&index_buffers[buffer_index], &mut uploader)?;

                    index_counts.push(primitive.index_count());

                    buffer_index += 1;
                }
            }

            uploader.done()?;
        }

        let res = AllocateResource {
            vertexs: vertex_buffers,
            indices: index_buffers,
            index_counts,
            repository,
        };
        self.allo_res = Some(res);

        Ok(())
    }

    pub fn record_command(&self, recorder: &HaCommandRecorder) {

        // TODO: handle unwrap().
        let res = self.allo_res.as_ref().unwrap();

        let element_count = res.vertexs.len();
        for i in 0..element_count {

            let vertex_buffer = &res.vertexs[i];
            let index_buffer = &res.indices[i];

            recorder
                .bind_vertex_buffers(0,&[CmdBufferBindingInfo { block: vertex_buffer, sub_block_index: None }])
                .bind_index_buffer(CmdBufferBindingInfo { block: index_buffer, sub_block_index: None })
                .draw_indexed(res.index_counts[i] as vkint, 1, 0, 0, 0);
        }
    }

    pub fn cleanup(&mut self) {

        if let Some(ref mut res) = self.allo_res {
            res.repository.cleanup();
        }
    }

    pub fn vertex_desc(&self) -> VertexInputDescription {
        Vertex::desc()
    }
}

struct AllocateResource {

    vertexs: Vec<HaVertexBlock>,
    indices: Vec<HaIndexBlock>,
    index_counts: Vec<usize>,
    repository: HaBufferRepository,
}


// TODO: Remove the following codes.
define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec3]
        position : [f32; 3],
    }
}
