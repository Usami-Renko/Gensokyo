
use ash;
use ash::version::V1_0;
use ash::version::DeviceV1_0;

use core::device::queue::QueueInfo;

use constant::VERBOSE;

pub struct HaLogicalDevice {

    pub handle: ash::Device<V1_0>,
    queues: Vec<QueueInfo>,

    pub graphics_queue_index: Option<usize>,
    pub present_queue_index:  Option<usize>,
}

impl HaLogicalDevice {

    pub fn new(handle: ash::Device<V1_0>, queues: Vec<QueueInfo>, graphics_queue_index: Option<usize>, present_queue_index:  Option<usize>) -> HaLogicalDevice {
        HaLogicalDevice {
            handle,
            queues,
            graphics_queue_index,
            present_queue_index,
        }
    }

    pub fn graphics_queue(&self) -> Option<&QueueInfo> {
        Some(&self.queues[self.graphics_queue_index?])
    }

    pub fn present_queue(&self) -> Option<&QueueInfo> {
        Some(&self.queues[self.present_queue_index?])
    }

    pub fn cleanup(&self) {

        unsafe {
            self.handle.destroy_device(None);

            if VERBOSE {
                println!("[Info] Logical Device had been destroy.");
            }
        }
    }
}
