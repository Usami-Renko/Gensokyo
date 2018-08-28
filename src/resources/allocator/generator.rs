
use core::physical::HaPhysicalDevice;
use core::device::HaLogicalDevice;

use resources::allocator::buffer::HaBufferAllocator;

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
}
