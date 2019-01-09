
use crate::assets::gltf::importer::{ GltfRenderEntity, GltfDataEntity };
use crate::assets::gltf::traits::{ GsGltfHierachy, GltfHierachyInstance };
use crate::assets::gltf::scene::{ GsGltfScene, GltfSceneIndex };
use crate::assets::gltf::material::{ GsGltfMaterial, GsGltfTexture, GsGltfSampler };

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::GsBufferAllocator;
use gsvk::buffer::instance::GsUniformBlock;
use gsvk::memory::transfer::{ GsBufferDataUploader, BufferUploadDst };
use gsvk::memory::transfer::{ GsBufferDataUpdater, BufferUpdateDst };
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;

// ------------------------------------------------------------------------------------
pub(super) struct GltfRawDataAgency {
    pub doc: gltf::Document,
    pub data_buffer: Vec<gltf::buffer::Data>,
    pub data_image : Vec<gltf::image::Data>,
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
#[derive(Default)]
pub struct GltfShareResource {

    pub(crate) materials: Vec<GsGltfMaterial>,
    pub(crate)textures : Vec<GsGltfTexture>,
    pub(crate)samplers : Vec<GsGltfSampler>,
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GsGltfEntity {

    data  : GltfDataEntity,
    render: GltfRenderEntity,
}

impl GsGltfEntity {

    pub fn new(render: GltfRenderEntity, data: GltfDataEntity) -> GsGltfEntity {
        GsGltfEntity { data, render }
    }

    pub fn record_command(&self, recorder: &GsCommandRecorder) {

        self.data.scene.record_command(recorder);
    }

    pub fn uniform_ref(&self) -> &GsUniformBlock {
        &self.render.pbr_uniform
    }
}

impl BufferUploadDst<GsGltfStorage> for GsGltfEntity {

    fn upload_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUploader, &GsGltfStorage) -> Result<(), AllocatorError>> {

        let func = |gltf_entity: &GsGltfEntity, uploader: &mut GsBufferDataUploader, data: &GsGltfStorage| {
            gltf_entity.data.scene.upload(uploader, &data.scene)
        };
        Box::new(func)
    }
}

impl BufferUpdateDst for GsGltfEntity {

    fn update_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUpdater) -> Result<(), AllocatorError>> {

        let func = |gltf_entity: &GsGltfEntity, updater: &mut GsBufferDataUpdater| {
            gltf_entity.data.scene.update_uniform(updater, &gltf_entity.render.pbr_uniform, &gltf_entity.data.share_res)
        };
        Box::new(func)
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GsGltfStorage {

    scene: GsGltfScene,
}

impl GsGltfStorage {

    pub(super) fn new(scene: GsGltfScene) -> GsGltfStorage {
        GsGltfStorage { scene }
    }

    pub(super) fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<GltfSceneIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {
        self.scene.allocate(allocator)
    }

    pub fn apply_transform(&mut self) {
        self.scene.apply_transform(&());
    }
}
// ------------------------------------------------------------------------------------
