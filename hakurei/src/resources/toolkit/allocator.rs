
use config::resources::ResourceConfig;
use core::physical::HaPhyDevice;
use core::device::HaDevice;
use core::swapchain::HaSwapchain;

use resources::allocator::{ HaBufferAllocator, BufferStorageType };
use resources::allocator::HaDescriptorAllocator;
use resources::allocator::{ HaImagePreAllocator, ImageStorageType };
use resources::descriptor::DescriptorPoolFlag;

use utility::model::ModelObjLoader;
use utility::dimension::Dimension2D;

pub struct AllocatorKit {

    physical: HaPhyDevice,
    device  : HaDevice,

    dimension: Dimension2D,
    config: ResourceConfig,
}

impl AllocatorKit {

    pub(crate) fn init(physical: &HaPhyDevice, device: &HaDevice, swapchain: &HaSwapchain, config: ResourceConfig) -> AllocatorKit {
        AllocatorKit {
            physical: physical.clone(),
            device  : device.clone(),

            dimension: swapchain.extent,
            config,
        }
    }

    pub fn buffer(&self, ty: BufferStorageType) -> HaBufferAllocator {
        HaBufferAllocator::new(&self.physical, &self.device, ty)
    }

    pub fn descriptor(&self, flags: &[DescriptorPoolFlag]) -> HaDescriptorAllocator {
        HaDescriptorAllocator::new(&self.device, flags)
    }

    pub fn image(&self, ty: ImageStorageType) -> HaImagePreAllocator {
        HaImagePreAllocator::new(&self.physical, &self.device, ty, self.config.image_load.clone())
    }

    pub fn swapchain_dimension(&self) -> Dimension2D {
        self.dimension
    }

    pub fn obj_loader(&self) -> ModelObjLoader {
        ModelObjLoader::new()
    }
}
