
use ash::vk;

use config::resources::ResourceConfig;

use gsvk::core::physical::HaPhyDevice;
use gsvk::core::device::HaDevice;
use gsvk::core::swapchain::HaSwapchain;

use gsvk::buffer::allocator::HaBufferAllocator;
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;

use gsvk::image::allocator::HaImageAllocator;
use gsvk::image::allocator::types::ImageMemoryTypeAbs;

use gsvk::descriptor::allocator::HaDescriptorAllocator;

use gsvk::types::vkDim2D;

use assets::model::ModelGltfLoader;

pub struct AllocatorKit {

    physical: HaPhyDevice,
    device  : HaDevice,

    dimension: vkDim2D,
    config: ResourceConfig,
}

impl AllocatorKit {

    pub(crate) fn init(physical: &HaPhyDevice, device: &HaDevice, swapchain: &HaSwapchain, config: ResourceConfig) -> AllocatorKit {

        AllocatorKit {
            physical: physical.clone(),
            device  : device.clone(),

            dimension: swapchain.extent(),
            config,
        }
    }

    pub fn buffer<B: BufferMemoryTypeAbs + Copy>(&self, typ: B) -> HaBufferAllocator<B> {
        HaBufferAllocator::new(&self.physical, &self.device, typ)
    }

    pub fn descriptor(&self, flags: vk::DescriptorPoolCreateFlags) -> HaDescriptorAllocator {
        HaDescriptorAllocator::new(&self.device, flags)
    }

    pub fn image<I: ImageMemoryTypeAbs  + Copy>(&self, typ: I) -> HaImageAllocator<I> {
        HaImageAllocator::new(&self.physical, &self.device, typ)
    }

    pub fn swapchain_dimension(&self) -> vkDim2D {
        self.dimension
    }

    pub fn gltf_loader(&self) -> ModelGltfLoader {
        ModelGltfLoader::new()
    }
}
