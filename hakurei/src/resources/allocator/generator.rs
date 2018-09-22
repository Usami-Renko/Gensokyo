
use core::physical::HaPhysicalDevice;
use core::device::HaLogicalDevice;

use resources::allocator::buffer::{ HaHostBufferAllocator, HaDeviceBufferAllocator };
use resources::allocator::descriptor::HaDescriptorAllocator;
use resources::allocator::image::HaImageAllocator;
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

    pub fn host_buffer(&self) -> HaHostBufferAllocator {
        HaHostBufferAllocator::new(self.physical, self.device)
    }

    pub fn device_buffer(&self) -> HaDeviceBufferAllocator {
        HaDeviceBufferAllocator::new(self.physical, self.device)
    }

    pub fn descriptor(&self, flags: &[DescriptorPoolFlag]) -> HaDescriptorAllocator {
        HaDescriptorAllocator::new(self.device, flags)
    }

    pub fn image(&self) -> HaImageAllocator {
        HaImageAllocator::new(self.physical, self.device)
    }
}
