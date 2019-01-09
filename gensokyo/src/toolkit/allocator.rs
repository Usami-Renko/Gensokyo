
use ash::vk;

use gsvk::core::physical::GsPhyDevice;
use gsvk::core::device::GsDevice;
use gsvk::core::swapchain::GsChain;

use gsvk::buffer::allocator::GsBufferAllocator;
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;

use gsvk::image::allocator::GsImageAllocator;
use gsvk::image::allocator::types::ImageMemoryTypeAbs;

use gsvk::descriptor::allocator::GsDescriptorAllocator;
use gsvk::types::vkDim2D;

use crate::config::resources::ResourceConfig;
use crate::assets::io::ImageLoader;

pub struct AllocatorKit {

    physical: GsPhyDevice,
    device  : GsDevice,

    dimension: vkDim2D,
    config: ResourceConfig,
}

impl AllocatorKit {

    pub(crate) fn init(physical: &GsPhyDevice, device: &GsDevice, swapchain: &GsChain, config: ResourceConfig) -> AllocatorKit {

        AllocatorKit {
            physical: physical.clone(),
            device  : device.clone(),

            dimension: swapchain.extent(),
            config,
        }
    }

    pub fn buffer<B: BufferMemoryTypeAbs>(&self, typ: B) -> GsBufferAllocator<B> {
        GsBufferAllocator::new(&self.physical, &self.device, typ)
    }

    pub fn descriptor(&self, flags: vk::DescriptorPoolCreateFlags) -> GsDescriptorAllocator {
        GsDescriptorAllocator::new(&self.device, flags)
    }

    pub fn image<I: ImageMemoryTypeAbs>(&self, typ: I) -> GsImageAllocator<I> {
        GsImageAllocator::new(&self.physical, &self.device, typ)
    }

    pub fn swapchain_dimension(&self) -> vkDim2D {
        self.dimension
    }

    pub fn image_loader(&self) -> ImageLoader {
        ImageLoader::new(self.config.image_load.clone())
    }
}
