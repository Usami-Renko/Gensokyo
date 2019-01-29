
use ash::vk;

use gsvk::core::GsDevice;

use gsvk::buffer::allocator::GsBufferAllocator;
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;

use gsvk::image::allocator::GsImageAllocator;
use gsvk::image::allocator::types::ImageMemoryTypeAbs;

use gsvk::descriptor::allocator::GsDescriptorAllocator;

use crate::config::resources::ResourceConfig;
use crate::assets::glTF::importer::GsglTFImporter;
use crate::assets::io::ImageLoader;

pub struct AllocatorKit {

    device  : GsDevice,

    config: ResourceConfig,
}

impl AllocatorKit {

    pub(crate) fn init(device: &GsDevice, config: ResourceConfig) -> AllocatorKit {

        AllocatorKit {
            device: device.clone(),
            config,
        }
    }

    pub fn buffer<B: BufferMemoryTypeAbs>(&self, typ: B) -> GsBufferAllocator<B> {
        GsBufferAllocator::new(&self.device, typ)
    }

    pub fn descriptor(&self, flags: vk::DescriptorPoolCreateFlags) -> GsDescriptorAllocator {
        GsDescriptorAllocator::new(&self.device, flags)
    }

    pub fn image<I: ImageMemoryTypeAbs>(&self, typ: I) -> GsImageAllocator<I> {
        GsImageAllocator::new(&self.device, typ)
    }

    pub fn image_loader(&self) -> ImageLoader {
        ImageLoader::new(self.config.image_load.clone())
    }

    pub fn gltf_loader<'a, 's: 'a>(&'s self) -> GsglTFImporter<'a> {
        GsglTFImporter { device: &self.device }
    }
}
