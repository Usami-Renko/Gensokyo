
use gsvk::buffer::allocator::GsBufferAllocator;
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;

use gsvk::image::allocator::GsImageAllocator;
use gsvk::image::allocator::types::ImageMemoryTypeAbs;

use gsvk::descriptor::allocator::GsDescriptorAllocator;
use gsvk::image::{ GsSampler, SamplerCIBuilder };

use crate::assets::glTF::importer::GsglTFImporter;
use crate::assets::io::ImageLoader;

use crate::initialize::initializer::AssetInitializer;
use crate::initialize::traits::{ FromInitializer, FromInitializerP1 };

impl<B> FromInitializerP1<B> for GsBufferAllocator<B>
    where
        B: BufferMemoryTypeAbs {

    fn new(initializer: &AssetInitializer, typ: B) -> GsBufferAllocator<B> {
        GsBufferAllocator::create(&initializer.device, typ)
    }
}

impl FromInitializer for GsDescriptorAllocator {

    fn new(initializer: &AssetInitializer) -> GsDescriptorAllocator {
        GsDescriptorAllocator::create(&initializer.device)
    }
}

impl<I> FromInitializerP1<I> for GsImageAllocator<I>
    where
        I: ImageMemoryTypeAbs {

    fn new(initializer: &AssetInitializer, typ: I) -> GsImageAllocator<I> {
        GsImageAllocator::create(&initializer.device, typ)
    }
}

impl FromInitializer for ImageLoader {

    fn new(initializer: &AssetInitializer) -> ImageLoader {
        ImageLoader::from(initializer.config.image_load.clone())
    }
}

impl FromInitializer for GsglTFImporter {

    fn new(initializer: &AssetInitializer) -> GsglTFImporter {
        GsglTFImporter { device: initializer.device.clone() }
    }
}

impl FromInitializer<SamplerCIBuilder> for GsSampler {

    fn new(initializer: &AssetInitializer) -> SamplerCIBuilder {
        GsSampler::construct(&initializer.device)
    }
}
