
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;
use gsvk::prelude::common::*;
use gsvk::prelude::api::*;
use gs::prelude::*;

use std::path::Path;

const ARMOR_MODEL_PATH     : &'static str = "models/armor.gltf";
const STOREFLOOR_MODEL_PATH: &'static str = "models/plane.gltf";

const ARMOR_COLOR_TEXTURE_PATH  : &'static str = "textures/color_astc_8x8_unorm.png";
const ARMOR_NORMAL_TEXTURE_PATH : &'static str = "textures/normal_astc_8x8_unorm.png";

const STONEFLOOR_COLOR_TEXTURE_PATH  : &'static str = "textures/stonefloor02_color_astc_8x8_unorm.png";
const STONEFLOOR_NORMAL_TEXTURE_PATH : &'static str = "textures/stonefloor01_normal_astc_8x8_unorm.png";

pub struct ModelResource {

    pub model: GsglTFEntity,
    pub color_map : GsCombinedImgSampler,
    pub normal_map: GsCombinedImgSampler,
}

struct ModelTextureTmp {
    color  : GsCombinedImgSampler,
    normal : GsCombinedImgSampler,
}

pub struct ModelAsset {

    pub armor: ModelResource,
    pub storefloor: ModelResource,

    pub buffer_repository : GsBufferRepository<Device>,
    pub uniform_repository: GsBufferRepository<Host>,
    pub image_repository  : GsImageRepository<Device>,
}

pub fn load_models(initializer: &AssetInitializer) -> GsResult<ModelAsset> {

    let (armor_model, storefloor_model, buffer_repo, uniform_repo) = load_gltfs(initializer)?;
    let (armor_texture, storefloor_texture, image_repo) = load_images(initializer)?;

    let result = ModelAsset {
        armor: ModelResource {
            model: armor_model,
            color_map : armor_texture.color,
            normal_map: armor_texture.normal,
        },
        storefloor: ModelResource {
            model: storefloor_model,
            color_map : storefloor_texture.color,
            normal_map: storefloor_texture.normal,
        },
        buffer_repository  : buffer_repo,
        uniform_repository : uniform_repo,
        image_repository   : image_repo,
    };
    Ok(result)
}

pub fn load_gltfs(initializer: &AssetInitializer) -> GsResult<(GsglTFEntity, GsglTFEntity, GsBufferRepository<Device>, GsBufferRepository<Host>)> {

    let mut buffer_allocator = GsBufferAllocator::new(initializer, BufferStorageType::DEVICE);
    let mut uniform_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);

    let gltf_importer = GsglTFImporter::new(initializer);

    let (mut armor_entity, armor_data) = gltf_importer.load(Path::new(ARMOR_MODEL_PATH))?;
    let armor_vertex_index  = buffer_allocator.assign_v2(&armor_data.vertex_allot_delegate())?;
    let armor_unifrom_index = uniform_allocator.assign_v2(&armor_data.uniform_allot_delegate(0))?;

    let (mut storefloor_entity, storefloor_data) = gltf_importer.load(Path::new(STOREFLOOR_MODEL_PATH))?;
    let storefloor_vertex_index  = buffer_allocator.assign_v2(&storefloor_data.vertex_allot_delegate())?;
    let storefloor_uniform_index = uniform_allocator.assign_v2(&storefloor_data.uniform_allot_delegate(0))?;

    let buffer_distributor = buffer_allocator.allocate()?;
    let uniform_distributor = uniform_allocator.allocate()?;
    armor_entity.acquire_vertex(armor_vertex_index, &buffer_distributor);
    armor_entity.acquire_uniform(armor_unifrom_index, &uniform_distributor);

    storefloor_entity.acquire_vertex(storefloor_vertex_index, &buffer_distributor);
    storefloor_entity.acquire_uniform(storefloor_uniform_index, &uniform_distributor);

    let mut buffer_repository = buffer_distributor.into_repository();
    let mut uniform_repository = uniform_distributor.into_repository();

    buffer_repository.data_uploader()?
        .upload_v2(&armor_entity.vertex_upload_delegate().unwrap(), &armor_data)?
        .upload_v2(&storefloor_entity.vertex_upload_delegate().unwrap(), &armor_data)?
        .finish()?;

    let mut uniform_repository = uniform_distributor.into_repository();
    uniform_repository.data_uploader()?
        .upload_v2(&armor_entity.uniform_upload_delegate().unwrap(), &storefloor_data)?
        .upload_v2(&storefloor_entity.uniform_upload_delegate().unwrap(), &storefloor_data)?
        .finish()?;

    Ok((armor_entity, storefloor_entity, buffer_repository, uniform_repository))
}

fn load_images(initializer: &AssetInitializer) -> GsResult<(ModelTextureTmp, ModelTextureTmp, GsImageRepository<Device>)> {

    let image_loader = ImageLoader::new(initializer);

    let mut allocator = GsImageAllocator::new(initializer);

    // load armor texture.
    // color texture
    let armor_color_index = assign_sampled_image(&image_loader, &mut allocator, 0, ARMOR_COLOR_TEXTURE_PATH)?;
    // normal texture.
    let armor_normal_index = assign_sampled_image(&image_loader, &mut allocator, 0, ARMOR_NORMAL_TEXTURE_PATH)?;

    // load store floor texture.
    // color texture.
    let store_color_index = assign_sampled_image(&image_loader, &mut allocator, 0, STONEFLOOR_COLOR_TEXTURE_PATH)?;
    // normal texture.
    let store_normal_index = assign_sampled_image(&image_loader, &mut allocator, 0, STONEFLOOR_NORMAL_TEXTURE_PATH)?;

    let image_distributor = allocator.allocate()?;

    let armor_texture = ModelTextureTmp {
        color : image_distributor.acquire(armor_color_index),
        normal: image_distributor.acquire(armor_normal_index),
    };

    let store_floor_texture = ModelTextureTmp {
        color : image_distributor.acquire(store_color_index),
        normal: image_distributor.acquire(store_normal_index),
    };

    let image_repository = image_distributor.into_repository();

    Ok((armor_texture, store_floor_texture, image_repository))
}

fn assign_sampled_image(loader: &ImageLoader, image_allocator: &mut GsImageAllocator<Device>, binding: vkuint, path: &'static str) -> GsResult<GsAssignIndex<ICombinedImg>> {

    let texture_data = loader.load_2d(Path::new(path), GsImageFormat::default())?;
    let sampled_ci = GsCombinedImgSampler::new(binding, texture_data, ImagePipelineStage::FragmentStage);
    image_allocator.assign(sampled_ci)
}
