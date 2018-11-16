
use config::resources::ResourceConfig;

use vk::core::physical::HaPhyDevice;
use vk::core::device::HaDevice;
use vk::core::swapchain::HaSwapchain;

use vk::resources::buffer::BufferStorageType;
use vk::resources::image::ImageStorageType;
use vk::resources::descriptor::DescriptorPoolFlag;
use vk::utils::types::vkDimension2D;

use resources::allocator::buffer::HaBufferAllocator;
use resources::allocator::descriptor::HaDescriptorAllocator;
use resources::allocator::image::HaImageAllocator;

use assets::model::ModelGltfLoader;

pub struct AllocatorKit {

    physical: HaPhyDevice,
    device  : HaDevice,

    dimension: vkDimension2D,
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

    pub fn buffer(&self, ty: BufferStorageType) -> HaBufferAllocator {
        HaBufferAllocator::new(&self.physical, &self.device, ty)
    }

    pub fn descriptor(&self, flags: &[DescriptorPoolFlag]) -> HaDescriptorAllocator {
        HaDescriptorAllocator::new(&self.device, flags)
    }

    pub fn image(&self, ty: ImageStorageType) -> HaImageAllocator {
        HaImageAllocator::new(&self.physical, &self.device, ty, self.config.image_load.clone())
    }

    pub fn swapchain_dimension(&self) -> vkDimension2D {
        self.dimension
    }

    pub fn gltf_loader(&self) -> ModelGltfLoader {
        ModelGltfLoader::new()
    }
}
