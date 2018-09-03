
use core::physical::HaPhysicalDevice;
use core::device::HaLogicalDevice;

use resources::allocator::buffer::HaBufferAllocator;
use resources::allocator::descriptor::HaDescriptorAllocator;
use resources::descriptor::DescriptorPoolFlag;

pub struct ResourceGenerator<'re> {

    physical: &'re HaPhysicalDevice,
    device  : &'re HaLogicalDevice,
}

impl<'re> ResourceGenerator<'re> {

    pub fn init(physical: &'re HaPhysicalDevice, device: &'re HaLogicalDevice) -> ResourceGenerator<'re> {
        ResourceGenerator {
            physical,
            device,
        }
    }

    pub fn buffer_allocator(&self) -> HaBufferAllocator {
        HaBufferAllocator::new(self.physical, self.device)
    }

    pub fn descriptor_allocator(&self, flags: &[DescriptorPoolFlag]) -> HaDescriptorAllocator {
        HaDescriptorAllocator::new(self.device, flags)
    }
}
