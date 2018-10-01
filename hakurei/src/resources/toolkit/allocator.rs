
use core::physical::HaPhyDevice;
use core::device::HaDevice;

use resources::allocator::{ HaBufferAllocator, BufferStorageType };
use resources::allocator::HaDescriptorAllocator;
use resources::allocator::HaImageAllocator;
use resources::memory::HaMemoryType;
use resources::descriptor::DescriptorPoolFlag;

pub struct AllocatorKit {

    physical: HaPhyDevice,
    device  : HaDevice,
}

impl AllocatorKit {

    pub fn init(physical: &HaPhyDevice, device: &HaDevice) -> AllocatorKit {
        AllocatorKit {
            physical: physical.clone(),
            device  : device.clone(),
        }
    }

    pub fn buffer(&self, ty: BufferStorageType) -> HaBufferAllocator {
        HaBufferAllocator::new(&self.physical, &self.device, ty)
    }

    pub fn descriptor(&self, flags: &[DescriptorPoolFlag]) -> HaDescriptorAllocator {
        HaDescriptorAllocator::new(&self.device, flags)
    }

    pub fn image(&self) -> HaImageAllocator {
        // TODO: Currently only work for Device memory
        HaImageAllocator::new(&self.physical, &self.device, HaMemoryType::DeviceMemory)
    }
}
