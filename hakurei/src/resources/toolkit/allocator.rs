
use core::physical::HaPhyDevice;
use core::device::HaDevice;
use core::swapchain::HaSwapchain;

use resources::allocator::{ HaBufferAllocator, BufferStorageType };
use resources::allocator::HaDescriptorAllocator;
use resources::allocator::{ HaImageAllocator, ImageStorageType };
use resources::descriptor::DescriptorPoolFlag;

use utility::dimension::Dimension2D;

pub struct AllocatorKit {

    physical: HaPhyDevice,
    device  : HaDevice,

    dimension: Dimension2D,
}

impl AllocatorKit {

    pub fn init(physical: &HaPhyDevice, device: &HaDevice, swapchain: &HaSwapchain) -> AllocatorKit {
        AllocatorKit {
            physical: physical.clone(),
            device  : device.clone(),

            dimension: swapchain.extent,
        }
    }

    pub fn buffer(&self, ty: BufferStorageType) -> HaBufferAllocator {
        HaBufferAllocator::new(&self.physical, &self.device, ty)
    }

    pub fn descriptor(&self, flags: &[DescriptorPoolFlag]) -> HaDescriptorAllocator {
        HaDescriptorAllocator::new(&self.device, flags)
    }

    pub fn image(&self, ty: ImageStorageType) -> HaImageAllocator {
        HaImageAllocator::new(&self.physical, &self.device, ty)
    }

    pub fn swapchain_dimension(&self) -> Dimension2D {
        self.dimension
    }
}
