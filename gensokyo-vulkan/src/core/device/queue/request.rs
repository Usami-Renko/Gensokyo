
use ash::vk;
use ash::version::DeviceV1_0;


use crate::core::physical::GsPhysicalDevice;
use crate::core::device::enums::{ DeviceQueueIndex, PrefabQueuePriority };
use crate::core::device::queue::target::{ GsQueue, QueueUsage, QueueInitInfo };

use crate::types::vkuint;
use crate::error::{ VkResult, VkError };

pub trait QueueRequester {

    /// Request a new virtual queue in Device creation, return its reference index.
    fn request_queue(&mut self, usage: QueueUsage, priority: PrefabQueuePriority) -> DeviceQueueIndex;
    /// Check if device support current requested queues and generate physical queues based on current requested virtual queues.
    fn inspect_queue_availability(&mut self, physical: &GsPhysicalDevice) -> VkResult<()>;
    /// Generate `QueueInitInfo` based on current requested queues information.
    fn to_queue_infos(&self) -> Vec<QueueInitInfo>;
    /// Get the handle of Queue from Vulkan, and collect all virtual queues into `GsQueue`.
    fn collect_queues(&self, device: &ash::Device) -> Vec<GsQueue>;
    /// Print the information of requested queues.
    fn print_message(&self);
}

struct PhysicalQueue {

    priority: PrefabQueuePriority,
    family_index: vkuint,
    queue_index : vkuint,
}

struct VirtualQueue {

    usage: QueueUsage,
    phy_index: Option<usize>, // this field is no use in SFSQ and SFMQ strategy.
    priority: PrefabQueuePriority,
}


/// Single queue in a specific queue family.
pub struct SFSQ {

    phy_queue : Option<PhysicalQueue>,
    vir_queues: Vec<VirtualQueue>,

    queue_priority: PrefabQueuePriority,
}

impl SFSQ {

    pub fn new(priority: PrefabQueuePriority) -> SFSQ {

        SFSQ {
            phy_queue: None,
            vir_queues: vec![],
            queue_priority: priority,
        }
    }
}

impl QueueRequester for SFSQ {

    fn request_queue(&mut self, usage: QueueUsage, priority: PrefabQueuePriority) -> DeviceQueueIndex {

        let new_queue = VirtualQueue {
            usage, priority,
            phy_index: None,
        };

        let reference_index = self.vir_queues.len();
        self.vir_queues.push(new_queue);

        DeviceQueueIndex(reference_index)
    }

    fn inspect_queue_availability(&mut self, physical: &GsPhysicalDevice) -> VkResult<()> {

        let optimal_family = select_optimal_queue_family(physical, &self.vir_queues,
            |family_index, requested_capability| {
                physical.families.is_queue_support_capability(family_index, requested_capability)
            });

        if let Some(family_index) = optimal_family {

            let physical_queue = PhysicalQueue {
                priority: self.queue_priority,
                family_index,
                queue_index: 0, // request the first queue in queue family.
            };

            self.phy_queue = Some(physical_queue);

            Ok(())

        } else {
            Err(VkError::unsupported("Required Queue Operations"))
        }
    }

    // `inspect_queue_available()` method must be call before using `to_queue_infos()` function."
    fn to_queue_infos(&self) -> Vec<QueueInitInfo> {

        // TODO: unwrap() not handle.
        let phy_queue = self.phy_queue.as_ref().unwrap();

        let result = QueueInitInfo {
            family_index: phy_queue.family_index,
            priorities: vec![phy_queue.priority.value()],
        };
        vec![result]
    }

    // `inspect_queue_available()` method must be call before using `collect_queues()` function.
    fn collect_queues(&self, device: &ash::Device) -> Vec<GsQueue> {

        // TODO: unwrap() not handle.
        let phy_queue = self.phy_queue.as_ref().unwrap();
        let unique_phy_queue = unsafe {
            device.get_device_queue(phy_queue.family_index, phy_queue.queue_index)
        };

        self.vir_queues.iter().map(|virtual_queue|
            GsQueue::new(unique_phy_queue, virtual_queue.usage, phy_queue.family_index, phy_queue.queue_index)
        ).collect()
    }

    fn print_message(&self) {

        if let Some(ref phy_queue) = self.phy_queue {

            println!("[Info] Single Family - Single Queue Strategy.");
            println!("[Info] Generate Physical Queue:");
            println!("\tfamily index | queue count | priority");
            println!("\t{:12} | {:11} | {:?}", phy_queue.family_index, 1, phy_queue.priority);

            println!("[Info] Generate Virtual Queues: {}", self.vir_queues.len());
            println!("\tphysical index | usage");

            for virtual_queue in self.vir_queues.iter() {
                println!("\t{:11?} | {:?}", virtual_queue.phy_index, virtual_queue.usage);
            }
            println!();
        } else {
            println!("Physical queue has not yet generated.");
        }
    }
}

/// Multiple queues in a specific queue family.
pub struct SFMQ {

    phy_queues: Vec<PhysicalQueue>,
    vir_queues: Vec<VirtualQueue>,
}

impl SFMQ {

    pub fn new() -> SFMQ {

        SFMQ {
            phy_queues: vec![],
            vir_queues: vec![],
        }
    }
}

impl QueueRequester for SFMQ {

    fn request_queue(&mut self, usage: QueueUsage, priority: PrefabQueuePriority) -> DeviceQueueIndex {

        let new_queue = VirtualQueue {
            usage, priority,
            phy_index: None,
        };

        let reference_index = self.vir_queues.len();
        self.vir_queues.push(new_queue);

        DeviceQueueIndex(reference_index)
    }

    fn inspect_queue_availability(&mut self, physical: &GsPhysicalDevice) -> VkResult<()> {

        let optimal_family = select_optimal_queue_family(physical, &self.vir_queues,
            |family_index, requested_capability| {
                physical.families.is_queue_support_capability(family_index, requested_capability) &&
                    physical.families.is_queue_count_enough(family_index, self.vir_queues.len())
            });

        if let Some(family_index) = optimal_family {

            self.phy_queues = self.vir_queues.iter_mut().enumerate().map(|(index, virtual_queue)| {

                virtual_queue.phy_index = Some(index);

                PhysicalQueue {
                    priority: virtual_queue.priority,
                    family_index,
                    queue_index: index as _,
                }
            }).collect();

            Ok(())
        } else {
            Err(VkError::unsupported("Required Queue Operations"))
        }
    }

    // `inspect_queue_available()` method must be call before using `to_queue_infos()` function."
    fn to_queue_infos(&self) -> Vec<QueueInitInfo> {

        let priorities = self.phy_queues.iter()
            .map(|phy_queue| phy_queue.priority.value()).collect();

        let result = QueueInitInfo {
            family_index: self.phy_queues[0].family_index,
            priorities,
        };
        vec![result]
    }

    // `inspect_queue_available()` method must be call before using `collect_queues()` function.
    fn collect_queues(&self, device: &ash::Device) -> Vec<GsQueue> {

        let queue_handles: Vec<vk::Queue> = unsafe {
            self.phy_queues.iter().map(|phy_queue| {
                device.get_device_queue(phy_queue.family_index, phy_queue.queue_index)
            }).collect()
        };

        queue_handles.into_iter().enumerate().map(|(i, handle)|
            GsQueue::new(handle, self.vir_queues[i].usage, self.phy_queues[i].family_index, self.phy_queues[i].queue_index)
        ).collect()
    }

    fn print_message(&self) {

        if self.phy_queues.is_empty() {

            println!("Physical queue has not yet generated.");
        } else {

            println!("[Info] Single Family - Multi Queues Strategy.");
            println!("[Info] Generate Physical Queue:");
            println!("\tfamily index | queue index | priority");
            for phy_queue in self.phy_queues.iter() {
                println!("\t{:12} | {:11} | {:?}", phy_queue.family_index, phy_queue.queue_index, phy_queue.priority);
            }

            println!("[Info] Generate Virtual Queue: {}", self.vir_queues.len());
            println!("\tphysical index | usage");
            for virtual_queue in self.vir_queues.iter() {
                println!("\t{:12?} | {:?}", virtual_queue.phy_index, virtual_queue.usage);
            }
        }
    }
}

fn select_optimal_queue_family(physical: &GsPhysicalDevice, vir_queues: &Vec<VirtualQueue>, test_func: impl Fn(vkuint, vk::QueueFlags) -> bool) -> Option<u32> {

    let candidate_indices = if physical.families.family_indices.is_share_same_family {

        vec![physical.families.family_indices.graphics_index]
    } else {

        vec![
            physical.families.family_indices.graphics_index,
            physical.families.family_indices.present_index,
            physical.families.family_indices.transfer_index,
        ]
    };

    let requested_queue_capability = vir_queues.iter().fold(
        vk::QueueFlags::empty(),
        |sum, virtual_queue| {
            sum | virtual_queue.usage.vk_flag()
        });

    // Search and select an queue family support both Graphics and Transfer operation.
    candidate_indices.into_iter().find(|&family_index| {
        test_func(family_index, requested_queue_capability)
    })
}
